// #include "rvemu.h"

// #define QUADRANT(data) (((data) >> 0) & 0x3)

// void insn_decode(insn_t *insn, u32 data) {
//     u32 quadrant = QUADRANT(data);
//     switch (quadrant) {
//     case 0x0: fatal("unimplemented");
//     case 0x1: fatal("unimplemented");
//     case 0x2: fatal("unimplemented");
//     case 0x3: fatal("unimplemented");
//     default: unreachable();
//     }
// }

use crate::rvemu::{fatal, Insn};

#[macro_export]
macro_rules! quadrant {
    ($data:expr) => {
        ((($data) >> 0) & 0x3)
    };
}

pub fn insn_decode(_insn: Insn, data: u32) {
    let quadrant = quadrant!(data);

    match quadrant {
        0x0 => fatal("unimplemented"),
        0x1 => fatal("unimplemented"),
        0x2 => fatal("unimplemented"),
        0x3 => fatal("unimplemented"),
        _ => unimplemented!(),
    }
}
