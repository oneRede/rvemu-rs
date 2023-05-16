// const EI_NIDENT: u16 = 16;
// const ELFMAG: &str = "\\177ELF";

// const EM_RISCV: u16 = 243;

// const EI_CLASS: u8 = 4;
// const ELFCLASSNONE: u8 = 0;
// const ELFCLASS32: u8 = 1;
// const ELFCLASS64: u8 = 2;
// const ELFCLASSNUM: u8 = 3;

// const PT_LOAD: u8 = 1;

// const PF_X: u16 = 0x1;
// const PF_W: u16 = 0x2;
// const PF_R: u16 = 0x4;

// const R_X86_64_PC32: u8 = 2;

// struct ELF64_EHDR_T {
//     e_ident: [u8; EI_NIDENT],
//     e_type: u16,
//     e_machine: u16,
//     e_version: u32,
//     e_entry: u64,
//     e_phoff: u64,
//     e_shoff: u64,
//     e_flags: u32,
//     e_ehsize: u16,
//     e_phentsize: u16,
//     e_phnum: u16,
//     e_shentsize: u16,
//     e_shnum: u16,
//     e_shstrndx: u16,
// }

// struct ELF64_PHDR_T {
//     p_type: u32,
//     p_flags: u32,
//     p_offset: u64,
//     p_vaddr: u64,
//     p_paddr: u64,
//     p_filesz: u64,
//     p_memsz: u64,
//     p_align: u64,
// }

// struct ELF64_SHDR_T {
//     sh_name: u32,
//     sh_type: u32,
//     sh_flags: u32,
//     sh_addr: u32,
//     sh_offset: u64,
//     sh_size: u64,
//     sh_link: u32,
//     sh_info: u32,
//     sh_addralign: u64,
//     sh_entsize: u64,
// }

// struct ELF64_SYM_T {
//     st_name: u32,
//     st_info: u8,
//     st_other: u8,
//     st_shndx: u16,
//     st_value: u64,
//     st_size: u64,
// }

// struct ELF64_RELA_T {
//     r_offset: u64,
//     r_type: u32,
//     r_sym: u32,
//     r_addend: i64,
// }
