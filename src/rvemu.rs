use std::{alloc::alloc, alloc::Layout, cmp::max, cmp::min, mem, process::exit};

const GUEST_MEMORY_OFFSET: u64 = 0x088800000000;
const STACK_CAP: usize = 256;
const STR_MAX_PREALLOC: u64 = 1024 * 1024;

macro_rules! fatalf {
    ($($fmt:expr),+) => {
        println!("fatal: {:?}: {:?} [TODO] {:?}\n", file!(), line!(), $($fmt),*);
        exit(1)
    }
}

macro_rules! fatal {
    ($fmt:expr) => {
        fatalf!($fmt);
    };
}

macro_rules! round_down {
    ($x:expr,$k:expr) => {
        ($x) & (-$k)
    };
}

macro_rules! round_up {
    ($x:expr,$k:expr) => {
        ((($x) + ($k) - 1) & -($k))
    };
}

macro_rules! min {
    ($x:expr,$y:expr) => {
        min($x, $y)
    };
}

macro_rules! max {
    ($x:expr,$y:expr) => {
        max($x, $y)
    };
}

macro_rules! array_size {
    ($arr:expr) => {
        $arr.len()
    };
}

macro_rules! to_host {
    ($addr:expr) => {
        addr + GUEST_MEMORY_OFFSET
    };
}

macro_rules! to_guest {
    ($addr:expr) => {
        addr - GUEST_MEMORY_OFFSET
    };
}

enum InsnTypeT {
    InsnLb,
    InsnLh,
    InsnLw,
    InsnLd,
    InsnLbu,
    InsnLhu,
    InsnLwu,
    InsnFence,
    InsnFenceI,
    InsnAddi,
    InsnSlli,
    InsnSlti,
    InsnSltiu,
    InsnXori,
    InsnSrli,
    InsnSrai,
    InsnOri,
    InsnAndi,
    InsnAuipc,
    InsnAddiw,
    InsnSlliw,
    InsnSrliw,
    InsnSraiw,
    InsnSb,
    InsnSh,
    InsnSw,
    InsnSd,
    InsnAdd,
    InsnSll,
    InsnSlt,
    InsnSltu,
    InsnXor,
    InsnSrl,
    InsnOr,
    InsnAnd,
    InsnMul,
    InsnMulh,
    InsnMulhsu,
    InsnMulhu,
    InsnDiv,
    InsnDivu,
    InsnRem,
    InsnRemu,
    InsnSub,
    InsnSra,
    InsnLui,
    InsnAddw,
    InsnSllw,
    InsnSrlw,
    InsnMulw,
    InsnDivw,
    InsnDivuw,
    InsnRemw,
    InsnRemuw,
    InsnSubw,
    InsnSraw,
    InsnBeq,
    InsnBne,
    InsnBlt,
    InsnBge,
    InsnBltu,
    InsnBgeu,
    InsnJalr,
    InsnJal,
    InsnEcall,
    InsnCsrrc,
    InsnCsrrci,
    InsnCsrrs,
    InsnCsrrsi,
    InsnCsrrw,
    InsnCsrrwi,
    InsnFlw,
    InsnFsw,
    InsnFmaddS,
    InsnFmsubS,
    InsnFnmsubS,
    InsnFnmaddS,
    InsnFaddS,
    InsnFsubS,
    InsnFmulS,
    InsnFdivS,
    InsnFsqrtS,
    InsnFsgnjS,
    InsnFsgnjnS,
    InsnFsgnjxS,
    InsnFminS,
    InsnFmaxS,
    InsnFcvtWS,
    InsnFcvtWuS,
    InsnFmvXW,
    InsnFeqS,
    InsnFltS,
    InsnFleS,
    InsnFclassS,
    InsnFcvtSW,
    InsnFcvtSWu,
    InsnFmvWX,
    InsnFcvtLS,
    InsnFcvtLuS,
    InsnFcvtSL,
    InsnFcvtSLu,
    InsnFld,
    InsnFsd,
    InsnFmaddD,
    InsnFmsubD,
    InsnFnmsubD,
    InsnFnmaddD,
    InsnFaddD,
    InsnFsubD,
    InsnFmulD,
    InsnFdivD,
    InsnFsqrtD,
    InsnFsgnjD,
    InsnFsgnjnD,
    InsnFsgnjxD,
    InsnFminD,
    InsnFmaxD,
    InsnFcvtSD,
    InsnFcvtDS,
    InsnFeqD,
    InsnFltD,
    InsnFleD,
    InsnFclassD,
    InsnFcvtWD,
    InsnFcvtWuD,
    InsnFcvtDW,
    InsnFcvtDWu,
    InsnFcvtLD,
    InsnFcvtLuD,
    InsnFmvXD,
    InsnFcvtDL,
    InsnFcvtDLu,
    InsnFmvDX,
    NumInsns,
}

struct InsnT {
    rd: i8,
    rs1: i8,
    rs2: i8,
    rs3: i8,
    imm: i32,
    csr: i16,
    ins: InsnTypeT,
    rvc: bool,
    cont: bool,
}

struct StackT {
    top: i64,
    elems: [u64; STACK_CAP],
}
#[derive(Copy, Clone)]
struct StrHdrT {
    len: u64,
    alloc: u64,
    buf: StrT,
}

macro_rules! str_hdr {
    ($s:expr) => {
        (s - mem::size_of::<StrHdrT>()) as StrHdrT
    };
}

type StrT = *mut char;

macro_rules! declare_static_str {
    ($name:expr) => {
        $name = str_new();
    };
}

fn str_new() -> StrT {
    let layout = Layout::new::<[StrHdrT; 1]>();
    unsafe {
        let ptr = alloc(layout);
        let str_hdr_t = ptr as *mut StrHdrT;
        (*str_hdr_t).buf as *mut char
    }
}

fn str_len(str_t: StrT) -> u64 {
    (unsafe { *(str_t as *mut StrHdrT) }).len
}

struct MmuT {
    entry: u64,
    host_alloc: u64,
    alloc: u64,
    base: u64,
}

// inline void mmu_write(u64 addr, u8 *data, size_t len) {
//     memcpy((void *)TO_HOST(addr), (void *)data, len);
// }

fn mmu_write(mut addr: u64, data: *const u8, len: usize) {
    let ptr64 = data as *const u64;
    addr = unsafe {
        *ptr64
    }
}

#[test]
fn test_fatal() {
    fatal!("test");
}

#[test]
fn test_round_down() {
    let n = 81;
    let k = 8;
    let r = round_down!(n, k);
    println!("{:?}", r);
}

#[test]
fn test_round_up() {
    let n = 81;
    let k = 8;
    let r = round_up!(n, k);
    println!("{:?}", r);
}

#[test]
fn test_min() {
    let x: i32 = 123;
    let y: i32 = 456;
    let min = min!(x, y);
    println!("{:?}", min)
}

#[test]
fn test_max() {
    let x: i32 = 123;
    let y: i32 = 456;
    let min = max!(x, y);
    println!("{:?}", min)
}

#[test]
fn test_arr_size() {
    let arr = [1, 2, 3];
    let vec = vec![1, 2, 3];
    let n = array_size!(arr);
    println!("{:?}", n);
    let n = array_size!(vec);
    println!("{:?}", n)
}
