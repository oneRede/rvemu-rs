use libc::close;

use crate::{
    fatal,
    reg::GpRegTypeT,
    rvemu::{machine_get_gp_reg, Machine},
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
    ($reg:expr, $name:ident) => {
        // unimplement
    };
}

pub fn sys_unimplemented(m: Machine) {
    fatal!(format!(
        "unimplemented syscall: {}",
        machine_get_gp_reg(m, GpRegTypeT::A7 as i32)
    ));
}

#[allow(dead_code)]
pub fn sys_exit(_m: Machine) -> u64{
    get!(GpRegTypeT::A0, code);
    // TODO: exit implement
    // exit(0)
    0
}

pub fn sys_close(_m: Machine) -> u64{
    get!(GpRegTypeT::A0, fd);
    let fd = 3;
    if fd > 2 {
        return unsafe { close(0) as u64 };
    }
    return 0;
}
