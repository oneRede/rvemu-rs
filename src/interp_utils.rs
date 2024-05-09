#[inline]
pub fn mulhu(a: u64, b: u64) -> u64 {
    let mut t: u64;
    let mut y1: u32;
    let mut y2: u32;
    let y3: u32;

    let a0 = a;
    let a1 = a >> 32;
    let b0 = b;
    let b1 = b >> 32;

    t = a1 * b0 + (a0 * b0 >> 32);
    y1 = t as u32;
    y2 = (t >> 32) as u32;
    t = a0 * b1 + y1 as u64;
    y1 = t as u32;
    t = a1 * b1 + y1 as u64;
    t = a1 * b1 + (y2 as u64) + (t >> 32) as u64;

    y2 = t as u32;
    y3 = (t >> 32) as u32;

    ((y3 << 32) | y2) as u64
}

#[inline]
pub fn mulh(a: i64, b: i64) -> i64 {
    let negate = (a < 0) != (b < 0);
    let res = mulhu(a.abs() as u64, b.abs() as u64);
    if negate {
        return (!res as i64) + ((a * b == 0) as i64);
    } else {
        return res as i64;
    }
}

#[inline]
pub fn mulhsu(a: i64, b: u64) -> i64 {
    let negate = a < 0;
    let res = mulhu(a.abs() as u64, b);
    if negate {
        return (!res as i64) + ((a * b as i64 == 0) as i64);
    } else {
        return res as i64;
    }
}

#[macro_export]
macro_rules! f32_sign {
    () => {
        1u32 << 31
    };
}

#[macro_export]
macro_rules! f64_sign {
    () => {
        1u64 << 63
    };
}

#[inline]
pub fn fsgnj32(a: u32, b: u32, n: bool, x: bool) -> u32 {
    let t = if n { f32_sign!() } else { 0 };
    let v = if x { a } else { t };

    return (a & !f32_sign!()) | ((v ^ b) & f32_sign!()) as u32;
}

#[inline]
pub fn fsgnj64(a: u64, b: u64, n: bool, x: bool) -> u64 {
    let t = if n { f64_sign!() } else { 0 };
    let v = if x { a } else { t };

    return (a & !f64_sign!()) | ((v ^ b) & f64_sign!()) as u64;
}

#[allow(dead_code)]
struct U32F32 {
    ui: u32,
    f: f32,
}

#[allow(dead_code)]
impl U32F32 {
    pub fn new() -> Self {
        Self { ui: 0, f: 0f32 }
    }
}

#[macro_export]
macro_rules! sign_f32_ui {
    ($data:ident) => {
        (($data as u32) >> 31) != 0
    };
}

#[macro_export]
macro_rules! exp_f32_ui {
    ($data:ident) => {
        ((($data) >> 23) & 0xff) as i16
    };
}

#[macro_export]
macro_rules! frac_f32_ui {
    ($data:ident) => {
        $data & 0x007FFFFF
    };
}

#[macro_export]
macro_rules! is_nan_f32_ui {
    ($data:ident) => {
        ((!$data & 0x7F800000) == 0) && (($data & 0x007FFFFF) != 0)
    };
}

#[macro_export]
macro_rules! is_sign_nan_f32_ui {
    ($data:ident) => {
        ((!$data & 0x7FC00000) == 0x7F800000) && (($data & 0x003FFFFF) != 0)
    };
}

#[inline]
pub fn f32_classify(a: f32) -> u16 {
    let mut u_a = U32F32::new();
    let ui_a: u32;

    u_a.f = a;
    ui_a = u_a.ui;

    let inf_or_nan = exp_f32_ui!(ui_a) == 0xff;
    let subnormal_or_zero = exp_f32_ui!(ui_a) == 0;
    let sign = sign_f32_ui!(ui_a);
    let frac_zero = frac_f32_ui!(ui_a) == 0;
    let is_nan = is_nan_f32_ui!(ui_a);
    let is_s_nan = is_sign_nan_f32_ui!(ui_a);

    ((sign && inf_or_nan && frac_zero) as u16) << 0
        | ((sign && !inf_or_nan && !subnormal_or_zero) as u16) << 1
        | ((sign && subnormal_or_zero && !frac_zero) as u16) << 2
        | ((sign && subnormal_or_zero && frac_zero) as u16) << 3
        | ((!sign && inf_or_nan && frac_zero) as u16) << 7
        | ((!sign && !inf_or_nan && !subnormal_or_zero) as u16) << 6
        | ((!sign && subnormal_or_zero && !frac_zero) as u16) << 5
        | ((!sign && subnormal_or_zero && frac_zero) as u16) << 4
        | ((is_nan && is_s_nan) as u16) << 8
        | ((is_nan && !is_s_nan) as u16) << 9
}

#[allow(dead_code)]
struct U64F64 {
    ui: u64,
    f: f64,
}

#[allow(dead_code)]
impl U64F64 {
    pub fn new() -> Self {
        Self { ui: 0, f: 0f64 }
    }
}

#[macro_export]
macro_rules! sign_f64_ui {
    ($data:ident) => {
        (($data as u64) >> 63) != 0
    };
}

#[macro_export]
macro_rules! exp_f64_ui {
    ($data:ident) => {
        ((($data) >> 52) & 0x7ff) as i16
    };
}

#[macro_export]
macro_rules! frac_f64_ui {
    ($data:ident) => {
        $data & 0x000FFFFFFFFFFFFF
    };
}

#[macro_export]
macro_rules! is_nan_f64_ui {
    ($data:ident) => {
        ((!$data & 0x7FF0000000000000) == 0) && (($data & 0x000FFFFFFFFFFFFF) != 0)
    };
}

#[macro_export]
macro_rules! is_sign_nan_f64_ui {
    ($data:ident) => {
        ((!$data & 0x7FF8000000000000) == 0x7FF0000000000000) && (($data & 0x0007FFFFFFFFFFFF) != 0)
    };
}

#[inline]
pub fn f64_classify(a: f64) -> u16 {
    let mut u_a = U64F64::new();
    let ui_a: u64;

    u_a.f = a;
    ui_a = u_a.ui;

    let inf_or_nan = exp_f64_ui!(ui_a) == 0x7ff;
    let subnormal_or_zero = exp_f64_ui!(ui_a) == 0;
    let sign = sign_f64_ui!(ui_a);
    let frac_zero = frac_f64_ui!(ui_a) == 0;
    let is_nan = is_nan_f64_ui!(ui_a);
    let is_s_nan = is_sign_nan_f64_ui!(ui_a);

    ((sign && inf_or_nan && frac_zero) as u16) << 0
        | ((sign && !inf_or_nan && !subnormal_or_zero) as u16) << 1
        | ((sign && subnormal_or_zero && !frac_zero) as u16) << 2
        | ((sign && subnormal_or_zero && frac_zero) as u16) << 3
        | ((!sign && inf_or_nan && frac_zero) as u16) << 7
        | ((!sign && !inf_or_nan && !subnormal_or_zero) as u16) << 6
        | ((!sign && subnormal_or_zero && !frac_zero) as u16) << 5
        | ((!sign && subnormal_or_zero && frac_zero) as u16) << 4
        | ((is_nan && is_s_nan) as u16) << 8
        | ((is_nan && !is_s_nan) as u16) << 9
}
