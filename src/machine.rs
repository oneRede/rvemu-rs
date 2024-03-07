use std::{fs::File, os::fd::AsRawFd};

use crate::{mmu::mmu_load_elf, rvemu::{fatal, Machine}};

pub fn machine_load_program(m: &mut Machine, prog: &str) {
    let file = File::open(prog).unwrap();
    let fd = file.as_raw_fd();

    if fd == -1{
        fatal("wrong fd num!!")
    }

    mmu_load_elf(&mut m.mmu, fd);

    m.state.pc = m.mmu.entry;
}