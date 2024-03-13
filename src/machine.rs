use std::{fs::OpenOptions, os::fd::AsRawFd};

use crate::{
    mmu::mmu_load_elf,
    rvemu::Machine,
};

pub fn machine_load_program(m: &mut Machine, prog: &str) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(prog)
        .expect("open file failed!!");
    
    let fd = file.as_raw_fd();

    mmu_load_elf(&mut m.mmu, fd);

    m.state.pc = m.mmu.entry;
}
