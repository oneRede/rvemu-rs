use std::ptr;

use crate::{
    decode::insn_decode,
    interp_utils::{f32_classify, f64_classify},
    reg::GpRegTypeT,
    rvemu::{ExitReason, Insn, State},
    to_host,
};

pub fn func_empty(_state: &mut State, _insn: Insn) {}

#[macro_export]
macro_rules! func {
    ($ty:ty) => {
        unimplemented!()
    };
}

pub fn func_lb(_state: &mut State, _insn: Insn) {
    func!(i8)
}

pub fn func_lh(_state: &mut State, _insn: Insn) {
    func!(i16)
}

pub fn func_lw(_state: &mut State, _insn: Insn) {
    func!(i32)
}

pub fn func_ld(_state: &mut State, _insn: Insn) {
    func!(i64)
}

pub fn func_lbu(_state: &mut State, _insn: Insn) {
    func!(u8)
}

pub fn func_lhu(_state: &mut State, _insn: Insn) {
    func!(u8)
}

pub fn func_lwu(_state: &mut State, _insn: Insn) {
    func!(u8)
}

#[macro_export]
macro_rules! func1 {
    ($expr:expr) => {
        unimplemented!()
    };
}

pub fn func_addi(_state: &mut State, _insn: Insn) {
    func1!(rs1 + imm)
}

pub fn func_slli(_state: &mut State, _insn: Insn) {
    func1!(rs1 << (imm & 0x3f))
}

pub fn func_slti(_state: &mut State, _insn: Insn) {
    func1!((rs1 as i64) < (imm as i64))
}

pub fn func_sltiu(_state: &mut State, _insn: Insn) {
    func1!((rs1 as u64) < (imm as u64))
}

pub fn func_xori(_state: &mut State, _insn: Insn) {
    func1!(rs1 ^ imm)
}

pub fn func_srli(_state: &mut State, _insn: Insn) {
    func1!(rs1 >> (imm & 0x3f))
}

pub fn func_srai(_state: &mut State, _insn: Insn) {
    func1!(rs1 as i64 >> (imm & 0x3f))
}

pub fn func_ori(_state: &mut State, _insn: Insn) {
    func1!(rs1 | (imm as i64))
}

pub fn func_andi(_state: &mut State, _insn: Insn) {
    func1!(rs1 | (imm as u64))
}

pub fn func_addiw(_state: &mut State, _insn: Insn) {
    func1!((rs1 + imm) as i64)
}

pub fn func_slliw(_state: &mut State, _insn: Insn) {
    func1!((rs1 << (imm & 0x1f)) as i64)
}

pub fn func_srliw(_state: &mut State, _insn: Insn) {
    func1!(((rs1 as u32) >> (imm & 0x1f)) as i64)
}

pub fn func_sraiw(_state: &mut State, _insn: Insn) {
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

pub fn func_sb(_state: &mut State, _insn: Insn) {
    func2!(u8)
}

pub fn func_sh(_state: &mut State, _insn: Insn) {
    func2!(u16)
}

pub fn func_sw(_state: &mut State, _insn: Insn) {
    func2!(u32)
}

pub fn func_sd(_state: &mut State, _insn: Insn) {
    func2!(u64)
}

#[macro_export]
macro_rules! func3 {
    ($expr:expr) => {
        unimplemented!()
    };
}

pub fn func_add(_state: &mut State, _insn: Insn) {
    func3!(rs1 + rs2)
}

pub fn func_sll(_state: &mut State, _insn: Insn) {
    func3!(rs1 << (rs2 & 0x3f))
}

pub fn func_slt(_state: &mut State, _insn: Insn) {
    func3!((rs1 as i64) < (rs2 as i64))
}

pub fn func_sltu(_state: &mut State, _insn: Insn) {
    func3!((rs1 as u64) < (rs2 as u64))
}

pub fn func_xor(_state: &mut State, _insn: Insn) {
    func3!(rs1 ^ rs2)
}

pub fn func_srl(_state: &mut State, _insn: Insn) {
    func3!(rs1 >> (rs2 & 0x3f))
}

pub fn func_or(_state: &mut State, _insn: Insn) {
    func3!(rs1 | rs2)
}

pub fn func_and(_state: &mut State, _insn: Insn) {
    func3!(rs1 & rs2)
}

pub fn func_mul(_state: &mut State, _insn: Insn) {
    func3!(rs1 * rs2)
}

pub fn func_mulh(_state: &mut State, _insn: Insn) {
    func3!(mulh(rs1, rs2))
}

pub fn func_mulhsu(_state: &mut State, _insn: Insn) {
    func3!(mulhsu(rs1, rs2))
}

pub fn func_mulhu(_state: &mut State, _insn: Insn) {
    func3!(mulhu(rs1, rs2))
}

pub fn func_sub(_state: &mut State, _insn: Insn) {
    func3!(rs1 - rs2)
}

pub fn func_sra(_state: &mut State, _insn: Insn) {
    func3!((rs1 as i64) >> (rs2 & 0x3f))
}

pub fn func_remu(_state: &mut State, _insn: Insn) {
    func3!(unimplemented!())
}

pub fn func_addw(_state: &mut State, _insn: Insn) {
    func3!((rs1 + rs2) as i64)
}

pub fn func_sllw(_state: &mut State, _insn: Insn) {
    func3!((rs1 << (rs2 & 0x1f)) as i64)
}

pub fn func_srlw(_state: &mut State, _insn: Insn) {
    func3!(((rs1 as u32) >> (rs2 & 0x1f)) as i64)
}

pub fn func_mulw(_state: &mut State, _insn: Insn) {
    func3!((rs1 * rs2) as i64)
}

pub fn func_divw(_state: &mut State, _insn: Insn) {
    func3!(unimplemented!())
}

pub fn func_divuw(_state: &mut State, _insn: Insn) {
    func3!(unimplemented!())
}

pub fn func_remw(_state: &mut State, _insn: Insn) {
    func3!(unimplemented!())
}

pub fn func_remuw(_state: &mut State, _insn: Insn) {
    func3!(unimplemented!())
}

pub fn func_subw(_state: &mut State, _insn: Insn) {
    func3!((rs1 - rs2) as i64)
}

pub fn func_sraw(_state: &mut State, _insn: Insn) {
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

pub fn func_lui(state: &mut State, insn: Insn) {
    state.gp_regs[insn.rd as usize] = insn.imm as u64;
}

#[macro_export]
macro_rules! func4 {
    ($expr:expr) => {
        unimplemented!()
    };
}

pub fn func_beq(_state: &mut State, _insn: Insn) {
    func4!(rs1 == rs2)
}

pub fn func_bne(_state: &mut State, _insn: Insn) {
    func4!(rs1 != rs2)
}

pub fn func_blt(_state: &mut State, _insn: Insn) {
    func4!(rs1 < rs2)
}

pub fn func_bge(_state: &mut State, _insn: Insn) {
    func4!(rs1 >= rs2)
}

pub fn func_bltu(_state: &mut State, _insn: Insn) {
    func4!(rs1 < rs2)
}

pub fn func_bgeu(_state: &mut State, _insn: Insn) {
    func4!(rs1 >= rs2)
}

pub fn func_jalr(state: &mut State, insn: Insn) {
    let rs1 = state.gp_regs[insn.rs1 as usize];
    state.gp_regs[insn.rd as usize] = state.pc + (if insn.rvc { 2 } else { 4 });
    state.exit_reason = ExitReason::IndirectBranch;
    state.reenter_pc = rs1 + (insn.imm as u64 & !1u64);
}

pub fn func_jal(state: &mut State, insn: Insn) {
    state.gp_regs[insn.rd as usize] = state.pc + (if insn.rvc { 2 } else { 4 });
    state.pc = state.pc + insn.imm as u64;
    state.reenter_pc = state.pc;
    state.exit_reason = ExitReason::IndirectBranch;
}

pub fn func_ecall(state: &mut State, _insn: Insn) {
    state.exit_reason = ExitReason::Ecall;
    state.reenter_pc = state.pc + 4;
}

#[macro_export]
macro_rules! func5 {
    ($expr:expr) => {
        unimplemented!()
    };
}

pub fn func_csrrw(_state: &mut State, _insn: Insn) {
    func5!(unimplemented!())
}

pub fn func_csrrs(_state: &mut State, _insn: Insn) {
    func5!(unimplemented!())
}

pub fn func_csrrc(_state: &mut State, _insn: Insn) {
    func5!(unimplemented!())
}

pub fn func_csrrwi(_state: &mut State, _insn: Insn) {
    func5!(unimplemented!())
}

pub fn func_csrrsi(_state: &mut State, _insn: Insn) {
    func5!(unimplemented!())
}

pub fn func_csrrci(_state: &mut State, _insn: Insn) {
    func5!(unimplemented!())
}

pub fn func_flw(state: &mut State, insn: Insn) {
    let addr = state.gp_regs[insn.rs1 as usize] + insn.imm as u64;
    state.fp_regs[insn.rd as usize].v = to_host!(addr) | (-1i64 << 32) as u64;
}

pub fn func_fld(state: &mut State, insn: Insn) {
    let addr = state.gp_regs[insn.rs1 as usize] + insn.imm as u64;
    state.fp_regs[insn.rd as usize].v = to_host!(addr);
}

#[macro_export]
macro_rules! func6 {
    ($ty:ty) => {
        unimplemented!()
    };
}

pub fn func_fsw(_state: &mut State, _insn: Insn) {
    func6!(unimplemented!())
}

pub fn func_fsd(_state: &mut State, _insn: Insn) {
    func6!(unimplemented!())
}

#[macro_export]
macro_rules! func7 {
    ($expr:expr) => {
        unimplemented!()
    };
}

pub fn func_fmadd_s(_state: &mut State, _insn: Insn) {
    func7!(rs1 * rs2 + rs3)
}

pub fn func_fmsub_s(_state: &mut State, _insn: Insn) {
    func7!(rs1 * rs2 - rs3)
}

pub fn func_fnmsub_s(_state: &mut State, _insn: Insn) {
    func7!(-(rs1 * rs2) + rs3)
}

pub fn func_fnmadd_s(_state: &mut State, _insn: Insn) {
    func7!(-(rs1 * rs2) - rs3)
}

#[macro_export]
macro_rules! func8 {
    ($expr:expr) => {
        unimplemented!()
    };
}

pub fn func_fmadd_d(_state: &mut State, _insn: Insn) {
    func8!(rs1 * rs2 + rs3)
}

pub fn func_fmsub_d(_state: &mut State, _insn: Insn) {
    func8!(rs1 * rs2 - rs3)
}

pub fn func_fnmsub_d(_state: &mut State, _insn: Insn) {
    func8!(-(rs1 * rs2) + rs3)
}

pub fn func_fnmadd_d(_state: &mut State, _insn: Insn) {
    func8!(-(rs1 * rs2) - rs3)
}

#[macro_export]
macro_rules! func9 {
    ($expr:expr) => {
        unimplemented!()
    };
}

pub fn func_fadd_s(_state: &mut State, _insn: Insn) {
    func9!(rs1 + rs2)
}

pub fn func_fsub_s(_state: &mut State, _insn: Insn) {
    func9!(rs1 - rs2)
}

pub fn func_fmul_s(_state: &mut State, _insn: Insn) {
    func9!(rs1 * rs2)
}

pub fn func_fdiv_s(_state: &mut State, _insn: Insn) {
    func9!(rs1 / rs2)
}

pub fn func_fsqrt_s(_state: &mut State, _insn: Insn) {
    func9!(sqrtf(rs1))
}

pub fn func_fmin_s(_state: &mut State, _insn: Insn) {
    func9!(if rs1 < rs2 { rs1 } else { rs2 })
}

pub fn func_fmax_s(_state: &mut State, _insn: Insn) {
    func9!(if rs1 > rs2 { rs1 } else { rs2 })
}

#[macro_export]
macro_rules! func10 {
    ($expr:expr) => {
        unimplemented!()
    };
}

pub fn func_fadd_d(_state: &mut State, _insn: Insn) {
    func10!(rs1 + rs2)
}

pub fn func_fsub_d(_state: &mut State, _insn: Insn) {
    func10!(rs1 - rs2)
}

pub fn func_fmul_d(_state: &mut State, _insn: Insn) {
    func10!(rs1 * rs2)
}

pub fn func_fdiv_d(_state: &mut State, _insn: Insn) {
    func10!(rs1 / rs2)
}

pub fn func_fsqrt_d(_state: &mut State, _insn: Insn) {
    func10!(sqrtf(rs1))
}

pub fn func_fmin_d(_state: &mut State, _insn: Insn) {
    func10!(if rs1 < rs2 { rs1 } else { rs2 })
}

pub fn func_fmax_d(_state: &mut State, _insn: Insn) {
    func10!(if rs1 > rs2 { rs1 } else { rs2 })
}

#[macro_export]
macro_rules! func11 {
    ($n:expr, $x:expr) => {
        unimplemented!()
    };
}

pub fn func_fsgnj_s(_state: &mut State, _insn: Insn) {
    func11!(false, false)
}

pub fn func_fsgnjn_s(_state: &mut State, _insn: Insn) {
    func11!(true, false)
}

pub fn func_fsgnjx_s(_state: &mut State, _insn: Insn) {
    func11!(false, true)
}

#[macro_export]
macro_rules! func12 {
    ($n:expr, $x:expr) => {
        unimplemented!()
    };
}

pub fn func_fsgnj_d(_state: &mut State, _insn: Insn) {
    func11!(false, false)
}

pub fn func_fsgnjn_d(_state: &mut State, _insn: Insn) {
    func11!(true, false)
}

pub fn func_fsgnjx_d(_state: &mut State, _insn: Insn) {
    func11!(false, true)
}

pub fn func_fcvt_w_s(state: &mut State, insn: Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].f as u64;
}

pub fn func_fcvt_wu_s(state: &mut State, insn: Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].f as u64;
}

pub fn func_fcvt_w_d(state: &mut State, insn: Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].d as u64;
}

pub fn func_fcvt_wu_d(state: &mut State, insn: Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].d as u64;
}

pub fn func_fcvt_s_w(state: &mut State, insn: Insn) {
    state.fp_regs[insn.rd as usize].f = state.fp_regs[insn.rs1 as usize].d as f32;
}

pub fn func_fcvt_s_wu(state: &mut State, insn: Insn) {
    state.fp_regs[insn.rd as usize].f = state.fp_regs[insn.rs1 as usize].d as f32;
}

pub fn func_fcvt_d_w(state: &mut State, insn: Insn) {
    state.fp_regs[insn.rd as usize].d = state.fp_regs[insn.rs1 as usize].d as f64;
}

pub fn func_fcvt_d_wu(state: &mut State, insn: Insn) {
    state.fp_regs[insn.rd as usize].d = state.fp_regs[insn.rs1 as usize].d as f64;
}

pub fn func_fmv_x_w(state: &mut State, insn: Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].w as u64;
}

pub fn func_fmv_w_x(state: &mut State, insn: Insn) {
    state.fp_regs[insn.rd as usize].w = state.gp_regs[insn.rs1 as usize] as u32;
}

pub fn func_fmv_x_d(state: &mut State, insn: Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].v as u64;
}

pub fn func_fmv_d_x(state: &mut State, insn: Insn) {
    state.fp_regs[insn.rd as usize].v = state.gp_regs[insn.rs1 as usize] as u64;
}

#[macro_export]
macro_rules! func13 {
    ($expr:expr) => {
        unimplemented!()
    };
}

pub fn func_feq_s(_state: &mut State, _insn: Insn) {
    func10!(rs1 == rs2)
}

pub fn func_flt_s(_state: &mut State, _insn: Insn) {
    func10!(rs1 < rs2)
}

pub fn func_fle_s(_state: &mut State, _insn: Insn) {
    func10!(rs1 <= rs2)
}

#[macro_export]
macro_rules! func14 {
    ($expr:expr) => {
        unimplemented!()
    };
}

pub fn func_feq_d(_state: &mut State, _insn: Insn) {
    func10!(rs1 == rs2)
}

pub fn func_flt_d(_state: &mut State, _insn: Insn) {
    func10!(rs1 < rs2)
}

pub fn func_fle_d(_state: &mut State, _insn: Insn) {
    func10!(rs1 <= rs2)
}

pub fn func_fclass_s(state: &mut State, insn: Insn) {
    state.gp_regs[insn.rd as usize] = f32_classify(state.fp_regs[insn.rs1 as usize].f) as u64;
}

pub fn func_fclass_d(state: &mut State, insn: Insn) {
    state.gp_regs[insn.rd as usize] = f64_classify(state.fp_regs[insn.rs1 as usize].d) as u64;
}

pub fn func_fcvt_l_s(state: &mut State, insn: Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].f as u64;
}

pub fn func_fcvt_lu_s(state: &mut State, insn: Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].f as u64;
}

pub fn func_fcvt_l_d(state: &mut State, insn: Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].d as u64;
}

pub fn func_fcvt_lu_d(state: &mut State, insn: Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].d as u64;
}

pub fn func_fcvt_s_l(state: &mut State, insn: Insn) {
    state.fp_regs[insn.rd as usize].f = state.fp_regs[insn.rs1 as usize].d as f32;
}

pub fn func_fcvt_s_lu(state: &mut State, insn: Insn) {
    state.fp_regs[insn.rd as usize].f = state.fp_regs[insn.rs1 as usize].d as f32;
}

pub fn func_fcvt_d_l(state: &mut State, insn: Insn) {
    state.fp_regs[insn.rd as usize].f = state.fp_regs[insn.rs1 as usize].d as f32;
}

pub fn func_fcvt_d_lu(state: &mut State, insn: Insn) {
    state.fp_regs[insn.rd as usize].f = state.fp_regs[insn.rs1 as usize].d as f32;
}

pub fn func_fcvt_s_d(state: &mut State, insn: Insn) {
    state.fp_regs[insn.rd as usize].f = state.fp_regs[insn.rs1 as usize].d as f32;
}

pub fn func_fcvt_d_s(state: &mut State, insn: Insn) {
    state.fp_regs[insn.rd as usize].f = state.fp_regs[insn.rs1 as usize].d as f32;
}

pub static FUNCS: [fn(&mut State, Insn); 133] = [
    func_lb,
    func_lh,
    func_lw,
    func_ld,
    func_lbu,
    func_lhu,
    func_lwu,
    func_empty, // fence
    func_empty, // fence_i
    func_addi,
    func_slli,
    func_slti,
    func_sltiu,
    func_xori,
    func_srli,
    func_srai,
    func_ori,
    func_andi,
    func_auipc,
    func_addiw,
    func_slliw,
    func_srliw,
    func_sraiw,
    func_sb,
    func_sh,
    func_sw,
    func_sd,
    func_add,
    func_sll,
    func_slt,
    func_sltu,
    func_xor,
    func_srl,
    func_or,
    func_and,
    func_mul,
    func_mulh,
    func_mulhsu,
    func_mulhu,
    func_div,
    func_divu,
    func_rem,
    func_remu,
    func_sub,
    func_sra,
    func_lui,
    func_addw,
    func_sllw,
    func_srlw,
    func_mulw,
    func_divw,
    func_divuw,
    func_remw,
    func_remuw,
    func_subw,
    func_sraw,
    func_beq,
    func_bne,
    func_blt,
    func_bge,
    func_bltu,
    func_bgeu,
    func_jalr,
    func_jal,
    func_ecall,
    func_csrrw,
    func_csrrs,
    func_csrrc,
    func_csrrwi,
    func_csrrsi,
    func_csrrci,
    func_flw,
    func_fsw,
    func_fmadd_s,
    func_fmsub_s,
    func_fnmsub_s,
    func_fnmadd_s,
    func_fadd_s,
    func_fsub_s,
    func_fmul_s,
    func_fdiv_s,
    func_fsqrt_s,
    func_fsgnj_s,
    func_fsgnjn_s,
    func_fsgnjx_s,
    func_fmin_s,
    func_fmax_s,
    func_fcvt_w_s,
    func_fcvt_wu_s,
    func_fmv_x_w,
    func_feq_s,
    func_flt_s,
    func_fle_s,
    func_fclass_s,
    func_fcvt_s_w,
    func_fcvt_s_wu,
    func_fmv_w_x,
    func_fcvt_l_s,
    func_fcvt_lu_s,
    func_fcvt_s_l,
    func_fcvt_s_lu,
    func_fld,
    func_fsd,
    func_fmadd_d,
    func_fmsub_d,
    func_fnmsub_d,
    func_fnmadd_d,
    func_fadd_d,
    func_fsub_d,
    func_fmul_d,
    func_fdiv_d,
    func_fsqrt_d,
    func_fsgnj_d,
    func_fsgnjn_d,
    func_fsgnjx_d,
    func_fmin_d,
    func_fmax_d,
    func_fcvt_s_d,
    func_fcvt_d_s,
    func_feq_d,
    func_flt_d,
    func_fle_d,
    func_fclass_d,
    func_fcvt_w_d,
    func_fcvt_wu_d,
    func_fcvt_d_w,
    func_fcvt_d_wu,
    func_fcvt_l_d,
    func_fcvt_lu_d,
    func_fmv_x_d,
    func_fcvt_d_l,
    func_fcvt_d_lu,
    func_fmv_d_x,
];

pub fn exec_block_interp(mut state: State) {
    let mut insn: Insn = Insn::new();
    loop {
        let data: *const u8 = ptr::null();
        let data: *const u32 = unsafe { data.add(to_host!(state.pc) as usize) } as *const u32;
        insn_decode(&mut insn, *(unsafe { data.as_ref().unwrap() }));

        FUNCS.get(insn.i_type as usize).unwrap()(&mut state, insn);
        state.gp_regs[GpRegTypeT::Zero as usize] = 0;

        if insn.cont {
            break;
        }

        state.pc += if insn.rvc { 2 } else { 4 };
    }
}
