use std::{os::raw::c_void, process::exit, ptr};

use libc::{
    c_char, close, gettimeofday, lseek, open, openat, read, stat, timeval, timezone, O_APPEND,
    O_CREAT, O_EXCL, O_RDONLY, O_RDWR, O_TRUNC, O_WRONLY,
};
use rvemu_rs::rewrite_flag;

use crate::{
    fatal,
    mmu::mmu_alloc,
    reg::GpRegTypeT::{A0, A1, A2, A7},
    rvemu::{machine_get_gp_reg, Machine},
    to_host,
};

pub const SYS_EXIT: usize = 93;
pub const SYS_EXIT_GROUP: usize = 94;
pub const SYS_GETPID: usize = 172;
pub const SYS_KILL: usize = 129;
pub const SYS_TGKILL: usize = 131;
pub const SYS_READ: usize = 63;
pub const SYS_WRITE: usize = 64;
pub const SYS_OPENAT: usize = 56;
pub const SYS_CLOSE: usize = 57;
pub const SYS_LSEEK: usize = 62;
pub const SYS_BRK: usize = 214;
pub const SYS_LINKAT: usize = 37;
pub const SYS_UNLINKAT: usize = 35;
pub const SYS_MKDIRAT: usize = 34;
pub const SYS_RENAMEAT: usize = 38;
pub const SYS_CHDIR: usize = 49;
pub const SYS_GETCWD: usize = 17;
pub const SYS_FSTAT: usize = 80;
pub const SYS_FSTATAT: usize = 79;
pub const SYS_FACCESSAT: usize = 48;
pub const SYS_PREAD: usize = 67;
pub const SYS_PWRITE: usize = 68;
pub const SYS_UNAME: usize = 160;
pub const SYS_GETUID: usize = 174;
pub const SYS_GETEUID: usize = 175;
pub const SYS_GETGID: usize = 176;
pub const SYS_GETEGID: usize = 177;
pub const SYS_GETTID: usize = 178;
pub const SYS_SYSINFO: usize = 179;
pub const SYS_MMAP: usize = 222;
pub const SYS_MUNMAP: usize = 215;
pub const SYS_MREMAP: usize = 216;
pub const SYS_MPROTECT: usize = 226;
pub const SYS_PRLIMIT64: usize = 261;
pub const SYS_GETMAINVARS: usize = 2011;
pub const SYS_RT_SIGACTION: usize = 134;
pub const SYS_WRITEV: usize = 66;
pub const SYS_GETTIMEOFDAY: usize = 169;
pub const SYS_TIMES: usize = 153;
pub const SYS_FCNTL: usize = 25;
pub const SYS_FTRUNCATE: usize = 46;
pub const SYS_GETDENTS: usize = 61;
pub const SYS_DUP: usize = 23;
pub const SYS_DUP3: usize = 24;
pub const SYS_READLINKAT: usize = 78;
pub const SYS_RT_SIGPROCMASK: usize = 135;
pub const SYS_IOCTL: usize = 29;
pub const SYS_GETRLIMIT: usize = 163;
pub const SYS_SETRLIMIT: usize = 164;
pub const SYS_GETRUSAGE: usize = 165;
pub const SYS_CLOCK_GETTIME: usize = 113;
pub const SYS_SET_TID_ADDRESS: usize = 96;
pub const SYS_SET_ROBUST_LIST: usize = 99;
pub const SYS_MADVISE: usize = 233;
pub const SYS_STATX: usize = 291;

pub const OLD_SYSCALL_THRESHOLD: usize = 1024;
pub const SYS_OPEN: usize = 1024;
pub const SYS_LINK: usize = 1025;
pub const SYS_UNLINK: usize = 1026;
pub const SYS_MKDIR: usize = 1030;
pub const SYS_ACCESS: usize = 1033;
pub const SYS_STAT: usize = 1038;
pub const SYS_LSTAT: usize = 1039;
pub const SYS_TIME: usize = 1062;

#[macro_export]
macro_rules! get {
    ($reg:tt, $name:ident, $m:ident) => {
        let $name: u64 = machine_get_gp_reg(*$m, $reg as i32);
    };
}

#[macro_export]
macro_rules! get_mut {
    ($reg:tt, $name:ident, $m:ident) => {
        let mut $name: u64 = machine_get_gp_reg(*$m, $reg as i32);
    };
}

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub fn sys_unimplemented(m: &mut Machine) -> u64 {
    fatal!(format!(
        "unimplemented syscall: {}",
        machine_get_gp_reg(*m, A7 as i32)
    ));
    0
}

#[allow(dead_code)]
pub fn sys_exit(m: &mut Machine) -> u64 {
    get!(A0, code, m);
    exit(code as i32);
}

pub fn sys_close(m: &mut Machine) -> u64 {
    get!(A0, fd, m);
    if fd > 2 {
        return unsafe { close(0) as u64 };
    }
    return 0;
}

pub fn sys_write(m: &mut Machine) -> u64 {
    get!(A0, fd, m);
    get!(A1, ptr, m);
    get!(A2, len, m);
    let pp: *mut u8 = ptr::null_mut();
    let ptr = unsafe { pp.add(to_host!(ptr) as usize) } as *const c_void;
    return unsafe { libc::write(fd as i32, ptr, len as usize) } as u64;
}

pub fn sys_fstat(m: &mut Machine) -> u64 {
    get!(A0, fd, m);
    get!(A1, addr, m);

    let ptr: *mut u8 = ptr::null_mut();
    let ptr: *mut stat = unsafe { ptr.add(to_host!(addr) as usize) } as *mut stat;

    return unsafe { libc::fstat(fd as i32, ptr) as u64 };
}

pub fn sys_gettimeofday(m: &mut Machine) -> u64 {
    get!(A0, tv_addr, m);
    get!(A1, tz_addr, m);

    let ptr: *mut u8 = ptr::null_mut();
    let tv = unsafe { ptr.add(tv_addr as usize) } as *mut timeval;
    let mut tz: *mut timezone = ptr::null_mut();
    if tz_addr != 0 {
        let pp: *mut u8 = ptr::null_mut();
        let pp = unsafe { pp.add(to_host!(tv_addr) as usize) } as *mut timezone;
        tz = pp;
    }
    return unsafe { gettimeofday(tv, tz as *mut timezone) } as u64;
}

pub fn sys_brk(m: &mut Machine) -> u64 {
    get_mut!(A0, addr, m);
    if addr == 0 {
        addr = m.mmu.alloc;
    }
    assert!(addr >= m.mmu.base);
    let incr = (addr - m.mmu.alloc) as i64;
    mmu_alloc(&mut m.mmu, incr);
    return addr;
}

pub const NEWLIB_O_RDONLY: i32 = 0x0;
pub const NEWLIB_O_WRONLY: i32 = 0x1;
pub const NEWLIB_O_RDWR: i32 = 0x2;
pub const NEWLIB_O_APPEND: i32 = 0x8;
pub const NEWLIB_O_CREAT: i32 = 0x200;
pub const NEWLIB_O_TRUNC: i32 = 0x400;
pub const NEWLIB_O_EXCL: i32 = 0x800;

pub fn convert_flags(flags: i32) -> i32 {
    let mut host_flags: i32 = 0;

    rewrite_flag!(O_RDONLY);
    rewrite_flag!(O_WRONLY);
    rewrite_flag!(O_RDWR);
    rewrite_flag!(O_APPEND);
    rewrite_flag!(O_CREAT);
    rewrite_flag!(O_TRUNC);
    rewrite_flag!(O_EXCL);

    return host_flags;
}

pub fn sys_openat(m: &mut Machine) -> u64 {
    get!(A0, dir_fd, m);
    get!(A1, name_ptr, m);
    get!(A2, flags, m);
    let ptr: *mut char = ptr::null_mut();
    let ptr = unsafe { ptr.add(to_host!(name_ptr) as usize) } as *const c_char;
    return unsafe { openat(dir_fd as i32, ptr, flags as i32) } as u64;
}

pub fn sys_open(m: &mut Machine) -> u64 {
    get!(A0, name_ptr, m);
    get!(A1, flags, m);
    let ptr: *mut char = ptr::null_mut();
    let ptr = unsafe { ptr.add(to_host!(name_ptr) as usize) } as *const c_char;
    let ret = unsafe { open(ptr, flags as i32) } as u64;
    return ret;
}

pub fn sys_lseek(m: &mut Machine) -> u64 {
    get!(A0, fd, m);
    get!(A1, offset, m);
    get!(A2, when_ce, m);

    return unsafe { lseek(fd as i32, offset as i64, when_ce as i32) as u64 };
}

pub fn sys_read(m: &mut Machine) -> u64 {
    get!(A0, fd, m);
    get!(A1, buf_ptr, m);
    get!(A2, count, m);

    let ptr: *mut char = ptr::null_mut();
    let ptr = unsafe { ptr.add(to_host!(buf_ptr) as usize) } as *mut c_void;

    return unsafe { read(fd as i32, ptr, count as usize) as u64 };
}

pub const SYSCALL_TABLE: [fn(&mut Machine) -> u64; 42] = [
    sys_exit,
    sys_exit,
    sys_read,
    sys_unimplemented,
    sys_write,
    sys_openat,
    sys_close,
    sys_fstat,
    sys_unimplemented,
    sys_lseek,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_brk,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_gettimeofday,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
];

pub const OLD_SYSCALL_TABLE: [fn(&mut Machine) -> u64; 8] = [
    sys_open,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
    sys_unimplemented,
];

pub fn do_syscall(m: &mut Machine, n: u64) -> u64 {
    let mut f: Option<fn(&mut Machine) -> u64> = None;
    if n < SYSCALL_TABLE.len() as u64 {
        f = Some(SYSCALL_TABLE[n as usize])
    } else if (n as usize - OLD_SYSCALL_THRESHOLD) < OLD_SYSCALL_TABLE.len() {
        f = Some(OLD_SYSCALL_TABLE[n as usize - OLD_SYSCALL_THRESHOLD])
    }
    if f.is_none() {
        fatal!("unknown syscall");
    }
    return f.unwrap()(m);
}
