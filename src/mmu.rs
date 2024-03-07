use std::{fs::File, io::Read, mem::size_of, os::fd::FromRawFd};

use crate::{
    elfdef::{Ehdr, EI_CLASS, ELFCLASS64, ELFMAG, EM_RISCV},
    rvemu::{fatal, Mmu},
};

pub fn mmu_load_elf(mmu: &mut Mmu, fd: i32) {
    let mut buf: [u8; size_of::<Ehdr>()] = [0; 64];

    let mut file = unsafe { File::from_raw_fd(fd) };

    let rs = file.read(&mut buf[..]);
    if rs.unwrap() != size_of::<Ehdr>() {
        fatal("file too small");
    }

    let ehdr: Ehdr = unsafe { std::ptr::read(buf.as_ptr() as *const Ehdr) };
    let elf_h = unsafe { *(buf[..4].as_ptr() as *const u32) };
    let elf_mag = unsafe { *((ELFMAG).as_ptr() as *const u32) };

    if elf_h != elf_mag {
        fatal("bad elf file")
    }

    if ehdr.e_machine != EM_RISCV || ehdr.e_ident[EI_CLASS] != ELFCLASS64 {
        fatal("only riscv64 elf file is supported");
    }

    mmu.entry = ehdr.e_entry;
}
