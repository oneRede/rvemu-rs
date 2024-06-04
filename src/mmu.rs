use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    mem::size_of,
    os::{fd::FromRawFd, raw::c_void}, slice,
};

use libc::{mmap, munmap, MAP_ANONYMOUS, MAP_FIXED, MAP_PRIVATE};

use crate::{
    elfdef::{
        Ehdr, Phdr, EI_CLASS, ELFCLASS64, ELFMAG, EM_RISCV, PF_R, PF_W, PF_X, PROT_EXEC, PROT_READ,
        PROT_WRITE, PT_LOAD,
    },
    fatal, max, round_down, round_up,
    rvemu::{get_ptr, Mmu},
    to_guest, to_host,
};

pub fn load_phdr(phdr: &mut Phdr, ehdr: &Ehdr, i: i64, file: &mut File) {
    let size_phdr = size_of::<Phdr>();
    let seek = SeekFrom::Start(ehdr.e_phoff + ((ehdr.e_phentsize as i64) * i) as u64);
    if let Err(_) = file.seek(seek) {
        fatal!("seek file failed");
    }

    let mut buf = unsafe { slice::from_raw_parts_mut(phdr as *mut Phdr as *mut u8, size_phdr) };
    if let Err(_) = file.read(&mut buf) {
        fatal!("file too small");
    }
}

pub fn flags_to_mmap_prot(flags: i32) -> i32 {
    let r = if (flags & PF_R) != 0 { PROT_READ } else { 0 };
    let w = if (flags & PF_W) != 0 { PROT_WRITE } else { 0 };
    let x = if (flags & PF_X) != 0 { PROT_EXEC } else { 0 };
    return r | w | x;
}

pub fn mmu_load_segment(mmu: &mut Mmu, phdr: Phdr, fd: i32) {
    let page_size = page_size::get();
    let vaddr: u64 = to_host!(phdr.p_vaddr);
    let aligned_vaddr: u64 = round_down!(vaddr, page_size);
    let filesz = phdr.p_memsz + vaddr - aligned_vaddr;
    let memsz = phdr.p_memsz + vaddr - aligned_vaddr;
    let prot = flags_to_mmap_prot(phdr.p_flags as i32);

    let ptr_align = get_ptr(aligned_vaddr);
    let ptr = unsafe {
        mmap(
            ptr_align as *mut c_void,
            filesz.try_into().unwrap(),
            prot,
            MAP_PRIVATE | MAP_FIXED,
            fd,
            round_down!(phdr.p_offset, page_size) as i64,
        )
    };
    assert_eq!(ptr as u64, aligned_vaddr);

    let remianing_bss = round_up!(memsz, page_size) - round_up!(filesz, page_size);
    if remianing_bss > 0 {
        let ptr_align = get_ptr(aligned_vaddr + round_up!(filesz, page_size));
        let ptr = unsafe {
            mmap(
                ptr_align as *mut c_void,
                remianing_bss as usize,
                prot,
                MAP_ANONYMOUS | MAP_PRIVATE | MAP_FIXED,
                -1,
                0,
            )
        };
        assert_eq!(ptr as u64, aligned_vaddr + round_up!(filesz, page_size));
    }

    mmu.host_alloc = max!(mmu.host_alloc, aligned_vaddr + round_up!(memsz, page_size));
    mmu.alloc = to_guest!(mmu.host_alloc);
    mmu.base = mmu.alloc;
}

pub fn mmu_load_elf(mut mmu: &mut Mmu, fd: i32) {
    const SIZE_EHDR: usize = size_of::<Ehdr>();
    let mut buf: [u8; SIZE_EHDR] = [0; 64];
    let mut file = unsafe { File::from_raw_fd(fd) };

    let rs = file.read(&mut buf[..]);
    if rs.unwrap() != SIZE_EHDR {
        fatal!("file too small");
    }

    let ehdr: Ehdr = unsafe { std::ptr::read(buf.as_ptr() as *const Ehdr) };
    if !ELFMAG.eq(&buf[..4]) {
        fatal!("bad elf file")
    }

    if ehdr.e_machine != EM_RISCV || ehdr.e_ident[EI_CLASS] != ELFCLASS64 {
        fatal!("only riscv64 elf file is supported");
    }

    mmu.entry = ehdr.e_entry;

    let mut phdr: Phdr = Phdr::new();
    for i in 0..ehdr.e_phnum {
        load_phdr(&mut phdr, &ehdr, i.into(), &mut file);
        if phdr.p_type == PT_LOAD {
            mmu_load_segment(&mut mmu, phdr, fd);
        }
    }
}

pub fn mmu_alloc(mmu: &mut Mmu, sz: i64) -> u64 {
    let pz = page_size::get();
    let base = mmu.alloc;
    assert!(base >= mmu.base);

    mmu.alloc += sz as u64;
    assert!(mmu.alloc >= mmu.base);
    if sz > 0 && mmu.alloc > to_guest!(mmu.host_alloc) {
        let ptr = get_ptr(mmu.host_alloc);
        if unsafe {
            mmap(
                ptr as *mut c_void,
                round_up!(sz, pz) as usize,
                (PROT_READ | PROT_WRITE) as i32,
                MAP_ANONYMOUS | MAP_PRIVATE,
                -1i32,
                0,
            )
        }
        .is_null()
        {
            fatal!("mmap failed!")
        }
        mmu.host_alloc += round_up!(sz, pz);
    } else if sz < 0 && round_up!(mmu.alloc, pz) < to_guest!(mmu.host_alloc) {
        let len = to_guest!(mmu.host_alloc) - round_up!(mmu.alloc, pz);
        let ptr = get_ptr(mmu.alloc);
        if unsafe { munmap(ptr as *mut c_void, len as usize) } == -1 {
            fatal!("munmap failed!")
        }
        mmu.host_alloc -= len;
    }
    return base;
}
