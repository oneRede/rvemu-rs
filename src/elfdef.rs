pub const EI_NIDENT: usize = 16;
pub const ELFMAG: &[u8; 4] = b"\x7fELF";

pub const EM_RISCV: u16 = 243;

pub const EI_CLASS: usize = 4;
pub const ELFCLASSNONE: u64 = 0;
pub const ELFCLASS32: u64 = 1;
pub const ELFCLASS64: u8 = 2;
pub const ELFCLASSNUM: u64 = 3;

pub const PT_LOAD: u64 = 1;

pub const PF_X: u64 = 0x1;
pub const PF_W: u64 = 0x2;
pub const PF_R: u64 = 0x4;

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
