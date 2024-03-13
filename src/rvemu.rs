use std::process::exit;

#[derive(Clone, Copy)]
pub enum InsnType {
    InsnAddi,
    NumInsns,
}

#[derive(Clone, Copy)]
pub struct Insn {
    pub rd: i8,
    pub rs1: i8,
    pub rs2: i8,
    pub imm: i32,
    pub i_type: InsnType,
    pub rvc: bool,
    pub cont: bool,
}

impl Insn {
    pub fn new() -> Insn {
        Insn {
            rd: 0,
            rs1: 0,
            rs2: 0,
            imm: 0,
            i_type: InsnType::InsnAddi,
            rvc: false,
            cont: false,
        }
    }
}

pub struct Mmu {
    pub entry: u64,
    pub host_alloc: u64,
    pub alloc: u64,
    pub base: u64,
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            entry: 0,
            host_alloc: 0,
            alloc: 0,
            base: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ExitReasonT {
    None,
    DirectBranch,
    IndirectBranch,
    Ecall,
}

#[derive(Clone, Copy)]
pub struct State {
    pub exit_reason: ExitReasonT,
    pub gp_regs: [u64; 32],
    pub pc: u64,
}

impl State {
    pub fn new() -> State {
        State {
            exit_reason: ExitReasonT::None,
            gp_regs: [0; 32],
            pc: 0,
        }
    }
}

pub struct Machine {
    pub state: State,
    pub mmu: Mmu,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            state: State::new(),
            mmu: Mmu::new(),
        }
    }
}

pub fn fatal(msg: &str) {
    println!("fatal: {}:{} {}", file!(), line!(), msg);
    exit(1)
}
