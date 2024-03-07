pub struct Mmu {
    pub entry: u64,
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu { entry: 0 }
    }
}

pub struct State {
    pub gp_regs: [u64; 32],
    pub pc: u64,
}

impl State {
    pub fn new() -> State {
        State {
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
}
