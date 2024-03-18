use std::{fs::OpenOptions, os::fd::AsRawFd};

use crate::{
    interp::exec_block_interp,
    mmu::mmu_load_elf,
    rvemu::{ExitReason, Machine},
};

pub fn machine_step(m: Machine) -> ExitReason {
    loop {
        exec_block_interp(m.state);

        if m.state.exit_reason == ExitReason::IndirectBranch
            || m.state.exit_reason == ExitReason::DirectBranch
        {
            continue;
        }
        break;
    }
    assert!(m.state.exit_reason == ExitReason::Ecall);
    return ExitReason::Ecall;
}

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
