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
    let t = if n {f32_sign!()} else {0};
    let v = if x {a} else {t};

    return (a & !f32_sign!()) | ((v ^b) & f32_sign!()) as u32
}

#[inline]
pub fn fsgnj64(a: u64, b: u64, n: bool, x: bool) -> u64 {
    let t = if n {f64_sign!()} else {0};
    let v = if x {a} else {t};

    return (a & !f64_sign!()) | ((v ^b) & f64_sign!()) as u64
}

#[allow(dead_code)]
struct U32F32{
    ui: u32,
    f: f32,
}

#[macro_export]
macro_rules! sign_f32_ui {
    ($data:ident) => {
        (($data as u32) >> 31) as bool
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
        ((!$data & 0x7F800000) == 0) && ($data & 0x007FFFFF)
    };
}

#[macro_export]
macro_rules! is_sign_nan_f32_ui {
    ($data:ident) => {
        ((!$data & 0x7FC00000) == 0x7F800000) && ($data & 0x003FFFFF)
    };
}