use std::env;

use crate::{machine::machine_load_program, rvemu::Machine};

pub mod elfdef;
pub mod rvemu;
pub mod mmu;
pub mod machine;

fn main() {
    let args:Vec<String> = env::args().collect();
    assert_eq!(args.len()> 1, true);

    let mut machine = Machine::new();
    machine_load_program(&mut machine, &args[1]);

    println!("entry: {:x}\n", machine.mmu.entry)
}