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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
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
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn afl_shm_init(_: *mut sharedmem_t, _: size_t, dumb_mode: libc::c_uchar)
     -> *mut u8_0;
    #[no_mangle]
    fn afl_shm_deinit(_: *mut sharedmem_t);
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
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
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
    static mut doc_path: *mut u8_0;
    #[no_mangle]
    fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
              __shortopts: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    #[no_mangle]
    fn setitimer(__which: __itimer_which_t, __new: *const itimerval,
                 __old: *mut itimerval) -> libc::c_int;
    #[no_mangle]
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                 __oact: *mut sigaction) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
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
/*
   american fuzzy lop++ - type definitions and minor macros
   --------------------------------------------------------

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

 */
pub type u8_0 = uint8_t;
pub type u32_0 = uint32_t;
/*

   Ugh. There is an unintended compiler / glibc #include glitch caused by
   combining the u64 type an %llu in format strings, necessitating a workaround.

   In essence, the compiler is always looking for 'unsigned long long' for %llu.
   On 32-bit systems, the u64 type (aliased to uint64_t) is expanded to
   'unsigned long long' in <bits/types.h>, so everything checks out.

   But on 64-bit systems, it is #ifdef'ed in the same file as 'unsigned long'.
   Now, it only happens in circumstances where the type happens to have the
   expected bit width, *but* the compiler does not know that... and complains
   about 'unsigned long' being unsafe to pass to %llu.

 */
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
pub type rlim_t = __rlim_t;
// extern unsigned char *trace_bits;
/* ID of the SHM region              */
/* shared memory region */
/* actual allocated size */
/* in use by shmem app */
/*
   american fuzzy lop++ - error-checking, memory-zeroing alloc routines
   --------------------------------------------------------------------

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

   This allocator is not designed to resist malicious attackers (the canaries
   are small and predictable), but provides a robust and portable way to detect
   use-after-free, off-by-one writes, stale pointers, and so on.

 */
/* Initial size used for ck_maybe_grow */
// Be careful! _WANT_ORIGINAL_AFL_ALLOC is not compatible with custom mutators
// afl++ stuff without memory corruption checks - for speed
/* User-facing macro to sprintf() to a dynamically allocated buffer. */
/* Macro to enforce allocation limits as a last-resort defense against
   integer overflows. */
/* Macro to check malloc() failures and the like. */
/* Allocator increments for ck_realloc_block(). */
/* Allocate a buffer, explicitly not zeroing it. Returns NULL for zero-sized
   requests. */
/* Allocate a buffer, returning zeroed memory. */
#[inline]
unsafe extern "C" fn DFL_ck_alloc(mut size: u32_0) -> *mut libc::c_void {
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 { return 0 as *mut libc::c_void }
    mem = DFL_ck_alloc_nozero(size);
    return memset(mem, 0 as libc::c_int, size as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn next_pow2(mut in_0: size_t) -> size_t {
    if in_0 == 0 as libc::c_int as libc::c_ulong ||
           in_0 > -(1 as libc::c_int) as size_t {
        return 0 as libc::c_int as size_t
    }
    let mut out: size_t =
        in_0.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    out |= out >> 1 as libc::c_int;
    out |= out >> 2 as libc::c_int;
    out |= out >> 4 as libc::c_int;
    out |= out >> 8 as libc::c_int;
    out |= out >> 16 as libc::c_int;
    return out.wrapping_add(1 as libc::c_int as libc::c_ulong);
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
#[inline]
unsafe extern "C" fn DFL_ck_free(mut mem: *mut libc::c_void) {
    if mem.is_null() { return }
    free(mem);
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
   american fuzzy lop++ - test case minimizer
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

   A simple test case minimizer that takes an input file and tries to remove
   as much data as possible while keeping the binary in a crashing state
   *or* producing consistent instrumentation output (the mode is auto-selected
   based on the initially observed behavior).

 */
static mut mask_bitmap: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
/* Mask for trace bits (-B)          */
static mut in_file: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
/* Minimizer input test case         */
static mut output_file: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
/* Minimizer output file             */
static mut in_data: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
/* Input data for trimming           */
static mut in_len: u32_0 = 0;
/* Input data length                 */
static mut orig_cksum: u32_0 = 0;
/* Original checksum                 */
static mut total_execs: u32_0 = 0;
/* Total number of execs             */
static mut missed_hangs: u32_0 = 0;
/* Misses due to hangs               */
static mut missed_crashes: u32_0 = 0;
/* Misses due to crashes             */
static mut missed_paths: u32_0 = 0;
/* Misses due to exec path diffs     */
static mut crash_mode: u8_0 = 0;
/* Crash-centric mode?               */
static mut hang_mode: u8_0 = 0;
/* Minimize as long as it hangs      */
static mut exit_crash: u8_0 = 0;
/* Treat non-zero exit as crash?     */
static mut edges_only: u8_0 = 0;
/* Ignore hit counts?                */
static mut exact_mode: u8_0 = 0;
/* Require path match for crashes?   */
static mut stop_soon: u8_0 = 0;
/* Ctrl-C pressed?                   */
/*
 * forkserver section
 */
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
/* Apply mask to classified bitmap (if set). */
unsafe extern "C" fn apply_mask(mut mem: *mut u32_0, mut mask: *mut u32_0) {
    let mut i: u32_0 =
        ((1 as libc::c_int) << 16 as libc::c_int >> 2 as libc::c_int) as
            u32_0;
    if mask.is_null() { return }
    loop  {
        let fresh4 = i;
        i = i.wrapping_sub(1);
        if !(fresh4 != 0) { break ; }
        *mem &= !*mask;
        mem = mem.offset(1);
        mask = mask.offset(1)
    };
}
/* See if any bytes are set in the bitmap. */
#[inline]
unsafe extern "C" fn anything_set(mut fsrv: *mut afl_forkserver_t) -> u8_0 {
    let mut ptr: *mut u32_0 = (*fsrv).trace_bits as *mut u32_0;
    let mut i: u32_0 =
        ((1 as libc::c_int) << 16 as libc::c_int >> 2 as libc::c_int) as
            u32_0;
    loop  {
        let fresh5 = i;
        i = i.wrapping_sub(1);
        if !(fresh5 != 0) { break ; }
        let fresh6 = ptr;
        ptr = ptr.offset(1);
        if *fresh6 != 0 { return 1 as libc::c_int as u8_0 }
    }
    return 0 as libc::c_int as u8_0;
}
unsafe extern "C" fn at_exit_handler() { afl_fsrv_killall(); }
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
               b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
               175 as libc::c_int);
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
               b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
               177 as libc::c_int);
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
               b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
               180 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   185 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   185 as libc::c_int);
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
               b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
               203 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   205 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   205 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    lseek(ret, 0 as libc::c_int as __off_t, 0 as libc::c_int);
    return ret;
}
/* Write modified data to file for testing. If use_stdin is clear, the old file
   is unlinked and a new one is created. Otherwise, out_fd is rewound and
   truncated. */
unsafe extern "C" fn write_to_testcase(mut fsrv: *mut afl_forkserver_t,
                                       mut mem: *mut libc::c_void,
                                       mut len: u32_0) {
    let mut fd: s32 = (*fsrv).out_fd; /* Ignore errors. */
    if (*fsrv).use_stdin == 0 {
        unlink((*fsrv).out_file as *const libc::c_char);
        fd =
            open((*fsrv).out_file as *const libc::c_char,
                 0o1 as libc::c_int | 0o100 as libc::c_int |
                     0o200 as libc::c_int, 0o600 as libc::c_int);
        if fd < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                       as *const u8 as *const libc::c_char, (*fsrv).out_file);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   227 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
    } else { lseek(fd, 0 as libc::c_int as __off_t, 0 as libc::c_int); }
    let mut _len: u32_0 = len;
    let mut _res: s32 = write(fd, mem, _len as size_t) as s32;
    if _res as libc::c_uint != _len {
        if _res < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                       as *const u8 as *const libc::c_char, (*fsrv).out_file);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   233 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   233 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    if (*fsrv).use_stdin != 0 {
        if ftruncate(fd, len as __off_t) != 0 {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mftruncate() failed\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   237 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        lseek(fd, 0 as libc::c_int as __off_t, 0 as libc::c_int);
    } else { close(fd); };
}
/* Execute target application. Returns 0 if the changes are a dud, or
   1 if they should be kept. */
unsafe extern "C" fn run_target(mut fsrv: *mut afl_forkserver_t,
                                mut argv: *mut *mut libc::c_char,
                                mut mem: *mut u8_0, mut len: u32_0,
                                mut first_run: u8_0) -> u8_0 {
    let mut it: itimerval =
        itimerval{it_interval: timeval{tv_sec: 0, tv_usec: 0,},
                  it_value: timeval{tv_sec: 0, tv_usec: 0,},};
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut cksum: u32_0 = 0;
    (*fsrv).child_timed_out = 0 as libc::c_int as u8_0;
    memset((*fsrv).trace_bits as *mut libc::c_void, 0 as libc::c_int,
           (*fsrv).map_size as libc::c_ulong);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   272 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   272 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   279 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   279 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    if (*fsrv).child_pid <= 0 as libc::c_int {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mFork server is misbehaving (OOM?)\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
               283 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   299 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   299 as libc::c_int);
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
               b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
               314 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if hang_mode == 0 {
        classify_counts((*fsrv).trace_bits);
        apply_mask((*fsrv).trace_bits as *mut u32_0,
                   mask_bitmap as *mut u32_0);
    }
    total_execs = total_execs.wrapping_add(1);
    if stop_soon != 0 {
        printf(b"\x1b[0m\x1b[1;91m\n+++ Minimization aborted by user +++\n\x1b[0m\x00"
                   as *const u8 as *const libc::c_char);
        close(write_to_file(output_file, in_data, in_len));
        exit(1 as libc::c_int);
    }
    /* Always discard inputs that time out, unless we are in hang mode */
    if hang_mode != 0 {
        if (*fsrv).child_timed_out != 0 { return 1 as libc::c_int as u8_0 }
        if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as
               libc::c_schar as libc::c_int >> 1 as libc::c_int >
               0 as libc::c_int ||
               status & 0x7f as libc::c_int == 0 as libc::c_int &&
                   (status & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                       86 as libc::c_int ||
               status & 0x7f as libc::c_int == 0 as libc::c_int &&
                   (status & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0
                   && exit_crash as libc::c_int != 0 {
            missed_crashes = missed_crashes.wrapping_add(1)
        } else { missed_hangs = missed_hangs.wrapping_add(1) }
        return 0 as libc::c_int as u8_0
    }
    if (*fsrv).child_timed_out != 0 {
        missed_hangs = missed_hangs.wrapping_add(1);
        return 0 as libc::c_int as u8_0
    }
    /* Handle crashing inputs depending on current mode. */
    if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar as
           libc::c_int >> 1 as libc::c_int > 0 as libc::c_int ||
           status & 0x7f as libc::c_int == 0 as libc::c_int &&
               (status & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                   86 as libc::c_int ||
           status & 0x7f as libc::c_int == 0 as libc::c_int &&
               (status & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0 &&
               exit_crash as libc::c_int != 0 {
        if first_run != 0 { crash_mode = 1 as libc::c_int as u8_0 }
        if crash_mode != 0 {
            if exact_mode == 0 { return 1 as libc::c_int as u8_0 }
        } else {
            missed_crashes = missed_crashes.wrapping_add(1);
            return 0 as libc::c_int as u8_0
        }
    } else if crash_mode != 0 {
        missed_paths = missed_paths.wrapping_add(1);
        return 0 as libc::c_int as u8_0
    }
    cksum =
        hash32((*fsrv).trace_bits as *const libc::c_void, (*fsrv).map_size,
               0xa5b35705 as libc::c_uint);
    if first_run != 0 { orig_cksum = cksum }
    if orig_cksum == cksum { return 1 as libc::c_int as u8_0 }
    missed_paths = missed_paths.wrapping_add(1);
    return 0 as libc::c_int as u8_0;
}
/* Handle non-crashing inputs appropriately. */
/* Actually minimize! */
unsafe extern "C" fn minimize(mut fsrv: *mut afl_forkserver_t,
                              mut argv: *mut *mut libc::c_char) {
    static mut alpha_map: [u32_0; 256] = [0; 256];
    let mut tmp_buf: *mut u8_0 = DFL_ck_alloc_nozero(in_len) as *mut u8_0;
    let mut orig_len: u32_0 = in_len;
    let mut stage_o_len: u32_0 = 0;
    let mut del_len: u32_0 = 0;
    let mut set_len: u32_0 = 0;
    let mut del_pos: u32_0 = 0;
    let mut set_pos: u32_0 = 0;
    let mut i: u32_0 = 0;
    let mut alpha_size: u32_0 = 0;
    let mut cur_pass: u32_0 = 0 as libc::c_int as u32_0;
    let mut syms_removed: u32_0 = 0;
    let mut alpha_del0: u32_0 = 0 as libc::c_int as u32_0;
    let mut alpha_del1: u32_0 = 0;
    let mut alpha_del2: u32_0 = 0;
    let mut alpha_d_total: u32_0 = 0 as libc::c_int as u32_0;
    let mut changed_any: u8_0 = 0;
    let mut prev_del: u8_0 = 0;
    /* **********************
   * BLOCK NORMALIZATION *
   ***********************/
    set_len =
        next_pow2(in_len.wrapping_div(128 as libc::c_int as libc::c_uint) as
                      size_t) as u32_0;
    set_pos = 0 as libc::c_int as u32_0;
    if set_len < 4 as libc::c_int as libc::c_uint {
        set_len = 4 as libc::c_int as u32_0
    }
    printf(b"\x1b[1;94m[*] \x1b[0m\x1b[1;97mStage #0: \x1b[0mOne-time block normalization...\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    while set_pos < in_len {
        let mut use_len: u32_0 =
            ({
                 let mut _a: u32_0 = set_len;
                 let mut _b: libc::c_uint = in_len.wrapping_sub(set_pos);
                 if _a < _b { _a } else { _b }
             });
        i = 0 as libc::c_int as u32_0;
        while i < use_len {
            if *in_data.offset(set_pos.wrapping_add(i) as isize) as
                   libc::c_int != '0' as i32 {
                break ;
            }
            i = i.wrapping_add(1)
        }
        if i != use_len {
            memcpy(tmp_buf as *mut libc::c_void,
                   in_data as *const libc::c_void, in_len as libc::c_ulong);
            memset(tmp_buf.offset(set_pos as isize) as *mut libc::c_void,
                   '0' as i32, use_len as libc::c_ulong);
            let mut res: u8_0 = 0;
            res =
                run_target(fsrv, argv, tmp_buf, in_len,
                           0 as libc::c_int as u8_0);
            if res != 0 {
                memset(in_data.offset(set_pos as isize) as *mut libc::c_void,
                       '0' as i32, use_len as libc::c_ulong);
                /*        changed_any = 1; value is not used */
                alpha_del0 =
                    (alpha_del0 as libc::c_uint).wrapping_add(use_len) as
                        u32_0 as u32_0
            }
        }
        set_pos =
            (set_pos as libc::c_uint).wrapping_add(set_len) as u32_0 as u32_0
    }
    alpha_d_total =
        (alpha_d_total as libc::c_uint).wrapping_add(alpha_del0) as u32_0 as
            u32_0;
    printf(b"\x1b[1;92m[+] \x1b[0mBlock normalization complete, %u byte%s replaced.\x00"
               as *const u8 as *const libc::c_char, alpha_del0,
           if alpha_del0 == 1 as libc::c_int as libc::c_uint {
               b"\x00" as *const u8 as *const libc::c_char
           } else { b"s\x00" as *const u8 as *const libc::c_char });
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    loop  {
        cur_pass = cur_pass.wrapping_add(1);
        printf(b"\x1b[1;94m[*] \x1b[0m\x1b[1;93m--- \x1b[1;97mPass #%u \x1b[1;93m---\x00"
                   as *const u8 as *const libc::c_char, cur_pass);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        changed_any = 0 as libc::c_int as u8_0;
        /* *****************
   * BLOCK DELETION *
   ******************/
        del_len =
            next_pow2(in_len.wrapping_div(16 as libc::c_int as libc::c_uint)
                          as size_t) as u32_0;
        stage_o_len = in_len;
        printf(b"\x1b[1;94m[*] \x1b[0m\x1b[1;97mStage #1: \x1b[0mRemoving blocks of data...\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        loop  {
            if del_len == 0 { del_len = 1 as libc::c_int as u32_0 }
            del_pos = 0 as libc::c_int as u32_0;
            prev_del = 1 as libc::c_int as u8_0;
            printf(b"\x1b[1;90m    Block length = %u, remaining size = %u\n\x1b[0m\x00"
                       as *const u8 as *const libc::c_char, del_len, in_len);
            while del_pos < in_len {
                let mut res_0: u8_0 = 0;
                let mut tail_len: s32 = 0;
                tail_len =
                    in_len.wrapping_sub(del_pos).wrapping_sub(del_len) as s32;
                if tail_len < 0 as libc::c_int { tail_len = 0 as libc::c_int }
                /* If we have processed at least one full block (initially, prev_del == 1),
       and we did so without deleting the previous one, and we aren't at the
       very end of the buffer (tail_len > 0), and the current block is the same
       as the previous one... skip this step as a no-op. */
                if prev_del == 0 && tail_len != 0 &&
                       memcmp(in_data.offset(del_pos as
                                                 isize).offset(-(del_len as
                                                                     isize))
                                  as *const libc::c_void,
                              in_data.offset(del_pos as isize) as
                                  *const libc::c_void,
                              del_len as libc::c_ulong) == 0 {
                    del_pos =
                        (del_pos as libc::c_uint).wrapping_add(del_len) as
                            u32_0 as u32_0
                } else {
                    prev_del = 0 as libc::c_int as u8_0;
                    /* Head */
                    memcpy(tmp_buf as *mut libc::c_void,
                           in_data as *const libc::c_void,
                           del_pos as libc::c_ulong);
                    /* Tail */
                    memcpy(tmp_buf.offset(del_pos as isize) as
                               *mut libc::c_void,
                           in_data.offset(del_pos as
                                              isize).offset(del_len as isize)
                               as *const libc::c_void,
                           tail_len as libc::c_ulong);
                    res_0 =
                        run_target(fsrv, argv, tmp_buf,
                                   del_pos.wrapping_add(tail_len as
                                                            libc::c_uint),
                                   0 as libc::c_int as u8_0);
                    if res_0 != 0 {
                        memcpy(in_data as *mut libc::c_void,
                               tmp_buf as *const libc::c_void,
                               del_pos.wrapping_add(tail_len as libc::c_uint)
                                   as libc::c_ulong);
                        prev_del = 1 as libc::c_int as u8_0;
                        in_len =
                            del_pos.wrapping_add(tail_len as libc::c_uint);
                        changed_any = 1 as libc::c_int as u8_0
                    } else {
                        del_pos =
                            (del_pos as libc::c_uint).wrapping_add(del_len) as
                                u32_0 as u32_0
                    }
                }
            }
            if !(del_len > 1 as libc::c_int as libc::c_uint &&
                     in_len >= 1 as libc::c_int as libc::c_uint) {
                break ;
            }
            del_len =
                (del_len as
                     libc::c_uint).wrapping_div(2 as libc::c_int as
                                                    libc::c_uint) as u32_0 as
                    u32_0
        }
        printf(b"\x1b[1;92m[+] \x1b[0mBlock removal complete, %u bytes deleted.\x00"
                   as *const u8 as *const libc::c_char,
               stage_o_len.wrapping_sub(in_len));
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        if in_len == 0 && changed_any as libc::c_int != 0 {
            printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0m\x1b[1;91mDown to zero bytes - check the command line and mem limit!\x1b[0m\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
        if cur_pass > 1 as libc::c_int as libc::c_uint && changed_any == 0 {
            break ;
        }
        /* ************************
   * ALPHABET MINIMIZATION *
   *************************/
        alpha_size = 0 as libc::c_int as u32_0;
        alpha_del1 = 0 as libc::c_int as u32_0;
        syms_removed = 0 as libc::c_int as u32_0;
        memset(alpha_map.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<[u32_0; 256]>() as libc::c_ulong);
        i = 0 as libc::c_int as u32_0;
        while i < in_len {
            if alpha_map[*in_data.offset(i as isize) as usize] == 0 {
                alpha_size = alpha_size.wrapping_add(1)
            }
            alpha_map[*in_data.offset(i as isize) as usize] =
                alpha_map[*in_data.offset(i as isize) as
                              usize].wrapping_add(1);
            i = i.wrapping_add(1)
        }
        printf(b"\x1b[1;94m[*] \x1b[0m\x1b[1;97mStage #2: \x1b[0mMinimizing symbols (%u code point%s)...\x00"
                   as *const u8 as *const libc::c_char, alpha_size,
               if alpha_size == 1 as libc::c_int as libc::c_uint {
                   b"\x00" as *const u8 as *const libc::c_char
               } else { b"s\x00" as *const u8 as *const libc::c_char });
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int as u32_0;
        while i < 256 as libc::c_int as libc::c_uint {
            let mut r: u32_0 = 0;
            let mut res_1: u8_0 = 0;
            if !(i == '0' as i32 as libc::c_uint ||
                     alpha_map[i as usize] == 0) {
                memcpy(tmp_buf as *mut libc::c_void,
                       in_data as *const libc::c_void,
                       in_len as libc::c_ulong);
                r = 0 as libc::c_int as u32_0;
                while r < in_len {
                    if *tmp_buf.offset(r as isize) as libc::c_uint == i {
                        *tmp_buf.offset(r as isize) = '0' as i32 as u8_0
                    }
                    r = r.wrapping_add(1)
                }
                res_1 =
                    run_target(fsrv, argv, tmp_buf, in_len,
                               0 as libc::c_int as u8_0);
                if res_1 != 0 {
                    memcpy(in_data as *mut libc::c_void,
                           tmp_buf as *const libc::c_void,
                           in_len as libc::c_ulong);
                    syms_removed = syms_removed.wrapping_add(1);
                    alpha_del1 =
                        (alpha_del1 as
                             libc::c_uint).wrapping_add(alpha_map[i as usize])
                            as u32_0 as u32_0;
                    changed_any = 1 as libc::c_int as u8_0
                }
            }
            i = i.wrapping_add(1)
        }
        alpha_d_total =
            (alpha_d_total as libc::c_uint).wrapping_add(alpha_del1) as u32_0
                as u32_0;
        printf(b"\x1b[1;92m[+] \x1b[0mSymbol minimization finished, %u symbol%s (%u byte%s) replaced.\x00"
                   as *const u8 as *const libc::c_char, syms_removed,
               if syms_removed == 1 as libc::c_int as libc::c_uint {
                   b"\x00" as *const u8 as *const libc::c_char
               } else { b"s\x00" as *const u8 as *const libc::c_char },
               alpha_del1,
               if alpha_del1 == 1 as libc::c_int as libc::c_uint {
                   b"\x00" as *const u8 as *const libc::c_char
               } else { b"s\x00" as *const u8 as *const libc::c_char });
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        /* *************************
   * CHARACTER MINIMIZATION *
   **************************/
        alpha_del2 = 0 as libc::c_int as u32_0;
        printf(b"\x1b[1;94m[*] \x1b[0m\x1b[1;97mStage #3: \x1b[0mCharacter minimization...\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        memcpy(tmp_buf as *mut libc::c_void, in_data as *const libc::c_void,
               in_len as libc::c_ulong);
        i = 0 as libc::c_int as u32_0;
        while i < in_len {
            let mut res_2: u8_0 = 0;
            let mut orig: u8_0 = *tmp_buf.offset(i as isize);
            if !(orig as libc::c_int == '0' as i32) {
                *tmp_buf.offset(i as isize) = '0' as i32 as u8_0;
                res_2 =
                    run_target(fsrv, argv, tmp_buf, in_len,
                               0 as libc::c_int as u8_0);
                if res_2 != 0 {
                    *in_data.offset(i as isize) = '0' as i32 as u8_0;
                    alpha_del2 = alpha_del2.wrapping_add(1);
                    changed_any = 1 as libc::c_int as u8_0
                } else { *tmp_buf.offset(i as isize) = orig }
            }
            i = i.wrapping_add(1)
        }
        alpha_d_total =
            (alpha_d_total as libc::c_uint).wrapping_add(alpha_del2) as u32_0
                as u32_0;
        printf(b"\x1b[1;92m[+] \x1b[0mCharacter minimization done, %u byte%s replaced.\x00"
                   as *const u8 as *const libc::c_char, alpha_del2,
               if alpha_del2 == 1 as libc::c_int as libc::c_uint {
                   b"\x00" as *const u8 as *const libc::c_char
               } else { b"s\x00" as *const u8 as *const libc::c_char });
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        if !(changed_any != 0) { break ; }
    }
    if !tmp_buf.is_null() { DFL_ck_free(tmp_buf as *mut libc::c_void); }
    if hang_mode != 0 {
        printf(b"\n\x1b[1;90m     File size reduced by : \x1b[0m%0.02f%% (to %u byte%s)\n\x1b[1;90m    Characters simplified : \x1b[0m%0.02f%%\n\x1b[1;90m     Number of execs done : \x1b[0m%u\n\x1b[1;90m          Fruitless execs : \x1b[0mtermination=%u crash=%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               100 as libc::c_int as libc::c_double -
                   in_len as libc::c_double *
                       100 as libc::c_int as libc::c_double /
                       orig_len as libc::c_double, in_len,
               if in_len == 1 as libc::c_int as libc::c_uint {
                   b"\x00" as *const u8 as *const libc::c_char
               } else { b"s\x00" as *const u8 as *const libc::c_char },
               alpha_d_total as libc::c_double *
                   100 as libc::c_int as libc::c_double /
                   (if in_len != 0 {
                        in_len
                    } else { 1 as libc::c_int as libc::c_uint }) as
                       libc::c_double, total_execs, missed_paths,
               missed_crashes);
        return
    }
    printf(b"\n\x1b[1;90m     File size reduced by : \x1b[0m%0.02f%% (to %u byte%s)\n\x1b[1;90m    Characters simplified : \x1b[0m%0.02f%%\n\x1b[1;90m     Number of execs done : \x1b[0m%u\n\x1b[1;90m          Fruitless execs : \x1b[0mpath=%u crash=%u hang=%s%u\n\n\x00"
               as *const u8 as *const libc::c_char,
           100 as libc::c_int as libc::c_double -
               in_len as libc::c_double * 100 as libc::c_int as libc::c_double
                   / orig_len as libc::c_double, in_len,
           if in_len == 1 as libc::c_int as libc::c_uint {
               b"\x00" as *const u8 as *const libc::c_char
           } else { b"s\x00" as *const u8 as *const libc::c_char },
           alpha_d_total as libc::c_double *
               100 as libc::c_int as libc::c_double /
               (if in_len != 0 {
                    in_len
                } else { 1 as libc::c_int as libc::c_uint }) as
                   libc::c_double, total_execs, missed_paths, missed_crashes,
           if missed_hangs != 0 {
               b"\x1b[1;91m\x00" as *const u8 as *const libc::c_char
           } else { b"\x00" as *const u8 as *const libc::c_char },
           missed_hangs);
    if total_execs > 50 as libc::c_int as libc::c_uint &&
           missed_hangs.wrapping_mul(10 as libc::c_int as libc::c_uint) >
               total_execs && hang_mode == 0 {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0m\x1b[1;91mFrequent timeouts - results may be skewed.\x1b[0m\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
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
    let mut x: *mut u8_0 = 0 as *mut u8_0;
    (*fsrv).dev_null_fd =
        open(b"/dev/null\x00" as *const u8 as *const libc::c_char,
             0o2 as libc::c_int);
    if (*fsrv).dev_null_fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open /dev/null\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
               682 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    if (*fsrv).out_file.is_null() {
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
        (*fsrv).out_file =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/.afl-tmin-temp-%u\x00" as *const u8 as
                                  *const libc::c_char, use_dir, getpid());
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-tmin.c\x00" as *const u8 as
                                *const libc::c_char, 695 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/.afl-tmin-temp-%u\x00" as *const u8 as
                              *const libc::c_char, use_dir, getpid());
                 _tmp
             })
    }
    unlink((*fsrv).out_file as *const libc::c_char);
    (*fsrv).out_fd =
        open((*fsrv).out_file as *const libc::c_char,
             0o2 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
             0o600 as libc::c_int);
    if (*fsrv).out_fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, (*fsrv).out_file);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
               703 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   712 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   715 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   725 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   728 as libc::c_int);
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
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 759 as libc::c_int);
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
                                    b"src/afl-tmin.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    765 as libc::c_int);
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
                                    b"src/afl-tmin.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    768 as libc::c_int);
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
    printf(b"\n%s [ options ] -- /path/to/target_app [ ... ]\n\nRequired parameters:\n  -i file       - input test case to be shrunk by the tool\n  -o file       - final output location for the minimized data\n\nExecution control settings:\n  -f file       - input file read by the tested program (stdin)\n  -t msec       - timeout for each run (%d ms)\n  -m megs       - memory limit for child process (%d MB)\n  -Q            - use binary-only instrumentation (QEMU mode)\n  -U            - use unicorn-based instrumentation (Unicorn mode)\n  -W            - use qemu-based instrumentation with Wine (Wine mode)\n                  (Not necessary, here for consistency with other afl-* tools)\n\nMinimization settings:\n  -e            - solve for edge coverage only, ignore hit counts\n  -x            - treat non-zero exit codes as crashes\n\n  -H            - minimize a hang (hang mode)\nFor additional tips, please consult %s/README.md.\n\nEnvironment variables used:\nTMPDIR: directory to use for temporary input files\nASAN_OPTIONS: custom settings for ASAN\n              (must contain abort_on_error=1 and symbolize=0)\nMSAN_OPTIONS: custom settings for MSAN\n              (must contain exitcode=86 and symbolize=0)\nAFL_PRELOAD: LD_PRELOAD / DYLD_INSERT_LIBRARIES settings for target\nAFL_TMIN_EXACT: require execution paths to match for crashing inputs\n\x00"
               as *const u8 as *const libc::c_char, argv0,
           1000 as libc::c_int, 50 as libc::c_int, doc_path);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   866 as libc::c_int);
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
                                    b"src/afl-tmin.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    887 as libc::c_int);
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
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   903 as libc::c_int);
            exit(1 as libc::c_int);
        }
    };
}
/* Read mask bitmap from file. This is for the -B option. */
unsafe extern "C" fn read_bitmap(mut fname: *mut u8_0) {
    let mut fd: s32 = open(fname as *const libc::c_char, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                   as *const u8 as *const libc::c_char, fname);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
               915 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    let mut _len: u32_0 = ((1 as libc::c_int) << 16 as libc::c_int) as u32_0;
    let mut _res: s32 =
        read(fd, mask_bitmap as *mut libc::c_void, _len as size_t) as s32;
    if _res as libc::c_uint != _len {
        if _res < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort read from %s\x00"
                       as *const u8 as *const libc::c_char, fname);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   917 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        } else {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort read from %s\x00"
                       as *const u8 as *const libc::c_char, fname);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   917 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    close(fd);
}
/* Main entry point */
unsafe fn main_0(mut argc: libc::c_int, mut argv_orig: *mut *mut libc::c_char,
                 mut envp: *mut *mut libc::c_char) -> libc::c_int {
    let mut opt: s32 = 0;
    let mut mem_limit_given: u8_0 = 0 as libc::c_int as u8_0;
    let mut timeout_given: u8_0 = 0 as libc::c_int as u8_0;
    let mut unicorn_mode: u8_0 = 0 as libc::c_int as u8_0;
    let mut use_wine: u8_0 = 0 as libc::c_int as u8_0;
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
    printf(b"\x1b[0;36mafl-tmin++2.63d\x1b[0m by Michal Zalewski\n\x00" as
               *const u8 as *const libc::c_char);
    loop  {
        opt =
            getopt(argc, argv,
                   b"+i:o:f:m:t:B:xeQUWHh\x00" as *const u8 as
                       *const libc::c_char);
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
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 947 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                in_file = optarg as *mut u8_0
            }
            111 => {
                if !output_file.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -o options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 953 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                output_file = optarg as *mut u8_0
            }
            102 => {
                if !(*fsrv).out_file.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -f options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 959 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*fsrv).use_stdin = 0 as libc::c_int as u8_0;
                (*fsrv).out_file = optarg as *mut u8_0
            }
            101 => {
                if edges_only != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -e options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 966 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if hang_mode != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mEdges only and hang mode are mutually exclusive.\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 968 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                edges_only = 1 as libc::c_int as u8_0
            }
            120 => {
                if exit_crash != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -x options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 974 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                exit_crash = 1 as libc::c_int as u8_0
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
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 982 as libc::c_int);
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
                               b"src/afl-tmin.c\x00" as *const u8 as
                                   *const libc::c_char, 994 as libc::c_int);
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
                                   b"src/afl-tmin.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   1003 as libc::c_int);
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
                               b"src/afl-tmin.c\x00" as *const u8 as
                                   *const libc::c_char, 1007 as libc::c_int);
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
                               b"src/afl-tmin.c\x00" as *const u8 as
                                   *const libc::c_char, 1010 as libc::c_int);
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
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 1018 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                timeout_given = 1 as libc::c_int as u8_0;
                (*fsrv).exec_tmout = atoi(optarg) as u32_0;
                if (*fsrv).exec_tmout < 10 as libc::c_int as libc::c_uint ||
                       *optarg.offset(0 as libc::c_int as isize) as
                           libc::c_int == '-' as i32 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mDangerously low value of -t\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 1024 as libc::c_int);
                    exit(1 as libc::c_int);
                }
            }
            81 => {
                if (*fsrv).qemu_mode != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -Q options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 1030 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if mem_limit_given == 0 {
                    (*fsrv).mem_limit = 200 as libc::c_int as u64_0
                }
                (*fsrv).qemu_mode = 1 as libc::c_int as u8_0
            }
            85 => {
                if unicorn_mode != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -Q options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 1038 as libc::c_int);
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
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 1046 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*fsrv).qemu_mode = 1 as libc::c_int as u8_0;
                use_wine = 1 as libc::c_int as u8_0;
                if mem_limit_given == 0 {
                    (*fsrv).mem_limit = 0 as libc::c_int as u64_0
                }
            }
            72 => {
                /* Hang Mode */
                /* Minimizes a testcase to the minimum that still times out */
                if hang_mode != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultipe -H options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 1058 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if edges_only != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mEdges only and hang mode are mutually exclusive.\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 1060 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                hang_mode = 1 as libc::c_int as u8_0
            }
            66 => {
                /* load bitmap */
                /* This is a secret undocumented option! It is speculated to be useful
           if you have a baseline "boring" input file and another "interesting"
           file you want to minimize.

           You can dump a binary bitmap for the boring file using
           afl-showmap -b, and then load it into afl-tmin via -B. The minimizer
           will then minimize to preserve only the edges that are unique to
           the interesting input file, but ignoring everything from the
           original map.

           The option may be extended and made more official if it proves
           to be useful. */
                if !mask_bitmap.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -B options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-tmin.c\x00" as *const u8 as
                               *const libc::c_char, 1079 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                mask_bitmap =
                    DFL_ck_alloc(((1 as libc::c_int) << 16 as libc::c_int) as
                                     u32_0) as *mut u8_0;
                read_bitmap(optarg as *mut u8_0);
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
    if optind == argc || in_file.is_null() || output_file.is_null() {
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
    atexit(Some(at_exit_handler as unsafe extern "C" fn() -> ()));
    setup_signal_handlers();
    set_up_environment(fsrv);
    find_binary(fsrv, *argv.offset(optind as isize) as *mut u8_0);
    detect_file_args(argv.offset(optind as isize), (*fsrv).out_file,
                     &mut (*fsrv).use_stdin);
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
    exact_mode =
        !get_afl_env(b"AFL_TMIN_EXACT\x00" as *const u8 as *const libc::c_char
                         as *mut libc::c_char).is_null() as libc::c_int as
            u8_0;
    if hang_mode as libc::c_int != 0 && exact_mode as libc::c_int != 0 {
        printf(b"AFL_TMIN_EXACT won\'t work for loops in hang mode, ignoring.\x00"
                   as *const u8 as *const libc::c_char);
        exact_mode = 0 as libc::c_int as u8_0
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    read_initial_file();
    afl_fsrv_start(fsrv, use_argv, &mut stop_soon,
                   if !get_afl_env(b"AFL_DEBUG_CHILD_OUTPUT\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char).is_null() {
                       1 as libc::c_int
                   } else { 0 as libc::c_int } as u8_0);
    printf(b"\x1b[1;94m[*] \x1b[0mPerforming dry run (mem limit = %llu MB, timeout = %u ms%s)...\x00"
               as *const u8 as *const libc::c_char, (*fsrv).mem_limit,
           (*fsrv).exec_tmout,
           if edges_only as libc::c_int != 0 {
               b", edges only\x00" as *const u8 as *const libc::c_char
           } else { b"\x00" as *const u8 as *const libc::c_char });
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    run_target(fsrv, use_argv, in_data, in_len, 1 as libc::c_int as u8_0);
    if hang_mode as libc::c_int != 0 && (*fsrv).child_timed_out == 0 {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mTarget binary did not time out but hang minimization mode (-H) was set (-t %u).\x00"
                   as *const u8 as *const libc::c_char, (*fsrv).exec_tmout);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
               1146 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if (*fsrv).child_timed_out as libc::c_int != 0 && hang_mode == 0 {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mTarget binary times out (adjusting -t may help). Use -H to minimize a hang.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
               1151 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if hang_mode != 0 {
        printf(b"\x1b[1;92m[+] \x1b[0mProgram hangs as expected, minimizing in \x1b[0;36mhang\x1b[0m mode.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    } else if crash_mode == 0 {
        printf(b"\x1b[1;92m[+] \x1b[0mProgram terminates normally, minimizing in \x1b[0;36minstrumented\x1b[0m mode.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        if anything_set(fsrv) == 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mNo instrumentation detected.\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-tmin.c\x00" as *const u8 as *const libc::c_char,
                   1162 as libc::c_int);
            exit(1 as libc::c_int);
        }
    } else {
        printf(b"\x1b[1;92m[+] \x1b[0mProgram exits with a signal, minimizing in \x1b[0;35m%scrash\x1b[0m mode.\x00"
                   as *const u8 as *const libc::c_char,
               if exact_mode as libc::c_int != 0 {
                   b"EXACT \x00" as *const u8 as *const libc::c_char
               } else { b"\x00" as *const u8 as *const libc::c_char });
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    minimize(fsrv, use_argv);
    printf(b"\x1b[1;94m[*] \x1b[0mWriting output to \'%s\'...\x00" as
               *const u8 as *const libc::c_char, output_file);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    unlink((*fsrv).out_file as *const libc::c_char);
    if !(*fsrv).out_file.is_null() {
        DFL_ck_free((*fsrv).out_file as *mut libc::c_void);
    }
    (*fsrv).out_file = 0 as *mut u8_0;
    close(write_to_file(output_file, in_data, in_len));
    printf(b"\x1b[1;92m[+] \x1b[0mWe\'re done here. Have a nice day!\n\x00" as
               *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    afl_shm_deinit(&mut shm);
    afl_fsrv_deinit(fsrv);
    if !(*fsrv).target_path.is_null() {
        DFL_ck_free((*fsrv).target_path as *mut libc::c_void);
    }
    if !mask_bitmap.is_null() {
        DFL_ck_free(mask_bitmap as *mut libc::c_void);
    }
    if !in_data.is_null() { DFL_ck_free(in_data as *mut libc::c_void); }
    argv_cpy_free(argv);
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
