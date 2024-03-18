pub const EI_NIDENT: usize = 16;
pub const ELFMAG: &[u8; 4] = b"\x7fELF";

pub const EM_RISCV: u16 = 243;

pub const EI_CLASS: usize = 4;
pub const ELFCLASSNONE: u64 = 0;
pub const ELFCLASS32: u64 = 1;
pub const ELFCLASS64: u8 = 2;
pub const ELFCLASSNUM: u64 = 3;

pub const PT_LOAD: u32 = 1;

pub const PF_X: i32 = 0x1;
pub const PF_W: i32 = 0x2;
pub const PF_R: i32 = 0x4;

pub const PROT_NONE: u32 = 0x00;
pub const PROT_READ: u32 = 0x01;
pub const PROT_WRITE: u32 = 0x02;
pub const PROT_EXEC: u32 = 0x04;

#[macro_export]
macro_rules! round_down {
    ($x:expr, $k:expr) => {
        (($x as i64) & -($k as i64)) as u64
    };
}

#[macro_export]
macro_rules! round_up {
    ($x:expr, $k:expr) => {
        ((($x as i64) + ($k as i64) - 1) & -($k as i64)) as u64
    };
}

#[macro_export]
macro_rules! min {
    ($x:expr, $y:expr) => {
        if $y > $x {
            $x
        } else {
            $y
        }
    };
}
#[macro_export]
macro_rules! max {
    ($x:expr, $y:expr) => {
        if $y < $x {
            $x
        } else {
            $y
        }
    };
}

#[macro_export]
macro_rules! to_host {
    ($addr:expr) => {
        $addr + 0x088800000000u64
    };
}

#[macro_export]
macro_rules! to_guest {
    ($addr:expr) => {
        $addr - 0x088800000000u64
    };
}

#[repr(C)]
pub struct Ehdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsiz: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl Phdr {
    pub fn new() -> Phdr {
        Phdr {
            p_type: 0,
            p_flags: 0,
            p_offset: 0,
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: 0,
            p_memsz: 0,
            p_align: 0,
        }
    }
}
