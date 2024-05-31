use std::ptr;

use crate::{
    decode::insn_decode,
    interp_utils::{f32_classify, f64_classify, fsgnj32, fsgnj64, mulh, mulhsu, mulhu},
    reg::GpRegTypeT,
    rvemu::{ExitReason, Insn, State},
    to_host,
};
use rvemu_rs::{
    p_func1, p_func10, p_func11, p_func12, p_func13, p_func14, p_func2, p_func3, p_func4, p_func5,
    p_func6, p_func7, p_func8, p_func9,
};

pub fn func_empty(_state: &mut State, _insn: &mut Insn) {}

pub fn func_lb(state: &mut State, insn: &mut Insn) {
    p_func1!(i8);
}

pub fn func_lh(state: &mut State, insn: &mut Insn) {
    p_func1!(i16);
}

pub fn func_lw(state: &mut State, insn: &mut Insn) {
    p_func1!(i32);
}

pub fn func_ld(state: &mut State, insn: &mut Insn) {
    p_func1!(i64);
}

pub fn func_lbu(state: &mut State, insn: &mut Insn) {
    p_func1!(u8);
}

pub fn func_lhu(state: &mut State, insn: &mut Insn) {
    p_func1!(u16);
}

pub fn func_lwu(state: &mut State, insn: &mut Insn) {
    p_func1!(u32);
}

pub fn func_addi(state: &mut State, insn: &mut Insn) {
    p_func2!(rs1 + imm);
}

pub fn func_slli(state: &mut State, insn: &mut Insn) {
    p_func2!(rs1 << (imm & 0x3f));
}

pub fn func_slti(state: &mut State, insn: &mut Insn) {
    p_func2!(rs1 < imm);
}

pub fn func_sltiu(state: &mut State, insn: &mut Insn) {
    p_func2!(rs1 < imm);
}

pub fn func_xori(state: &mut State, insn: &mut Insn) {
    p_func2!(rs1 ^ imm);
}

pub fn func_srli(state: &mut State, insn: &mut Insn) {
    p_func2!(rs1 >> (imm & 0x3f));
}

pub fn func_srai(state: &mut State, insn: &mut Insn) {
    p_func2!(rs1 >> (imm & 0x3f));
}

pub fn func_ori(state: &mut State, insn: &mut Insn) {
    p_func2!(rs1 | (imm));
}

pub fn func_andi(state: &mut State, insn: &mut Insn) {
    p_func2!(rs1 & imm);
}

pub fn func_addiw(state: &mut State, insn: &mut Insn) {
    p_func2!(rs1 + imm);
}

pub fn func_slliw(state: &mut State, insn: &mut Insn) {
    p_func2!(rs1 << (imm & 0x1f));
}

pub fn func_srliw(state: &mut State, insn: &mut Insn) {
    p_func2!(rs1 >> (imm & 0x1f));
}

pub fn func_sraiw(state: &mut State, insn: &mut Insn) {
    p_func2!(rs1 >> (imm & 0x1f));
}

pub fn func_auipc(state: &mut State, insn: &mut Insn) {
    let val = state.pc + insn.imm as u64;
    state.gp_regs[insn.rd as usize] = val;
}

pub fn func_sb(state: &mut State, insn: &mut Insn) {
    p_func3!(u8);
}

pub fn func_sh(state: &mut State, insn: &mut Insn) {
    p_func3!(u16);
}

pub fn func_sw(state: &mut State, insn: &mut Insn) {
    p_func3!(u32);
}

pub fn func_sd(state: &mut State, insn: &mut Insn) {
    p_func3!(u64);
}

pub fn func_add(state: &mut State, insn: &mut Insn) {
    p_func4!(rs1 + rs2);
}

pub fn func_sll(state: &mut State, insn: &mut Insn) {
    p_func4!(rs1 << (rs2 & 0x3f));
}

pub fn func_slt(state: &mut State, insn: &mut Insn) {
    p_func4!((rs1 as i64) < (rs2 as i64));
}

pub fn func_sltu(state: &mut State, insn: &mut Insn) {
    p_func4!((rs1 as u64) < (rs2 as u64));
}

pub fn func_xor(state: &mut State, insn: &mut Insn) {
    p_func4!(rs1 ^ rs2);
}

pub fn func_srl(state: &mut State, insn: &mut Insn) {
    p_func4!(rs1 >> (rs2 & 0x3f));
}

pub fn func_or(state: &mut State, insn: &mut Insn) {
    p_func4!(rs1 | rs2);
}

pub fn func_and(state: &mut State, insn: &mut Insn) {
    p_func4!(rs1 & rs2);
}

pub fn func_mul(state: &mut State, insn: &mut Insn) {
    p_func4!(rs1 * rs2);
}

pub fn func_mulh(state: &mut State, insn: &mut Insn) {
    p_func4!(mulh(rs1 as i64, rs2 as i64));
}

pub fn func_mulhsu(state: &mut State, insn: &mut Insn) {
    p_func4!(mulhsu(rs1 as i64, rs2));
}

pub fn func_mulhu(state: &mut State, insn: &mut Insn) {
    p_func4!(mulhu(rs1, rs2));
}

pub fn func_sub(state: &mut State, insn: &mut Insn) {
    p_func4!(rs1 - rs2);
}

pub fn func_sra(state: &mut State, insn: &mut Insn) {
    p_func4!((rs1 as i64) >> (rs2 & 0x3f));
}

pub fn func_remu(state: &mut State, insn: &mut Insn) {
    p_func4!(if rs2 == 0 { rs1 } else { rs1 % rs2 });
}

pub fn func_addw(state: &mut State, insn: &mut Insn) {
    p_func4!((rs1 + rs2) as i64);
}

pub fn func_sllw(state: &mut State, insn: &mut Insn) {
    p_func4!((rs1 << (rs2 & 0x1f)) as i64);
}

pub fn func_srlw(state: &mut State, insn: &mut Insn) {
    p_func4!(((rs1 as u32) >> (rs2 & 0x1f)) as i64);
}

pub fn func_mulw(state: &mut State, insn: &mut Insn) {
    p_func4!((rs1 * rs2) as i64);
}

pub fn func_divw(state: &mut State, insn: &mut Insn) {
    p_func4!(if rs2 == 0 { u64::MAX } else { rs1 / rs2 });
}

pub fn func_divuw(state: &mut State, insn: &mut Insn) {
    p_func4!(if rs2 == 0 { u64::MAX } else { rs1 / rs2 });
}

pub fn func_remw(state: &mut State, insn: &mut Insn) {
    p_func4!(if rs2 == 0 { rs1 } else { rs1 % rs2 });
}

pub fn func_remuw(state: &mut State, insn: &mut Insn) {
    p_func4!(if rs2 == 0 { rs1 } else { rs1 % rs2 });
}

pub fn func_subw(state: &mut State, insn: &mut Insn) {
    p_func4!((rs1 - rs2));
}

pub fn func_sraw(state: &mut State, insn: &mut Insn) {
    p_func4!(((rs1 as i32) >> (rs2 & 0x1f)));
}

pub fn func_div(state: &mut State, insn: &mut Insn) {
    let rs1 = state.gp_regs[insn.rs1 as usize];
    let rs2 = state.gp_regs[insn.rs2 as usize];
    let rd: i128;

    if rs2 == 0 {
        rd = u64::MAX as i128;
    } else if rs1 as i64 == i64::MIN && rs2 == u64::MAX {
        rd = i64::MIN as i128;
    } else {
        rd = (rs1 / rs2) as i128;
    }
    state.gp_regs[insn.rd as usize] = rd as u64;
}

pub fn func_divu(state: &mut State, insn: &mut Insn) {
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

pub fn func_rem(state: &mut State, insn: &mut Insn) {
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

pub fn func_lui(state: &mut State, insn: &mut Insn) {
    state.gp_regs[insn.rd as usize] = insn.imm as u64;
}

pub fn func_beq(state: &mut State, insn: &mut Insn) {
    p_func5!(rs1 == rs2);
}

pub fn func_bne(state: &mut State, insn: &mut Insn) {
    p_func5!(rs1 != rs2);
}

pub fn func_blt(state: &mut State, insn: &mut Insn) {
    p_func5!(rs1 < rs2);
}

pub fn func_bge(state: &mut State, insn: &mut Insn) {
    p_func5!(rs1 >= rs2);
}

pub fn func_bltu(state: &mut State, insn: &mut Insn) {
    p_func5!(rs1 < rs2);
}

pub fn func_bgeu(state: &mut State, insn: &mut Insn) {
    p_func5!(rs1 >= rs2);
}

pub fn func_jalr(state: &mut State, insn: &mut Insn) {
    let rs1 = state.gp_regs[insn.rs1 as usize];
    state.gp_regs[insn.rd as usize] = state.pc + (if insn.rvc { 2 } else { 4 });
    state.exit_reason = ExitReason::IndirectBranch;
    state.reenter_pc = ((rs1 as i128 + insn.imm as i128) as u64) & !1u64;
}

pub fn func_jal(state: &mut State, insn: &mut Insn) {
    state.gp_regs[insn.rd as usize] = state.pc + (if insn.rvc { 2 } else { 4 });
    state.pc = ((state.pc as i64) + (insn.imm as i64)) as u64;
    state.reenter_pc = state.pc;
    state.exit_reason = ExitReason::IndirectBranch;
}

pub fn func_ecall(state: &mut State, _insn: &mut Insn) {
    state.exit_reason = ExitReason::Ecall;
    state.reenter_pc = state.pc + 4;
}

pub fn func_csrrw(state: &mut State, insn: &mut Insn) {
    p_func6!();
}

pub fn func_csrrs(state: &mut State, insn: &mut Insn) {
    p_func6!();
}

pub fn func_csrrc(state: &mut State, insn: &mut Insn) {
    p_func6!();
}

pub fn func_csrrwi(state: &mut State, insn: &mut Insn) {
    p_func6!();
}

pub fn func_csrrsi(state: &mut State, insn: &mut Insn) {
    p_func6!();
}

pub fn func_csrrci(state: &mut State, insn: &mut Insn) {
    p_func6!();
}

pub fn func_flw(state: &mut State, insn: &mut Insn) {
    let addr = state.gp_regs[insn.rs1 as usize] + insn.imm as u64;
    state.fp_regs[insn.rd as usize].v = to_host!(addr) | (-1i64 << 32) as u64;
}

pub fn func_fld(state: &mut State, insn: &mut Insn) {
    let addr = state.gp_regs[insn.rs1 as usize] + insn.imm as u64;
    state.fp_regs[insn.rd as usize].v = to_host!(addr);
}

pub fn func_fsw(state: &mut State, insn: &mut Insn) {
    p_func3!(u32);
}

pub fn func_fsd(state: &mut State, insn: &mut Insn) {
    p_func3!(u32);
}

pub fn func_fmadd_s(state: &mut State, insn: &mut Insn) {
    p_func7!(rs1 * rs2 + rs3);
}

pub fn func_fmsub_s(state: &mut State, insn: &mut Insn) {
    p_func7!(rs1 * rs2 - rs3);
}

pub fn func_fnmsub_s(state: &mut State, insn: &mut Insn) {
    p_func7!(-(rs1 * rs2) + rs3);
}

pub fn func_fnmadd_s(state: &mut State, insn: &mut Insn) {
    p_func7!(-(rs1 * rs2) - rs3);
}

pub fn func_fmadd_d(state: &mut State, insn: &mut Insn) {
    p_func8!(rs1 * rs2 + rs3);
}

pub fn func_fmsub_d(state: &mut State, insn: &mut Insn) {
    p_func8!(rs1 * rs2 - rs3);
}

pub fn func_fnmsub_d(state: &mut State, insn: &mut Insn) {
    p_func8!(-(rs1 * rs2) + rs3);
}

pub fn func_fnmadd_d(state: &mut State, insn: &mut Insn) {
    p_func8!(-(rs1 * rs2) - rs3);
}

pub fn func_fadd_s(state: &mut State, insn: &mut Insn) {
    p_func9!(rs1 + rs2);
}

pub fn func_fsub_s(state: &mut State, insn: &mut Insn) {
    p_func9!(rs1 - rs2);
}

pub fn func_fmul_s(state: &mut State, insn: &mut Insn) {
    p_func9!(rs1 * rs2);
}

pub fn func_fdiv_s(state: &mut State, insn: &mut Insn) {
    p_func9!(rs1 / rs2);
}

pub fn func_fsqrt_s(state: &mut State, insn: &mut Insn) {
    p_func9!(f32::sqrt(rs1));
}

pub fn func_fmin_s(state: &mut State, insn: &mut Insn) {
    p_func9!(if rs1 < rs2 { rs1 } else { rs2 });
}

pub fn func_fmax_s(state: &mut State, insn: &mut Insn) {
    p_func9!(if rs1 > rs2 { rs1 } else { rs2 });
}

pub fn func_fadd_d(state: &mut State, insn: &mut Insn) {
    p_func10!(rs1 + rs2);
}

pub fn func_fsub_d(state: &mut State, insn: &mut Insn) {
    p_func10!(rs1 - rs2);
}

pub fn func_fmul_d(state: &mut State, insn: &mut Insn) {
    p_func10!(rs1 * rs2);
}

pub fn func_fdiv_d(state: &mut State, insn: &mut Insn) {
    p_func10!(rs1 / rs2);
}

pub fn func_fsqrt_d(state: &mut State, insn: &mut Insn) {
    p_func10!(f64::sqrt(rs1));
}

pub fn func_fmin_d(state: &mut State, insn: &mut Insn) {
    p_func10!(if rs1 < rs2 { rs1 } else { rs2 });
}

pub fn func_fmax_d(state: &mut State, insn: &mut Insn) {
    p_func10!(if rs1 > rs2 { rs1 } else { rs2 });
}

pub fn func_fsgnj_s(state: &mut State, insn: &mut Insn) {
    p_func11!([false, false]);
}

pub fn func_fsgnjn_s(state: &mut State, insn: &mut Insn) {
    p_func11!([true, false]);
}

pub fn func_fsgnjx_s(state: &mut State, insn: &mut Insn) {
    p_func11!([false, true]);
}

pub fn func_fsgnj_d(state: &mut State, insn: &mut Insn) {
    p_func12!([false, false]);
}

pub fn func_fsgnjn_d(state: &mut State, insn: &mut Insn) {
    p_func12!([true, false]);
}

pub fn func_fsgnjx_d(state: &mut State, insn: &mut Insn) {
    p_func12!([false, true]);
}

pub fn func_fcvt_w_s(state: &mut State, insn: &mut Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].f as u64;
}

pub fn func_fcvt_wu_s(state: &mut State, insn: &mut Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].f as u64;
}

pub fn func_fcvt_w_d(state: &mut State, insn: &mut Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].d as u64;
}

pub fn func_fcvt_wu_d(state: &mut State, insn: &mut Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].d as u64;
}

pub fn func_fcvt_s_w(state: &mut State, insn: &mut Insn) {
    state.fp_regs[insn.rd as usize].f = state.fp_regs[insn.rs1 as usize].d as f32;
}

pub fn func_fcvt_s_wu(state: &mut State, insn: &mut Insn) {
    state.fp_regs[insn.rd as usize].f = state.fp_regs[insn.rs1 as usize].d as f32;
}

pub fn func_fcvt_d_w(state: &mut State, insn: &mut Insn) {
    state.fp_regs[insn.rd as usize].d = state.fp_regs[insn.rs1 as usize].d as f64;
}

pub fn func_fcvt_d_wu(state: &mut State, insn: &mut Insn) {
    state.fp_regs[insn.rd as usize].d = state.fp_regs[insn.rs1 as usize].d as f64;
}

pub fn func_fmv_x_w(state: &mut State, insn: &mut Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].w as u64;
}

pub fn func_fmv_w_x(state: &mut State, insn: &mut Insn) {
    state.fp_regs[insn.rd as usize].w = state.gp_regs[insn.rs1 as usize] as u32;
}

pub fn func_fmv_x_d(state: &mut State, insn: &mut Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].v as u64;
}

pub fn func_fmv_d_x(state: &mut State, insn: &mut Insn) {
    state.fp_regs[insn.rd as usize].v = state.gp_regs[insn.rs1 as usize] as u64;
}

pub fn func_feq_s(state: &mut State, insn: &mut Insn) {
    p_func13!(rs1 == rs2);
}

pub fn func_flt_s(state: &mut State, insn: &mut Insn) {
    p_func13!(rs1 < rs2);
}

pub fn func_fle_s(state: &mut State, insn: &mut Insn) {
    p_func13!(rs1 <= rs2);
}

pub fn func_feq_d(state: &mut State, insn: &mut Insn) {
    p_func14!(rs1 == rs2);
}

pub fn func_flt_d(state: &mut State, insn: &mut Insn) {
    p_func14!(rs1 < rs2);
}

pub fn func_fle_d(state: &mut State, insn: &mut Insn) {
    p_func14!(rs1 <= rs2);
}

pub fn func_fclass_s(state: &mut State, insn: &mut Insn) {
    state.gp_regs[insn.rd as usize] = f32_classify(state.fp_regs[insn.rs1 as usize].f) as u64;
}

pub fn func_fclass_d(state: &mut State, insn: &mut Insn) {
    state.gp_regs[insn.rd as usize] = f64_classify(state.fp_regs[insn.rs1 as usize].d) as u64;
}

pub fn func_fcvt_l_s(state: &mut State, insn: &mut Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].f as u64;
}

pub fn func_fcvt_lu_s(state: &mut State, insn: &mut Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].f as u64;
}

pub fn func_fcvt_l_d(state: &mut State, insn: &mut Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].d as u64;
}

pub fn func_fcvt_lu_d(state: &mut State, insn: &mut Insn) {
    state.gp_regs[insn.rd as usize] = state.fp_regs[insn.rs1 as usize].d as u64;
}

pub fn func_fcvt_s_l(state: &mut State, insn: &mut Insn) {
    state.fp_regs[insn.rd as usize].f = state.fp_regs[insn.rs1 as usize].d as f32;
}

pub fn func_fcvt_s_lu(state: &mut State, insn: &mut Insn) {
    state.fp_regs[insn.rd as usize].f = state.fp_regs[insn.rs1 as usize].d as f32;
}

pub fn func_fcvt_d_l(state: &mut State, insn: &mut Insn) {
    state.fp_regs[insn.rd as usize].d = state.fp_regs[insn.rs1 as usize].d as f64;
}

pub fn func_fcvt_d_lu(state: &mut State, insn: &mut Insn) {
    state.fp_regs[insn.rd as usize].d = state.fp_regs[insn.rs1 as usize].d as f64;
}

pub fn func_fcvt_s_d(state: &mut State, insn: &mut Insn) {
    state.fp_regs[insn.rd as usize].f = state.fp_regs[insn.rs1 as usize].d as f32;
}

pub fn func_fcvt_d_s(state: &mut State, insn: &mut Insn) {
    state.fp_regs[insn.rd as usize].d = state.fp_regs[insn.rs1 as usize].d as f64;
}

pub static FUNCS: [fn(&mut State, &mut Insn); 133] = [
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

pub fn exec_block_interp(state: &mut State) {
    let mut insn: Insn = Insn::new();
    loop {
        let data: *const u8 = ptr::null();
        let data: *const u32 = unsafe { data.add(to_host!(state.pc) as usize) } as *const u32;
        println!("{:?}", *unsafe { data.as_ref().unwrap() });
        insn_decode(&mut insn, *unsafe { data.as_ref().unwrap() });

        FUNCS.get(insn.i_type as usize).unwrap()(state, &mut insn);
        state.gp_regs[GpRegTypeT::Zero as usize] = 0;

        if insn.cont {
            break;
        }

        state.pc += if insn.rvc { 2 } else { 4 };
    }
}
