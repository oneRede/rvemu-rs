use std::env;

use sys_call::{init_sys_call, init_sys_call_table};

use crate::{
    machine::{machine_load_program, machine_setup, machine_step},
    reg::GpRegTypeT,
    rvemu::{machine_get_gp_reg, machine_set_gp_reg, ExitReason, Machine},
    sys_call::do_syscall,
};

pub mod decode;
pub mod elfdef;
pub mod interp;
pub mod interp_utils;
pub mod machine;
pub mod mmu;
pub mod reg;
pub mod rvemu;
pub mod sys_call;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len() > 1, true);

    let mut machine = Machine::new();
    machine_load_program(&mut machine, &args[1]);
    machine_setup(&mut machine, args.len() as i32, args);
    init_sys_call();
    init_sys_call_table();

    loop {
        let reason = machine_step(&mut machine);
        assert!(reason == ExitReason::Ecall);
        let sys_call = machine_get_gp_reg(machine, GpRegTypeT::A7 as i32);
        let ret = do_syscall(&mut machine, sys_call);
        machine_set_gp_reg(&mut machine, GpRegTypeT::A0 as i32, ret);
    }
}
