use std::ptr;

use crate::{
    decode::insn_decode,
    reg::GpRegTypeT,
    rvemu::{Insn, State},
    to_host,
};

pub fn func_empty(_state: State, _insn: Insn) {}

#[macro_export]
macro_rules! func {
    ($ty:ty) => {
        unimplemented!()
    };
}

pub fn func_lb(_state: State, _insn: Insn) {
    func!(i8)
}

pub fn func_lh(_state: State, _insn: Insn) {
    func!(i16)
}

pub fn func_lw(_state: State, _insn: Insn) {
    func!(i32)
}

pub fn func_ld(_state: State, _insn: Insn) {
    func!(i64)
}

pub fn func_lbu(_state: State, _insn: Insn) {
    func!(u8)
}

pub fn func_lhu(_state: State, _insn: Insn) {
    func!(u8)
}

pub fn func_lwu(_state: State, _insn: Insn) {
    func!(u8)
}

#[macro_export]
macro_rules! func1 {
    ($expr:expr) => {
        unimplemented!()
    };
}

pub fn func_addi(_state: State, _insn: Insn) {
    func1!(rs1 + imm)
}

pub fn func_slli(_state: State, _insn: Insn) {
    func1!(rs1 << (imm & 0x3f))
}

pub fn func_slti(_state: State, _insn: Insn) {
    func1!((rs1 as i64) < (imm as i64))
}

pub fn func_sltiu(_state: State, _insn: Insn) {
    func1!((rs1 as u64) < (imm as u64))
}

pub fn func_xori(_state: State, _insn: Insn) {
    func1!(rs1 ^ imm)
}

pub fn func_srli(_state: State, _insn: Insn) {
    func1!(rs1 >> (imm & 0x3f))
}

pub fn func_srai(_state: State, _insn: Insn) {
    func1!(rs1 as i64 >> (imm & 0x3f))
}

pub fn func_ori(_state: State, _insn: Insn) {
    func1!(rs1 | (imm as i64))
}

pub fn func_andi(_state: State, _insn: Insn) {
    func1!(rs1 | (imm as u64))
}

pub fn func_andiw(_state: State, _insn: Insn) {
    func1!((rs1 + imm) as i64)
}

pub fn func_slliw(_state: State, _insn: Insn) {
    func1!((rs1 << (imm & 0x1f)) as i64)
}

pub fn func_srliw(_state: State, _insn: Insn) {
    func1!(((rs1 as u32) >> (imm & 0x1f)) as i64)
}

pub fn func_sraiw(_state: State, _insn: Insn) {
    func1!(((rs1 as i32) >> (imm & 0x1f)) as i64)
}

pub fn func_auipc(state: &mut State, insn: Insn) {
    let val = state.pc + insn.imm as u64;
    state.gp_regs[insn.rd as usize] = val;
}

#[macro_export]
macro_rules! func2 {
    ($ty:ty) => {
        unimplemented!()
    };
}

pub fn func_sb(_state: State, _insn: Insn) {
    func2!(u8)
}

pub fn func_sh(_state: State, _insn: Insn) {
    func2!(u16)
}

pub fn func_sw(_state: State, _insn: Insn) {
    func2!(u32)
}

pub fn func_sd(_state: State, _insn: Insn) {
    func2!(u64)
}

#[macro_export]
macro_rules! func3 {
    ($expr:expr) => {
        unimplemented!()
    };
}

pub fn func_add(_state: State, _insn: Insn) {
    func3!(rs1 + rs2)
}

pub fn func_sll(_state: State, _insn: Insn) {
    func3!(rs1 << (rs2 & 0x3f))
}

pub fn func_slt(_state: State, _insn: Insn) {
    func3!((rs1 as i64) < (rs2 as i64))
}

pub fn func_sltu(_state: State, _insn: Insn) {
    func3!((rs1 as u64) < (rs2 as u64))
}

pub fn func_xor(_state: State, _insn: Insn) {
    func3!(rs1 ^ rs2)
}

pub fn func_srl(_state: State, _insn: Insn) {
    func3!(rs1 >> (rs2 & 0x3f))
}

pub fn func_or(_state: State, _insn: Insn) {
    func3!(rs1 | rs2)
}

pub fn func_and(_state: State, _insn: Insn) {
    func3!(rs1 & rs2)
}

pub fn func_mul(_state: State, _insn: Insn) {
    func3!(rs1 * rs2)
}

pub fn func_mulh(_state: State, _insn: Insn) {
    func3!(mulh(rs1, rs2))
}

pub fn func_mulhsu(_state: State, _insn: Insn) {
    func3!(mulhsu(rs1, rs2))
}

pub fn func_mulhu(_state: State, _insn: Insn) {
    func3!(mulhu(rs1, rs2))
}

pub fn func_sub(_state: State, _insn: Insn) {
    func3!(rs1 - rs2)
}

pub fn func_sra(_state: State, _insn: Insn) {
    func3!((rs1 as i64) >> (rs2 & 0x3f))
}

pub fn func_remu(_state: State, _insn: Insn) {
    func3!(unimplemented!())
}

pub fn func_addw(_state: State, _insn: Insn) {
    func3!((rs1 + rs2) as i64)
}

pub fn func_sllw(_state: State, _insn: Insn) {
    func3!((rs1 << (rs2 & 0x1f)) as i64)
}

pub fn func_srlw(_state: State, _insn: Insn) {
    func3!(((rs1 as u32) >> (rs2 & 0x1f)) as i64)
}

pub fn func_mulw(_state: State, _insn: Insn) {
    func3!((rs1 * rs2) as i64)
}

pub fn func_divw(_state: State, _insn: Insn) {
    func3!(unimplemented!())
}

pub fn func_divuw(_state: State, _insn: Insn) {
    func3!(unimplemented!())
}

pub fn func_remw(_state: State, _insn: Insn) {
    func3!(unimplemented!())
}

pub fn func_remuw(_state: State, _insn: Insn) {
    func3!(unimplemented!())
}

pub fn func_subw(_state: State, _insn: Insn) {
    func3!((rs1 - rs2) as i64)
}

pub fn func_sraw(_state: State, _insn: Insn) {
    func3!(((rs1 as i32) >> (rs2 & 0x1f)) as i64)
}

pub fn func_div(state: &mut State, insn: Insn) {
    let rs1 = state.gp_regs[insn.rs1 as usize];
    let rs2 = state.gp_regs[insn.rs2 as usize];
    let mut rd = 0;

    if rs2 == 0 {
        rd = u64::MAX;
    } else if rs1 as i64 == i64::MIN && rs2 == u64::MAX {
        //NOTE: i64::MIN to u64
        rd = i64::MIN as u64
    }
    state.gp_regs[insn.rd as usize] = rd
}

pub fn func_divu(state: &mut State, insn: Insn) {
    let rs1 = state.gp_regs[insn.rs1 as usize];
    let rs2 = state.gp_regs[insn.rs2 as usize];
    let rd;

    if rs2 == 0 {
        rd = u64::MAX;
    } else {
        rd = rs1 / rs2
    }
    state.gp_regs[insn.rd as usize] = rd
}

pub fn func_rem(state: &mut State, insn: Insn) {
    let rs1 = state.gp_regs[insn.rs1 as usize];
    let rs2 = state.gp_regs[insn.rs2 as usize];
    let rd;

    if rs2 == 0 {
        rd = rs1;
    } else if rs1 as i64 == i64::MIN && rs2 == u64::MAX {
        rd = 0
    } else {
        rd = rs1 % rs2;
    }
    state.gp_regs[insn.rd as usize] = rd
}

pub fn func_lui(state: &mut State, insn: Insn){
    state.gp_regs[insn.rd as usize] = insn.imm as u64;
}

pub static FUNCS: [fn(State, Insn); 133] = [
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty, func_empty,
    func_empty, func_empty, func_empty, func_empty, func_empty,
];

pub fn exec_block_interp(mut state: State) {
    let mut insn: Insn = Insn::new();
    loop {
        let data: *const u8 = ptr::null();
        let data: *const u32 = unsafe { data.add(to_host!(state.pc) as usize) } as *const u32;
        insn_decode(&mut insn, *(unsafe { data.as_ref().unwrap() }));

        FUNCS.get(insn.i_type as usize).unwrap()(state, insn);
        state.gp_regs[GpRegTypeT::Zero as usize] = 0;

        if insn.cont {
            break;
        }

        state.pc += if insn.rvc { 2 } else { 4 };
    }
}
