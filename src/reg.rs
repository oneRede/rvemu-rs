pub enum GpRegTypeT {
    Zero,
    RA,
    Sp,
    Gp,
    Tp,
    T0,
    T1,
    T2,
    S0,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
    NumGpRegS,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum FpRegTypeT {
    Ft0,
    Ft1,
    Ft2,
    Ft3,
    Ft4,
    Ft5,
    Ft6,
    Ft7,
    Fs0,
    Fs1,
    Fa0,
    Fa1,
    Fa2,
    Fa3,
    Fa4,
    Fa5,
    Fa6,
    Fa7,
    Fs2,
    Fs3,
    Fs4,
    Fs5,
    Fs6,
    Fs7,
    Fs8,
    Fs9,
    Fs10,
    Fs11,
    Ft8,
    Ft9,
    Ft10,
    Ft11,
    NumFpRegs,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct FpRegT {
    pub v: u64,
    pub w: u32,
    pub d: f64,
    pub f: f32,
}

impl FpRegT {
    pub fn new() -> Self {
        Self {
            v: 0,
            w: 0,
            d: 0f64,
            f: 0f32,
        }
    }
}
