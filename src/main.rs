mod elfdef;
mod rvemu;

fn main() {
    let d8: [u8;8] = [0,0,0,0,0,0,0,1];
    println!("{:?}", d8);
    let ptr8 = &d8 as *const u8;
    let ptr64 = ptr8 as *const u64;
    let dd = unsafe { *ptr64 };
    println!("{:?}", dd);
}
