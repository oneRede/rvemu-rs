use crate::{
    decode::insn_decode,
    reg::GpRegTypeT,
    rvemu::{Insn, State},
    to_host,
};

pub static FUNCS: Vec<fn(State, Insn)> = vec![];

pub fn exec_block_interp(mut state: State) {
    let insn: Insn = Insn::new();
    loop {
        let data = to_host!(state.pc) as u32;
        insn_decode(insn, data);

        FUNCS.get(insn.i_type as usize).unwrap()(state, insn);
        state.gp_regs[GpRegTypeT::Zero as usize] = 0;

        if insn.cont {
            break;
        }

        state.pc += if insn.rvc { 2 } else { 4 };
    }
}
