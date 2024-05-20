use std::env;

use crate::{
    machine::{machine_load_program, machine_step},
    rvemu::{ExitReason, Machine},
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

    loop {
        let reason = machine_step(machine);
        assert!(reason == ExitReason::Ecall);
    }
}
