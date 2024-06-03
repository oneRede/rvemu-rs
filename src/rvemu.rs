use crate::{
    reg::{FpRegT, FpRegTypeT, GpRegTypeT},
    to_host,
};
use std::ptr;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum InsnType {
    InsnLb,
    InsnLh,
    InsnLw,
    InsnLd,
    InsnLbu,
    InsnLhu,
    InsnLwu,
    InsnFence,
    InsnFenceI,
    InsnAddi,
    InsnSlli,
    InsnSlti,
    InsnSltiu,
    InsnXori,
    InsnSrli,
    InsnSrai,
    InsnOri,
    InsnAndi,
    InsnAuipc,
    InsnAddiw,
    InsnSlliw,
    InsnSrliw,
    InsnSraiw,
    InsnSb,
    InsnSh,
    InsnSw,
    InsnSd,
    InsnAdd,
    InsnSll,
    InsnSlt,
    InsnSltu,
    InsnXor,
    InsnSrl,
    InsnOr,
    InsnAnd,
    InsnMul,
    InsnMulh,
    InsnMulhsu,
    InsnMulhu,
    InsnDiv,
    InsnDivu,
    InsnRem,
    InsnRemu,
    InsnSub,
    InsnSra,
    InsnLui,
    InsnAddw,
    InsnSllw,
    InsnSrlw,
    InsnMulw,
    InsnDivw,
    InsnDivuw,
    InsnRemw,
    InsnRemuw,
    InsnSubw,
    InsnSraw,
    InsnBeq,
    InsnBne,
    InsnBlt,
    InsnBge,
    InsnBltu,
    InsnBgeu,
    InsnJalr,
    InsnJal,
    InsnEcall,
    InsnCsrrc,
    InsnCsrrci,
    InsnCsrrs,
    InsnCsrrsi,
    InsnCsrrw,
    InsnCsrrwi,
    InsnFlw,
    InsnFsw,
    InsnFmaddS,
    InsnFmsubS,
    InsnFnmsubS,
    InsnFnmaddS,
    InsnFaddS,
    InsnFsubS,
    InsnFmulS,
    InsnFdivS,
    InsnFsqrtS,
    InsnFsgnjS,
    InsnFsgnjnS,
    InsnFsgnjxS,
    InsnFminS,
    InsnFmaxS,
    InsnFcvtWS,
    InsnFcvtWuS,
    InsnFmvXW,
    InsnFeqS,
    InsnFltS,
    InsnFleS,
    InsnFclassS,
    InsnFcvtSW,
    InsnFcvtSWu,
    InsnFmvWX,
    InsnFcvtLS,
    InsnFcvtLuS,
    InsnFcvtSL,
    InsnFcvtSLu,
    InsnFld,
    InsnFsd,
    InsnFmaddD,
    InsnFmsubD,
    InsnFnmsubD,
    InsnFnmaddD,
    InsnFaddD,
    InsnFsubD,
    InsnFmulD,
    InsnFdivD,
    InsnFsqrtD,
    InsnFsgnjD,
    InsnFsgnjnD,
    InsnFsgnjxD,
    InsnFminD,
    InsnFmaxD,
    InsnFcvtSD,
    InsnFcvtDS,
    InsnFeqD,
    InsnFltD,
    InsnFleD,
    InsnFclassD,
    InsnFcvtWD,
    InsnFcvtWuD,
    InsnFcvtDW,
    InsnFcvtDWu,
    InsnFcvtLD,
    InsnFcvtLuD,
    InsnFmvXD,
    InsnFcvtDL,
    InsnFcvtDLu,
    InsnFmvDX,
    NumInsns,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Insn {
    pub rd: i8,
    pub rs1: i8,
    pub rs2: i8,
    pub rs3: i8,
    pub imm: i32,
    pub csr: i16,
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
            rs3: 0,
            imm: 0,
            csr: 0,
            i_type: InsnType::InsnAddi,
            rvc: false,
            cont: false,
        }
    }
}

#[derive(Clone, Copy)]
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

#[derive(Debug)]
#[derive(Clone, Copy, PartialEq)]
pub enum ExitReason {
    None,
    DirectBranch,
    IndirectBranch,
    Ecall,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Csr {
    Fflags = 0x001,
    Frm = 0x002,
    Fcsr = 0x003,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct State {
    pub exit_reason: ExitReason,
    pub gp_regs: [u64; GpRegTypeT::NumGpRegS as usize],
    pub pc: u64,
    pub reenter_pc: u64,
    pub fp_regs: [FpRegT; FpRegTypeT::NumFpRegs as usize],
}

impl State {
    pub fn new() -> State {
        State {
            exit_reason: ExitReason::None,
            gp_regs: [0; GpRegTypeT::NumGpRegS as usize],
            pc: 0,
            reenter_pc: 0,
            fp_regs: [FpRegT::new(); FpRegTypeT::NumFpRegs as usize],
        }
    }
}

#[derive(Clone, Copy)]
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

#[macro_export]
macro_rules! fatal {
    ($msg: expr) => {
        println!("fatal: {}:{} {}", file!(), line!(), $msg)
    };
}

pub fn mmu_write(addr: u64, data: *const u8, len: usize) {
    let n_ptr: *mut u8 = ptr::null_mut();
    let n_ptr = unsafe { n_ptr.add(to_host!(addr) as usize) };
    unsafe { n_ptr.copy_from(data, len) }
}

pub fn machine_get_gp_reg(m: Machine, reg: i32) -> u64 {
    assert!(reg >= 0 && reg <= GpRegTypeT::NumGpRegS as i32);
    return m.state.gp_regs[reg as usize];
}

pub fn machine_set_gp_reg(m: &mut Machine, reg: i32, data: u64) {
    assert!(reg >= 0 && reg <= GpRegTypeT::NumGpRegS as i32);
    m.state.gp_regs[reg as usize] = data;
}
