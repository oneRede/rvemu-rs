use std::ptr;

use crate::{
    decode::insn_decode,
    reg::GpRegTypeT,
    rvemu::{Insn, State},
    to_host,
};

pub fn func_empty(_state: State, _insn: Insn) {}

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
