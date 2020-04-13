#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(asm, const_raw_ptr_to_usize_cast, extern_types, main,
           ptr_wrapping_offset_from, register_tool)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type cmp_map;
    pub type __dirstream;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn setenv(__name: *const libc::c_char, __value: *const libc::c_char,
              __replace: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn afl_shm_init(_: *mut sharedmem_t, _: size_t, dumb_mode: libc::c_uchar)
     -> *mut u8_0;
    #[no_mangle]
    fn afl_shm_deinit(_: *mut sharedmem_t);
    #[no_mangle]
    fn afl_fsrv_init(fsrv: *mut afl_forkserver_t);
    #[no_mangle]
    fn afl_fsrv_start(fsrv: *mut afl_forkserver_t,
                      argv: *mut *mut libc::c_char, stop_soon_p: *mut u8_0,
                      debug_child_output: u8_0);
    #[no_mangle]
    fn afl_fsrv_killall();
    #[no_mangle]
    fn afl_fsrv_deinit(fsrv: *mut afl_forkserver_t);
    #[no_mangle]
    fn access(__name: *const libc::c_char, __type: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int)
     -> __off_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn dup(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn execv(__path: *const libc::c_char, __argv: *const *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn setsid() -> __pid_t;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn setitimer(__which: __itimer_which_t, __new: *const itimerval,
                 __old: *mut itimerval) -> libc::c_int;
    #[no_mangle]
    fn detect_file_args(argv: *mut *mut libc::c_char, prog_in: *mut u8_0,
                        use_stdin: *mut u8_0);
    #[no_mangle]
    fn check_environment_vars(env: *mut *mut libc::c_char);
    #[no_mangle]
    fn argv_cpy_dup(argc: libc::c_int, argv: *mut *mut libc::c_char)
     -> *mut *mut libc::c_char;
    #[no_mangle]
    fn argv_cpy_free(argv: *mut *mut libc::c_char);
    #[no_mangle]
    fn get_qemu_argv(own_loc: *mut u8_0, target_path_p: *mut *mut u8_0,
                     argc: libc::c_int, argv: *mut *mut libc::c_char)
     -> *mut *mut libc::c_char;
    #[no_mangle]
    fn get_wine_argv(own_loc: *mut u8_0, target_path_p: *mut *mut u8_0,
                     argc: libc::c_int, argv: *mut *mut libc::c_char)
     -> *mut *mut libc::c_char;
    #[no_mangle]
    fn get_afl_env(env: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    static mut be_quiet: u8_0;
    #[no_mangle]
    static mut doc_path: *mut u8_0;
    #[no_mangle]
    fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
              __shortopts: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                 __oact: *mut sigaction) -> libc::c_int;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
               __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit)
     -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __rlim_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type u8_0 = uint8_t;
pub type u32_0 = uint32_t;
pub type u64_0 = libc::c_ulonglong;
pub type s32 = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sharedmem {
    pub shm_id: s32,
    pub cmplog_shm_id: s32,
    pub map: *mut u8_0,
    pub size_alloc: size_t,
    pub size_used: size_t,
    pub cmplog_mode: libc::c_int,
    pub cmp_map: *mut cmp_map,
}
/*
   american fuzzy lop++ - shared memory related header
   ---------------------------------------------------

   Originally written by Michal Zalewski

   Forkserver design by Jann Horn <jannhorn@googlemail.com>

   Now maintained by Marc Heuse <mh@mh-sec.de>,
                     Heiko Eißfeldt <heiko.eissfeldt@hexco.de>,
                     Andrea Fioraldi <andreafioraldi@gmail.com>,
                     Dominik Maier <mail@dmnk.co>

   Copyright 2016, 2017 Google Inc. All rights reserved.
   Copyright 2019-2020 AFLplusplus Project. All rights reserved.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at:

     http://www.apache.org/licenses/LICENSE-2.0

   Shared code to handle the shared memory. This is used by the fuzzer
   as well the other components like afl-tmin, afl-showmap, etc...

 */
pub type sharedmem_t = sharedmem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct afl_forkserver {
    pub uses_asan: u8_0,
    pub trace_bits: *mut u8_0,
    pub use_stdin: u8_0,
    pub fsrv_pid: s32,
    pub child_pid: s32,
    pub out_dir_fd: s32,
    pub out_fd: s32,
    pub dev_urandom_fd: s32,
    pub dev_null_fd: s32,
    pub fsrv_ctl_fd: s32,
    pub fsrv_st_fd: s32,
    pub exec_tmout: u32_0,
    pub map_size: u32_0,
    pub snapshot: u32_0,
    pub mem_limit: u64_0,
    pub out_file: *mut u8_0,
    pub target_path: *mut u8_0,
    pub plot_file: *mut FILE,
    pub child_timed_out: u8_0,
    pub use_fauxsrv: u8_0,
    pub prev_timed_out: u32_0,
    pub qemu_mode: u8_0,
    pub cmplog_binary: *mut libc::c_char,
    pub init_child_func: Option<unsafe extern "C" fn(_: *mut afl_forkserver,
                                                     _:
                                                         *mut *mut libc::c_char)
                                    -> ()>,
    pub function_opt: *mut u8_0,
    pub function_ptr: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: *mut u8_0, _: u32_0)
                                 -> ()>,
}
// extern unsigned char *trace_bits;
/* ID of the SHM region              */
/* shared memory region */
/* actual allocated size */
/* in use by shmem app */
/*
   american fuzzy lop++ - forkserver header
   ----------------------------------------

   Originally written by Michal Zalewski

   Forkserver design by Jann Horn <jannhorn@googlemail.com>

   Now maintained by Marc Heuse <mh@mh-sec.de>,
                     Heiko Eißfeldt <heiko.eissfeldt@hexco.de>,
                     Andrea Fioraldi <andreafioraldi@gmail.com>,
                     Dominik Maier <mail@dmnk.co>>

   Copyright 2016, 2017 Google Inc. All rights reserved.
   Copyright 2019-2020 AFLplusplus Project. All rights reserved.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at:

     http://www.apache.org/licenses/LICENSE-2.0

   Shared code that implements a forkserver. This is used by the fuzzer
   as well the other components like afl-tmin.

 */
pub type afl_forkserver_t = afl_forkserver;
pub type __itimer_which = libc::c_uint;
pub const ITIMER_PROF: __itimer_which = 2;
pub const ITIMER_VIRTUAL: __itimer_which = 1;
pub const ITIMER_REAL: __itimer_which = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
pub type __itimer_which_t = libc::c_int;
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut siginfo_t,
                                                  _: *mut libc::c_void)
                                 -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type C2RustUnnamed_10 = libc::c_uint;
pub const DT_WHT: C2RustUnnamed_10 = 14;
pub const DT_SOCK: C2RustUnnamed_10 = 12;
pub const DT_LNK: C2RustUnnamed_10 = 10;
pub const DT_REG: C2RustUnnamed_10 = 8;
pub const DT_BLK: C2RustUnnamed_10 = 6;
pub const DT_DIR: C2RustUnnamed_10 = 4;
pub const DT_CHR: C2RustUnnamed_10 = 2;
pub const DT_FIFO: C2RustUnnamed_10 = 1;
pub const DT_UNKNOWN: C2RustUnnamed_10 = 0;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = libc::c_int;
#[inline]
unsafe extern "C" fn DFL_ck_alloc_nozero(mut size: u32_0)
 -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 { return 0 as *mut libc::c_void }
    if size > 0x40000000 as libc::c_int as libc::c_uint {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBad alloc request: %u bytes\x00"
                   as *const u8 as *const libc::c_char, size);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"include/alloc-inl.h\x00" as *const u8 as *const libc::c_char,
               92 as libc::c_int);
        abort();
    }
    ret = malloc(size as libc::c_ulong);
    if ret.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mOut of memory: can\'t allocate %u bytes\x00"
                   as *const u8 as *const libc::c_char, size);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"include/alloc-inl.h\x00" as *const u8 as *const libc::c_char,
               94 as libc::c_int);
        abort();
    }
    return ret;
}
#[inline]
unsafe extern "C" fn DFL_ck_alloc(mut size: u32_0) -> *mut libc::c_void {
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 { return 0 as *mut libc::c_void }
    mem = DFL_ck_alloc_nozero(size);
    return memset(mem, 0 as libc::c_int, size as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn DFL_ck_free(mut mem: *mut libc::c_void) {
    if mem.is_null() { return }
    free(mem);
}
#[inline]
unsafe extern "C" fn DFL_ck_strdup(mut str: *mut u8_0) -> *mut u8_0 {
    let mut ret: *mut u8_0 = 0 as *mut u8_0;
    let mut size: u32_0 = 0;
    if str.is_null() { return 0 as *mut u8_0 }
    size =
        strlen(str as
                   *mut libc::c_char).wrapping_add(1 as libc::c_int as
                                                       libc::c_ulong) as
            u32_0;
    if size > 0x40000000 as libc::c_int as libc::c_uint {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBad alloc request: %u bytes\x00"
                   as *const u8 as *const libc::c_char, size);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"include/alloc-inl.h\x00" as *const u8 as *const libc::c_char,
               172 as libc::c_int);
        abort();
    }
    ret = malloc(size as libc::c_ulong) as *mut u8_0;
    if ret.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mOut of memory: can\'t allocate %u bytes\x00"
                   as *const u8 as *const libc::c_char, size);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"include/alloc-inl.h\x00" as *const u8 as *const libc::c_char,
               174 as libc::c_int);
        abort();
    }
    return memcpy(ret as *mut libc::c_void, str as *const libc::c_void,
                  size as libc::c_ulong) as *mut u8_0;
}
/* a program that includes afl-forkserver needs to define these */
/* Target uses ASAN?                */
/* SHM with instrumentation bitmap  */
/* use stdin for sending data       */
/* PID of the fork server           */
/* PID of the fuzzed program        */
/* FD of the lock file              */
/* Persistent fd for fsrv->out_file */
/* Persistent fd for /dev/urandom   */
/* Persistent fd for /dev/null      */
/* Fork server control pipe (write) */
/* Fork server status pipe (read)   */
/* Configurable exec timeout (ms)   */
/* map size used by the target      */
/* is snapshot feature used         */
/* Memory cap for child (MB)        */
/* File to fuzz, if any             */
/* Path of the target */
/* Gnuplot output file              */
/* Traced process timed out?        */
/* Fauxsrv for non-forking targets? */
/* if prev forkserver run timed out */
/* if running in qemu mode or not   */
/* the name of the cmplog binary    */
/* Function to kick off the forkserver child */
/* for autodictionary: afl ptr      */
/*
   american fuzzy lop++ - map display utility
   ------------------------------------------

   Originally written by Michal Zalewski

   Forkserver design by Jann Horn <jannhorn@googlemail.com>

   Now maintained by Marc Heuse <mh@mh-sec.de>,
                        Heiko Eißfeldt <heiko.eissfeldt@hexco.de> and
                        Andrea Fioraldi <andreafioraldi@gmail.com>

   Copyright 2016, 2017 Google Inc. All rights reserved.
   Copyright 2019-2020 AFLplusplus Project. All rights reserved.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at:

     http://www.apache.org/licenses/LICENSE-2.0

   A very simple tool that runs the targeted binary and displays
   the contents of the trace bitmap in a human-readable form. Useful in
   scripts to eliminate redundant inputs and perform other checks.

   Exit code is 2 if the target program crashes; 1 if it times out or
   there is a problem executing it; or 0 if execution is successful.

 */
static mut stdin_file: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* stdin file                        */
static mut in_dir: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
/* input folder                      */
static mut at_file: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
/* Substitution string for @@             */
static mut in_data: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
/* Input data                        */
static mut total: u32_0 = 0;
static mut highest: u32_0 = 0;
/* tuple content information         */
static mut in_len: u32_0 = 0;
/* Input data length                 */
static mut arg_offset: u32_0 = 0;
static mut total_execs: u32_0 = 0;
/* Total number of execs             */
static mut quiet_mode: u8_0 = 0;
/* Hide non-essential messages?      */
static mut edges_only: u8_0 = 0;
/* Ignore hit counts?                */
static mut raw_instr_output: u8_0 = 0;
/* Do not apply AFL filters          */
static mut cmin_mode: u8_0 = 0;
/* Generate output in afl-cmin mode? */
static mut binary_mode: u8_0 = 0;
/* Write output as a binary map      */
static mut keep_cores: u8_0 = 0;
/* Allow coredumps?                  */
static mut stop_soon: u8_0 = 0;
/* Ctrl-C pressed?                   */
static mut child_crashed: u8_0 = 0;
/* Child crashed?                    */
/* Classify tuple counts. Instead of mapping to individual bits, as in
   afl-fuzz.c, we map to more user-friendly numbers between 1 and 8. */
static mut count_class_human: [u8_0; 256] =
    [0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     2 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     4 as libc::c_int as u8_0, 4 as libc::c_int as u8_0,
     4 as libc::c_int as u8_0, 4 as libc::c_int as u8_0,
     5 as libc::c_int as u8_0, 5 as libc::c_int as u8_0,
     5 as libc::c_int as u8_0, 5 as libc::c_int as u8_0,
     5 as libc::c_int as u8_0, 5 as libc::c_int as u8_0,
     5 as libc::c_int as u8_0, 5 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0];
static mut count_class_binary: [u8_0; 256] =
    [0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     2 as libc::c_int as u8_0, 4 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     16 as libc::c_int as u8_0, 16 as libc::c_int as u8_0,
     16 as libc::c_int as u8_0, 16 as libc::c_int as u8_0,
     16 as libc::c_int as u8_0, 16 as libc::c_int as u8_0,
     16 as libc::c_int as u8_0, 16 as libc::c_int as u8_0,
     32 as libc::c_int as u8_0, 32 as libc::c_int as u8_0,
     32 as libc::c_int as u8_0, 32 as libc::c_int as u8_0,
     32 as libc::c_int as u8_0, 32 as libc::c_int as u8_0,
     32 as libc::c_int as u8_0, 32 as libc::c_int as u8_0,
     32 as libc::c_int as u8_0, 32 as libc::c_int as u8_0,
     32 as libc::c_int as u8_0, 32 as libc::c_int as u8_0,
     32 as libc::c_int as u8_0, 32 as libc::c_int as u8_0,
     32 as libc::c_int as u8_0, 32 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 64 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
     128 as libc::c_int as u8_0, 128 as libc::c_int as u8_0];
unsafe extern "C" fn classify_counts(mut mem: *mut u8_0,
                                     mut map: *const u8_0) {
    let mut i: u32_0 = ((1 as libc::c_int) << 16 as libc::c_int) as u32_0;
    if edges_only != 0 {
        loop  {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 != 0) { break ; }
            if *mem != 0 { *mem = 1 as libc::c_int as u8_0 }
            mem = mem.offset(1)
        }
    } else if raw_instr_output == 0 {
        loop  {
            let fresh1 = i;
            i = i.wrapping_sub(1);
            if !(fresh1 != 0) { break ; }
            *mem = *map.offset(*mem as isize);
            mem = mem.offset(1)
        }
    };
}
/* Get rid of temp files (atexit handler). */
unsafe extern "C" fn at_exit_handler() {
    if !stdin_file.is_null() { unlink(stdin_file); };
}
/* Write results. */
unsafe extern "C" fn write_results_to_file(mut fsrv: *mut afl_forkserver_t,
                                           mut outfile: *mut u8_0) -> u32_0 {
    let mut fd: s32 = 0; /* Ignore errors */
    let mut i: u32_0 = 0;
    let mut ret: u32_0 = 0 as libc::c_int as u32_0;
    let mut cco: u8_0 =
        !getenv(b"AFL_CMIN_CRASHES_ONLY\x00" as *const u8 as
                    *const libc::c_char).is_null() as libc::c_int as u8_0;
    let mut caa: u8_0 =
        !getenv(b"AFL_CMIN_ALLOW_ANY\x00" as *const u8 as
                    *const libc::c_char).is_null() as libc::c_int as u8_0;
    if strncmp(outfile as *const libc::c_char,
               b"/dev/\x00" as *const u8 as *const libc::c_char,
               5 as libc::c_int as libc::c_ulong) == 0 {
        fd = open(outfile as *const libc::c_char, 0o1 as libc::c_int);
        if fd < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                       as *const u8 as *const libc::c_char, (*fsrv).out_file);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 157 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
    } else if strcmp(outfile as *const libc::c_char,
                     b"-\x00" as *const u8 as *const libc::c_char) == 0 {
        fd = dup(1 as libc::c_int);
        if fd < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open stdout\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 162 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
    } else {
        unlink(outfile as *const libc::c_char);
        fd =
            open(outfile as *const libc::c_char,
                 0o1 as libc::c_int | 0o100 as libc::c_int |
                     0o200 as libc::c_int, 0o600 as libc::c_int);
        if fd < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                       as *const u8 as *const libc::c_char, outfile);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 168 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
    }
    if binary_mode != 0 {
        i = 0 as libc::c_int as u32_0;
        while i < ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint {
            if *(*fsrv).trace_bits.offset(i as isize) != 0 {
                ret = ret.wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
        let mut _len: u32_0 =
            ((1 as libc::c_int) << 16 as libc::c_int) as u32_0;
        let mut _res: s32 =
            write(fd, (*fsrv).trace_bits as *const libc::c_void,
                  _len as size_t) as s32;
        if _res as libc::c_uint != _len {
            if _res < 0 as libc::c_int {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char, outfile);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-showmap.c\x00" as *const u8 as
                           *const libc::c_char, 177 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            } else {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char, outfile);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-showmap.c\x00" as *const u8 as
                           *const libc::c_char, 177 as libc::c_int);
                exit(1 as libc::c_int);
            }
        }
        close(fd);
    } else {
        let mut f: *mut FILE =
            fdopen(fd, b"w\x00" as *const u8 as *const libc::c_char);
        if f.is_null() {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mfdopen() failed\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 184 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        i = 0 as libc::c_int as u32_0;
        while i < ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint {
            if !(*(*fsrv).trace_bits.offset(i as isize) == 0) {
                ret = ret.wrapping_add(1);
                total =
                    (total as
                         libc::c_uint).wrapping_add(*(*fsrv).trace_bits.offset(i
                                                                                   as
                                                                                   isize)
                                                        as libc::c_uint) as
                        u32_0 as u32_0;
                if highest <
                       *(*fsrv).trace_bits.offset(i as isize) as libc::c_uint
                   {
                    highest = *(*fsrv).trace_bits.offset(i as isize) as u32_0
                }
                if cmin_mode != 0 {
                    if (*fsrv).child_timed_out != 0 { break ; }
                    if caa == 0 &&
                           child_crashed as libc::c_int != cco as libc::c_int
                       {
                        break ;
                    }
                    fprintf(f,
                            b"%u%u\n\x00" as *const u8 as *const libc::c_char,
                            *(*fsrv).trace_bits.offset(i as isize) as
                                libc::c_int, i);
                } else {
                    fprintf(f,
                            b"%06u:%u\n\x00" as *const u8 as
                                *const libc::c_char, i,
                            *(*fsrv).trace_bits.offset(i as isize) as
                                libc::c_int);
                }
            }
            i = i.wrapping_add(1)
        }
        fclose(f);
    }
    return ret;
}
/* Write results. */
unsafe extern "C" fn write_results(mut fsrv: *mut afl_forkserver_t) -> u32_0 {
    return write_results_to_file(fsrv, (*fsrv).out_file);
}
/* Write modified data to file for testing. If use_stdin is clear, the old file
   is unlinked and a new one is created. Otherwise, out_fd is rewound and
   truncated. */
unsafe extern "C" fn write_to_testcase(mut fsrv: *mut afl_forkserver_t,
                                       mut mem: *mut libc::c_void,
                                       mut len: u32_0) {
    lseek((*fsrv).out_fd, 0 as libc::c_int as __off_t, 0 as libc::c_int);
    let mut _len: u32_0 = len;
    let mut _res: s32 = write((*fsrv).out_fd, mem, _len as size_t) as s32;
    if _res as libc::c_uint != _len {
        if _res < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                       as *const u8 as *const libc::c_char, (*fsrv).out_file);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 230 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        } else {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                       as *const u8 as *const libc::c_char, (*fsrv).out_file);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 230 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    if ftruncate((*fsrv).out_fd, len as __off_t) != 0 {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mftruncate() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-showmap.c\x00" as *const u8 as *const libc::c_char,
               231 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    lseek((*fsrv).out_fd, 0 as libc::c_int as __off_t, 0 as libc::c_int);
}
/* Execute target application. Returns 0 if the changes are a dud, or
   1 if they should be kept. */
unsafe extern "C" fn run_target_forkserver(mut fsrv: *mut afl_forkserver_t,
                                           mut argv: *mut *mut libc::c_char,
                                           mut mem: *mut u8_0, mut len: u32_0)
 -> u8_0 {
    let mut it: itimerval =
        itimerval{it_interval: timeval{tv_sec: 0, tv_usec: 0,},
                  it_value: timeval{tv_sec: 0, tv_usec: 0,},};
    let mut status: libc::c_int = 0 as libc::c_int;
    memset((*fsrv).trace_bits as *mut libc::c_void, 0 as libc::c_int,
           ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong);
    asm!("" : : : "memory" : "volatile");
    write_to_testcase(fsrv, mem as *mut libc::c_void, len);
    let mut res: s32 = 0;
    /* we have the fork server up and running, so simply
     tell it to have at it, and then read back PID. */
    res =
        write((*fsrv).fsrv_ctl_fd,
              &mut (*fsrv).prev_timed_out as *mut u32_0 as
                  *const libc::c_void, 4 as libc::c_int as size_t) as s32;
    if res != 4 as libc::c_int {
        if stop_soon != 0 { return 0 as libc::c_int as u8_0 }
        if res < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to request new process from fork server (OOM?)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 258 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        } else {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to request new process from fork server (OOM?)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 258 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    res =
        read((*fsrv).fsrv_st_fd,
             &mut (*fsrv).child_pid as *mut s32 as *mut libc::c_void,
             4 as libc::c_int as size_t) as s32;
    if res != 4 as libc::c_int {
        if stop_soon != 0 { return 0 as libc::c_int as u8_0 }
        if res < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to request new process from fork server (OOM?)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 265 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        } else {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to request new process from fork server (OOM?)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 265 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    if (*fsrv).child_pid <= 0 as libc::c_int {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mFork server is misbehaving (OOM?)\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-showmap.c\x00" as *const u8 as *const libc::c_char,
               269 as libc::c_int);
        exit(1 as libc::c_int);
    }
    /* Configure timeout, wait for child, cancel timeout. */
    if (*fsrv).exec_tmout != 0 {
        it.it_value.tv_sec =
            (*fsrv).exec_tmout.wrapping_div(1000 as libc::c_int as
                                                libc::c_uint) as __time_t;
        it.it_value.tv_usec =
            (*fsrv).exec_tmout.wrapping_rem(1000 as libc::c_int as
                                                libc::c_uint).wrapping_mul(1000
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
                as __suseconds_t
    }
    setitimer(ITIMER_REAL as libc::c_int, &mut it, 0 as *mut itimerval);
    res =
        read((*fsrv).fsrv_st_fd,
             &mut status as *mut libc::c_int as *mut libc::c_void,
             4 as libc::c_int as size_t) as s32;
    if res != 4 as libc::c_int {
        if stop_soon != 0 { return 0 as libc::c_int as u8_0 }
        if res < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to communicate with fork server (OOM?)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 285 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        } else {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to communicate with fork server (OOM?)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 285 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    (*fsrv).child_pid = 0 as libc::c_int;
    it.it_value.tv_sec = 0 as libc::c_int as __time_t;
    it.it_value.tv_usec = 0 as libc::c_int as __suseconds_t;
    setitimer(ITIMER_REAL as libc::c_int, &mut it, 0 as *mut itimerval);
    asm!("" : : : "memory" : "volatile");
    /* Clean up bitmap, analyze exit condition, etc. */
    if *((*fsrv).trace_bits as *mut u32_0) == 0xfee1dead as libc::c_uint {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to execute \'%s\'\x00"
                   as *const u8 as *const libc::c_char,
               *argv.offset(0 as libc::c_int as isize));
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-showmap.c\x00" as *const u8 as *const libc::c_char,
               300 as libc::c_int);
        exit(1 as libc::c_int);
    }
    classify_counts((*fsrv).trace_bits,
                    if binary_mode as libc::c_int != 0 {
                        count_class_binary.as_ptr()
                    } else { count_class_human.as_ptr() });
    total_execs = total_execs.wrapping_add(1);
    if stop_soon != 0 {
        printf(b"\x1b[0m\x1b[1;91m\n+++ afl-showmap folder mode aborted by user +++\n\x1b[0m\x00"
                   as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    /* Always discard inputs that time out. */
    if (*fsrv).child_timed_out != 0 { return 0 as libc::c_int as u8_0 }
    /* Handle crashing inputs depending on current mode. */
    if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar as
           libc::c_int >> 1 as libc::c_int > 0 as libc::c_int ||
           status & 0x7f as libc::c_int == 0 as libc::c_int &&
               (status & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                   86 as libc::c_int ||
           status & 0x7f as libc::c_int == 0 as libc::c_int &&
               (status & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0 {
        return 0 as libc::c_int as u8_0
    }
    return 0 as libc::c_int as u8_0;
}
/* Read initial file. */
#[no_mangle]
pub unsafe extern "C" fn read_file(mut in_file: *mut u8_0) -> u32_0 {
    let mut st: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut fd: s32 = open(in_file as *const libc::c_char, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mUnable to open \'%s\'\x00"
                   as *const u8 as *const libc::c_char, in_file);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    if fstat(fd, &mut st) != 0 || st.st_size == 0 {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mZero-sized input file \'%s\'.\x00"
                   as *const u8 as *const libc::c_char, in_file);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    in_len = st.st_size as u32_0;
    in_data = DFL_ck_alloc_nozero(in_len) as *mut u8_0;
    let mut _len: u32_0 = in_len;
    let mut _res: s32 =
        read(fd, in_data as *mut libc::c_void, _len as size_t) as s32;
    if _res as libc::c_uint != _len {
        if _res < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort read from %s\x00"
                       as *const u8 as *const libc::c_char, in_file);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 346 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        } else {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort read from %s\x00"
                       as *const u8 as *const libc::c_char, in_file);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 346 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    close(fd);
    // OKF("Read %u byte%s from '%s'.", in_len, in_len == 1 ? "" : "s", in_file);
    return in_len;
}
/* Execute target application. */
unsafe extern "C" fn run_target(mut fsrv: *mut afl_forkserver_t,
                                mut argv: *mut *mut libc::c_char) {
    static mut it: itimerval =
        itimerval{it_interval: timeval{tv_sec: 0, tv_usec: 0,},
                  it_value: timeval{tv_sec: 0, tv_usec: 0,},};
    let mut status: libc::c_int = 0 as libc::c_int;
    if quiet_mode == 0 {
        printf(b"-- Program output begins --\n\x1b[0m\x00" as *const u8 as
                   *const libc::c_char);
    }
    asm!("" : : : "memory" : "volatile");
    (*fsrv).child_pid = fork();
    if (*fsrv).child_pid < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mfork() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-showmap.c\x00" as *const u8 as *const libc::c_char,
               369 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    if (*fsrv).child_pid == 0 {
        let mut r: rlimit = rlimit{rlim_cur: 0, rlim_max: 0,};
        if quiet_mode != 0 {
            let mut fd: s32 =
                open(b"/dev/null\x00" as *const u8 as *const libc::c_char,
                     0o2 as libc::c_int);
            if fd < 0 as libc::c_int ||
                   dup2(fd, 1 as libc::c_int) < 0 as libc::c_int ||
                   dup2(fd, 2 as libc::c_int) < 0 as libc::c_int {
                *((*fsrv).trace_bits as *mut u32_0) =
                    0xfee1dead as libc::c_uint;
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mDescriptor initialization failed\x00"
                           as *const u8 as *const libc::c_char);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-showmap.c\x00" as *const u8 as
                           *const libc::c_char, 382 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            }
            close(fd);
        }
        if (*fsrv).mem_limit != 0 {
            r.rlim_cur = ((*fsrv).mem_limit as rlim_t) << 20 as libc::c_int;
            r.rlim_max = r.rlim_cur;
            setrlimit(RLIMIT_AS as libc::c_int, &mut r);
            /* Ignore errors */
            /* ^RLIMIT_AS */
        } /* Ignore errors */
        if keep_cores == 0 {
            r.rlim_cur = 0 as libc::c_int as rlim_t;
            r.rlim_max = r.rlim_cur
        } else {
            r.rlim_cur = -(1 as libc::c_int) as __rlim_t;
            r.rlim_max = r.rlim_cur
        }
        setrlimit(RLIMIT_CORE as libc::c_int, &mut r);
        if getenv(b"LD_BIND_LAZY\x00" as *const u8 as
                      *const libc::c_char).is_null() {
            setenv(b"LD_BIND_NOW\x00" as *const u8 as *const libc::c_char,
                   b"1\x00" as *const u8 as *const libc::c_char,
                   0 as libc::c_int);
        }
        setsid();
        execv((*fsrv).target_path as *const libc::c_char,
              argv as *const *mut libc::c_char);
        *((*fsrv).trace_bits as *mut u32_0) = 0xfee1dead as libc::c_uint;
        exit(0 as libc::c_int);
    }
    /* Configure timeout, wait for child, cancel timeout. */
    if (*fsrv).exec_tmout != 0 {
        (*fsrv).child_timed_out = 0 as libc::c_int as u8_0;
        it.it_value.tv_sec =
            (*fsrv).exec_tmout.wrapping_div(1000 as libc::c_int as
                                                libc::c_uint) as __time_t;
        it.it_value.tv_usec =
            (*fsrv).exec_tmout.wrapping_rem(1000 as libc::c_int as
                                                libc::c_uint).wrapping_mul(1000
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
                as __suseconds_t
    }
    setitimer(ITIMER_REAL as libc::c_int, &mut it, 0 as *mut itimerval);
    if waitpid((*fsrv).child_pid, &mut status, 0 as libc::c_int) <=
           0 as libc::c_int {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mwaitpid() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-showmap.c\x00" as *const u8 as *const libc::c_char,
               436 as libc::c_int);
        exit(1 as libc::c_int);
    }
    (*fsrv).child_pid = 0 as libc::c_int;
    it.it_value.tv_sec = 0 as libc::c_int as __time_t;
    it.it_value.tv_usec = 0 as libc::c_int as __suseconds_t;
    setitimer(ITIMER_REAL as libc::c_int, &mut it, 0 as *mut itimerval);
    asm!("" : : : "memory" : "volatile");
    /* Clean up bitmap, analyze exit condition, etc. */
    if *((*fsrv).trace_bits as *mut u32_0) == 0xfee1dead as libc::c_uint {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to execute \'%s\'\x00"
                   as *const u8 as *const libc::c_char,
               *argv.offset(0 as libc::c_int as isize));
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-showmap.c\x00" as *const u8 as *const libc::c_char,
               448 as libc::c_int);
        exit(1 as libc::c_int);
    }
    classify_counts((*fsrv).trace_bits,
                    if binary_mode as libc::c_int != 0 {
                        count_class_binary.as_ptr()
                    } else { count_class_human.as_ptr() });
    if quiet_mode == 0 {
        printf(b"\x1b[0m-- Program output ends --\n\x00" as *const u8 as
                   *const libc::c_char);
    }
    if (*fsrv).child_timed_out == 0 && stop_soon == 0 &&
           ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as
               libc::c_schar as libc::c_int >> 1 as libc::c_int >
               0 as libc::c_int {
        ::std::ptr::write_volatile(&mut child_crashed as *mut u8_0,
                                   1 as libc::c_int as u8_0)
    }
    if quiet_mode == 0 {
        if (*fsrv).child_timed_out != 0 {
            printf(b"\x1b[1;91m\n+++ Program timed off +++\n\x1b[0m\x00" as
                       *const u8 as *const libc::c_char);
        } else if stop_soon != 0 {
            printf(b"\x1b[1;91m\n+++ Program aborted by user +++\n\x1b[0m\x00"
                       as *const u8 as *const libc::c_char);
        } else if child_crashed != 0 {
            printf(b"\x1b[1;91m\n+++ Program killed by signal %u +++\n\x1b[0m\x00"
                       as *const u8 as *const libc::c_char,
                   status & 0x7f as libc::c_int);
        }
    };
}
/* Handle Ctrl-C and the like. */
unsafe extern "C" fn handle_stop_sig(mut sig: libc::c_int) {
    ::std::ptr::write_volatile(&mut stop_soon as *mut u8_0,
                               1 as libc::c_int as u8_0);
    afl_fsrv_killall();
}
/* Do basic preparations - persistent fds, filenames, etc. */
unsafe extern "C" fn set_up_environment(mut fsrv: *mut afl_forkserver_t) {
    setenv(b"ASAN_OPTIONS\x00" as *const u8 as *const libc::c_char,
           b"abort_on_error=1:detect_leaks=0:symbolize=0:allocator_may_return_null=1\x00"
               as *const u8 as *const libc::c_char, 0 as libc::c_int);
    setenv(b"MSAN_OPTIONS\x00" as *const u8 as *const libc::c_char,
           b"exit_code=86:symbolize=0:abort_on_error=1:allocator_may_return_null=1:msan_track_origins=0\x00"
               as *const u8 as *const libc::c_char, 0 as libc::c_int);
    if !get_afl_env(b"AFL_PRELOAD\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char).is_null() {
        if (*fsrv).qemu_mode != 0 {
            let mut qemu_preload: *mut u8_0 =
                getenv(b"QEMU_SET_ENV\x00" as *const u8 as
                           *const libc::c_char) as *mut u8_0;
            let mut afl_preload: *mut u8_0 =
                getenv(b"AFL_PRELOAD\x00" as *const u8 as *const libc::c_char)
                    as *mut u8_0;
            let mut buf: *mut u8_0 = 0 as *mut u8_0;
            let mut i: s32 = 0;
            let mut afl_preload_size: s32 =
                strlen(afl_preload as *const libc::c_char) as s32;
            i = 0 as libc::c_int;
            while i < afl_preload_size {
                if *afl_preload.offset(i as isize) as libc::c_int ==
                       ',' as i32 {
                    fflush(stdout);
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mComma (\',\') is not allowed in AFL_PRELOAD when -Q is specified!\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-showmap.c\x00" as *const u8 as
                               *const libc::c_char, 512 as libc::c_int);
                    printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                               *const u8 as *const libc::c_char,
                           strerror(*__errno_location()));
                    exit(1 as libc::c_int);
                }
                i += 1
            }
            if !qemu_preload.is_null() {
                buf =
                    ({
                         let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                         let mut _len: s32 =
                             snprintf(0 as *mut libc::c_char,
                                      0 as libc::c_int as libc::c_ulong,
                                      b"%s,LD_PRELOAD=%s,DYLD_INSERT_LIBRARIES=%s\x00"
                                          as *const u8 as *const libc::c_char,
                                      qemu_preload, afl_preload, afl_preload);
                         if _len < 0 as libc::c_int {
                             printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                        as *const u8 as *const libc::c_char);
                             printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    b"func_unknown\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"src/afl-showmap.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    518 as libc::c_int);
                             exit(1 as libc::c_int);
                         }
                         _tmp =
                             DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0)
                                 as *mut u8_0;
                         snprintf(_tmp as *mut libc::c_char,
                                  (_len + 1 as libc::c_int) as libc::c_ulong,
                                  b"%s,LD_PRELOAD=%s,DYLD_INSERT_LIBRARIES=%s\x00"
                                      as *const u8 as *const libc::c_char,
                                  qemu_preload, afl_preload, afl_preload);
                         _tmp
                     })
            } else {
                buf =
                    ({
                         let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                         let mut _len: s32 =
                             snprintf(0 as *mut libc::c_char,
                                      0 as libc::c_int as libc::c_ulong,
                                      b"LD_PRELOAD=%s,DYLD_INSERT_LIBRARIES=%s\x00"
                                          as *const u8 as *const libc::c_char,
                                      afl_preload, afl_preload);
                         if _len < 0 as libc::c_int {
                             printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                        as *const u8 as *const libc::c_char);
                             printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    b"func_unknown\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"src/afl-showmap.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    521 as libc::c_int);
                             exit(1 as libc::c_int);
                         }
                         _tmp =
                             DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0)
                                 as *mut u8_0;
                         snprintf(_tmp as *mut libc::c_char,
                                  (_len + 1 as libc::c_int) as libc::c_ulong,
                                  b"LD_PRELOAD=%s,DYLD_INSERT_LIBRARIES=%s\x00"
                                      as *const u8 as *const libc::c_char,
                                  afl_preload, afl_preload);
                         _tmp
                     })
            }
            setenv(b"QEMU_SET_ENV\x00" as *const u8 as *const libc::c_char,
                   buf as *const libc::c_char, 1 as libc::c_int);
            DFL_ck_free(buf as *mut libc::c_void);
        } else {
            setenv(b"LD_PRELOAD\x00" as *const u8 as *const libc::c_char,
                   getenv(b"AFL_PRELOAD\x00" as *const u8 as
                              *const libc::c_char), 1 as libc::c_int);
            setenv(b"DYLD_INSERT_LIBRARIES\x00" as *const u8 as
                       *const libc::c_char,
                   getenv(b"AFL_PRELOAD\x00" as *const u8 as
                              *const libc::c_char), 1 as libc::c_int);
        }
    };
}
/* Setup signal handlers, duh. */
unsafe extern "C" fn setup_signal_handlers() {
    let mut sa: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_9{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    sa.__sigaction_handler.sa_handler = None;
    sa.sa_flags = 0x10000000 as libc::c_int;
    sa.__sigaction_handler.sa_sigaction = None;
    sigemptyset(&mut sa.sa_mask);
    /* Various ways of saying "stop". */
    sa.__sigaction_handler.sa_handler =
        Some(handle_stop_sig as unsafe extern "C" fn(_: libc::c_int) -> ());
    sigaction(1 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(2 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(15 as libc::c_int, &mut sa, 0 as *mut sigaction);
}
/* Show banner. */
unsafe extern "C" fn show_banner() {
    printf(b"\x1b[0;36mafl-showmap++2.63d\x1b[0m by Michal Zalewski\n\x00" as
               *const u8 as *const libc::c_char);
}
/* Display usage hints. */
unsafe extern "C" fn usage(mut argv0: *mut u8_0) {
    show_banner();
    printf(b"\n%s [ options ] -- /path/to/target_app [ ... ]\n\nRequired parameters:\n  -o file       - file to write the trace data to\n\nExecution control settings:\n  -t msec       - timeout for each run (none)\n  -m megs       - memory limit for child process (%d MB)\n  -Q            - use binary-only instrumentation (QEMU mode)\n  -U            - use Unicorn-based instrumentation (Unicorn mode)\n  -W            - use qemu-based instrumentation with Wine (Wine mode)\n                  (Not necessary, here for consistency with other afl-* tools)\n\nOther settings:\n  -i dir        - process all files in this directory, -o must be a directory\n                  and each bitmap will be written there individually.\n  -q            - sink program\'s output and don\'t show messages\n  -e            - show edge coverage only, ignore hit counts\n  -r            - show real tuple values instead of AFL filter values\n  -c            - allow core dumps\n\nThis tool displays raw tuple data captured by AFL instrumentation.\nFor additional help, consult %s/README.md.\n\nEnvironment variables used:\nAFL_PRELOAD: LD_PRELOAD / DYLD_INSERT_LIBRARIES settings for target\nAFL_DEBUG: enable extra developer output\nAFL_QUIET: do not print extra informational outputAFL_CMIN_CRASHES_ONLY: (cmin_mode) only write tuples for crashing inputs\nAFL_CMIN_ALLOW_ANY: (cmin_mode) write tuples for crashing inputs also\nLD_BIND_LAZY: do not set LD_BIND_NOW env var for target\n\x00"
               as *const u8 as *const libc::c_char, argv0, 50 as libc::c_int,
           doc_path);
    exit(1 as libc::c_int);
}
/* Find binary. */
unsafe extern "C" fn find_binary(mut fsrv: *mut afl_forkserver_t,
                                 mut fname: *mut u8_0) {
    let mut env_path: *mut u8_0 = 0 as *mut u8_0;
    let mut st: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if !strchr(fname as *const libc::c_char, '/' as i32).is_null() ||
           {
               env_path =
                   getenv(b"PATH\x00" as *const u8 as *const libc::c_char) as
                       *mut u8_0;
               env_path.is_null()
           } {
        (*fsrv).target_path = DFL_ck_strdup(fname);
        if stat((*fsrv).target_path as *const libc::c_char, &mut st) != 0 ||
               !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
                     0o100000 as libc::c_int as libc::c_uint) ||
               st.st_mode & 0o111 as libc::c_int as libc::c_uint == 0 ||
               st.st_size < 4 as libc::c_int as libc::c_long {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mProgram \'%s\' not found or not executable\x00"
                       as *const u8 as *const libc::c_char, fname);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 630 as libc::c_int);
            exit(1 as libc::c_int);
        }
    } else {
        while !env_path.is_null() {
            let mut cur_elem: *mut u8_0 = 0 as *mut u8_0;
            let mut delim: *mut u8_0 =
                strchr(env_path as *const libc::c_char, ':' as i32) as
                    *mut u8_0;
            if !delim.is_null() {
                cur_elem =
                    DFL_ck_alloc((delim.wrapping_offset_from(env_path) as
                                      libc::c_long +
                                      1 as libc::c_int as libc::c_long) as
                                     u32_0) as *mut u8_0;
                memcpy(cur_elem as *mut libc::c_void,
                       env_path as *const libc::c_void,
                       delim.wrapping_offset_from(env_path) as libc::c_long as
                           libc::c_ulong);
                delim = delim.offset(1)
            } else { cur_elem = DFL_ck_strdup(env_path) }
            env_path = delim;
            if *cur_elem.offset(0 as libc::c_int as isize) != 0 {
                (*fsrv).target_path =
                    ({
                         let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                         let mut _len: s32 =
                             snprintf(0 as *mut libc::c_char,
                                      0 as libc::c_int as libc::c_ulong,
                                      b"%s/%s\x00" as *const u8 as
                                          *const libc::c_char, cur_elem,
                                      fname);
                         if _len < 0 as libc::c_int {
                             printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                        as *const u8 as *const libc::c_char);
                             printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    b"func_unknown\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"src/afl-showmap.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    651 as libc::c_int);
                             exit(1 as libc::c_int);
                         }
                         _tmp =
                             DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0)
                                 as *mut u8_0;
                         snprintf(_tmp as *mut libc::c_char,
                                  (_len + 1 as libc::c_int) as libc::c_ulong,
                                  b"%s/%s\x00" as *const u8 as
                                      *const libc::c_char, cur_elem, fname);
                         _tmp
                     })
            } else { (*fsrv).target_path = DFL_ck_strdup(fname) }
            DFL_ck_free(cur_elem as *mut libc::c_void);
            if stat((*fsrv).target_path as *const libc::c_char, &mut st) == 0
                   &&
                   st.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
                       0o100000 as libc::c_int as libc::c_uint &&
                   st.st_mode & 0o111 as libc::c_int as libc::c_uint != 0 &&
                   st.st_size >= 4 as libc::c_int as libc::c_long {
                break ;
            }
            DFL_ck_free((*fsrv).target_path as *mut libc::c_void);
            (*fsrv).target_path = 0 as *mut u8_0
        }
        if (*fsrv).target_path.is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mProgram \'%s\' not found or not executable\x00"
                       as *const u8 as *const libc::c_char, fname);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 667 as libc::c_int);
            exit(1 as libc::c_int);
        }
    };
}
/* Main entry point */
unsafe fn main_0(mut argc: libc::c_int, mut argv_orig: *mut *mut libc::c_char,
                 mut envp: *mut *mut libc::c_char) -> libc::c_int {
    // TODO: u64 mem_limit = MEM_LIMIT;                  /* Memory limit (MB) */
    let mut opt: s32 = 0;
    let mut i: s32 = 0;
    let mut mem_limit_given: u8_0 = 0 as libc::c_int as u8_0;
    let mut timeout_given: u8_0 = 0 as libc::c_int as u8_0;
    let mut unicorn_mode: u8_0 = 0 as libc::c_int as u8_0;
    let mut use_wine: u8_0 = 0 as libc::c_int as u8_0;
    let mut tcnt: u32_0 = 0 as libc::c_int as u32_0;
    let mut use_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut argv: *mut *mut libc::c_char = argv_cpy_dup(argc, argv_orig);
    let mut fsrv_var: afl_forkserver_t =
        {
            let mut init =
                afl_forkserver{uses_asan: 0 as libc::c_int as u8_0,
                               trace_bits: 0 as *mut u8_0,
                               use_stdin: 0,
                               fsrv_pid: 0,
                               child_pid: 0,
                               out_dir_fd: 0,
                               out_fd: 0,
                               dev_urandom_fd: 0,
                               dev_null_fd: 0,
                               fsrv_ctl_fd: 0,
                               fsrv_st_fd: 0,
                               exec_tmout: 0,
                               map_size: 0,
                               snapshot: 0,
                               mem_limit: 0,
                               out_file: 0 as *mut u8_0,
                               target_path: 0 as *mut u8_0,
                               plot_file: 0 as *mut FILE,
                               child_timed_out: 0,
                               use_fauxsrv: 0,
                               prev_timed_out: 0,
                               qemu_mode: 0,
                               cmplog_binary: 0 as *mut libc::c_char,
                               init_child_func: None,
                               function_opt: 0 as *mut u8_0,
                               function_ptr: None,};
            init
        };
    let mut fsrv: *mut afl_forkserver_t = &mut fsrv_var;
    afl_fsrv_init(fsrv);
    doc_path =
        if access(b"/usr/local/share/doc/afl\x00" as *const u8 as
                      *const libc::c_char, 0 as libc::c_int) != 0 {
            b"docs\x00" as *const u8 as *const libc::c_char
        } else {
            b"/usr/local/share/doc/afl\x00" as *const u8 as
                *const libc::c_char
        } as *mut u8_0;
    if !getenv(b"AFL_QUIET\x00" as *const u8 as *const libc::c_char).is_null()
       {
        be_quiet = 1 as libc::c_int as u8_0
    }
    loop  {
        opt =
            getopt(argc, argv,
                   b"+i:o:f:m:t:A:eqZQUWbcrh\x00" as *const u8 as
                       *const libc::c_char);
        if !(opt > 0 as libc::c_int) { break ; }
        match opt {
            105 => {
                if !in_dir.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -i options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-showmap.c\x00" as *const u8 as
                               *const libc::c_char, 699 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                in_dir = optarg as *mut u8_0
            }
            111 => {
                if !(*fsrv).out_file.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -o options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-showmap.c\x00" as *const u8 as
                               *const libc::c_char, 705 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*fsrv).out_file = optarg as *mut u8_0
            }
            109 => {
                let mut suffix: u8_0 = 'M' as i32 as u8_0;
                if mem_limit_given != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -m options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-showmap.c\x00" as *const u8 as
                               *const libc::c_char, 713 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                mem_limit_given = 1 as libc::c_int as u8_0;
                if strcmp(optarg,
                          b"none\x00" as *const u8 as *const libc::c_char) ==
                       0 {
                    (*fsrv).mem_limit = 0 as libc::c_int as u64_0
                } else {
                    if sscanf(optarg,
                              b"%llu%c\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut (*fsrv).mem_limit as *mut u64_0,
                              &mut suffix as *mut u8_0) < 1 as libc::c_int ||
                           *optarg.offset(0 as libc::c_int as isize) as
                               libc::c_int == '-' as i32 {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBad syntax used for -m\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-showmap.c\x00" as *const u8 as
                                   *const libc::c_char, 725 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                    match suffix as libc::c_int {
                        84 => {
                            (*fsrv).mem_limit =
                                ((*fsrv).mem_limit as
                                     libc::c_ulonglong).wrapping_mul((1024 as
                                                                          libc::c_int
                                                                          *
                                                                          1024
                                                                              as
                                                                              libc::c_int)
                                                                         as
                                                                         libc::c_ulonglong)
                                    as u64_0 as u64_0
                        }
                        71 => {
                            (*fsrv).mem_limit =
                                ((*fsrv).mem_limit as
                                     libc::c_ulonglong).wrapping_mul(1024 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulonglong)
                                    as u64_0 as u64_0
                        }
                        107 => {
                            (*fsrv).mem_limit =
                                ((*fsrv).mem_limit as
                                     libc::c_ulonglong).wrapping_div(1024 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulonglong)
                                    as u64_0 as u64_0
                        }
                        77 => { }
                        _ => {
                            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnsupported suffix or bad syntax for -m\x00"
                                       as *const u8 as *const libc::c_char);
                            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   b"func_unknown\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"src/afl-showmap.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   734 as libc::c_int);
                            exit(1 as libc::c_int);
                        }
                    }
                    if (*fsrv).mem_limit <
                           5 as libc::c_int as libc::c_ulonglong {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mDangerously low value of -m\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-showmap.c\x00" as *const u8 as
                                   *const libc::c_char, 738 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                    if ::std::mem::size_of::<rlim_t>() as libc::c_ulong ==
                           4 as libc::c_int as libc::c_ulong &&
                           (*fsrv).mem_limit >
                               2000 as libc::c_int as libc::c_ulonglong {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mValue of -m out of range on 32-bit systems\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-showmap.c\x00" as *const u8 as
                                   *const libc::c_char, 741 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                }
            }
            102 => {
                // only in here to avoid a compiler warning for use_stdin
                (*fsrv).use_stdin = 0 as libc::c_int as u8_0;
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mOption -f is not supported in afl-showmap\x00"
                           as *const u8 as *const libc::c_char);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-showmap.c\x00" as *const u8 as
                           *const libc::c_char, 750 as libc::c_int);
                exit(1 as libc::c_int);
            }
            116 => {
                if timeout_given != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -t options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-showmap.c\x00" as *const u8 as
                               *const libc::c_char, 756 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                timeout_given = 1 as libc::c_int as u8_0;
                if strcmp(optarg,
                          b"none\x00" as *const u8 as *const libc::c_char) !=
                       0 {
                    (*fsrv).exec_tmout = atoi(optarg) as u32_0;
                    if (*fsrv).exec_tmout < 20 as libc::c_int as libc::c_uint
                           ||
                           *optarg.offset(0 as libc::c_int as isize) as
                               libc::c_int == '-' as i32 {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mDangerously low value of -t\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-showmap.c\x00" as *const u8 as
                                   *const libc::c_char, 764 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                }
            }
            101 => {
                if edges_only != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -e options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-showmap.c\x00" as *const u8 as
                               *const libc::c_char, 772 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if raw_instr_output != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0m-e and -r are mutually exclusive\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-showmap.c\x00" as *const u8 as
                               *const libc::c_char, 773 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                edges_only = 1 as libc::c_int as u8_0
            }
            113 => {
                if quiet_mode != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -q options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-showmap.c\x00" as *const u8 as
                               *const libc::c_char, 779 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                quiet_mode = 1 as libc::c_int as u8_0
            }
            90 => {
                /* This is an undocumented option to write data in the syntax expected
           by afl-cmin. Nobody else should have any use for this. */
                cmin_mode = 1 as libc::c_int as u8_0;
                quiet_mode = 1 as libc::c_int as u8_0
            }
            65 => {
                /* Another afl-cmin specific feature. */
                at_file = optarg as *mut u8_0
            }
            81 => {
                if (*fsrv).qemu_mode != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -Q options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-showmap.c\x00" as *const u8 as
                               *const libc::c_char, 799 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if mem_limit_given == 0 {
                    (*fsrv).mem_limit = 200 as libc::c_int as u64_0
                }
                (*fsrv).qemu_mode = 1 as libc::c_int as u8_0
            }
            85 => {
                if unicorn_mode != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -U options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-showmap.c\x00" as *const u8 as
                               *const libc::c_char, 807 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if mem_limit_given == 0 {
                    (*fsrv).mem_limit = 200 as libc::c_int as u64_0
                }
                unicorn_mode = 1 as libc::c_int as u8_0
            }
            87 => {
                /* Wine+QEMU mode */
                if use_wine != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -W options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-showmap.c\x00" as *const u8 as
                               *const libc::c_char, 815 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*fsrv).qemu_mode = 1 as libc::c_int as u8_0;
                use_wine = 1 as libc::c_int as u8_0;
                if mem_limit_given == 0 {
                    (*fsrv).mem_limit = 0 as libc::c_int as u64_0
                }
            }
            98 => {
                /* Secret undocumented mode. Writes output in raw binary format
           similar to that dumped by afl-fuzz in <out_dir/queue/fuzz_bitmap. */
                binary_mode = 1 as libc::c_int as u8_0
            }
            99 => {
                if keep_cores != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -c options not supported\x00"
                               as *const u8 as
                               *const libc::c_char); // skip anything that starts with '.'
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-showmap.c\x00" as *const u8 as
                               *const libc::c_char, 833 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                keep_cores = 1 as libc::c_int as u8_0
            }
            114 => {
                if raw_instr_output != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -r options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-showmap.c\x00" as *const u8 as
                               *const libc::c_char, 839 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if edges_only != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0m-e and -r are mutually exclusive\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-showmap.c\x00" as *const u8 as
                               *const libc::c_char, 840 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                raw_instr_output = 1 as libc::c_int as u8_0
            }
            104 => {
                usage(*argv.offset(0 as libc::c_int as isize) as *mut u8_0);
                return -(1 as libc::c_int)
            }
            _ => {
                usage(*argv.offset(0 as libc::c_int as isize) as *mut u8_0);
            }
        }
    }
    if optind == argc || (*fsrv).out_file.is_null() {
        usage(*argv.offset(0 as libc::c_int as isize) as *mut u8_0);
    }
    check_environment_vars(envp);
    let mut shm: sharedmem_t =
        {
            let mut init =
                sharedmem{shm_id: 0 as libc::c_int,
                          cmplog_shm_id: 0,
                          map: 0 as *mut u8_0,
                          size_alloc: 0,
                          size_used: 0,
                          cmplog_mode: 0,
                          cmp_map: 0 as *mut cmp_map,};
            init
        };
    (*fsrv).trace_bits =
        afl_shm_init(&mut shm,
                     ((1 as libc::c_int) << 16 as libc::c_int) as size_t,
                     0 as libc::c_int as libc::c_uchar);
    setup_signal_handlers();
    set_up_environment(fsrv);
    find_binary(fsrv, *argv.offset(optind as isize) as *mut u8_0);
    if quiet_mode == 0 {
        show_banner();
        printf(b"\x1b[1;94m[*] \x1b[0mExecuting \'%s\'...\x00" as *const u8 as
                   *const libc::c_char, (*fsrv).target_path);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    if !in_dir.is_null() {
        if !at_file.is_null() {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mOptions -A and -i are mutually exclusive\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 874 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        detect_file_args(argv.offset(optind as isize),
                         b"\x00" as *const u8 as *const libc::c_char as
                             *mut u8_0, &mut (*fsrv).use_stdin);
    } else {
        detect_file_args(argv.offset(optind as isize), at_file,
                         &mut (*fsrv).use_stdin);
    }
    i = optind;
    while i < argc {
        if strcmp(*argv.offset(i as isize),
                  b"@@\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            arg_offset = i as u32_0
        }
        i += 1
    }
    if (*fsrv).qemu_mode != 0 {
        if use_wine != 0 {
            use_argv =
                get_wine_argv(*argv.offset(0 as libc::c_int as isize) as
                                  *mut u8_0, &mut (*fsrv).target_path,
                              argc - optind, argv.offset(optind as isize))
        } else {
            use_argv =
                get_qemu_argv(*argv.offset(0 as libc::c_int as isize) as
                                  *mut u8_0, &mut (*fsrv).target_path,
                              argc - optind, argv.offset(optind as isize))
        }
    } else { use_argv = argv.offset(optind as isize) }
    if !in_dir.is_null() {
        let mut dir_in: *mut DIR = 0 as *mut DIR;
        let mut dir_out: *mut DIR = 0 as *mut DIR;
        let mut dir_ent: *mut dirent = 0 as *mut dirent;
        let mut done: libc::c_int = 0 as libc::c_int;
        let mut infile: [u8_0; 4096] = [0; 4096];
        let mut outfile: [u8_0; 4096] = [0; 4096];
        (*fsrv).dev_null_fd =
            open(b"/dev/null\x00" as *const u8 as *const libc::c_char,
                 0o2 as libc::c_int);
        if (*fsrv).dev_null_fd < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open /dev/null\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 910 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        dir_in = opendir(in_dir as *const libc::c_char);
        if dir_in.is_null() {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mcannot open directory %s\x00"
                       as *const u8 as *const libc::c_char, in_dir);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 912 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        dir_out = opendir((*fsrv).out_file as *const libc::c_char);
        if dir_out.is_null() {
            if mkdir((*fsrv).out_file as *const libc::c_char,
                     0o700 as libc::c_int as __mode_t) != 0 {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mcannot create output directory %s\x00"
                           as *const u8 as *const libc::c_char,
                       (*fsrv).out_file);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-showmap.c\x00" as *const u8 as
                           *const libc::c_char, 916 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            }
        }
        let mut use_dir: *mut u8_0 =
            b".\x00" as *const u8 as *const libc::c_char as *mut u8_0;
        if access(use_dir as *const libc::c_char,
                  4 as libc::c_int | 2 as libc::c_int | 1 as libc::c_int) != 0
           {
            use_dir =
                get_afl_env(b"TMPDIR\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char) as *mut u8_0;
            if use_dir.is_null() {
                use_dir =
                    b"/tmp\x00" as *const u8 as *const libc::c_char as
                        *mut u8_0
            }
        }
        stdin_file =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/.afl-showmap-temp-%u\x00" as *const u8 as
                                  *const libc::c_char, use_dir, getpid());
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-showmap.c\x00" as *const u8 as
                                *const libc::c_char, 927 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/.afl-showmap-temp-%u\x00" as *const u8 as
                              *const libc::c_char, use_dir, getpid());
                 _tmp
             }) as *mut libc::c_char;
        unlink(stdin_file);
        atexit(Some(at_exit_handler as unsafe extern "C" fn() -> ()));
        (*fsrv).out_fd =
            open(stdin_file,
                 0o2 as libc::c_int | 0o100 as libc::c_int |
                     0o200 as libc::c_int, 0o600 as libc::c_int);
        if (*fsrv).out_fd < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                       as *const u8 as *const libc::c_char, (*fsrv).out_file);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 931 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        if arg_offset != 0 && *argv.offset(arg_offset as isize) != stdin_file
           {
            DFL_ck_free(*argv.offset(arg_offset as isize) as
                            *mut libc::c_void);
            let ref mut fresh2 = *argv.offset(arg_offset as isize);
            *fresh2 = strdup(stdin_file)
        }
        if !get_afl_env(b"AFL_DEBUG\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char).is_null() {
            let mut i_0: libc::c_int = optind;
            printf(b"\x1b[0;35m[D]\x1b[0m %s:\x00" as *const u8 as
                       *const libc::c_char, (*fsrv).target_path);
            while !(*argv.offset(i_0 as isize)).is_null() {
                let fresh3 = i_0;
                i_0 = i_0 + 1;
                printf(b" \"%s\"\x00" as *const u8 as *const libc::c_char,
                       *argv.offset(fresh3 as isize));
            }
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
            printf(b"\x1b[0;35m[D]\x1b[0m %d - %d = %d, %s\n\x00" as *const u8
                       as *const libc::c_char, arg_offset, optind,
                   arg_offset.wrapping_sub(optind as libc::c_uint),
                   infile.as_mut_ptr());
        }
        afl_fsrv_start(fsrv, use_argv, &mut stop_soon,
                       if !get_afl_env(b"AFL_DEBUG_CHILD_OUTPUT\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char).is_null() {
                           1 as libc::c_int
                       } else { 0 as libc::c_int } as u8_0);
        while done == 0 as libc::c_int &&
                  { dir_ent = readdir(dir_in); !dir_ent.is_null() } {
            if (*dir_ent).d_name[0 as libc::c_int as usize] as libc::c_int ==
                   '.' as i32 {
                continue ;
            }
            /* Posix and Solaris do not know d_type and DT_REG */
            if (*dir_ent).d_type as libc::c_int != DT_REG as libc::c_int {
                continue ; // only regular files
            }
            snprintf(infile.as_mut_ptr() as *mut libc::c_char,
                     ::std::mem::size_of::<[u8_0; 4096]>() as libc::c_ulong,
                     b"%s/%s\x00" as *const u8 as *const libc::c_char, in_dir,
                     (*dir_ent).d_name.as_mut_ptr());
            /* use stat() */
            snprintf(outfile.as_mut_ptr() as *mut libc::c_char,
                     ::std::mem::size_of::<[u8_0; 4096]>() as libc::c_ulong,
                     b"%s/%s\x00" as *const u8 as *const libc::c_char,
                     (*fsrv).out_file, (*dir_ent).d_name.as_mut_ptr());
            if read_file(infile.as_mut_ptr()) != 0 {
                run_target_forkserver(fsrv, use_argv, in_data, in_len);
                DFL_ck_free(in_data as *mut libc::c_void);
                tcnt = write_results_to_file(fsrv, outfile.as_mut_ptr())
            }
        }
        if quiet_mode == 0 {
            printf(b"\x1b[1;92m[+] \x1b[0mProcessed %u input files.\x00" as
                       *const u8 as *const libc::c_char, total_execs);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
        closedir(dir_in);
        if !dir_out.is_null() { closedir(dir_out); }
    } else { run_target(fsrv, use_argv); tcnt = write_results(fsrv) }
    if quiet_mode == 0 {
        if tcnt == 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mNo instrumentation detected\x1b[0m\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-showmap.c\x00" as *const u8 as
                       *const libc::c_char, 997 as libc::c_int);
            exit(1 as libc::c_int);
        }
        printf(b"\x1b[1;92m[+] \x1b[0mCaptured %u tuples (highest value %u, total values %u) in \'%s\'.\x1b[0m\x00"
                   as *const u8 as *const libc::c_char, tcnt, highest, total,
               (*fsrv).out_file);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    if !stdin_file.is_null() {
        unlink(stdin_file);
        DFL_ck_free(stdin_file as *mut libc::c_void);
        stdin_file = 0 as *mut libc::c_char
    }
    afl_shm_deinit(&mut shm);
    let mut ret: u32_0 =
        (child_crashed as libc::c_int * 2 as libc::c_int +
             (*fsrv).child_timed_out as libc::c_int) as u32_0;
    if !(*fsrv).target_path.is_null() {
        DFL_ck_free((*fsrv).target_path as *mut libc::c_void);
    }
    afl_fsrv_deinit(fsrv);
    if !stdin_file.is_null() { DFL_ck_free(stdin_file as *mut libc::c_void); }
    argv_cpy_free(argv);
    exit(ret as libc::c_int);
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    let mut vars: Vec<*mut libc::c_char> = Vec::new();
    for (var_name, var_value) in ::std::env::vars() {
        let var: String = format!("{}={}", var_name, var_value);
        vars.push(::std::ffi::CString::new(var).expect("Failed to convert environment variable into CString.").into_raw());
    };
    vars.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char,
                                    vars.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
