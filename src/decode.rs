use crate::{
    fatal,
    reg::GpRegTypeT,
    rvemu::{Insn, InsnType},
};

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
        (((($data) >> 7) as i8) & 0x1f) as i8
    };
}

#[macro_export]
macro_rules! rs1 {
    ($data:ident) => {
        (($data >> 15) & 0x1f) as i8
    };
}

#[macro_export]
macro_rules! rs2 {
    ($data:ident) => {
        (($data >> 20) & 0x1f) as i8
    };
}

#[macro_export]
macro_rules! rs3 {
    ($data:ident) => {
        (($data >> 27) & 0x1f) as i8
    };
}

#[macro_export]
macro_rules! func_t2 {
    ($data:ident) => {
        (($data >> 25) & 0x3)
    };
}

#[macro_export]
macro_rules! func_t3 {
    ($data:ident) => {
        (($data >> 12) & 0x7)
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
    insn.rd = rd;
    return insn;
}

#[inline]
pub fn insn_itype_read(data: u32) -> Insn {
    let imm: i32 = (data as i32) >> 20;
    let rs1 = rs1!(data);
    let rd = rd!(data);

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rd = rd;
    insn.rs1 = rs1;
    return insn;
}

#[inline]
pub fn insn_jtype_read(data: u32) -> Insn {
    let imm20 = (data >> 31) & 0x1;
    let imm101 = (data >> 21) & 0x3ff;
    let imm11 = (data >> 20) & 0x1;
    let imm1912 = (data >> 12) & 0xff;

    let mut imm: i32 = ((imm20 << 20) | (imm1912 << 12) | (imm11 << 11) | (imm101 << 1)) as i32;
    imm = (imm << 11) >> 11;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rd = rd!(data);

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
    insn.rs1 = rs1!(data);
    insn.rs2 = rs2!(data);

    return insn;
}

#[inline]
pub fn insn_rtype_read(data: u32) -> Insn {
    let mut insn = Insn::new();

    insn.rs1 = rs1!(data);
    insn.rs2 = rs2!(data);
    insn.rd = rd!(data);

    return insn;
}

#[inline]
pub fn insn_stype_read(data: u32) -> Insn {
    let imm115 = (data >> 25) & 0x7f;
    let imm40 = (data >> 7) & 0x1f;

    let imm = ((imm115 << 5) | imm40) as i32;
    let imm = ((imm << 20) >> 20) as i32;

    let mut insn = Insn::new();

    insn.rs1 = rs1!(data);
    insn.rs2 = rs2!(data);
    insn.imm = imm;

    return insn;
}

#[inline]
pub fn insn_csrtype_read(data: u32) -> Insn {
    let mut insn = Insn::new();

    insn.csr = (data >> 20) as i16;
    insn.rs1 = rs1!(data);
    insn.rd = rd!(data);

    return insn;
}

#[inline]
pub fn insn_fprtype_read(data: u32) -> Insn {
    let mut insn = Insn::new();

    insn.rs1 = rs1!(data);
    insn.rs2 = rs2!(data);
    insn.rs3 = rs3!(data);
    insn.rd = rd!(data);

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
        (($data >> 7) & 0x7) as i8
    };
}

#[macro_export]
macro_rules! rp2 {
    ($data:ident) => {
        (($data >> 2) & 0x7) as i8
    };
}

#[macro_export]
macro_rules! rc1 {
    ($data:ident) => {
        (($data >> 7) & 0x1f) as i8
    };
}

#[macro_export]
macro_rules! rc2 {
    ($data:ident) => {
        (($data >> 2) & 0x1f) as i8
    };
}

#[inline]
pub fn insn_catype_read(data: u16) -> Insn {
    let mut insn = Insn::new();
    insn.rd = rp1!(data) + 8;
    insn.rs2 = rp2!(data) + 8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_crtype_read(data: u16) -> Insn {
    let mut insn = Insn::new();
    insn.rs1 = rc1!(data);
    insn.rs2 = rc2!(data);
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_citype_read(data: u16) -> Insn {
    let imm40: u32 = ((data >> 2) & 0x1f) as u32;
    let imm5: u32 = ((data >> 12) & 0x1) as u32;
    let mut imm: i32 = ((imm5 << 5) | imm40) as i32;
    imm = (imm << 26) >> 26;

    let mut insn = Insn::new();
    insn.imm = imm as i32;
    insn.rd = rc1!(data);
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
    insn.rd = rc1!(data);
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
    insn.rd = rc1!(data);
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
    insn.rd = rc1!(data);
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
    insn.rd = rc1!(data);
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
    insn.rs1 = rp1!(data) + 8;
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
    insn.rd = rp1!(data) + 8;
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
    insn.rs1 = rp1!(data) + 8;
    insn.rs2 = rp2!(data) + 8;
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
    insn.rs1 = rp1!(data) + 8;
    insn.rs2 = rp2!(data) + 8;
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_cjtype_read(data: u16) -> Insn {
    let data = data as u32;
    let imm5: u32 = (data >> 2) & 0x1;
    let imm31: u32 = (data >> 3) & 0x7;
    let imm7: u32 = (data >> 6) & 0x1;
    let imm6: u32 = (data >> 7) & 0x1;
    let imm10: u32 = (data >> 8) & 0x1;
    let imm98: u32 = (data >> 9) & 0x3;
    let imm4: u32 = (data >> 11) & 0x1;
    let imm11: u32 = (data >> 12) & 0x1;
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
    let data = data as u32;
    let imm6: u32 = (data >> 5) & 0x1;
    let imm2: u32 = (data  >> 6) & 0x1;
    let imm53: u32 = (data >> 10) & 0x7;
    let imm: i32 = ((imm6 << 6) | (imm2 << 2) | (imm53 << 3)) as i32;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rs1 = rp1!(data) + 8;
    insn.rd = rp2!(data) + 8;
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
    insn.rs1 = rp1!(data) + 8;
    insn.rd = rp2!(data) + 8;
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
    insn.rs2 = rc2!(data);
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_csstype_read2(data: u16) -> Insn {
    let imm76: u32 = ((data as u32) >> 7) & 0x3;
    let imm52: u32 = ((data as u32) >> 9) & 0xf;
    let imm = ((imm76 << 6) | (imm52 << 2)) as i32;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rs2 = rc2!(data);
    insn.rvc = true;

    return insn;
}

#[inline]
pub fn insn_ciwtype_read(data: u16) -> Insn {
    let data = data as u32;
    let imm3: u32 = (data >> 5) & 0x1;
    let imm2: u32 = (data >> 6) & 0x1;
    let imm96: u32 = (data >> 7) & 0xf;
    let imm54: u32 = (data  >> 11) & 0x3;
    let imm = ((imm3 << 3) | (imm2 << 2) | (imm96 << 6) | (imm54 << 4)) as i32;

    let mut insn = Insn::new();
    insn.imm = imm;
    insn.rd = rp2!(data) + 8;
    insn.rvc = true;

    return insn;
}

pub fn insn_decode(insn: &mut Insn, data: u32) {
    let quadrant = quadrant!(data);

    match quadrant {
        0x0 => {
            let copcode = cop_code!(data);
            match copcode {
                0x0 => {
                    *insn = insn_ciwtype_read(data as u16);
                    insn.rs1 = GpRegTypeT::Sp as i8;
                    insn.i_type = InsnType::InsnAddi;
                    assert!(insn.imm != 0);
                    return;
                }
                0x1 => {
                    *insn = insn_cltype_read2(data as u16);
                    insn.i_type = InsnType::InsnFld;
                    return;
                }
                0x2 => {
                    *insn = insn_cltype_read(data as u16);
                    insn.i_type = InsnType::InsnLw;
                    return;
                }
                0x3 => {
                    *insn = insn_cltype_read2(data as u16);
                    insn.i_type = InsnType::InsnLd;
                    return;
                }
                0x5 => {
                    *insn = insn_cstype_read(data as u16);
                    insn.i_type = InsnType::InsnFsd;
                    return;
                }
                0x6 => {
                    *insn = insn_cstype_read2(data as u16);
                    insn.i_type = InsnType::InsnSw;
                    return;
                }
                0x7 => {
                    *insn = insn_cstype_read(data as u16);
                    insn.i_type = InsnType::InsnSd;
                    return;
                }
                _ => {
                    println!("data: {}", data);
                    fatal!("unimplemented");
                }
            }
        }
        0x1 => {
            let copcode = cop_code!(data);
            match copcode {
                0x0 => {
                    *insn = insn_citype_read(data as u16);
                    insn.rs1 = insn.rd;
                    insn.i_type = InsnType::InsnAddi;
                    return;
                }
                0x1 => {
                    *insn = insn_citype_read(data as u16);
                    assert!(insn.rd != 0);
                    insn.rs1 = insn.rd;
                    insn.i_type = InsnType::InsnAddiw;
                    return;
                }
                0x2 => {
                    *insn = insn_citype_read(data as u16);
                    insn.rs1 = GpRegTypeT::Zero as i8;
                    insn.i_type = InsnType::InsnAddi;
                    return;
                }
                0x3 => {
                    let rd = rc1!(data);
                    if rd == 2 {
                        *insn = insn_citype_read3(data as u16);
                        assert!(insn.imm != 0);
                        insn.rs1 = insn.rd;
                        insn.i_type = InsnType::InsnAddi;
                        return;
                    } else {
                        *insn = insn_citype_read5(data as u16);
                        assert!(insn.imm != 0);
                        insn.i_type = InsnType::InsnLui;
                        return;
                    }
                }
                0x4 => {
                    let cfunct2high = cfunc_t2_high!(data);
                    match cfunct2high {
                        0x0 | 0x1 | 0x2 => {
                            *insn = insn_cbtype_read2(data as u16);
                            insn.rs1 = insn.rd;
                            if cfunct2high == 0x0 {
                                insn.i_type = InsnType::InsnSrli;
                                return;
                            } else if cfunct2high == 0x1 {
                                insn.i_type = InsnType::InsnSrai;
                                return;
                            } else {
                                insn.i_type = InsnType::InsnAndi;
                                return;
                            }
                        }
                        0x3 => {
                            let cfunct1 = cfunc_t1!(data);
                            match cfunct1 {
                                0x0 => {
                                    let cfunct2low = cfunc_t2_low!(data);

                                    *insn = insn_catype_read(data as u16);
                                    insn.rs1 = insn.rd;

                                    match cfunct2low {
                                        0x0 => {
                                            insn.i_type = InsnType::InsnSub;
                                            return;
                                        }
                                        0x1 => {
                                            insn.i_type = InsnType::InsnXor;
                                            return;
                                        }
                                        0x2 => {
                                            insn.i_type = InsnType::InsnOr;
                                            return;
                                        }
                                        0x3 => {
                                            insn.i_type = InsnType::InsnAnd;
                                            return;
                                        }
                                        _ => {
                                            unreachable!()
                                        }
                                    }
                                }
                                0x1 => {
                                    let cfunct2low = cfunc_t2_low!(data);

                                    *insn = insn_catype_read(data as u16);
                                    insn.rs1 = insn.rd;
                                    match cfunct2low {
                                        0x0 => {
                                            insn.i_type = InsnType::InsnSubw;
                                            return;
                                        }
                                        0x1 => {
                                            insn.i_type = InsnType::InsnAddw;
                                            return;
                                        }
                                        _ => {
                                            unreachable!()
                                        }
                                    }
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                        _ => {}
                    }
                }
                0x5 => {
                    *insn = insn_cjtype_read(data as u16);
                    insn.rd = GpRegTypeT::Zero as i8;
                    insn.i_type = InsnType::InsnJal;
                    insn.cont = true;
                    return;
                }
                0x6 | 0x7 => {
                    *insn = insn_cbtype_read(data as u16);
                    insn.rs2 = GpRegTypeT::Zero as i8;
                    if copcode == 0x6 {
                        insn.i_type = InsnType::InsnBeq;
                        return;
                    } else {
                        insn.i_type = InsnType::InsnBne;
                        return;
                    }
                }
                _ => {
                    fatal!("unrecognized copcode");
                }
            }
        }
        0x2 => {
            let copcode = cop_code!(data);
            match copcode {
                0x0 => {
                    *insn = insn_citype_read(data as u16);
                    insn.rs1 = insn.rd;
                    insn.i_type = InsnType::InsnSlli;
                    return;
                }
                0x1 => {
                    *insn = insn_citype_read2(data as u16);
                    insn.rs1 = GpRegTypeT::Sp as i8;
                    insn.i_type = InsnType::InsnFld;
                    return;
                }
                0x2 => {
                    *insn = insn_citype_read4(data as u16);
                    assert!(insn.rd != 0);
                    insn.rs1 = GpRegTypeT::Sp as i8;
                    insn.i_type = InsnType::InsnLw;
                    return;
                }
                0x3 => {
                    *insn = insn_citype_read2(data as u16);
                    assert!(insn.rd != 0);
                    insn.rs1 = GpRegTypeT::Sp as i8;
                    insn.i_type = InsnType::InsnLd;
                    return;
                }
                0x4 => {
                    let cfunct1 = cfunc_t1!(data);
                    match cfunct1 {
                        0x0 => {
                            *insn = insn_crtype_read(data as u16);

                            if insn.rs2 == 0 {
                                assert!(insn.rs1 != 0);
                                insn.rd = GpRegTypeT::Zero as i8;
                                insn.i_type = InsnType::InsnJalr;
                                insn.cont = true;
                                return;
                            } else {
                                insn.rd = insn.rs1;
                                insn.rs1 = GpRegTypeT::Zero as i8;
                                insn.i_type = InsnType::InsnAdd;
                                return;
                            }
                        }
                        0x1 => {
                            *insn = insn_crtype_read(data as u16);
                            if insn.rs1 == 0 && insn.rs2 == 0 {
                                fatal!("unimplmented");
                                return;
                            } else if insn.rs2 == 0 {
                                insn.rd = GpRegTypeT::RA as i8;
                                insn.i_type = InsnType::InsnJalr;
                                insn.cont = true;
                                return;
                            } else {
                                insn.rd = insn.rs1;
                                insn.i_type = InsnType::InsnAdd;
                                return;
                            }
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                0x5 => {
                    *insn = insn_csstype_read(data as u16);
                    insn.rs1 = GpRegTypeT::Sp as i8;
                    insn.i_type = InsnType::InsnFsd;
                    return;
                }
                0x6 => {
                    *insn = insn_csstype_read2(data as u16);
                    insn.rs1 = GpRegTypeT::Sp as i8;
                    insn.i_type = InsnType::InsnSw;
                    return;
                }
                0x7 => {
                    *insn = insn_csstype_read(data as u16);
                    insn.rs1 = GpRegTypeT::Sp as i8;
                    insn.i_type = InsnType::InsnSd;
                    return;
                }
                _ => {
                    fatal!("unrecognized copcode")
                }
            }
        }
        0x3 => {
            let opcode = op_code!(data);
            match opcode {
                0x0 => {
                    let funct3 = func_t3!(data);
                    *insn = insn_itype_read(data);
                    match funct3 {
                        0x0 => {
                            insn.i_type = InsnType::InsnLb;
                            return;
                        }
                        0x1 => {
                            insn.i_type = InsnType::InsnLh;
                            return;
                        }
                        0x2 => {
                            insn.i_type = InsnType::InsnLw;
                            return;
                        }
                        0x3 => {
                            insn.i_type = InsnType::InsnLd;
                            return;
                        }
                        0x4 => {
                            insn.i_type = InsnType::InsnLbu;
                            return;
                        }
                        0x5 => {
                            insn.i_type = InsnType::InsnLhu;
                            return;
                        }
                        0x6 => {
                            insn.i_type = InsnType::InsnLwu;
                            return;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                0x1 => {
                    let funct3 = func_t3!(data);
                    *insn = insn_itype_read(data);
                    match funct3 {
                        0x2 => {
                            insn.i_type = InsnType::InsnFlw;
                            return;
                        }
                        0x3 => {
                            insn.i_type = InsnType::InsnFld;
                            return;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                0x3 => {
                    let funct3 = func_t3!(data);
                    *insn = insn_itype_read(data);
                    match funct3 {
                        0x0 => {
                            let mut _insn = Insn::new();
                            _insn.rd = 0;
                            *insn = _insn;
                            insn.i_type = InsnType::InsnFence;
                            return;
                        }
                        0x1 => {
                            let mut _insn = Insn::new();
                            _insn.rd = 0;
                            *insn = _insn;
                            insn.i_type = InsnType::InsnFenceI;
                            return;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                0x4 => {
                    let funct3 = func_t3!(data);

                    *insn = insn_itype_read(data);
                    match funct3 {
                        0x0 => {
                            insn.i_type = InsnType::InsnAddi;
                            return;
                        }
                        0x1 => {
                            let imm116 = imm_116!(data);
                            if imm116 == 0 {
                                insn.i_type = InsnType::InsnSlli;
                                return;
                            } else {
                                unreachable!();
                            }
                        }
                        0x2 => {
                            insn.i_type = InsnType::InsnSlti;
                            return;
                        }
                        0x3 => {
                            insn.i_type = InsnType::InsnSltiu;
                            return;
                        }
                        0x4 => {
                            insn.i_type = InsnType::InsnXori;
                            return;
                        }
                        0x5 => {
                            let imm116 = imm_116!(data);

                            if imm116 == 0x0 {
                                /* SRLI */
                                insn.i_type = InsnType::InsnSrli;
                                return;
                            } else if imm116 == 0x10 {
                                /* SRAI */
                                insn.i_type = InsnType::InsnSrai;
                                return;
                            } else {
                                unreachable!();
                            }
                        }
                        0x6 => {
                            insn.i_type = InsnType::InsnOri;
                            return;
                        }
                        0x7 => {
                            insn.i_type = InsnType::InsnAndi;
                            return;
                        }
                        _ => {
                            fatal!("unrecognized funct3")
                        }
                    }
                }
                0x5 => {
                    *insn = insn_utype_read(data);
                    insn.i_type = InsnType::InsnAuipc;
                    return;
                }
                0x6 => {
                    let funct3 = func_t3!(data);
                    let funct7 = func_t7!(data);

                    *insn = insn_itype_read(data);
                    match funct3 {
                        0x0 => {
                            insn.i_type = InsnType::InsnAddiw;
                            return;
                        }
                        0x1 => {
                            assert!(funct7 == 0);
                            insn.i_type = InsnType::InsnSlliw;
                            return;
                        }
                        0x5 => match funct7 {
                            0x0 => {
                                insn.i_type = InsnType::InsnSrliw;
                                return;
                            }
                            0x20 => {
                                insn.i_type = InsnType::InsnSraiw;
                                return;
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        _ => {
                            fatal!("unimplemented")
                        }
                    }
                }
                0x8 => {
                    let funct3 = func_t3!(data);

                    *insn = insn_stype_read(data);
                    match funct3 {
                        0x0 => {
                            insn.i_type = InsnType::InsnSb;
                            return;
                        }
                        0x1 => {
                            insn.i_type = InsnType::InsnSh;
                            return;
                        }
                        0x2 => {
                            insn.i_type = InsnType::InsnSw;
                            return;
                        }
                        0x3 => {
                            insn.i_type = InsnType::InsnSd;
                            return;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                0x9 => {
                    let funct3 = func_t3!(data);

                    *insn = insn_stype_read(data);
                    match funct3 {
                        0x2 => {
                            insn.i_type = InsnType::InsnFsw;
                            return;
                        }
                        0x3 => {
                            insn.i_type = InsnType::InsnFsd;
                            return;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                0xc => {
                    *insn = insn_rtype_read(data);

                    let funct3 = func_t3!(data);
                    let funct7 = func_t7!(data);
                    match funct7 {
                        0x0 => match funct3 {
                            0x0 => {
                                insn.i_type = InsnType::InsnAdd;
                                return;
                            }
                            0x1 => {
                                insn.i_type = InsnType::InsnSll;
                                return;
                            }
                            0x2 => {
                                insn.i_type = InsnType::InsnSlt;
                                return;
                            }
                            0x3 => {
                                insn.i_type = InsnType::InsnSltu;
                                return;
                            }
                            0x4 => {
                                insn.i_type = InsnType::InsnXor;
                                return;
                            }
                            0x5 => {
                                insn.i_type = InsnType::InsnSrl;
                                return;
                            }
                            0x6 => {
                                insn.i_type = InsnType::InsnOr;
                                return;
                            }
                            0x7 => {
                                insn.i_type = InsnType::InsnAnd;
                                return;
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        0x1 => match funct3 {
                            0x0 => {
                                insn.i_type = InsnType::InsnMul;
                                return;
                            }
                            0x1 => {
                                insn.i_type = InsnType::InsnMulh;
                                return;
                            }
                            0x2 => {
                                insn.i_type = InsnType::InsnMulhsu;
                                return;
                            }
                            0x3 => {
                                insn.i_type = InsnType::InsnMulhu;
                                return;
                            }
                            0x4 => {
                                insn.i_type = InsnType::InsnDiv;
                                return;
                            }
                            0x5 => {
                                insn.i_type = InsnType::InsnDivu;
                                return;
                            }
                            0x6 => {
                                insn.i_type = InsnType::InsnRem;
                                return;
                            }
                            0x7 => {
                                insn.i_type = InsnType::InsnRemu;
                                return;
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        0x20 => match funct3 {
                            0x0 => {
                                insn.i_type = InsnType::InsnSub;
                                return;
                            }
                            0x5 => {
                                insn.i_type = InsnType::InsnSra;
                                return;
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        _ => {
                            unreachable!()
                        }
                    }
                }
                0xd => {
                    *insn = insn_utype_read(data);
                    insn.i_type = InsnType::InsnLui;
                    return;
                }
                0xe => {
                    *insn = insn_rtype_read(data);

                    let funct3 = func_t3!(data);
                    let funct7 = func_t7!(data);
                    match funct7 {
                        0x0 => match funct3 {
                            0x0 => {
                                insn.i_type = InsnType::InsnAddw;
                                return;
                            }
                            0x1 => {
                                insn.i_type = InsnType::InsnSllw;
                                return;
                            }
                            0x5 => {
                                insn.i_type = InsnType::InsnSrlw;
                                return;
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        0x1 => match funct3 {
                            0x0 => {
                                insn.i_type = InsnType::InsnMulw;
                                return;
                            }
                            0x4 => {
                                insn.i_type = InsnType::InsnDivw;
                                return;
                            }
                            0x5 => {
                                insn.i_type = InsnType::InsnDivuw;
                                return;
                            }
                            0x6 => {
                                insn.i_type = InsnType::InsnRemw;
                                return;
                            }
                            0x7 => {
                                insn.i_type = InsnType::InsnRemuw;
                                return;
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        0x20 => match funct3 {
                            0x0 => {
                                insn.i_type = InsnType::InsnSubw;
                                return;
                            }
                            0x5 => {
                                insn.i_type = InsnType::InsnSraw;
                                return;
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        _ => {}
                    }
                }
                0x10 => {
                    let funct2 = func_t2!(data);

                    *insn = insn_fprtype_read(data);
                    match funct2 {
                        0x0 => {
                            insn.i_type = InsnType::InsnFmaddS;
                            return;
                        }
                        0x1 => {
                            insn.i_type = InsnType::InsnFmaddD;
                            return;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                0x11 => {
                    let funct2 = func_t2!(data);

                    *insn = insn_fprtype_read(data);
                    match funct2 {
                        0x0 => {
                            insn.i_type = InsnType::InsnFmsubS;
                            return;
                        }
                        0x1 => {
                            insn.i_type = InsnType::InsnFmsubD;
                            return;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                0x12 => {
                    let funct2 = func_t2!(data);

                    *insn = insn_fprtype_read(data);
                    match funct2 {
                        0x0 => {
                            insn.i_type = InsnType::InsnFnmsubS;
                            return;
                        }
                        0x1 => {
                            insn.i_type = InsnType::InsnFnmsubD;
                            return;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                0x13 => {
                    let funct2 = func_t2!(data);

                    *insn = insn_fprtype_read(data);
                    match funct2 {
                        0x0 => {
                            insn.i_type = InsnType::InsnFnmaddS;
                            return;
                        }
                        0x1 => {
                            insn.i_type = InsnType::InsnFnmaddD;
                            return;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                0x14 => {
                    let funct7 = func_t7!(data);

                    *insn = insn_rtype_read(data);
                    match funct7 {
                        0x0 => {
                            insn.i_type = InsnType::InsnFaddS;
                            return;
                        }
                        0x1 => {
                            insn.i_type = InsnType::InsnFaddD;
                            return;
                        }
                        0x4 => {
                            insn.i_type = InsnType::InsnFsubS;
                            return;
                        }
                        0x5 => {
                            insn.i_type = InsnType::InsnFsubD;
                            return;
                        }
                        0x8 => {
                            insn.i_type = InsnType::InsnFmulS;
                            return;
                        }
                        0x9 => {
                            insn.i_type = InsnType::InsnFmulD;
                            return;
                        }
                        0xc => {
                            insn.i_type = InsnType::InsnFdivS;
                            return;
                        }
                        0xd => {
                            insn.i_type = InsnType::InsnFdivD;
                            return;
                        }
                        0x10 => {
                            let funct3 = func_t3!(data);
                            match funct3 {
                                0x0 => {
                                    insn.i_type = InsnType::InsnFsgnjS;
                                    return;
                                }
                                0x1 => {
                                    insn.i_type = InsnType::InsnFsgnjnS;
                                    return;
                                }
                                0x2 => {
                                    insn.i_type = InsnType::InsnFsgnjxS;
                                    return;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                        0x11 => {
                            let funct3 = func_t3!(data);
                            match funct3 {
                                0x0 => {
                                    insn.i_type = InsnType::InsnFsgnjD;
                                    return;
                                }
                                0x1 => {
                                    insn.i_type = InsnType::InsnFsgnjnD;
                                    return;
                                }
                                0x2 => {
                                    insn.i_type = InsnType::InsnFsgnjxD;
                                    return;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                        0x14 => {
                            let funct3 = func_t3!(data);
                            match funct3 {
                                0x0 => {
                                    insn.i_type = InsnType::InsnFminS;
                                    return;
                                }
                                0x1 => {
                                    insn.i_type = InsnType::InsnFmaxS;
                                    return;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                        0x15 => {
                            let funct3 = func_t3!(data);
                            match funct3 {
                                0x0 => {
                                    insn.i_type = InsnType::InsnFminD;
                                    return;
                                }
                                0x1 => {
                                    insn.i_type = InsnType::InsnFmaxD;
                                    return;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                        0x20 => {
                            assert!(rs2!(data) == 1);
                            insn.i_type = InsnType::InsnFcvtSD;
                            return;
                        }
                        0x21 => {
                            assert!(rs2!(data) == 1);
                            insn.i_type = InsnType::InsnFcvtDS;
                            return;
                        }
                        0x2c => {
                            assert!(rs2!(data) == 1);
                            insn.i_type = InsnType::InsnFsqrtS;
                            return;
                        }
                        0x2d => {
                            assert!(rs2!(data) == 1);
                            insn.i_type = InsnType::InsnFsqrtD;
                            return;
                        }
                        0x50 => {
                            let funct3 = func_t3!(data);
                            match funct3 {
                                0x0 => {
                                    insn.i_type = InsnType::InsnFleS;
                                    return;
                                }
                                0x1 => {
                                    insn.i_type = InsnType::InsnFltS;
                                    return;
                                }
                                0x2 => {
                                    insn.i_type = InsnType::InsnFeqS;
                                    return;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                        0x51 => {
                            let funct3 = func_t3!(data);
                            match funct3 {
                                0x0 => {
                                    insn.i_type = InsnType::InsnFleD;
                                    return;
                                }
                                0x1 => {
                                    insn.i_type = InsnType::InsnFltD;
                                    return;
                                }
                                0x2 => {
                                    insn.i_type = InsnType::InsnFeqD;
                                    return;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                        0x60 => {
                            let rs2 = rs2!(data);
                            match rs2 {
                                0x0 => {
                                    insn.i_type = InsnType::InsnFcvtWS;
                                    return;
                                }
                                0x1 => {
                                    insn.i_type = InsnType::InsnFcvtWuS;
                                    return;
                                }
                                0x2 => {
                                    insn.i_type = InsnType::InsnFcvtLS;
                                    return;
                                }
                                0x3 => {
                                    insn.i_type = InsnType::InsnFcvtLuS;
                                    return;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                        0x61 => {
                            let rs2 = rs2!(data);
                            match rs2 {
                                0x0 => {
                                    insn.i_type = InsnType::InsnFcvtWD;
                                    return;
                                }
                                0x1 => {
                                    insn.i_type = InsnType::InsnFcvtWuD;
                                    return;
                                }
                                0x2 => {
                                    insn.i_type = InsnType::InsnFcvtLD;
                                    return;
                                }
                                0x3 => {
                                    insn.i_type = InsnType::InsnFcvtLuD;
                                    return;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                        0x68 => {
                            let rs2 = rs2!(data);
                            match rs2 {
                                0x0 => {
                                    insn.i_type = InsnType::InsnFcvtSW;
                                    return;
                                }
                                0x1 => {
                                    insn.i_type = InsnType::InsnFcvtSWu;
                                    return;
                                }
                                0x2 => {
                                    insn.i_type = InsnType::InsnFcvtSL;
                                    return;
                                }
                                0x3 => {
                                    insn.i_type = InsnType::InsnFcvtSLu;
                                    return;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                        0x69 => {
                            let rs2 = rs2!(data);
                            match rs2 {
                                0x0 => {
                                    insn.i_type = InsnType::InsnFcvtDW;
                                    return;
                                }
                                0x1 => {
                                    insn.i_type = InsnType::InsnFcvtDWu;
                                    return;
                                }
                                0x2 => {
                                    insn.i_type = InsnType::InsnFcvtDL;
                                    return;
                                }
                                0x3 => {
                                    insn.i_type = InsnType::InsnFcvtDLu;
                                    return;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                        0x70 => {
                            let rs2 = rs2!(data);
                            match rs2 {
                                0x0 => {
                                    insn.i_type = InsnType::InsnFmvXW;
                                    return;
                                }
                                0x1 => {
                                    insn.i_type = InsnType::InsnFclassS;
                                    return;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                        0x71 => {
                            let rs2 = rs2!(data);
                            match rs2 {
                                0x0 => {
                                    insn.i_type = InsnType::InsnFmvXD;
                                    return;
                                }
                                0x1 => {
                                    insn.i_type = InsnType::InsnFclassD;
                                    return;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        }
                        0x78 => {
                            assert!(rs2!(data) == 0 && func_t3!(data) == 0);
                            insn.i_type = InsnType::InsnFmvWX;
                            return;
                        }
                        0x79 => {
                            assert!(rs2!(data) == 0 && func_t3!(data) == 0);
                            insn.i_type = InsnType::InsnFmvDX;
                            return;
                        }
                        _ => {}
                    }
                }
                0x18 => {
                    *insn = insn_btype_read(data);

                    let funct3 = func_t3!(data);
                    match funct3 {
                        0x0 => {
                            insn.i_type = InsnType::InsnBeq;
                            return;
                        }
                        0x1 => {
                            insn.i_type = InsnType::InsnBne;
                            return;
                        }
                        0x4 => {
                            insn.i_type = InsnType::InsnBlt;
                            return;
                        }
                        0x5 => {
                            insn.i_type = InsnType::InsnBge;
                            return;
                        }
                        0x6 => {
                            insn.i_type = InsnType::InsnBltu;
                            return;
                        }
                        0x7 => {
                            insn.i_type = InsnType::InsnBgeu;
                            return;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                0x19 => {
                    *insn = insn_itype_read(data);
                    insn.i_type = InsnType::InsnJalr;
                    insn.cont = true;
                    return;
                }
                0x1b => {
                    *insn = insn_jtype_read(data);
                    insn.i_type = InsnType::InsnJal;
                    insn.cont = true;
                    return;
                }
                0x1c => {
                    if data == 0x73 {
                        insn.i_type = InsnType::InsnEcall;
                        insn.cont = true;
                        return;
                    }

                    let funct3 = func_t3!(data);
                    *insn = insn_csrtype_read(data);

                    match funct3 {
                        0x1 => {
                            insn.i_type = InsnType::InsnCsrrw;
                            return;
                        }
                        0x2 => {
                            insn.i_type = InsnType::InsnCsrrs;
                            return;
                        }
                        0x3 => {
                            insn.i_type = InsnType::InsnCsrrc;
                            return;
                        }
                        0x5 => {
                            insn.i_type = InsnType::InsnCsrrwi;
                            return;
                        }
                        0x6 => {
                            insn.i_type = InsnType::InsnCsrrsi;
                            return;
                        }
                        0x7 => {
                            insn.i_type = InsnType::InsnCsrrci;
                            return;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                _ => {
                    unreachable!()
                }
            }
        }
        _ => unreachable!(),
    }
}
