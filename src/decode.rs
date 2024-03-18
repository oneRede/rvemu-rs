use crate::{fatal, rvemu::Insn};

#[macro_export]
macro_rules! quadrant {
    ($data:ident) => {
        (($data >> 0) & 0x3)
    };
}

pub fn insn_decode(_insn: Insn, data: u32) {
    let quadrant = quadrant!(data);

    match quadrant {
        0x0 => fatal!("unimplemented"),
        0x1 => fatal!("unimplemented"),
        0x2 => fatal!("unimplemented"),
        0x3 => fatal!("unimplemented"),
        _ => unimplemented!(),
    }
}
