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
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn abort() -> !;
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
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn afl_shm_init(_: *mut sharedmem_t, _: size_t, dumb_mode: libc::c_uchar)
     -> *mut u8_0;
    #[no_mangle]
    fn afl_shm_deinit(_: *mut sharedmem_t);
    /*
   american fuzzy lop++ - common routines header
   ---------------------------------------------

   Originally written by Michal Zalewski

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

   Gather some functions common to multiple executables

   - detect_file_args

 */
    /* STRINGIFY_VAL_SIZE_MAX will fit all stringify_ strings. */
    #[no_mangle]
    fn detect_file_args(argv: *mut *mut libc::c_char, prog_in_0: *mut u8_0,
                        use_stdin_0: *mut u8_0);
    #[no_mangle]
    fn check_environment_vars(env: *mut *mut libc::c_char);
    #[no_mangle]
    fn get_qemu_argv(own_loc: *mut u8_0, target_path_p: *mut *mut u8_0,
                     argc: libc::c_int, argv: *mut *mut libc::c_char)
     -> *mut *mut libc::c_char;
    #[no_mangle]
    fn get_wine_argv(own_loc: *mut u8_0, target_path_p: *mut *mut u8_0,
                     argc: libc::c_int, argv: *mut *mut libc::c_char)
     -> *mut *mut libc::c_char;
    #[no_mangle]
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn execv(__path: *const libc::c_char, __argv: *const *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
              __shortopts: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int)
     -> __off_t;
    #[no_mangle]
    fn access(__name: *const libc::c_char, __type: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    static mut doc_path: *mut u8_0;
    #[no_mangle]
    fn get_afl_env(env: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn setitimer(__which: __itimer_which_t, __new: *const itimerval,
                 __old: *mut itimerval) -> libc::c_int;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                 __oact: *mut sigaction) -> libc::c_int;
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
    fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit)
     -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
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
pub type uint16_t = __uint16_t;
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
pub type u16_0 = uint16_t;
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
#[inline]
unsafe extern "C" fn DFL_ck_free(mut mem: *mut libc::c_void) {
    if mem.is_null() { return }
    free(mem);
}
#[inline]
unsafe extern "C" fn DFL_ck_alloc(mut size: u32_0) -> *mut libc::c_void {
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 { return 0 as *mut libc::c_void }
    mem = DFL_ck_alloc_nozero(size);
    return memset(mem, 0 as libc::c_int, size as libc::c_ulong);
}
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
// extern unsigned char *trace_bits;
/* ID of the SHM region              */
/* shared memory region */
/* actual allocated size */
/* in use by shmem app */
/*
   american fuzzy lop++ - hashing function
   ---------------------------------------

   The hash32() function is a variant of MurmurHash3, a good
   non-cryptosafe hashing function developed by Austin Appleby.

   For simplicity, this variant does *NOT* accept buffer lengths
   that are not divisible by 8 bytes. The 32-bit version is otherwise
   similar to the original; the 64-bit one is a custom hack with
   mostly-unproven properties.

   Austin's original code is public domain.

   Other code written by Michal Zalewski

   Copyright 2016 Google Inc. All rights reserved.
   Copyright 2019-2020 AFLplusplus Project. All rights reserved.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at:

     http://www.apache.org/licenses/LICENSE-2.0

 */
#[inline]
unsafe extern "C" fn hash32(mut key: *const libc::c_void, mut len: u32_0,
                            mut seed: u32_0) -> u32_0 {
    let mut data: *const u64_0 = key as *mut u64_0;
    let mut h1: u64_0 = (seed ^ len) as u64_0;
    len >>= 3 as libc::c_int;
    loop  {
        let fresh0 = len;
        len = len.wrapping_sub(1);
        if !(fresh0 != 0) { break ; }
        let fresh1 = data;
        data = data.offset(1);
        let mut k1: u64_0 = *fresh1;
        k1 =
            (k1 as
                 libc::c_ulonglong).wrapping_mul(0x87c37b91114253d5 as
                                                     libc::c_ulonglong) as
                u64_0 as u64_0;
        k1 =
            k1 << 31 as libc::c_int |
                k1 >> 64 as libc::c_int - 31 as libc::c_int;
        k1 =
            (k1 as
                 libc::c_ulonglong).wrapping_mul(0x4cf5ad432745937f as
                                                     libc::c_ulonglong) as
                u64_0 as u64_0;
        h1 ^= k1;
        h1 =
            h1 << 27 as libc::c_int |
                h1 >> 64 as libc::c_int - 27 as libc::c_int;
        h1 =
            h1.wrapping_mul(5 as libc::c_int as
                                libc::c_ulonglong).wrapping_add(0x52dce729 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulonglong)
    }
    h1 ^= h1 >> 33 as libc::c_int;
    h1 =
        (h1 as
             libc::c_ulonglong).wrapping_mul(0xff51afd7ed558ccd as
                                                 libc::c_ulonglong) as u64_0
            as u64_0;
    h1 ^= h1 >> 33 as libc::c_int;
    h1 =
        (h1 as
             libc::c_ulonglong).wrapping_mul(0xc4ceb9fe1a85ec53 as
                                                 libc::c_ulonglong) as u64_0
            as u64_0;
    h1 ^= h1 >> 33 as libc::c_int;
    return h1 as u32_0;
}
/*
   american fuzzy lop++ - file format analyzer
   -------------------------------------------

   Originally written by Michal Zalewski

   Now maintained by Marc Heuse <mh@mh-sec.de>,
                        Heiko Eißfeldt <heiko.eissfeldt@hexco.de> and
                        Andrea Fioraldi <andreafioraldi@gmail.com>

   Copyright 2016, 2017 Google Inc. All rights reserved.
   Copyright 2019-2020 AFLplusplus Project. All rights reserved.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at:

     http://www.apache.org/licenses/LICENSE-2.0

   A nifty utility that grabs an input file and takes a stab at explaining
   its structure by observing how changes to it affect the execution path.

   If the output scrolls past the edge of the screen, pipe it to 'less -r'.

 */
static mut child_pid: s32 = 0;
/* PID of the tested program         */
static mut trace_bits: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
/* SHM with instrumentation bitmap   */
static mut in_file: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
/* Analyzer input test case          */
static mut prog_in: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
/* Targeted program input file       */
static mut in_data: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
/* Input data for analysis           */
static mut in_len: u32_0 = 0;
/* Input data length                 */
static mut orig_cksum: u32_0 = 0;
/* Original checksum                 */
static mut total_execs: u32_0 = 0;
/* Total number of execs             */
static mut exec_hangs: u32_0 = 0;
/* Total number of hangs             */
static mut exec_tmout: u32_0 = 1000 as libc::c_int as u32_0;
/* Exec timeout (ms)                 */
static mut mem_limit: u64_0 = 50 as libc::c_int as u64_0;
/* Memory limit (MB)                 */
static mut dev_null_fd: s32 = -(1 as libc::c_int);
/* FD to /dev/null                   */
static mut edges_only: u8_0 = 0;
/* Ignore hit counts?                */
static mut use_hex_offsets: u8_0 = 0;
/* Show hex offsets?                 */
static mut use_stdin: u8_0 = 1 as libc::c_int as u8_0;
/* Use stdin for program input?      */
static mut stop_soon: u8_0 = 0;
/* Ctrl-C pressed?                   */
static mut child_timed_out: u8_0 = 0;
/* Child timed out?                  */
static mut target_path: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
static mut qemu_mode: u8_0 = 0;
/* Potential "suspect" blob          */
/* Classify tuple counts. This is a slow & naive version, but good enough here.
 */
static mut count_class_lookup: [u8_0; 256] =
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
unsafe extern "C" fn classify_counts(mut mem: *mut u8_0) {
    let mut i: u32_0 = ((1 as libc::c_int) << 16 as libc::c_int) as u32_0;
    if edges_only != 0 {
        loop  {
            let fresh2 = i;
            i = i.wrapping_sub(1);
            if !(fresh2 != 0) { break ; }
            if *mem != 0 { *mem = 1 as libc::c_int as u8_0 }
            mem = mem.offset(1)
        }
    } else {
        loop  {
            let fresh3 = i;
            i = i.wrapping_sub(1);
            if !(fresh3 != 0) { break ; }
            *mem = count_class_lookup[*mem as usize];
            mem = mem.offset(1)
        }
    };
}
/* See if any bytes are set in the bitmap. */
#[inline]
unsafe extern "C" fn anything_set() -> u8_0 {
    let mut ptr: *mut u32_0 = trace_bits as *mut u32_0;
    let mut i: u32_0 =
        ((1 as libc::c_int) << 16 as libc::c_int >> 2 as libc::c_int) as
            u32_0;
    loop  {
        let fresh4 = i;
        i = i.wrapping_sub(1);
        if !(fresh4 != 0) { break ; }
        let fresh5 = ptr;
        ptr = ptr.offset(1);
        if *fresh5 != 0 { return 1 as libc::c_int as u8_0 }
    }
    return 0 as libc::c_int as u8_0;
}
/* Get rid of temp files (atexit handler). */
unsafe extern "C" fn at_exit_handler() {
    unlink(prog_in as *const libc::c_char);
    /* Ignore errors */
}
/* Read initial file. */
unsafe extern "C" fn read_initial_file() {
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
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                   as *const u8 as *const libc::c_char, in_file);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-analyze.c\x00" as *const u8 as *const libc::c_char,
               171 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    if fstat(fd, &mut st) != 0 || st.st_size == 0 {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mZero-sized input file.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-analyze.c\x00" as *const u8 as *const libc::c_char,
               173 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if st.st_size >=
           (10 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as
               libc::c_long {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mInput file is too large (%u MB max)\x00"
                   as *const u8 as *const libc::c_char,
               10 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int /
                   1024 as libc::c_int / 1024 as libc::c_int);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-analyze.c\x00" as *const u8 as *const libc::c_char,
               176 as libc::c_int);
        exit(1 as libc::c_int);
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
                   b"src/afl-analyze.c\x00" as *const u8 as
                       *const libc::c_char, 181 as libc::c_int);
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
                   b"src/afl-analyze.c\x00" as *const u8 as
                       *const libc::c_char, 181 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    close(fd);
    printf(b"\x1b[1;92m[+] \x1b[0mRead %u byte%s from \'%s\'.\x00" as
               *const u8 as *const libc::c_char, in_len,
           if in_len == 1 as libc::c_int as libc::c_uint {
               b"\x00" as *const u8 as *const libc::c_char
           } else { b"s\x00" as *const u8 as *const libc::c_char }, in_file);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
}
/* Write output file. */
unsafe extern "C" fn write_to_file(mut path: *mut u8_0, mut mem: *mut u8_0,
                                   mut len: u32_0) -> s32 {
    let mut ret: s32 = 0; /* Ignore errors */
    unlink(path as *const libc::c_char);
    ret =
        open(path as *const libc::c_char,
             0o2 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
             0o600 as libc::c_int);
    if ret < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, path);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-analyze.c\x00" as *const u8 as *const libc::c_char,
               199 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    let mut _len: u32_0 = len;
    let mut _res: s32 =
        write(ret, mem as *const libc::c_void, _len as size_t) as s32;
    if _res as libc::c_uint != _len {
        if _res < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                       as *const u8 as *const libc::c_char, path);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-analyze.c\x00" as *const u8 as
                       *const libc::c_char, 201 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        } else {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                       as *const u8 as *const libc::c_char, path);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-analyze.c\x00" as *const u8 as
                       *const libc::c_char, 201 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    lseek(ret, 0 as libc::c_int as __off_t, 0 as libc::c_int);
    return ret;
}
/* Execute target application. Returns exec checksum, or 0 if program
   times out. */
unsafe extern "C" fn run_target(mut argv: *mut *mut libc::c_char,
                                mut mem: *mut u8_0, mut len: u32_0,
                                mut first_run: u8_0) -> u32_0 {
    static mut it: itimerval =
        itimerval{it_interval: timeval{tv_sec: 0, tv_usec: 0,},
                  it_value: timeval{tv_sec: 0, tv_usec: 0,},};
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut prog_in_fd: s32 = 0;
    let mut cksum: u32_0 = 0;
    memset(trace_bits as *mut libc::c_void, 0 as libc::c_int,
           ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong);
    asm!("" : : : "memory" : "volatile");
    prog_in_fd = write_to_file(prog_in, mem, len);
    child_pid = fork();
    if child_pid < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mfork() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-analyze.c\x00" as *const u8 as *const libc::c_char,
               227 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    if child_pid == 0 {
        let mut r: rlimit = rlimit{rlim_cur: 0, rlim_max: 0,};
        if dup2((if use_stdin as libc::c_int != 0 {
                     prog_in_fd
                 } else { dev_null_fd }), 0 as libc::c_int) < 0 as libc::c_int
               || dup2(dev_null_fd, 1 as libc::c_int) < 0 as libc::c_int ||
               dup2(dev_null_fd, 2 as libc::c_int) < 0 as libc::c_int {
            *(trace_bits as *mut u32_0) = 0xfee1dead as libc::c_uint;
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mdup2() failed\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-analyze.c\x00" as *const u8 as
                       *const libc::c_char, 237 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        close(dev_null_fd);
        close(prog_in_fd);
        if mem_limit != 0 {
            r.rlim_cur = (mem_limit as rlim_t) << 20 as libc::c_int;
            r.rlim_max = r.rlim_cur;
            setrlimit(RLIMIT_AS as libc::c_int, &mut r);
            /* Ignore errors */
            /* ^RLIMIT_AS */
        } /* Ignore errors */
        r.rlim_cur = 0 as libc::c_int as rlim_t;
        r.rlim_max = r.rlim_cur;
        setrlimit(RLIMIT_CORE as libc::c_int, &mut r);
        execv(target_path as *const libc::c_char,
              argv as *const *mut libc::c_char);
        *(trace_bits as *mut u32_0) = 0xfee1dead as libc::c_uint;
        exit(0 as libc::c_int);
    }
    close(prog_in_fd);
    /* Configure timeout, wait for child, cancel timeout. */
    ::std::ptr::write_volatile(&mut child_timed_out as *mut u8_0,
                               0 as libc::c_int as u8_0);
    it.it_value.tv_sec =
        exec_tmout.wrapping_div(1000 as libc::c_int as libc::c_uint) as
            __time_t;
    it.it_value.tv_usec =
        exec_tmout.wrapping_rem(1000 as libc::c_int as
                                    libc::c_uint).wrapping_mul(1000 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
            as __suseconds_t;
    setitimer(ITIMER_REAL as libc::c_int, &mut it, 0 as *mut itimerval);
    if waitpid(child_pid, &mut status, 0 as libc::c_int) <= 0 as libc::c_int {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mwaitpid() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-analyze.c\x00" as *const u8 as *const libc::c_char,
               280 as libc::c_int);
        exit(1 as libc::c_int);
    }
    child_pid = 0 as libc::c_int;
    it.it_value.tv_sec = 0 as libc::c_int as __time_t;
    it.it_value.tv_usec = 0 as libc::c_int as __suseconds_t;
    setitimer(ITIMER_REAL as libc::c_int, &mut it, 0 as *mut itimerval);
    asm!("" : : : "memory" : "volatile");
    /* Clean up bitmap, analyze exit condition, etc. */
    if *(trace_bits as *mut u32_0) == 0xfee1dead as libc::c_uint {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to execute \'%s\'\x00"
                   as *const u8 as *const libc::c_char,
               *argv.offset(0 as libc::c_int as isize));
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-analyze.c\x00" as *const u8 as *const libc::c_char,
               293 as libc::c_int);
        exit(1 as libc::c_int);
    }
    classify_counts(trace_bits);
    total_execs = total_execs.wrapping_add(1);
    if stop_soon != 0 {
        printf(b"\x1b[0m\x1b[1;91m\n+++ Analysis aborted by user +++\n\x1b[0m\x00"
                   as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    /* Always discard inputs that time out. */
    if child_timed_out != 0 {
        exec_hangs = exec_hangs.wrapping_add(1);
        return 0 as libc::c_int as u32_0
    }
    cksum =
        hash32(trace_bits as *const libc::c_void,
               ((1 as libc::c_int) << 16 as libc::c_int) as u32_0,
               0xa5b35705 as libc::c_uint);
    /* We don't actually care if the target is crashing or not,
     except that when it does, the checksum should be different. */
    if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar as
           libc::c_int >> 1 as libc::c_int > 0 as libc::c_int ||
           status & 0x7f as libc::c_int == 0 as libc::c_int &&
               (status & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                   86 as libc::c_int ||
           status & 0x7f as libc::c_int == 0 as libc::c_int &&
               (status & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0 {
        cksum ^= 0xffffffff as libc::c_uint
    }
    if first_run != 0 { orig_cksum = cksum }
    return cksum;
}
/* Helper function to display a human-readable character. */
unsafe extern "C" fn show_char(mut val: u8_0) {
    match val as libc::c_int {
        0 | 127 => {
            printf(b"#%02x\x00" as *const u8 as *const libc::c_char,
                   val as libc::c_int);
        }
        _ => {
            printf(b" %c \x00" as *const u8 as *const libc::c_char,
                   val as libc::c_int);
        }
    };
}
/* Show the legend */
unsafe extern "C" fn show_legend() {
    printf(b"    \x1b[0;37m\x1b[100m 01 \x1b[0m - no-op block              \x1b[0;30m\x1b[102m 01 \x1b[0m - suspected length field\n    \x1b[1;97m\x1b[100m 01 \x1b[0m - superficial content      \x1b[0;30m\x1b[103m 01 \x1b[0m - suspected cksum or magic int\n    \x1b[0;30m\x1b[46m 01 \x1b[0m - critical stream          \x1b[0;30m\x1b[101m 01 \x1b[0m - suspected checksummed block\n    \x1b[0;30m\x1b[45m 01 \x1b[0m - \"magic value\" section\n\n\x00"
               as *const u8 as *const libc::c_char);
}
/* USE_COLOR */
/* Interpret and report a pattern in the input file. */
unsafe extern "C" fn dump_hex(mut buf: *mut u8_0, mut len: u32_0,
                              mut b_data: *mut u8_0) {
    let mut i: u32_0 = 0;
    i = 0 as libc::c_int as u32_0;
    while i < len {
        let mut rlen: u32_0 = 1 as libc::c_int as u32_0;
        let mut off: u32_0 = 0;
        /* ^USE_COLOR */
        let mut rtype: u8_0 =
            (*b_data.offset(i as isize) as libc::c_int & 0xf as libc::c_int)
                as u8_0;
        /* Look ahead to determine the length of run. */
        while i.wrapping_add(rlen) < len &&
                  *b_data.offset(i as isize) as libc::c_int >>
                      7 as libc::c_int ==
                      *b_data.offset(i.wrapping_add(rlen) as isize) as
                          libc::c_int >> 7 as libc::c_int {
            if (rtype as libc::c_int) <
                   *b_data.offset(i.wrapping_add(rlen) as isize) as
                       libc::c_int & 0xf as libc::c_int {
                rtype =
                    (*b_data.offset(i.wrapping_add(rlen) as isize) as
                         libc::c_int & 0xf as libc::c_int) as u8_0
            }
            rlen = rlen.wrapping_add(1)
        }
        /* Try to do some further classification based on length & value. */
        if rtype as libc::c_int == 0x3 as libc::c_int {
            match rlen {
                2 => {
                    let mut val: u16_0 =
                        *(in_data.offset(i as isize) as *mut u16_0);
                    /* Small integers may be length fields. */
                    if val as libc::c_int != 0 &&
                           (val as libc::c_uint <= in_len ||
                                ({
                                     let mut _ret: u16_0 = val;
                                     ((_ret as libc::c_int) <<
                                          8 as libc::c_int |
                                          _ret as libc::c_int >>
                                              8 as libc::c_int) as u16_0
                                 }) as libc::c_uint <= in_len) {
                        rtype = 0x4 as libc::c_int as u8_0
                    } else if val as libc::c_int != 0 &&
                                  abs(*in_data.offset(i as isize) as
                                          libc::c_int -
                                          *in_data.offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                              as isize) as
                                              libc::c_int) > 32 as libc::c_int
                     {
                        rtype = 0x5 as libc::c_int as u8_0
                    }
                }
                4 => {
                    let mut val_0: u32_0 =
                        *(in_data.offset(i as isize) as *mut u32_0);
                    /* Uniform integers may be checksums. */
                    /* Small integers may be length fields. */
                    if val_0 != 0 &&
                           (val_0 <= in_len ||
                                ({
                                     let mut _ret: u32_0 = val_0;
                                     (_ret << 24 as libc::c_int |
                                          _ret >> 24 as libc::c_int |
                                          _ret << 8 as libc::c_int &
                                              0xff0000 as libc::c_int as
                                                  libc::c_uint) |
                                         _ret >> 8 as libc::c_int &
                                             0xff00 as libc::c_int as
                                                 libc::c_uint
                                 }) <= in_len) {
                        rtype = 0x4 as libc::c_int as u8_0
                    } else if val_0 != 0 &&
                                  (*in_data.offset(i as isize) as libc::c_int
                                       >> 7 as libc::c_int !=
                                       *in_data.offset(i.wrapping_add(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                                                           as isize) as
                                           libc::c_int >> 7 as libc::c_int ||
                                       *in_data.offset(i as isize) as
                                           libc::c_int >> 7 as libc::c_int !=
                                           *in_data.offset(i.wrapping_add(2 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint)
                                                               as isize) as
                                               libc::c_int >> 7 as libc::c_int
                                       ||
                                       *in_data.offset(i as isize) as
                                           libc::c_int >> 7 as libc::c_int !=
                                           *in_data.offset(i.wrapping_add(3 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint)
                                                               as isize) as
                                               libc::c_int >>
                                               7 as libc::c_int) {
                        rtype = 0x5 as libc::c_int as u8_0
                    }
                }
                1 | 3 | 5 => { }
                _ => { rtype = 0x6 as libc::c_int as u8_0 }
            }
        }
        /* Uniform integers may be checksums. */
        /* Print out the entire run. */
        off = 0 as libc::c_int as u32_0;
        while off < rlen {
            /* Every 16 digits, display offset. */
            if i.wrapping_add(off).wrapping_rem(16 as libc::c_int as
                                                    libc::c_uint) == 0 {
                if off != 0 {
                    printf(b"\x1b[0m\x1b[1;96m>\x00" as *const u8 as
                               *const libc::c_char);
                }
                if use_hex_offsets != 0 {
                    printf(b"\x1b[0m\x1b[1;90m%s[%06x] \x1b[0m\x00" as
                               *const u8 as *const libc::c_char,
                           if i.wrapping_add(off) != 0 {
                               b"\n\x00" as *const u8 as *const libc::c_char
                           } else {
                               b"\x00" as *const u8 as *const libc::c_char
                           }, i.wrapping_add(off));
                } else {
                    printf(b"\x1b[0m\x1b[1;90m%s[%06u] \x1b[0m\x00" as
                               *const u8 as *const libc::c_char,
                           if i.wrapping_add(off) != 0 {
                               b"\n\x00" as *const u8 as *const libc::c_char
                           } else {
                               b"\x00" as *const u8 as *const libc::c_char
                           }, i.wrapping_add(off));
                }
            }
            match rtype as libc::c_int {
                0 => {
                    printf(b"\x1b[0;37m\x1b[100m\x00" as *const u8 as
                               *const libc::c_char);
                }
                1 => {
                    printf(b"\x1b[1;97m\x1b[100m\x00" as *const u8 as
                               *const libc::c_char);
                }
                2 => {
                    printf(b"\x1b[0;30m\x1b[46m\x00" as *const u8 as
                               *const libc::c_char);
                }
                3 => {
                    printf(b"\x1b[0;30m\x1b[45m\x00" as *const u8 as
                               *const libc::c_char);
                }
                4 => {
                    printf(b"\x1b[0;30m\x1b[102m\x00" as *const u8 as
                               *const libc::c_char);
                }
                5 => {
                    printf(b"\x1b[0;30m\x1b[103m\x00" as *const u8 as
                               *const libc::c_char);
                }
                6 => {
                    printf(b"\x1b[0;30m\x1b[101m\x00" as *const u8 as
                               *const libc::c_char);
                }
                _ => { }
            }
            show_char(*in_data.offset(i.wrapping_add(off) as isize));
            if off != rlen.wrapping_sub(1 as libc::c_int as libc::c_uint) &&
                   i.wrapping_add(off).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint).wrapping_rem(16
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint)
                       != 0 {
                printf(b" \x00" as *const u8 as *const libc::c_char);
            } else {
                printf(b"\x1b[0m \x00" as *const u8 as *const libc::c_char);
            }
            off = off.wrapping_add(1)
        }
        /* ^USE_COLOR */
        i =
            (i as
                 libc::c_uint).wrapping_add(rlen.wrapping_sub(1 as libc::c_int
                                                                  as
                                                                  libc::c_uint))
                as u32_0 as u32_0;
        i = i.wrapping_add(1)
    }
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    /* USE_COLOR */
}
/* Actually analyze! */
unsafe extern "C" fn analyze(mut argv: *mut *mut libc::c_char) {
    let mut i: u32_0 = 0; /* Intentional terminator. */
    let mut boring_len: u32_0 = 0 as libc::c_int as u32_0;
    let mut prev_xff: u32_0 = 0 as libc::c_int as u32_0;
    let mut prev_x01: u32_0 = 0 as libc::c_int as u32_0;
    let mut prev_s10: u32_0 = 0 as libc::c_int as u32_0;
    let mut prev_a10: u32_0 = 0 as libc::c_int as u32_0;
    let mut b_data: *mut u8_0 =
        DFL_ck_alloc(in_len.wrapping_add(1 as libc::c_int as libc::c_uint)) as
            *mut u8_0;
    let mut seq_byte: u8_0 = 0 as libc::c_int as u8_0;
    *b_data.offset(in_len as isize) = 0xff as libc::c_int as u8_0;
    printf(b"\x1b[1;94m[*] \x1b[0mAnalyzing input file (this may take a while)...\n\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    show_legend();
    /* USE_COLOR */
    i = 0 as libc::c_int as u32_0;
    while i < in_len {
        let mut xor_ff: u32_0 = 0;
        let mut xor_01: u32_0 = 0;
        let mut sub_10: u32_0 = 0;
        let mut add_10: u32_0 = 0;
        let mut xff_orig: u8_0 = 0;
        let mut x01_orig: u8_0 = 0;
        let mut s10_orig: u8_0 = 0;
        let mut a10_orig: u8_0 = 0;
        /* Perform walking byte adjustments across the file. We perform four
       operations designed to elicit some response from the underlying
       code. */
        let ref mut fresh6 = *in_data.offset(i as isize);
        *fresh6 = (*fresh6 as libc::c_int ^ 0xff as libc::c_int) as u8_0;
        xor_ff = run_target(argv, in_data, in_len, 0 as libc::c_int as u8_0);
        let ref mut fresh7 = *in_data.offset(i as isize);
        *fresh7 = (*fresh7 as libc::c_int ^ 0xfe as libc::c_int) as u8_0;
        xor_01 = run_target(argv, in_data, in_len, 0 as libc::c_int as u8_0);
        *in_data.offset(i as isize) =
            ((*in_data.offset(i as isize) as libc::c_int ^ 0x1 as libc::c_int)
                 - 0x10 as libc::c_int) as u8_0;
        sub_10 = run_target(argv, in_data, in_len, 0 as libc::c_int as u8_0);
        let ref mut fresh8 = *in_data.offset(i as isize);
        *fresh8 = (*fresh8 as libc::c_int + 0x20 as libc::c_int) as u8_0;
        add_10 = run_target(argv, in_data, in_len, 0 as libc::c_int as u8_0);
        let ref mut fresh9 = *in_data.offset(i as isize);
        *fresh9 = (*fresh9 as libc::c_int - 0x10 as libc::c_int) as u8_0;
        /* Classify current behavior. */
        xff_orig = (xor_ff == orig_cksum) as libc::c_int as u8_0;
        x01_orig = (xor_01 == orig_cksum) as libc::c_int as u8_0;
        s10_orig = (sub_10 == orig_cksum) as libc::c_int as u8_0;
        a10_orig = (add_10 == orig_cksum) as libc::c_int as u8_0;
        if xff_orig as libc::c_int != 0 && x01_orig as libc::c_int != 0 &&
               s10_orig as libc::c_int != 0 && a10_orig as libc::c_int != 0 {
            *b_data.offset(i as isize) = 0 as libc::c_int as u8_0;
            boring_len = boring_len.wrapping_add(1)
        } else if xff_orig as libc::c_int != 0 || x01_orig as libc::c_int != 0
                      || s10_orig as libc::c_int != 0 ||
                      a10_orig as libc::c_int != 0 {
            *b_data.offset(i as isize) = 0x1 as libc::c_int as u8_0;
            boring_len = boring_len.wrapping_add(1)
        } else if xor_ff == xor_01 && xor_ff == sub_10 && xor_ff == add_10 {
            *b_data.offset(i as isize) = 0x3 as libc::c_int as u8_0
        } else { *b_data.offset(i as isize) = 0x2 as libc::c_int as u8_0 }
        /* When all checksums change, flip most significant bit of b_data. */
        if prev_xff != xor_ff && prev_x01 != xor_01 && prev_s10 != sub_10 &&
               prev_a10 != add_10 {
            seq_byte = (seq_byte as libc::c_int ^ 0x80 as libc::c_int) as u8_0
        }
        let ref mut fresh10 = *b_data.offset(i as isize);
        *fresh10 =
            (*fresh10 as libc::c_int | seq_byte as libc::c_int) as u8_0;
        prev_xff = xor_ff;
        prev_x01 = xor_01;
        prev_s10 = sub_10;
        prev_a10 = add_10;
        i = i.wrapping_add(1)
    }
    dump_hex(in_data, in_len, b_data);
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    printf(b"\x1b[1;92m[+] \x1b[0mAnalysis complete. Interesting bits: %0.02f%% of the input file.\x00"
               as *const u8 as *const libc::c_char,
           100.0f64 -
               boring_len as libc::c_double *
                   100 as libc::c_int as libc::c_double /
                   in_len as libc::c_double);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    if exec_hangs != 0 {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0m\x1b[1;91mEncountered %u timeouts - results may be skewed.\x1b[0m\x00"
                   as *const u8 as *const libc::c_char, exec_hangs);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    DFL_ck_free(b_data as *mut libc::c_void);
}
/* Handle Ctrl-C and the like. */
unsafe extern "C" fn handle_stop_sig(mut sig: libc::c_int) {
    ::std::ptr::write_volatile(&mut stop_soon as *mut u8_0,
                               1 as libc::c_int as u8_0);
    if child_pid > 0 as libc::c_int { kill(child_pid, 9 as libc::c_int); };
}
/* Do basic preparations - persistent fds, filenames, etc. */
unsafe extern "C" fn set_up_environment() {
    let mut x: *mut u8_0 = 0 as *mut u8_0;
    dev_null_fd =
        open(b"/dev/null\x00" as *const u8 as *const libc::c_char,
             0o2 as libc::c_int);
    if dev_null_fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open /dev/null\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-analyze.c\x00" as *const u8 as *const libc::c_char,
               647 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    if prog_in.is_null() {
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
        prog_in =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/.afl-analyze-temp-%u\x00" as *const u8 as
                                  *const libc::c_char, use_dir, getpid());
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-analyze.c\x00" as *const u8 as
                                *const libc::c_char, 660 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/.afl-analyze-temp-%u\x00" as *const u8 as
                              *const libc::c_char, use_dir, getpid());
                 _tmp
             })
    }
    /* Set sane defaults... */
    x =
        get_afl_env(b"ASAN_OPTIONS\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut u8_0;
    if !x.is_null() {
        if strstr(x as *const libc::c_char,
                  b"abort_on_error=1\x00" as *const u8 as
                      *const libc::c_char).is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mCustom ASAN_OPTIONS set without abort_on_error=1 - please fix!\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-analyze.c\x00" as *const u8 as
                       *const libc::c_char, 671 as libc::c_int);
            exit(1 as libc::c_int);
        }
        if strstr(x as *const libc::c_char,
                  b"symbolize=0\x00" as *const u8 as
                      *const libc::c_char).is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mCustom ASAN_OPTIONS set without symbolize=0 - please fix!\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-analyze.c\x00" as *const u8 as
                       *const libc::c_char, 674 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    x =
        get_afl_env(b"MSAN_OPTIONS\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char) as *mut u8_0;
    if !x.is_null() {
        if strstr(x as *const libc::c_char,
                  b"exit_code=86\x00" as *const u8 as
                      *const libc::c_char).is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mCustom MSAN_OPTIONS set without exit_code=86 - please fix!\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-analyze.c\x00" as *const u8 as
                       *const libc::c_char, 684 as libc::c_int);
            exit(1 as libc::c_int);
        }
        if strstr(x as *const libc::c_char,
                  b"symbolize=0\x00" as *const u8 as
                      *const libc::c_char).is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mCustom MSAN_OPTIONS set without symbolize=0 - please fix!\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-analyze.c\x00" as *const u8 as
                       *const libc::c_char, 687 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    setenv(b"ASAN_OPTIONS\x00" as *const u8 as *const libc::c_char,
           b"abort_on_error=1:detect_leaks=0:symbolize=0:allocator_may_return_null=1\x00"
               as *const u8 as *const libc::c_char, 0 as libc::c_int);
    setenv(b"MSAN_OPTIONS\x00" as *const u8 as *const libc::c_char,
           b"exit_code=86:symbolize=0:abort_on_error=1:allocator_may_return_null=1:msan_track_origins=0\x00"
               as *const u8 as *const libc::c_char, 0 as libc::c_int);
    if !get_afl_env(b"AFL_PRELOAD\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char).is_null() {
        if qemu_mode != 0 {
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
                           b"src/afl-analyze.c\x00" as *const u8 as
                               *const libc::c_char, 718 as libc::c_int);
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
                                    b"src/afl-analyze.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    724 as libc::c_int);
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
                                    b"src/afl-analyze.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    727 as libc::c_int);
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
/* Display usage hints. */
unsafe extern "C" fn usage(mut argv0: *mut u8_0) {
    printf(b"\n%s [ options ] -- /path/to/target_app [ ... ]\n\nRequired parameters:\n  -i file       - input test case to be analyzed by the tool\n\nExecution control settings:\n  -f file       - input file read by the tested program (stdin)\n  -t msec       - timeout for each run (%d ms)\n  -m megs       - memory limit for child process (%d MB)\n  -Q            - use binary-only instrumentation (QEMU mode)\n  -U            - use unicorn-based instrumentation (Unicorn mode)\n  -W            - use qemu-based instrumentation with Wine (Wine mode)\n\nAnalysis settings:\n  -e            - look for edge coverage only, ignore hit counts\n\nFor additional tips, please consult %s/README.md.\n\nEnvironment variables used:\nTMPDIR: directory to use for temporary input files\nASAN_OPTIONS: custom settings for ASAN\n              (must contain abort_on_error=1 and symbolize=0)\nMSAN_OPTIONS: custom settings for MSAN\n              (must contain exitcode=86 and symbolize=0)\nAFL_PRELOAD: LD_PRELOAD / DYLD_INSERT_LIBRARIES settings for target\nAFL_ANALYZE_HEX: print file offsets in hexadecimal instead of decimal\nAFL_SKIP_BIN_CHECK: skip checking the location of and the target\n\x00"
               as *const u8 as *const libc::c_char, argv0,
           1000 as libc::c_int, 50 as libc::c_int, doc_path);
    exit(1 as libc::c_int);
}
/* Find binary. */
unsafe extern "C" fn find_binary(mut fname: *mut u8_0) {
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
        target_path = DFL_ck_strdup(fname);
        if stat(target_path as *const libc::c_char, &mut st) != 0 ||
               !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
                     0o100000 as libc::c_int as libc::c_uint) ||
               st.st_mode & 0o111 as libc::c_int as libc::c_uint == 0 ||
               st.st_size < 4 as libc::c_int as libc::c_long {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mProgram \'%s\' not found or not executable\x00"
                       as *const u8 as *const libc::c_char, fname);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-analyze.c\x00" as *const u8 as
                       *const libc::c_char, 821 as libc::c_int);
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
                target_path =
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
                                    b"src/afl-analyze.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    842 as libc::c_int);
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
            } else { target_path = DFL_ck_strdup(fname) }
            DFL_ck_free(cur_elem as *mut libc::c_void);
            if stat(target_path as *const libc::c_char, &mut st) == 0 &&
                   st.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
                       0o100000 as libc::c_int as libc::c_uint &&
                   st.st_mode & 0o111 as libc::c_int as libc::c_uint != 0 &&
                   st.st_size >= 4 as libc::c_int as libc::c_long {
                break ;
            }
            DFL_ck_free(target_path as *mut libc::c_void);
            target_path = 0 as *mut u8_0
        }
        if target_path.is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mProgram \'%s\' not found or not executable\x00"
                       as *const u8 as *const libc::c_char, fname);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-analyze.c\x00" as *const u8 as
                       *const libc::c_char, 857 as libc::c_int);
            exit(1 as libc::c_int);
        }
    };
}
/* Main entry point */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char,
                 mut envp: *mut *mut libc::c_char) -> libc::c_int {
    let mut opt: s32 = 0;
    let mut mem_limit_given: u8_0 = 0 as libc::c_int as u8_0;
    let mut timeout_given: u8_0 = 0 as libc::c_int as u8_0;
    let mut unicorn_mode: u8_0 = 0 as libc::c_int as u8_0;
    let mut use_wine: u8_0 = 0 as libc::c_int as u8_0;
    let mut use_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    doc_path =
        if access(b"/usr/local/share/doc/afl\x00" as *const u8 as
                      *const libc::c_char, 0 as libc::c_int) != 0 {
            b"docs\x00" as *const u8 as *const libc::c_char
        } else {
            b"/usr/local/share/doc/afl\x00" as *const u8 as
                *const libc::c_char
        } as *mut u8_0;
    printf(b"\x1b[0;36mafl-analyze++2.63d\x1b[0m by Michal Zalewski\n\x00" as
               *const u8 as *const libc::c_char);
    loop  {
        opt =
            getopt(argc, argv,
                   b"+i:f:m:t:eQUWh\x00" as *const u8 as *const libc::c_char);
        if !(opt > 0 as libc::c_int) { break ; }
        match opt {
            105 => {
                if !in_file.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -i options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-analyze.c\x00" as *const u8 as
                               *const libc::c_char, 881 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                in_file = optarg as *mut u8_0
            }
            102 => {
                if !prog_in.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -f options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-analyze.c\x00" as *const u8 as
                               *const libc::c_char, 887 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                use_stdin = 0 as libc::c_int as u8_0;
                prog_in = optarg as *mut u8_0
            }
            101 => {
                if edges_only != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -e options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-analyze.c\x00" as *const u8 as
                               *const libc::c_char, 894 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                edges_only = 1 as libc::c_int as u8_0
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
                           b"src/afl-analyze.c\x00" as *const u8 as
                               *const libc::c_char, 902 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                mem_limit_given = 1 as libc::c_int as u8_0;
                if optarg.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBad syntax used for -m\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-analyze.c\x00" as *const u8 as
                               *const libc::c_char, 905 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if strcmp(optarg,
                          b"none\x00" as *const u8 as *const libc::c_char) ==
                       0 {
                    mem_limit = 0 as libc::c_int as u64_0
                } else {
                    if sscanf(optarg,
                              b"%llu%c\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut mem_limit as *mut u64_0,
                              &mut suffix as *mut u8_0) < 1 as libc::c_int ||
                           *optarg.offset(0 as libc::c_int as isize) as
                               libc::c_int == '-' as i32 {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBad syntax used for -m\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-analyze.c\x00" as *const u8 as
                                   *const libc::c_char, 916 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                    match suffix as libc::c_int {
                        84 => {
                            mem_limit =
                                (mem_limit as
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
                            mem_limit =
                                (mem_limit as
                                     libc::c_ulonglong).wrapping_mul(1024 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulonglong)
                                    as u64_0 as u64_0
                        }
                        107 => {
                            mem_limit =
                                (mem_limit as
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
                                   b"src/afl-analyze.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   925 as libc::c_int);
                            exit(1 as libc::c_int);
                        }
                    }
                    if mem_limit < 5 as libc::c_int as libc::c_ulonglong {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mDangerously low value of -m\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-analyze.c\x00" as *const u8 as
                                   *const libc::c_char, 929 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                    if ::std::mem::size_of::<rlim_t>() as libc::c_ulong ==
                           4 as libc::c_int as libc::c_ulong &&
                           mem_limit >
                               2000 as libc::c_int as libc::c_ulonglong {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mValue of -m out of range on 32-bit systems\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-analyze.c\x00" as *const u8 as
                                   *const libc::c_char, 932 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                }
            }
            116 => {
                if timeout_given != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -t options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-analyze.c\x00" as *const u8 as
                               *const libc::c_char, 940 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                timeout_given = 1 as libc::c_int as u8_0;
                if optarg.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWrong usage of -t\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-analyze.c\x00" as *const u8 as
                               *const libc::c_char, 943 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                exec_tmout = atoi(optarg) as u32_0;
                if exec_tmout < 10 as libc::c_int as libc::c_uint ||
                       *optarg.offset(0 as libc::c_int as isize) as
                           libc::c_int == '-' as i32 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mDangerously low value of -t\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-analyze.c\x00" as *const u8 as
                               *const libc::c_char, 948 as libc::c_int);
                    exit(1 as libc::c_int);
                }
            }
            81 => {
                if qemu_mode != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -Q options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-analyze.c\x00" as *const u8 as
                               *const libc::c_char, 954 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if mem_limit_given == 0 {
                    mem_limit = 200 as libc::c_int as u64_0
                }
                qemu_mode = 1 as libc::c_int as u8_0
            }
            85 => {
                if unicorn_mode != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -U options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-analyze.c\x00" as *const u8 as
                               *const libc::c_char, 962 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if mem_limit_given == 0 {
                    mem_limit = 200 as libc::c_int as u64_0
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
                           b"src/afl-analyze.c\x00" as *const u8 as
                               *const libc::c_char, 970 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                qemu_mode = 1 as libc::c_int as u8_0;
                use_wine = 1 as libc::c_int as u8_0;
                if mem_limit_given == 0 {
                    mem_limit = 0 as libc::c_int as u64_0
                }
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
    if optind == argc || in_file.is_null() {
        usage(*argv.offset(0 as libc::c_int as isize) as *mut u8_0);
    }
    use_hex_offsets =
        !get_afl_env(b"AFL_ANALYZE_HEX\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char).is_null()
            as libc::c_int as u8_0;
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
    trace_bits =
        afl_shm_init(&mut shm,
                     ((1 as libc::c_int) << 16 as libc::c_int) as size_t,
                     0 as libc::c_int as libc::c_uchar);
    atexit(Some(at_exit_handler as unsafe extern "C" fn() -> ()));
    setup_signal_handlers();
    set_up_environment();
    find_binary(*argv.offset(optind as isize) as *mut u8_0);
    detect_file_args(argv.offset(optind as isize), prog_in, &mut use_stdin);
    if qemu_mode != 0 {
        if use_wine != 0 {
            use_argv =
                get_wine_argv(*argv.offset(0 as libc::c_int as isize) as
                                  *mut u8_0, &mut target_path, argc - optind,
                              argv.offset(optind as isize))
        } else {
            use_argv =
                get_qemu_argv(*argv.offset(0 as libc::c_int as isize) as
                                  *mut u8_0, &mut target_path, argc - optind,
                              argv.offset(optind as isize))
        }
    } else { use_argv = argv.offset(optind as isize) }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    read_initial_file();
    printf(b"\x1b[1;94m[*] \x1b[0mPerforming dry run (mem limit = %llu MB, timeout = %u ms%s)...\x00"
               as *const u8 as *const libc::c_char, mem_limit, exec_tmout,
           if edges_only as libc::c_int != 0 {
               b", edges only\x00" as *const u8 as *const libc::c_char
           } else { b"\x00" as *const u8 as *const libc::c_char });
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    run_target(use_argv, in_data, in_len, 1 as libc::c_int as u8_0);
    if child_timed_out != 0 {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mTarget binary times out (adjusting -t may help).\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-analyze.c\x00" as *const u8 as *const libc::c_char,
               1026 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if get_afl_env(b"AFL_SKIP_BIN_CHECK\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char).is_null() &&
           anything_set() == 0 {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mNo instrumentation detected.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-analyze.c\x00" as *const u8 as *const libc::c_char,
               1029 as libc::c_int);
        exit(1 as libc::c_int);
    }
    analyze(use_argv);
    printf(b"\x1b[1;92m[+] \x1b[0mWe\'re done here. Have a nice day!\n\x00" as
               *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    afl_shm_deinit(&mut shm);
    exit(0 as libc::c_int);
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
