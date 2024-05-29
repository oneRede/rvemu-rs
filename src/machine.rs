use std::{fs::OpenOptions, os::fd::AsRawFd};

use crate::{
    interp::exec_block_interp,
    mmu::{mmu_alloc, mmu_load_elf},
    reg::GpRegTypeT,
    rvemu::{mmu_write, ExitReason, Machine},
};

pub fn machine_step(m: &mut Machine) -> ExitReason {
    loop {
        m.state.exit_reason = ExitReason::None;
        exec_block_interp(&mut m.state);
        assert!(m.state.exit_reason != ExitReason::None);

        if m.state.exit_reason == ExitReason::IndirectBranch
            || m.state.exit_reason == ExitReason::DirectBranch
        {
            m.state.pc = m.state.reenter_pc;
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

pub fn machine_setup(m: &mut Machine, argc: i32, argv: Vec<String>) {
    let sz = 32 * 1024 * 1024;
    let stack = mmu_alloc(&mut m.mmu, sz);
    m.state.gp_regs[GpRegTypeT::Sp as usize] = stack + sz as u64;

    m.state.gp_regs[GpRegTypeT::Sp as usize] -= 8;
    m.state.gp_regs[GpRegTypeT::Sp as usize] -= 8;
    m.state.gp_regs[GpRegTypeT::Sp as usize] -= 8;

    let args: u64 = argc as u64 - 1;
    let mut i: usize = args as usize;
    while i > 0 {
        let len = argv[i].len();
        let addr = mmu_alloc(&mut m.mmu, (len + 1) as i64);
        mmu_write(addr, argv[i].as_ptr(), len);
        m.state.gp_regs[GpRegTypeT::Sp as usize] -= 8;
        let ap = (&addr) as *const u64 as *const u8;
        mmu_write(m.state.gp_regs[GpRegTypeT::Sp as usize], ap, 8);
        i -= 1;
    }

    m.state.gp_regs[GpRegTypeT::Sp as usize] -= 8;
    let ap: *const u8 = (&args) as *const u64 as *const u8;
    mmu_write(m.state.gp_regs[GpRegTypeT::Sp as usize], ap, 8);
}