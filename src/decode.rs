use crate::{fatal, rvemu::Insn};

#[macro_export]
macro_rules! quadrant {
    ($data:ident) => {
        (($data >> 0) & 0x3)
    };
}

#[macro_export]
macro_rules! op_code {
    ($data:ident) => {
        (($data >> 2) & 0x1f)
    };
}

#[macro_export]
macro_rules! rd {
    ($data:ident) => {
        (($data >> 7) & 0x1f)
    };
}

#[macro_export]
macro_rules! rs1 {
    ($data:ident) => {
        (($data >> 15) & 0x1f)
    };
}

#[macro_export]
macro_rules! rs2 {
    ($data:ident) => {
        (($data >> 20) & 0x1f)
    };
}

#[macro_export]
macro_rules! rs3 {
    ($data:ident) => {
        (($data >> 27) & 0x1f)
    };
}

#[macro_export]
macro_rules! func_t2 {
    ($data:ident) => {
        (($data >> 25) & 0x3)
    };
}

#[macro_export]
macro_rules! func_t7 {
    ($data:ident) => {
        (($data >> 25) & 0x7f)
    };
}

#[macro_export]
macro_rules! imm_116 {
    ($data:ident) => {
        (($data >> 26) & 0x3f)
    };
}

#[inline]
pub fn insn_utype_read(data: u32) -> Insn {
    let imm: i32 = (data & 0xfffff000) as i32;
    let rd = rd!(data);

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rd = rd as i8;
    return insn;
}

#[inline]
pub fn insn_itype_read(data: u32) -> Insn {
    let imm: i32 = (data as i32) >> 20;
    let rs1 = rs1!(data);
    let rd = rd!(data);

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rd = rd as i8;
    insn.rs1 = rs1 as i8;
    return insn;
}

#[inline]
pub fn insn_jtype_read(data: u32) -> Insn {
    let imm20 = data >> 31 & 0x1;
    let imm101 = data >> 21 & 0x3ff;
    let imm11 = data >> 20 & 0x1;
    let imm1912 = data >> 21 & 0xff;

    let mut imm: i32 = ((imm20 << 20) | (imm1912 << 12) | (imm11 << 11) | (imm101 << 1)) as i32;
    imm = (imm << 11) >> 11;

    let mut insn = Insn::new();
    insn.imm = imm;

    return insn;
}

#[inline]
pub fn insn_btype_read(data: u32) -> Insn {
    let imm12 = (data >> 31) & 0x1;
    let imm105 = (data >> 25) & 0x3f;
    let imm41 = (data >> 8) & 0xf;
    let imm11 = (data >> 7) & 0x1;

    let mut imm: i32 = ((imm12 << 12) | (imm11 << 11) | (imm105 << 5) | (imm41 << 1)) as i32;
    imm = (imm << 19) >> 19;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rs1 = rs1!(data) as i8;
    insn.rs2 = rs2!(data) as i8;

    return insn;
}

#[inline]
pub fn insn_rtype_read(data: u32) -> Insn {
    let mut insn = Insn::new();

    insn.rs1 = rs1!(data) as i8;
    insn.rs2 = rs2!(data) as i8;
    insn.rd = rd!(data) as i8;

    return insn;
}

#[inline]
pub fn insn_stype_read(data: u32) -> Insn {
    let imm115 = (data >> 25) & 0x7f;
    let imm40 = (data >> 7) & 0x1f;

    let imm = (imm115 << 5) | imm40;
    let imm = ((imm << 20) >> 20) as i32;

    let mut insn = Insn::new();

    insn.rs1 = rs1!(data) as i8;
    insn.rs2 = rs2!(data) as i8;
    insn.imm = imm;

    return insn;
}

#[inline]
pub fn insn_fprtype_read(data: u32) -> Insn {
    let mut insn = Insn::new();

    insn.rs1 = rs1!(data) as i8;
    insn.rs2 = rs2!(data) as i8;
    insn.rs3 = rs3!(data) as i8;
    insn.rd = rd!(data) as i8;

    return insn;
}

#[macro_export]
macro_rules! cop_code {
    ($data:ident) => {
        (($data >> 13) & 0x7)
    };
}

#[macro_export]
macro_rules! cfunc_t1 {
    ($data:ident) => {
        (($data >> 12) & 0x1)
    };
}

#[macro_export]
macro_rules! cfunc_t2_low {
    ($data:ident) => {
        (($data >> 5) & 0x3)
    };
}

#[macro_export]
macro_rules! cfunc_t2_high {
    ($data:ident) => {
        (($data >> 10) & 0x3)
    };
}

#[macro_export]
macro_rules! rp1 {
    ($data:ident) => {
        (($data >> 7) & 0x7)
    };
}

#[macro_export]
macro_rules! rp2 {
    ($data:ident) => {
        (($data >> 2) & 0x7)
    };
}

#[macro_export]
macro_rules! rc1 {
    ($data:ident) => {
        (($data >> 7) & 0x1f)
    };
}

#[macro_export]
macro_rules! rc2 {
    ($data:ident) => {
        (($data >> 2) & 0x1f)
    };
}

#[inline]
pub fn insn_catype_read(data: u16) -> Insn {
    let mut insn = Insn::new();
    insn.rd = rp1!(data) as i8 + 8;
    insn.rs2 = rp2!(data) as i8 + 8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_crtype_read(data: u16) -> Insn {
    let mut insn = Insn::new();
    insn.rd = rc1!(data) as i8;
    insn.rs2 = rc2!(data) as i8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_citype_read(data: u16) -> Insn {
    let imm40 = (data >> 2) & 0x1f;
    let imm5 = (data >> 12) & 0x1;
    let mut imm: i32 = ((imm5 << 5) | imm40) as i32;
    imm = (imm << 26) >> 26;

    let mut insn = Insn::new();
    insn.imm = imm as i32;
    insn.rd = rc1!(data) as i8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_citype_read2(data: u16) -> Insn {
    let imm86 = ((data >> 2) & 0x7) as u32;
    let imm43 = ((data >> 5) & 0x3) as u32;
    let imm5 = ((data >> 12) & 0x1) as u32;
    let imm = ((imm86 << 6) | (imm43 << 3) | (imm5 << 5)) as i32;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rd = rc1!(data) as i8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_citype_read3(data: u16) -> Insn {
    let imm5 = ((data >> 2) & 0x1) as u32;
    let imm87 = ((data >> 3) & 0x3) as u32;
    let imm6 = ((data >> 5) & 0x1) as u32;
    let imm4 = ((data >> 6) & 0x1) as u32;
    let imm9 = ((data >> 12) & 0x1) as u32;
    let mut imm = ((imm5 << 5) | (imm87 << 7) | (imm6 << 6) | (imm4 << 4) | (imm9 << 9)) as i32;
    imm = (imm << 22) >> 22;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rd = rc1!(data) as i8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_citype_read4(data: u16) -> Insn {
    let imm5 = ((data >> 12) & 0x1) as u32;
    let imm42 = ((data >> 4) & 0x7) as u32;
    let imm76 = ((data >> 2) & 0x3) as u32;
    let imm = ((imm5 << 5) | (imm42 << 2) | (imm76 << 6)) as i32;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rd = rc1!(data) as i8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_citype_read5(data: u16) -> Insn {
    let imm1612 = ((data >> 2) & 0x1f) as u32;
    let imm17 = ((data >> 12) & 0x1) as u32;
    let mut imm = ((imm1612 << 12) | (imm17 << 17)) as i32;
    imm = (imm << 14) >> 14;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rd = rc1!(data) as i8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_cbtype_read(data: u16) -> Insn {
    let imm5 = ((data >> 2) & 0x1) as u32;
    let imm21 = ((data >> 3) & 0x3) as u32;
    let imm76 = ((data >> 5) & 0x3) as u32;
    let imm43 = ((data >> 10) & 0x3) as u32;
    let imm8 = ((data >> 12) & 0x1) as u32;
    let mut imm = ((imm8 << 8) | (imm76 << 6) | (imm5 << 5) | (imm43 << 3) | (imm21 << 1)) as i32;
    imm = (imm << 23) >> 23;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rs1 = rp1!(data) as i8 + 8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_cbtype_read2(data: u16) -> Insn {
    let imm40: u32 = (data as u32 >> 2) & 0x1f;
    let imm5: u32 = (data as u32 >> 12) & 0x1;
    let mut imm: i32 = ((imm5 as i32) << 5) | imm40 as i32;
    imm = (imm << 26) >> 26;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rd = rp1!(data) as i8 + 8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_cstype_read(data: u16) -> Insn {
    let imm76: u32 = (data as u32 >> 5) & 0x3;
    let imm53: u32 = (data as u32 >> 10) & 0x7;
    let imm: i32 = ((imm76 << 6) | (imm53 << 3)) as i32;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rs1 = rp1!(data) as i8 + 8;
    insn.rs2 = rp2!(data) as i8 + 8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_cstype_read2(data: u16) -> Insn {
    let imm6: u32 = ((data as u32) >> 5) & 0x1;
    let imm2: u32 = ((data as u32) >> 6) & 0x1;
    let imm53: u32 = ((data as u32) >> 10) & 0x7;
    let imm: i32 = ((imm6 << 6) | (imm2 << 2) | (imm53 << 3)) as i32;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rs1 = rp1!(data) as i8 + 8;
    insn.rs2 = rp2!(data) as i8 + 8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_cjtype_read(data: u16) -> Insn {
    let imm5: u32 = ((data as u32) >> 2) & 0x1;
    let imm31: u32 = ((data as u32) >> 3) & 0x7;
    let imm7: u32 = ((data as u32) >> 6) & 0x1;
    let imm6: u32 = ((data as u32) >> 7) & 0x1;
    let imm10: u32 = ((data as u32) >> 8) & 0x1;
    let imm98: u32 = ((data as u32) >> 9) & 0x3;
    let imm4: u32 = ((data as u32) >> 11) & 0x1;
    let imm11: u32 = ((data as u32) >> 12) & 0x1;
    let mut imm = ((imm5 << 5)
        | (imm31 << 1)
        | (imm7 << 7)
        | (imm6 << 6)
        | (imm10 << 10)
        | (imm98 << 8)
        | (imm4 << 4)
        | (imm11 << 11)) as i32;
    imm = (imm << 20) >> 20;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_cltype_read(data: u16) -> Insn {
    let imm6: u32 = ((data as u32) >> 5) & 0x1;
    let imm2: u32 = ((data as u32) >> 6) & 0x1;
    let imm53: u32 = ((data as u32) >> 10) & 0x7;
    let imm: i32 = ((imm6 << 6) | (imm2 << 2) | (imm53 << 3)) as i32;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rs1 = rp1!(data) as i8 + 8;
    insn.rd = rp2!(data) as i8 + 8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_cltype_read2(data: u16) -> Insn {
    let imm76: u32 = ((data as u32) >> 5) & 0x3;
    let imm53: u32 = ((data as u32) >> 10) & 0x7;
    let imm = ((imm76 << 6) | (imm53 << 3)) as i32;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rs1 = rp1!(data) as i8 + 8;
    insn.rd = rp2!(data) as i8 + 8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_csstype_read(data: u16) -> Insn {
    let imm86: u32 = ((data as u32) >> 7) & 0x7;
    let imm53: u32 = ((data as u32) >> 10) & 0x7;
    let imm = ((imm86 << 6) | (imm53 << 3)) as i32;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rs2 = rc2!(data) as i8 + 8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_ciwtype_read(data: u16) -> Insn {
    let imm3: u32 = ((data as u32) >> 5) & 0x1;
    let imm2: u32 = ((data as u32) >> 6) & 0x1;
    let imm96: u32 = ((data as u32) >> 7) & 0xf;
    let imm54: u32 = ((data as u32) >> 11) & 0x3;
    let imm = ((imm3 << 3) | (imm2 << 2) | (imm96 << 6) | (imm54 << 4)) as i32;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rd = rp2!(data) as i8 + 8;
    insn.rvc = true;

    return insn;
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
