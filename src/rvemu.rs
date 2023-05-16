use std::{cmp::max, cmp::min, process::exit};

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
fn test_arr_size(){
    let arr = [1,2,3];
    let vec = vec![1,2,3];
    let n = array_size!(arr);
    println!("{:?}", n);
    let n = array_size!(vec);
    println!("{:?}", n)
}
