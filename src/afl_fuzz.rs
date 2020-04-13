#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]
#[macro_use]
extern crate c2rust_bitfields;
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn access(__name: *const libc::c_char, __type: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char,
               _: libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn random() -> libc::c_long;
    #[no_mangle]
    fn srandom(__seed: libc::c_uint);
    #[no_mangle]
    fn rand() -> libc::c_int;
    #[no_mangle]
    fn srand(__seed: libc::c_uint);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
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
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn afl_shm_init(_: *mut sharedmem_t, _: size_t, dumb_mode: libc::c_uchar)
     -> *mut u8_0;
    #[no_mangle]
    fn afl_shm_deinit(_: *mut sharedmem_t);
    #[no_mangle]
    fn afl_fsrv_init(fsrv: *mut afl_forkserver_t);
    #[no_mangle]
    fn afl_fsrv_init_dup(fsrv_to: *mut afl_forkserver_t,
                         from: *mut afl_forkserver_t);
    #[no_mangle]
    fn afl_fsrv_start(fsrv: *mut afl_forkserver_t,
                      argv: *mut *mut libc::c_char, stop_soon_p: *mut u8_0,
                      debug_child_output: u8_0);
    #[no_mangle]
    fn afl_fsrv_deinit(fsrv: *mut afl_forkserver_t);
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
              __shortopts: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void)
     -> libc::c_int;
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
    /* path to documentation dir        */
    /* Get unix time in milliseconds */
    #[no_mangle]
    fn get_cur_time() -> u64_0;
    #[no_mangle]
    fn save_cmdline(_: *mut afl_state_t, _: u32_0, _: *mut *mut libc::c_char);
    #[no_mangle]
    fn setup_signal_handlers();
    #[no_mangle]
    fn check_if_tty(_: *mut afl_state_t);
    #[no_mangle]
    fn fix_up_banner(_: *mut afl_state_t, _: *mut u8_0);
    #[no_mangle]
    fn check_binary(_: *mut afl_state_t, _: *mut u8_0);
    #[no_mangle]
    fn check_asan_opts();
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut power_names: [*mut libc::c_char; 8];
    #[no_mangle]
    fn afl_state_init(_: *mut afl_state_t);
    #[no_mangle]
    fn afl_state_deinit(_: *mut afl_state_t);
    #[no_mangle]
    fn read_afl_environment(_: *mut afl_state_t, _: *mut *mut libc::c_char);
    #[no_mangle]
    fn setup_custom_mutator(_: *mut afl_state_t);
    #[no_mangle]
    fn destroy_custom_mutator(_: *mut afl_state_t);
    #[no_mangle]
    fn destroy_queue(_: *mut afl_state_t);
    #[no_mangle]
    fn cull_queue(_: *mut afl_state_t);
    #[no_mangle]
    fn read_bitmap(_: *mut afl_state_t, _: *mut u8_0);
    #[no_mangle]
    fn write_bitmap(_: *mut afl_state_t);
    #[no_mangle]
    fn init_count_class16();
    #[no_mangle]
    fn load_extras(_: *mut afl_state_t, _: *mut u8_0);
    #[no_mangle]
    fn save_auto(_: *mut afl_state_t);
    #[no_mangle]
    fn load_auto(_: *mut afl_state_t);
    #[no_mangle]
    fn destroy_extras(_: *mut afl_state_t);
    #[no_mangle]
    fn write_stats_file(_: *mut afl_state_t, _: libc::c_double,
                        _: libc::c_double, _: libc::c_double);
    #[no_mangle]
    fn maybe_update_plot_file(_: *mut afl_state_t, _: libc::c_double,
                              _: libc::c_double);
    #[no_mangle]
    fn show_stats(_: *mut afl_state_t);
    #[no_mangle]
    fn show_init_stats(_: *mut afl_state_t);
    #[no_mangle]
    fn sync_fuzzers(_: *mut afl_state_t);
    #[no_mangle]
    fn fuzz_one(_: *mut afl_state_t) -> u8_0;
    #[no_mangle]
    fn bind_to_free_cpu(_: *mut afl_state_t);
    #[no_mangle]
    fn setup_post(_: *mut afl_state_t);
    #[no_mangle]
    fn read_testcases(_: *mut afl_state_t);
    #[no_mangle]
    fn perform_dry_run(_: *mut afl_state_t);
    #[no_mangle]
    fn pivot_inputs(_: *mut afl_state_t);
    #[no_mangle]
    fn find_start_position(_: *mut afl_state_t) -> u32_0;
    #[no_mangle]
    fn find_timeout(_: *mut afl_state_t);
    #[no_mangle]
    fn setup_dirs_fds(_: *mut afl_state_t);
    #[no_mangle]
    fn setup_cmdline_file(_: *mut afl_state_t, _: *mut *mut libc::c_char);
    #[no_mangle]
    fn setup_stdio_file(_: *mut afl_state_t);
    #[no_mangle]
    fn check_crash_handling();
    #[no_mangle]
    fn check_cpu_governor(_: *mut afl_state_t);
    #[no_mangle]
    fn get_core_count(_: *mut afl_state_t);
    #[no_mangle]
    fn fix_up_sync(_: *mut afl_state_t);
    /* Execs the child */
    #[no_mangle]
    fn cmplog_exec_child(fsrv: *mut afl_forkserver_t,
                         argv: *mut *mut libc::c_char);
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __rlim64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub type s64 = int64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmp_map {
    pub headers: [cmp_header; 65536],
    pub log: [[cmp_operands; 256]; 65536],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmp_operands {
    pub v0: u64_0,
    pub v1: u64_0,
}
/*
   american fuzzy lop++ - cmplog header
   ------------------------------------

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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C, packed)]
pub struct cmp_header {
    #[bitfield(name = "hits", ty = "libc::c_uint", bits = "0..=19")]
    #[bitfield(name = "cnt", ty = "libc::c_uint", bits = "20..=39")]
    #[bitfield(name = "id", ty = "libc::c_uint", bits = "40..=55")]
    #[bitfield(name = "shape", ty = "libc::c_uint", bits = "56..=60")]
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "61..=61")]
    pub hits_cnt_id_shape_type_0: [u8; 8],
}
pub type sharedmem_t = sharedmem;
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
pub type afl_forkserver_t = afl_forkserver;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub type rlim_t = __rlim64_t;
/*
   american fuzzy lop++ - prealloc a buffer to reuse small elements often
   ----------------------------------------------------------------------

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
/* If we know we'll reuse small elements often, we'll just preallocate a buffer,
 * then fall back to malloc */
// TODO: Replace free status check with bitmask+CLZ
pub type prealloc_status = libc::c_uint;
/* system malloc */
/* used in buf */
pub const PRE_STATUS_MALLOC: prealloc_status = 2;
/* free in buf */
pub const PRE_STATUS_USED: prealloc_status = 1;
pub const PRE_STATUS_UNUSED: prealloc_status = 0;
pub type pre_status_t = prealloc_status;
/*
   american fuzzy lop++ - linked list code
   ---------------------------------------

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
/* How many elements to allocate before malloc is needed */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_element {
    pub pre_status: pre_status_t,
    pub prev: *mut list_element,
    pub next: *mut list_element,
    pub data: *mut libc::c_void,
}
pub type element_t = list_element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct queue_entry {
    pub fname: *mut u8_0,
    pub len: u32_0,
    pub cal_failed: u8_0,
    pub trim_done: u8_0,
    pub was_fuzzed: u8_0,
    pub passed_det: u8_0,
    pub has_new_cov: u8_0,
    pub var_behavior: u8_0,
    pub favored: u8_0,
    pub fs_redundant: u8_0,
    pub fully_colorized: u8_0,
    pub bitmap_size: u32_0,
    pub fuzz_level: u32_0,
    pub exec_cksum: u32_0,
    pub exec_us: u64_0,
    pub handicap: u64_0,
    pub n_fuzz: u64_0,
    pub depth: u64_0,
    pub trace_mini: *mut u8_0,
    pub tc_ref: u32_0,
    pub next: *mut queue_entry,
    pub next_100: *mut queue_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extra_data {
    pub data: *mut u8_0,
    pub len: u32_0,
    pub hit_cnt: u32_0,
}
pub type C2RustUnnamed = libc::c_uint;
pub const FAULT_NOBITS: C2RustUnnamed = 5;
pub const FAULT_NOINST: C2RustUnnamed = 4;
pub const FAULT_ERROR: C2RustUnnamed = 3;
pub const FAULT_CRASH: C2RustUnnamed = 2;
pub const FAULT_TMOUT: C2RustUnnamed = 1;
pub const FAULT_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const POWER_SCHEDULES_NUM: C2RustUnnamed_0 = 8;
pub const RARE: C2RustUnnamed_0 = 7;
pub const MMOPT: C2RustUnnamed_0 = 6;
pub const EXPLOIT: C2RustUnnamed_0 = 5;
pub const QUAD: C2RustUnnamed_0 = 4;
pub const LIN: C2RustUnnamed_0 = 3;
pub const COE: C2RustUnnamed_0 = 2;
pub const FAST: C2RustUnnamed_0 = 1;
pub const EXPLORE: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MOpt_globals {
    pub finds: *mut u64_0,
    pub finds_v2: *mut u64_0,
    pub cycles: *mut u64_0,
    pub cycles_v2: *mut u64_0,
    pub cycles_v3: *mut u64_0,
    pub is_pilot_mode: u32_0,
    pub pTime: *mut u64_0,
    pub period: u64_0,
    pub havoc_stagename: *mut libc::c_char,
    pub splice_stageformat: *mut libc::c_char,
    pub havoc_stagenameshort: *mut libc::c_char,
    pub splice_stagenameshort: *mut libc::c_char,
}
pub type MOpt_globals_t = MOpt_globals;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct afl_env_vars {
    pub afl_skip_cpufreq: u8_0,
    pub afl_exit_when_done: u8_0,
    pub afl_no_affinity: u8_0,
    pub afl_skip_bin_check: u8_0,
    pub afl_dumb_forksrv: u8_0,
    pub afl_import_first: u8_0,
    pub afl_custom_mutator_only: u8_0,
    pub afl_no_ui: u8_0,
    pub afl_force_ui: u8_0,
    pub afl_i_dont_care_about_missing_crashes: u8_0,
    pub afl_bench_just_one: u8_0,
    pub afl_bench_until_crash: u8_0,
    pub afl_debug_child_output: u8_0,
    pub afl_autoresume: u8_0,
    pub afl_cal_fast: u8_0,
    pub afl_tmpdir: *mut u8_0,
    pub afl_post_library: *mut u8_0,
    pub afl_custom_mutator_library: *mut u8_0,
    pub afl_python_module: *mut u8_0,
    pub afl_path: *mut u8_0,
    pub afl_hang_tmout: *mut u8_0,
    pub afl_skip_crashes: *mut u8_0,
    pub afl_preload: *mut u8_0,
}
pub type afl_env_vars_t = afl_env_vars;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct afl_state {
    pub _id: u32_0,
    pub fsrv: afl_forkserver_t,
    pub shm: sharedmem_t,
    pub afl_env: afl_env_vars_t,
    pub argv: *mut *mut libc::c_char,
    pub orig_hit_cnt_puppet: u64_0,
    pub last_limit_time_start: u64_0,
    pub tmp_pilot_time: u64_0,
    pub total_pacemaker_time: u64_0,
    pub total_puppet_find: u64_0,
    pub temp_puppet_find: u64_0,
    pub most_time_key: u64_0,
    pub most_time: u64_0,
    pub most_execs_key: u64_0,
    pub most_execs: u64_0,
    pub old_hit_count: u64_0,
    pub force_ui_update: u64_0,
    pub mopt_globals_core: MOpt_globals_t,
    pub mopt_globals_pilot: MOpt_globals_t,
    pub limit_time_puppet: s32,
    pub SPLICE_CYCLES_puppet: s32,
    pub limit_time_sig: s32,
    pub key_puppet: s32,
    pub key_module: s32,
    pub w_init: libc::c_double,
    pub w_end: libc::c_double,
    pub w_now: libc::c_double,
    pub g_now: s32,
    pub g_max: s32,
    pub tmp_core_time: u64_0,
    pub swarm_now: s32,
    pub x_now: [[libc::c_double; 16]; 5],
    pub L_best: [[libc::c_double; 16]; 5],
    pub eff_best: [[libc::c_double; 16]; 5],
    pub G_best: [libc::c_double; 16],
    pub v_now: [[libc::c_double; 16]; 5],
    pub probability_now: [[libc::c_double; 16]; 5],
    pub swarm_fitness: [libc::c_double; 5],
    pub stage_finds_puppet: [[u64_0; 16]; 5],
    pub stage_finds_puppet_v2: [[u64_0; 16]; 5],
    pub stage_cycles_puppet_v2: [[u64_0; 16]; 5],
    pub stage_cycles_puppet_v3: [[u64_0; 16]; 5],
    pub stage_cycles_puppet: [[u64_0; 16]; 5],
    pub operator_finds_puppet: [u64_0; 16],
    pub core_operator_finds_puppet: [u64_0; 16],
    pub core_operator_finds_puppet_v2: [u64_0; 16],
    pub core_operator_cycles_puppet: [u64_0; 16],
    pub core_operator_cycles_puppet_v2: [u64_0; 16],
    pub core_operator_cycles_puppet_v3: [u64_0; 16],
    pub period_pilot_tmp: libc::c_double,
    pub key_lv: s32,
    pub in_dir: *mut u8_0,
    pub out_dir: *mut u8_0,
    pub tmp_dir: *mut u8_0,
    pub sync_dir: *mut u8_0,
    pub sync_id: *mut u8_0,
    pub power_name: *mut u8_0,
    pub use_banner: *mut u8_0,
    pub in_bitmap: *mut u8_0,
    pub file_extension: *mut u8_0,
    pub orig_cmdline: *mut u8_0,
    pub infoexec: *mut u8_0,
    pub hang_tmout: u32_0,
    pub cal_cycles: u8_0,
    pub cal_cycles_long: u8_0,
    pub no_unlink: u8_0,
    pub debug: u8_0,
    pub custom_only: u8_0,
    pub python_only: u8_0,
    pub stats_update_freq: u32_0,
    pub schedule: u8_0,
    pub havoc_max_mult: u8_0,
    pub use_radamsa: u8_0,
    pub radamsa_mutate_ptr: Option<unsafe extern "C" fn(_: *mut u8_0,
                                                        _: size_t,
                                                        _: *mut u8_0,
                                                        _: size_t, _: u32_0)
                                       -> size_t>,
    pub skip_deterministic: u8_0,
    pub force_deterministic: u8_0,
    pub use_splicing: u8_0,
    pub dumb_mode: u8_0,
    pub score_changed: u8_0,
    pub kill_signal: u8_0,
    pub resuming_fuzz: u8_0,
    pub timeout_given: u8_0,
    pub not_on_tty: u8_0,
    pub term_too_small: u8_0,
    pub no_forkserver: u8_0,
    pub crash_mode: u8_0,
    pub in_place_resume: u8_0,
    pub autoresume: u8_0,
    pub auto_changed: u8_0,
    pub no_cpu_meter_red: u8_0,
    pub no_arith: u8_0,
    pub shuffle_queue: u8_0,
    pub bitmap_changed: u8_0,
    pub unicorn_mode: u8_0,
    pub use_wine: u8_0,
    pub skip_requested: u8_0,
    pub run_over10m: u8_0,
    pub persistent_mode: u8_0,
    pub deferred_mode: u8_0,
    pub fixed_seed: u8_0,
    pub fast_cal: u8_0,
    pub disable_trim: u8_0,
    pub virgin_bits: [u8_0; 65536],
    pub virgin_tmout: [u8_0; 65536],
    pub virgin_crash: [u8_0; 65536],
    pub var_bytes: [u8_0; 65536],
    pub stop_soon: u8_0,
    pub clear_screen: u8_0,
    pub queued_paths: u32_0,
    pub queued_variable: u32_0,
    pub queued_at_start: u32_0,
    pub queued_discovered: u32_0,
    pub queued_imported: u32_0,
    pub queued_favored: u32_0,
    pub queued_with_cov: u32_0,
    pub pending_not_fuzzed: u32_0,
    pub pending_favored: u32_0,
    pub cur_skipped_paths: u32_0,
    pub cur_depth: u32_0,
    pub max_depth: u32_0,
    pub useless_at_start: u32_0,
    pub var_byte_count: u32_0,
    pub current_entry: u32_0,
    pub havoc_div: u32_0,
    pub total_crashes: u64_0,
    pub unique_crashes: u64_0,
    pub total_tmouts: u64_0,
    pub unique_tmouts: u64_0,
    pub unique_hangs: u64_0,
    pub total_execs: u64_0,
    pub last_crash_execs: u64_0,
    pub queue_cycle: u64_0,
    pub cycles_wo_finds: u64_0,
    pub trim_execs: u64_0,
    pub bytes_trim_in: u64_0,
    pub bytes_trim_out: u64_0,
    pub blocks_eff_total: u64_0,
    pub blocks_eff_select: u64_0,
    pub start_time: u64_0,
    pub last_path_time: u64_0,
    pub last_crash_time: u64_0,
    pub last_hang_time: u64_0,
    pub slowest_exec_ms: u32_0,
    pub subseq_tmouts: u32_0,
    pub stage_name: *mut u8_0,
    pub stage_short: *mut u8_0,
    pub syncing_party: *mut u8_0,
    pub stage_name_buf: [u8_0; 64],
    pub stage_cur: s32,
    pub stage_max: s32,
    pub splicing_with: s32,
    pub master_id: u32_0,
    pub master_max: u32_0,
    pub syncing_case: u32_0,
    pub stage_cur_byte: s32,
    pub stage_cur_val: s32,
    pub stage_val_type: u8_0,
    pub stage_finds: [u64_0; 32],
    pub stage_cycles: [u64_0; 32],
    pub rand_cnt: u32_0,
    pub rand_seed: [u32_0; 2],
    pub init_seed: s64,
    pub total_cal_us: u64_0,
    pub total_cal_cycles: u64_0,
    pub total_bitmap_size: u64_0,
    pub total_bitmap_entries: u64_0,
    pub cpu_core_count: s32,
    pub cpu_aff: s32,
    pub queue: *mut queue_entry,
    pub queue_cur: *mut queue_entry,
    pub queue_top: *mut queue_entry,
    pub q_prev100: *mut queue_entry,
    pub top_rated: [*mut queue_entry; 65536],
    pub extras: *mut extra_data,
    pub extras_cnt: u32_0,
    pub a_extras: *mut extra_data,
    pub a_extras_cnt: u32_0,
    pub post_init: Option<unsafe extern "C" fn(_: *mut afl_state)
                              -> *mut libc::c_void>,
    pub post_handler: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: *mut u8_0, _: u32_0,
                                                  _: *mut *mut u8_0)
                                 -> size_t>,
    pub post_deinit: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                -> *mut libc::c_void>,
    pub post_data: *mut libc::c_void,
    pub cmplog_binary: *mut libc::c_char,
    pub cmplog_fsrv: afl_forkserver_t,
    pub mutator: *mut custom_mutator,
    pub cmplog_fsrv_ctl_fd: s32,
    pub cmplog_fsrv_st_fd: s32,
    pub cmplog_prev_timed_out: u32_0,
    pub describe_op_buf_256: [u8_0; 256],
    pub maybe_add_auto: *mut libc::c_void,
    pub last_bitmap_cvg: libc::c_double,
    pub last_stability: libc::c_double,
    pub last_eps: libc::c_double,
    pub plot_prev_qp: u32_0,
    pub plot_prev_pf: u32_0,
    pub plot_prev_pnf: u32_0,
    pub plot_prev_ce: u32_0,
    pub plot_prev_md: u32_0,
    pub plot_prev_qc: u64_0,
    pub plot_prev_uc: u64_0,
    pub plot_prev_uh: u64_0,
    pub stats_last_stats_ms: u64_0,
    pub stats_last_plot_ms: u64_0,
    pub stats_last_ms: u64_0,
    pub stats_last_execs: u64_0,
    pub stats_avg_exec: libc::c_double,
    pub clean_trace: [u8_0; 65536],
    pub clean_trace_custom: [u8_0; 65536],
    pub first_trace: [u8_0; 65536],
    pub out_buf: *mut u8_0,
    pub out_size: size_t,
    pub out_scratch_buf: *mut u8_0,
    pub out_scratch_size: size_t,
    pub eff_buf: *mut u8_0,
    pub eff_size: size_t,
    pub in_buf: *mut u8_0,
    pub in_size: size_t,
    pub in_scratch_buf: *mut u8_0,
    pub in_scratch_size: size_t,
    pub ex_buf: *mut u8_0,
    pub ex_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct custom_mutator {
    pub name: *const libc::c_char,
    pub dh: *mut libc::c_void,
    pub pre_save_buf: *mut u8_0,
    pub pre_save_size: size_t,
    pub data: *mut libc::c_void,
    pub afl_custom_init: Option<unsafe extern "C" fn(_: *mut afl_state_t,
                                                     _: libc::c_uint)
                                    -> *mut libc::c_void>,
    pub afl_custom_fuzz: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                     _: *mut u8_0, _: size_t,
                                                     _: *mut *mut u8_0,
                                                     _: *mut u8_0, _: size_t,
                                                     _: size_t) -> size_t>,
    pub afl_custom_pre_save: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                         _: *mut u8_0,
                                                         _: size_t,
                                                         _: *mut *mut u8_0)
                                        -> size_t>,
    pub afl_custom_init_trim: Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_void,
                                                          _: *mut u8_0,
                                                          _: size_t) -> s32>,
    pub afl_custom_trim: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                     _: *mut *mut u8_0)
                                    -> size_t>,
    pub afl_custom_post_trim: Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_void,
                                                          _: u8_0) -> s32>,
    pub afl_custom_havoc_mutation: Option<unsafe extern "C" fn(_:
                                                                   *mut libc::c_void,
                                                               _: *mut u8_0,
                                                               _: size_t,
                                                               _:
                                                                   *mut *mut u8_0,
                                                               _: size_t)
                                              -> size_t>,
    pub afl_custom_havoc_mutation_probability: Option<unsafe extern "C" fn(_:
                                                                               *mut libc::c_void)
                                                          -> u8_0>,
    pub afl_custom_queue_get: Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_void,
                                                          _: *const u8_0)
                                         -> u8_0>,
    pub afl_custom_queue_new_entry: Option<unsafe extern "C" fn(_:
                                                                    *mut libc::c_void,
                                                                _:
                                                                    *const u8_0,
                                                                _:
                                                                    *const u8_0)
                                               -> ()>,
    pub afl_custom_deinit: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                      -> ()>,
}
pub type afl_state_t = afl_state;
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
/* Allocate a buffer, returning zeroed memory. */
#[inline]
unsafe extern "C" fn DFL_ck_alloc(mut size: u32_0) -> *mut libc::c_void {
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 { return 0 as *mut libc::c_void }
    mem = DFL_ck_alloc_nozero(size);
    return memset(mem, 0 as libc::c_int, size as libc::c_ulong);
}
/* Free memory, checking for double free and corrupted heap. When DEBUG_BUILD
   is set, the old memory will be also clobbered with 0xFF. */
#[inline]
unsafe extern "C" fn DFL_ck_free(mut mem: *mut libc::c_void) {
    if mem.is_null() { return }
    free(mem);
}
/* Create a buffer with a copy of a string. Returns NULL for NULL inputs. */
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
/*
   american fuzzy lop++ - fuzzer code
   --------------------------------

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

   This is the real deal: the program takes an instrumented binary and
   attempts a variety of basic fuzzing tricks, paying close attention to
   how they affect the execution path.

 */
unsafe extern "C" fn get_libradamsa_path(mut own_loc: *mut u8_0)
 -> *mut u8_0 {
    let mut tmp: *mut u8_0 = 0 as *mut u8_0;
    let mut cp: *mut u8_0 = 0 as *mut u8_0;
    let mut rsl: *mut u8_0 = 0 as *mut u8_0;
    let mut own_copy: *mut u8_0 = 0 as *mut u8_0;
    tmp =
        getenv(b"AFL_PATH\x00" as *const u8 as *const libc::c_char) as
            *mut u8_0;
    if !tmp.is_null() {
        cp =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/libradamsa.so\x00" as *const u8 as
                                  *const libc::c_char, tmp);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz.c\x00" as *const u8 as
                                *const libc::c_char, 37 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/libradamsa.so\x00" as *const u8 as
                              *const libc::c_char, tmp);
                 _tmp
             });
        if access(cp as *const libc::c_char, 1 as libc::c_int) != 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to find \'%s\'\x00"
                       as *const u8 as *const libc::c_char, cp);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
                   39 as libc::c_int);
            exit(1 as libc::c_int);
        }
        return cp
    }
    own_copy = DFL_ck_strdup(own_loc);
    rsl = strrchr(own_copy as *const libc::c_char, '/' as i32) as *mut u8_0;
    if !rsl.is_null() {
        *rsl = 0 as libc::c_int as u8_0;
        cp =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/libradamsa.so\x00" as *const u8 as
                                  *const libc::c_char, own_copy);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz.c\x00" as *const u8 as
                                *const libc::c_char, 52 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/libradamsa.so\x00" as *const u8 as
                              *const libc::c_char, own_copy);
                 _tmp
             });
        DFL_ck_free(own_copy as *mut libc::c_void);
        if access(cp as *const libc::c_char, 1 as libc::c_int) == 0 {
            return cp
        }
    } else { DFL_ck_free(own_copy as *mut libc::c_void); }
    if access(b"/usr/local/lib/afl/libradamsa.so\x00" as *const u8 as
                  *const libc::c_char, 1 as libc::c_int) == 0 {
        return DFL_ck_strdup(b"/usr/local/lib/afl/libradamsa.so\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut u8_0)
    }
    if access(b"/usr/local/bin/libradamsa.so\x00" as *const u8 as
                  *const libc::c_char, 1 as libc::c_int) == 0 {
        return DFL_ck_strdup(b"/usr/local/bin/libradamsa.so\x00" as *const u8
                                 as *const libc::c_char as *mut u8_0)
    }
    printf(b"\n\x1b[1;91m[-] \x1b[0mOops, unable to find the \'libradamsa.so\' binary. The binary must be built\n    separately using \'make radamsa\'. If you already have the binary installed,\n    you may need to specify AFL_PATH in the environment.\n\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mFailed to locate \'libradamsa.so\'.\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00" as
               *const u8 as *const libc::c_char,
           b"func_unknown\x00" as *const u8 as *const libc::c_char,
           b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
           80 as libc::c_int);
    exit(1 as libc::c_int);
}
/* Display usage hints. */
unsafe extern "C" fn usage(mut afl: *mut afl_state_t, mut argv0: *mut u8_0,
                           mut more_help: libc::c_int) {
    printf(b"\n%s [ options ] -- /path/to/fuzzed_app [ ... ]\n\nRequired parameters:\n  -i dir        - input directory with test cases\n  -o dir        - output directory for fuzzer findings\n\nExecution control settings:\n  -p schedule   - power schedules recompute a seed\'s performance score.\n                  <explore(default), fast, coe, lin, quad, exploit, mmopt, rare>\n                  see docs/power_schedules.md\n  -f file       - location read by the fuzzed program (stdin)\n  -t msec       - timeout for each run (auto-scaled, 50-%d ms)\n  -m megs       - memory limit for child process (%d MB)\n  -Q            - use binary-only instrumentation (QEMU mode)\n  -U            - use unicorn-based instrumentation (Unicorn mode)\n  -W            - use qemu-based instrumentation with Wine (Wine mode)\n\nMutator settings:\n  -R[R]         - add Radamsa as mutator, add another -R to exclusivly run it\n  -L minutes    - use MOpt(imize) mode and set the time limit for entering the\n                  pacemaker mode (minutes of no new paths). 0 = immediately,\n                  -1 = immediately and together with normal mutation).\n                  See docs/README.MOpt.md\n  -c program    - enable CmpLog by specifying a binary compiled for it.\n                  if using QEMU, just use -c 0.\n\nFuzzing behavior settings:\n  -N            - do not unlink the fuzzing input file (only for devices etc.!)\n  -d            - quick & dirty mode (skips deterministic steps)\n  -n            - fuzz without instrumentation (dumb mode)\n  -x dir        - optional fuzzer dictionary (see README.md, its really good!)\n\nTesting settings:\n  -s seed       - use a fixed seed for the RNG\n  -V seconds    - fuzz for a specific time then terminate\n  -E execs      - fuzz for a approx. no of total executions then terminate\n                  Note: not precise and can have several more executions.\n\nOther stuff:\n  -T text       - text banner to show on the screen\n  -M / -S id    - distributed mode (see docs/parallel_fuzzing.md)\n  -I command    - execute this command/script when a new crash is found\n  -B bitmap.txt - mutate a specific test case, use the out/fuzz_bitmap file\n  -C            - crash exploration mode (the peruvian rabbit thing)\n  -e ext        - file extension for the temporarily generated test case\n\n\x00"
               as *const u8 as *const libc::c_char, argv0,
           1000 as libc::c_int, 50 as libc::c_int);
    if more_help > 1 as libc::c_int {
        printf(b"Environment variables used:\nAFL_PATH: path to AFL support binaries\nAFL_QUIET: suppress forkserver status messages\nAFL_DEBUG_CHILD_OUTPUT: do not suppress stdout/stderr from target\nLD_BIND_LAZY: do not set LD_BIND_NOW env var for target\nAFL_BENCH_JUST_ONE: run the target just once\nAFL_DUMB_FORKSRV: use fork server without feedback from target\nAFL_CUSTOM_MUTATOR_LIBRARY: lib with afl_custom_fuzz() to mutate inputs\nAFL_CUSTOM_MUTATOR_ONLY: avoid AFL++\'s internal mutators\nAFL_PYTHON_MODULE: mutate and trim inputs with the specified Python module\nAFL_DEBUG: extra debugging output for Python mode trimming\nAFL_DISABLE_TRIM: disable the trimming of test cases\nAFL_NO_UI: switch status screen off\nAFL_FORCE_UI: force showing the status screen (for virtual consoles)\nAFL_NO_CPU_RED: avoid red color for showing very high cpu usage\nAFL_SKIP_CPUFREQ: do not warn about variable cpu clocking\nAFL_NO_SNAPSHOT: do not use the snapshot feature (if the snapshot lkm is loaded)\nAFL_NO_FORKSRV: run target via execve instead of using the forkserver\nAFL_NO_ARITH: skip arithmetic mutations in deterministic stage\nAFL_SHUFFLE_QUEUE: reorder the input queue randomly on startup\nAFL_FAST_CAL: limit the calibration stage to three cycles for speedup\nAFL_HANG_TMOUT: override timeout value (in milliseconds)\nAFL_PRELOAD: LD_PRELOAD / DYLD_INSERT_LIBRARIES settings for target\nAFL_TMPDIR: directory to use for input file generation (ramdisk recommended)\nAFL_IMPORT_FIRST: sync and import test cases from other fuzzer instances first\nAFL_NO_AFFINITY: do not check for an unused cpu core to use for fuzzing\nAFL_POST_LIBRARY: postprocess generated test cases before use as target input\nAFL_SKIP_CRASHES: during initial dry run do not terminate for crashing inputs\nAFL_I_DONT_CARE_ABOUT_MISSING_CRASHES: don\'t warn about core dump handlers\nASAN_OPTIONS: custom settings for ASAN\n              (must contain abort_on_error=1 and symbolize=0)\nMSAN_OPTIONS: custom settings for MSAN\n              (must contain exitcode=86 and symbolize=0)\nAFL_SKIP_BIN_CHECK: skip the check, if the target is an excutable\nAFL_EXIT_WHEN_DONE: exit when all inputs are run and no new finds are found\nAFL_BENCH_UNTIL_CRASH: exit soon when the first crashing input has been found\nAFL_AUTORESUME: resume fuzzing if directory specified by -o already exists\n\n\x00"
                   as *const u8 as *const libc::c_char);
    } else {
        printf(b"To view also the supported environment variables of afl-fuzz please use \"-hh\".\n\n\x00"
                   as *const u8 as *const libc::c_char);
    }
    printf(b"Compiled without python module support\n\x00" as *const u8 as
               *const libc::c_char);
    printf(b"For additional help please consult %s/README.md\n\n\x00" as
               *const u8 as *const libc::c_char, doc_path);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn stricmp(mut a: *const libc::c_char,
                             mut b: *const libc::c_char) -> libc::c_int {
    if a.is_null() || b.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mNull reference\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
               216 as libc::c_int);
        exit(1 as libc::c_int);
    }
    loop  {
        let mut d: libc::c_int = 0;
        d = tolower(*a as libc::c_int) - tolower(*b as libc::c_int);
        if d != 0 as libc::c_int || *a == 0 { return d }
        a = a.offset(1);
        b = b.offset(1)
    };
}
/* Main entry point */
unsafe fn main_0(mut argc: libc::c_int, mut argv_orig: *mut *mut libc::c_char,
                 mut envp: *mut *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut opt: s32 = 0;
    let mut prev_queued: u64_0 = 0 as libc::c_int as u64_0;
    let mut sync_interval_cnt: u32_0 = 0 as libc::c_int as u32_0;
    let mut seek_to: u32_0 = 0;
    let mut show_help: u32_0 = 0 as libc::c_int as u32_0;
    let mut extras_dir: *mut u8_0 = 0 as *mut u8_0;
    let mut mem_limit_given: u8_0 = 0 as libc::c_int as u8_0;
    let mut exit_1: u8_0 = 0 as libc::c_int as u8_0;
    let mut use_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut tz: timezone = timezone{tz_minuteswest: 0, tz_dsttime: 0,};
    let mut argv: *mut *mut libc::c_char = argv_cpy_dup(argc, argv_orig);
    let mut afl: *mut afl_state_t =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<afl_state_t>() as libc::c_ulong) as
            *mut afl_state_t;
    if afl.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mCould not create afl state\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
               245 as libc::c_int);
        exit(1 as libc::c_int);
    }
    afl_state_init(afl);
    afl_fsrv_init(&mut (*afl).fsrv);
    if !get_afl_env(b"AFL_DEBUG\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char).is_null() {
        (*afl).debug = 1 as libc::c_int as u8_0
    }
    read_afl_environment(afl, envp);
    exit_1 = ((*afl).afl_env.afl_bench_just_one != 0) as libc::c_int as u8_0;
    printf(b"\x1b[0;36mafl-fuzz++2.63d\x1b[0m based on afl by Michal Zalewski and a big online community\n\x00"
               as *const u8 as *const libc::c_char);
    doc_path =
        if access(b"/usr/local/share/doc/afl\x00" as *const u8 as
                      *const libc::c_char, 0 as libc::c_int) !=
               0 as libc::c_int {
            b"docs\x00" as *const u8 as *const libc::c_char as *mut u8_0
        } else {
            b"/usr/local/share/doc/afl\x00" as *const u8 as
                *const libc::c_char as *mut u8_0
        };
    gettimeofday(&mut tv, &mut tz as *mut timezone as *mut libc::c_void);
    (*afl).init_seed = tv.tv_sec ^ tv.tv_usec ^ getpid() as libc::c_long;
    loop  {
        opt =
            getopt(argc, argv,
                   b"+c:i:I:o:f:m:t:T:dnCB:S:M:x:QNUWe:p:s:V:E:L:hRP:\x00" as
                       *const u8 as *const libc::c_char);
        if !(opt > 0 as libc::c_int) { break ; }
        match opt {
            73 => { (*afl).infoexec = optarg as *mut u8_0 }
            99 => {
                (*afl).shm.cmplog_mode = 1 as libc::c_int;
                (*afl).cmplog_binary =
                    DFL_ck_strdup(optarg as *mut u8_0) as *mut libc::c_char
            }
            115 => {
                (*afl).init_seed =
                    strtoul(optarg, 0 as *mut *mut libc::c_char,
                            10 as libc::c_int) as s64;
                (*afl).fixed_seed = 1 as libc::c_int as u8_0
            }
            112 => {
                /* Power schedule */
                if stricmp(optarg,
                           b"fast\x00" as *const u8 as *const libc::c_char) ==
                       0 {
                    (*afl).schedule = FAST as libc::c_int as u8_0
                } else if stricmp(optarg,
                                  b"coe\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                    (*afl).schedule = COE as libc::c_int as u8_0
                } else if stricmp(optarg,
                                  b"exploit\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                    (*afl).schedule = EXPLOIT as libc::c_int as u8_0
                } else if stricmp(optarg,
                                  b"lin\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                    (*afl).schedule = LIN as libc::c_int as u8_0
                } else if stricmp(optarg,
                                  b"quad\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                    (*afl).schedule = QUAD as libc::c_int as u8_0
                } else if stricmp(optarg,
                                  b"mopt\x00" as *const u8 as
                                      *const libc::c_char) == 0 ||
                              stricmp(optarg,
                                      b"mmopt\x00" as *const u8 as
                                          *const libc::c_char) == 0 {
                    (*afl).schedule = MMOPT as libc::c_int as u8_0
                } else if stricmp(optarg,
                                  b"rare\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                    (*afl).schedule = RARE as libc::c_int as u8_0
                } else if stricmp(optarg,
                                  b"explore\x00" as *const u8 as
                                      *const libc::c_char) == 0 ||
                              stricmp(optarg,
                                      b"default\x00" as *const u8 as
                                          *const libc::c_char) == 0 ||
                              stricmp(optarg,
                                      b"normal\x00" as *const u8 as
                                          *const libc::c_char) == 0 ||
                              stricmp(optarg,
                                      b"afl\x00" as *const u8 as
                                          *const libc::c_char) == 0 {
                    (*afl).schedule = EXPLORE as libc::c_int as u8_0
                } else {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnknown -p power schedule\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 323 as libc::c_int);
                    exit(1 as libc::c_int);
                }
            }
            101 => {
                if !(*afl).file_extension.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -e options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 331 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).file_extension = optarg as *mut u8_0
            }
            105 => {
                /* input dir */
                if !(*afl).in_dir.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -i options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 339 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).in_dir = optarg as *mut u8_0;
                if strcmp((*afl).in_dir as *const libc::c_char,
                          b"-\x00" as *const u8 as *const libc::c_char) == 0 {
                    (*afl).in_place_resume = 1 as libc::c_int as u8_0
                }
            }
            111 => {
                /* output dir */
                if !(*afl).out_dir.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -o options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 348 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).out_dir = optarg as *mut u8_0
            }
            77 => {
                /* master sync ID */
                let mut c: *mut u8_0 = 0 as *mut u8_0;
                if !(*afl).sync_id.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -S or -M options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 356 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).sync_id = DFL_ck_strdup(optarg as *mut u8_0);
                c =
                    strchr((*afl).sync_id as *const libc::c_char, ':' as i32)
                        as *mut u8_0;
                if !c.is_null() {
                    *c = 0 as libc::c_int as u8_0;
                    if sscanf(c.offset(1 as libc::c_int as isize) as
                                  *const libc::c_char,
                              b"%u/%u\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut (*afl).master_id as *mut u32_0,
                              &mut (*afl).master_max as *mut u32_0) !=
                           2 as libc::c_int || (*afl).master_id == 0 ||
                           (*afl).master_max == 0 ||
                           (*afl).master_id > (*afl).master_max ||
                           (*afl).master_max >
                               1000000 as libc::c_int as libc::c_uint {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBogus master ID passed to -M\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-fuzz.c\x00" as *const u8 as
                                   *const libc::c_char, 366 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                }
                (*afl).force_deterministic = 1 as libc::c_int as u8_0
            }
            83 => {
                if !(*afl).sync_id.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -S or -M options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 378 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).sync_id = DFL_ck_strdup(optarg as *mut u8_0)
            }
            102 => {
                /* target file */
                if !(*afl).fsrv.out_file.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -f options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 384 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).fsrv.out_file = DFL_ck_strdup(optarg as *mut u8_0);
                (*afl).fsrv.use_stdin = 0 as libc::c_int as u8_0
            }
            120 => {
                /* dictionary */
                if !extras_dir.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -x options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 391 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                extras_dir = optarg as *mut u8_0
            }
            116 => {
                /* timeout */
                let mut suffix: u8_0 = 0 as libc::c_int as u8_0;
                if (*afl).timeout_given != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -t options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 399 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if sscanf(optarg,
                          b"%u%c\x00" as *const u8 as *const libc::c_char,
                          &mut (*afl).fsrv.exec_tmout as *mut u32_0,
                          &mut suffix as *mut u8_0) < 1 as libc::c_int ||
                       *optarg.offset(0 as libc::c_int as isize) as
                           libc::c_int == '-' as i32 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBad syntax used for -t\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 403 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if (*afl).fsrv.exec_tmout < 5 as libc::c_int as libc::c_uint {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mDangerously low value of -t\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 405 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if suffix as libc::c_int == '+' as i32 {
                    (*afl).timeout_given = 2 as libc::c_int as u8_0
                } else { (*afl).timeout_given = 1 as libc::c_int as u8_0 }
            }
            109 => {
                /* mem limit */
                let mut suffix_0: u8_0 = 'M' as i32 as u8_0;
                if mem_limit_given != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -m options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 420 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                mem_limit_given = 1 as libc::c_int as u8_0;
                if strcmp(optarg,
                          b"none\x00" as *const u8 as *const libc::c_char) ==
                       0 {
                    (*afl).fsrv.mem_limit = 0 as libc::c_int as u64_0
                } else {
                    if sscanf(optarg,
                              b"%llu%c\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut (*afl).fsrv.mem_limit as *mut u64_0,
                              &mut suffix_0 as *mut u8_0) < 1 as libc::c_int
                           ||
                           *optarg.offset(0 as libc::c_int as isize) as
                               libc::c_int == '-' as i32 {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBad syntax used for -m\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-fuzz.c\x00" as *const u8 as
                                   *const libc::c_char, 432 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                    match suffix_0 as libc::c_int {
                        84 => {
                            (*afl).fsrv.mem_limit =
                                ((*afl).fsrv.mem_limit as
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
                            (*afl).fsrv.mem_limit =
                                ((*afl).fsrv.mem_limit as
                                     libc::c_ulonglong).wrapping_mul(1024 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulonglong)
                                    as u64_0 as u64_0
                        }
                        107 => {
                            (*afl).fsrv.mem_limit =
                                ((*afl).fsrv.mem_limit as
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
                                   b"src/afl-fuzz.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   441 as libc::c_int);
                            exit(1 as libc::c_int);
                        }
                    }
                    if (*afl).fsrv.mem_limit <
                           5 as libc::c_int as libc::c_ulonglong {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mDangerously low value of -m\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-fuzz.c\x00" as *const u8 as
                                   *const libc::c_char, 445 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                    if ::std::mem::size_of::<rlim_t>() as libc::c_ulong ==
                           4 as libc::c_int as libc::c_ulong &&
                           (*afl).fsrv.mem_limit >
                               2000 as libc::c_int as libc::c_ulonglong {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mValue of -m out of range on 32-bit systems\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-fuzz.c\x00" as *const u8 as
                                   *const libc::c_char, 448 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                }
            }
            100 => {
                /* skip deterministic */
                if (*afl).skip_deterministic != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -d options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 456 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).skip_deterministic = 1 as libc::c_int as u8_0;
                (*afl).use_splicing = 1 as libc::c_int as u8_0
            }
            66 => {
                /* load bitmap */
                /* This is a secret undocumented option! It is useful if you find
           an interesting test case during a normal fuzzing process, and want
           to mutate it without rediscovering any of the test cases already
           found during an earlier run.

           To use this mode, you need to point -B to the fuzz_bitmap produced
           by an earlier run for the exact same binary... and that's it.

           I only used this once or twice to get variants of a particular
           file, so I'm not making this an official setting. */
                if !(*afl).in_bitmap.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -B options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 474 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).in_bitmap = optarg as *mut u8_0;
                read_bitmap(afl, (*afl).in_bitmap);
            }
            67 => {
                /* crash mode */
                if (*afl).crash_mode != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -C options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 482 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).crash_mode = FAULT_CRASH as libc::c_int as u8_0
            }
            110 => {
                /* dumb mode */
                if (*afl).dumb_mode != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -n options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 488 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if (*afl).afl_env.afl_dumb_forksrv != 0 {
                    (*afl).dumb_mode = 2 as libc::c_int as u8_0
                } else { (*afl).dumb_mode = 1 as libc::c_int as u8_0 }
            }
            84 => {
                /* banner */
                if !(*afl).use_banner.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -T options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 498 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).use_banner = optarg as *mut u8_0
            }
            81 => {
                /* QEMU mode */
                if (*afl).fsrv.qemu_mode != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -Q options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 504 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).fsrv.qemu_mode = 1 as libc::c_int as u8_0;
                if mem_limit_given == 0 {
                    (*afl).fsrv.mem_limit = 200 as libc::c_int as u64_0
                }
            }
            78 => {
                /* Unicorn mode */
                if (*afl).no_unlink != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -N options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 513 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).no_unlink = 1 as libc::c_int as u8_0
            }
            85 => {
                /* Unicorn mode */
                if (*afl).unicorn_mode != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -U options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 520 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).unicorn_mode = 1 as libc::c_int as u8_0;
                if mem_limit_given == 0 {
                    (*afl).fsrv.mem_limit = 200 as libc::c_int as u64_0
                }
            }
            87 => {
                /* Wine+QEMU mode */
                if (*afl).use_wine != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -W options not supported\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 529 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).fsrv.qemu_mode = 1 as libc::c_int as u8_0;
                (*afl).use_wine = 1 as libc::c_int as u8_0;
                if mem_limit_given == 0 {
                    (*afl).fsrv.mem_limit = 0 as libc::c_int as u64_0
                }
            }
            86 => {
                (*afl).most_time_key = 1 as libc::c_int as u64_0;
                if sscanf(optarg,
                          b"%llu\x00" as *const u8 as *const libc::c_char,
                          &mut (*afl).most_time as *mut u64_0) <
                       1 as libc::c_int ||
                       *optarg.offset(0 as libc::c_int as isize) as
                           libc::c_int == '-' as i32 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBad syntax used for -V\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 541 as libc::c_int);
                    exit(1 as libc::c_int);
                }
            }
            69 => {
                (*afl).most_execs_key = 1 as libc::c_int as u64_0;
                if sscanf(optarg,
                          b"%llu\x00" as *const u8 as *const libc::c_char,
                          &mut (*afl).most_execs as *mut u64_0) <
                       1 as libc::c_int ||
                       *optarg.offset(0 as libc::c_int as isize) as
                           libc::c_int == '-' as i32 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBad syntax used for -E\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 549 as libc::c_int);
                    exit(1 as libc::c_int);
                }
            }
            76 => {
                /* MOpt mode */
                if (*afl).limit_time_sig != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMultiple -L options not supported\x00"
                               as *const u8 as
                               *const libc::c_char); // in case it is a different implementation
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 555 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).havoc_max_mult = 32 as libc::c_int as u8_0;
                if sscanf(optarg,
                          b"%d\x00" as *const u8 as *const libc::c_char,
                          &mut (*afl).limit_time_puppet as *mut s32) <
                       1 as libc::c_int {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBad syntax used for -L\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 559 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if (*afl).limit_time_puppet == -(1 as libc::c_int) {
                    (*afl).limit_time_sig = -(1 as libc::c_int);
                    (*afl).limit_time_puppet = 0 as libc::c_int
                } else if (*afl).limit_time_puppet < 0 as libc::c_int {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0m-L value must be between 0 and 2000000 or -1\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 568 as libc::c_int);
                    exit(1 as libc::c_int);
                } else { (*afl).limit_time_sig = 1 as libc::c_int }
                let mut limit_time_puppet2: u64_0 =
                    ((*afl).limit_time_puppet * 60 as libc::c_int *
                         1000 as libc::c_int) as u64_0;
                if limit_time_puppet2 <
                       (*afl).limit_time_puppet as libc::c_ulonglong {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mlimit_time overflow\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 579 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                (*afl).limit_time_puppet = limit_time_puppet2 as s32;
                printf(b"limit_time_puppet %d\n\x00" as *const u8 as
                           *const libc::c_char, (*afl).limit_time_puppet);
                (*afl).swarm_now = 0 as libc::c_int;
                if (*afl).limit_time_puppet == 0 as libc::c_int {
                    (*afl).key_puppet = 1 as libc::c_int
                }
                let mut i: libc::c_int = 0;
                let mut tmp_swarm: libc::c_int = 0 as libc::c_int;
                if (*afl).g_now > (*afl).g_max {
                    (*afl).g_now = 0 as libc::c_int
                }
                (*afl).w_now =
                    ((*afl).w_init - (*afl).w_end) *
                        ((*afl).g_max - (*afl).g_now) as libc::c_double /
                        (*afl).g_max as libc::c_double + (*afl).w_end;
                tmp_swarm = 0 as libc::c_int;
                while tmp_swarm < 5 as libc::c_int {
                    let mut total_puppet_temp: libc::c_double = 0.0f64;
                    (*afl).swarm_fitness[tmp_swarm as usize] = 0.0f64;
                    i = 0 as libc::c_int;
                    while i < 16 as libc::c_int {
                        (*afl).stage_finds_puppet[tmp_swarm as
                                                      usize][i as usize] =
                            0 as libc::c_int as u64_0;
                        (*afl).probability_now[tmp_swarm as usize][i as usize]
                            = 0.0f64;
                        (*afl).x_now[tmp_swarm as usize][i as usize] =
                            (random() % 7000 as libc::c_int as libc::c_long)
                                as libc::c_double * 0.0001f64 + 0.1f64;
                        total_puppet_temp +=
                            (*afl).x_now[tmp_swarm as usize][i as usize];
                        (*afl).v_now[tmp_swarm as usize][i as usize] = 0.1f64;
                        (*afl).L_best[tmp_swarm as usize][i as usize] =
                            0.5f64;
                        (*afl).G_best[i as usize] = 0.5f64;
                        (*afl).eff_best[tmp_swarm as usize][i as usize] =
                            0.0f64;
                        i += 1
                    }
                    i = 0 as libc::c_int;
                    while i < 16 as libc::c_int {
                        (*afl).stage_cycles_puppet_v2[tmp_swarm as
                                                          usize][i as usize] =
                            (*afl).stage_cycles_puppet[tmp_swarm as
                                                           usize][i as usize];
                        (*afl).stage_finds_puppet_v2[tmp_swarm as
                                                         usize][i as usize] =
                            (*afl).stage_finds_puppet[tmp_swarm as
                                                          usize][i as usize];
                        (*afl).x_now[tmp_swarm as usize][i as usize] =
                            (*afl).x_now[tmp_swarm as usize][i as usize] /
                                total_puppet_temp;
                        i += 1
                    }
                    let mut x_temp: libc::c_double = 0.0f64;
                    i = 0 as libc::c_int;
                    while i < 16 as libc::c_int {
                        (*afl).probability_now[tmp_swarm as usize][i as usize]
                            = 0.0f64;
                        (*afl).v_now[tmp_swarm as usize][i as usize] =
                            (*afl).w_now *
                                (*afl).v_now[tmp_swarm as usize][i as usize] +
                                (rand() % 1000 as libc::c_int) as
                                    libc::c_double * 0.001f64 *
                                    ((*afl).L_best[tmp_swarm as
                                                       usize][i as usize] -
                                         (*afl).x_now[tmp_swarm as
                                                          usize][i as usize])
                                +
                                (rand() % 1000 as libc::c_int) as
                                    libc::c_double * 0.001f64 *
                                    ((*afl).G_best[i as usize] -
                                         (*afl).x_now[tmp_swarm as
                                                          usize][i as usize]);
                        (*afl).x_now[tmp_swarm as usize][i as usize] +=
                            (*afl).v_now[tmp_swarm as usize][i as usize];
                        if (*afl).x_now[tmp_swarm as usize][i as usize] >
                               1 as libc::c_int as libc::c_double {
                            (*afl).x_now[tmp_swarm as usize][i as usize] =
                                1 as libc::c_int as libc::c_double
                        } else if (*afl).x_now[tmp_swarm as usize][i as usize]
                                      < 0.05f64 {
                            (*afl).x_now[tmp_swarm as usize][i as usize] =
                                0.05f64
                        }
                        x_temp +=
                            (*afl).x_now[tmp_swarm as usize][i as usize];
                        i += 1
                    }
                    i = 0 as libc::c_int;
                    while i < 16 as libc::c_int {
                        (*afl).x_now[tmp_swarm as usize][i as usize] =
                            (*afl).x_now[tmp_swarm as usize][i as usize] /
                                x_temp;
                        if i != 0 as libc::c_int {
                            (*afl).probability_now[tmp_swarm as
                                                       usize][i as usize] =
                                (*afl).probability_now[tmp_swarm as
                                                           usize][(i -
                                                                       1 as
                                                                           libc::c_int)
                                                                      as
                                                                      usize] +
                                    (*afl).x_now[tmp_swarm as
                                                     usize][i as usize]
                        } else {
                            (*afl).probability_now[tmp_swarm as
                                                       usize][i as usize] =
                                (*afl).x_now[tmp_swarm as usize][i as usize]
                        }
                        i += 1
                    }
                    if (*afl).probability_now[tmp_swarm as
                                                  usize][(16 as libc::c_int -
                                                              1 as
                                                                  libc::c_int)
                                                             as usize] <
                           0.99f64 ||
                           (*afl).probability_now[tmp_swarm as
                                                      usize][(16 as
                                                                  libc::c_int
                                                                  -
                                                                  1 as
                                                                      libc::c_int)
                                                                 as usize] >
                               1.01f64 {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mERROR probability\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-fuzz.c\x00" as *const u8 as
                                   *const libc::c_char, 661 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                    tmp_swarm += 1
                }
                i = 0 as libc::c_int;
                while i < 16 as libc::c_int {
                    (*afl).core_operator_finds_puppet[i as usize] =
                        0 as libc::c_int as u64_0;
                    (*afl).core_operator_finds_puppet_v2[i as usize] =
                        0 as libc::c_int as u64_0;
                    (*afl).core_operator_cycles_puppet[i as usize] =
                        0 as libc::c_int as u64_0;
                    (*afl).core_operator_cycles_puppet_v2[i as usize] =
                        0 as libc::c_int as u64_0;
                    (*afl).core_operator_cycles_puppet_v3[i as usize] =
                        0 as libc::c_int as u64_0;
                    i += 1
                }
            }
            104 => { show_help = show_help.wrapping_add(1) }
            82 => {
                if (*afl).use_radamsa != 0 {
                    (*afl).use_radamsa = 2 as libc::c_int as u8_0
                } else { (*afl).use_radamsa = 1 as libc::c_int as u8_0 }
            }
            _ => {
                if show_help == 0 { show_help = 1 as libc::c_int as u32_0 }
            }
        }
    }
    if optind == argc || (*afl).in_dir.is_null() || (*afl).out_dir.is_null()
           || show_help != 0 {
        usage(afl, *argv.offset(0 as libc::c_int as isize) as *mut u8_0,
              show_help as libc::c_int);
    }
    printf(b"\x1b[1;92m[+] \x1b[0mafl++ is maintained by Marc \"van Hauser\" Heuse, Heiko \"hexcoder\" Ei\xc3\x9ffeldt, Andrea Fioraldi and Dominik Maier\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    printf(b"\x1b[1;92m[+] \x1b[0mafl++ is open source, get it at https://github.com/AFLplusplus/AFLplusplus\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    printf(b"\x1b[1;92m[+] \x1b[0mPower schedules from github.com/mboehme/aflfast\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    printf(b"\x1b[1;92m[+] \x1b[0mPython Mutator and llvm_mode whitelisting from github.com/choller/afl\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    printf(b"\x1b[1;92m[+] \x1b[0mafl-tmin fork server patch from github.com/nccgroup/TriforceAFL\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    printf(b"\x1b[1;92m[+] \x1b[0mMOpt Mutator from github.com/puppet-meteor/MOpt-AFL\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    if !(*afl).sync_id.is_null() &&
           (*afl).force_deterministic as libc::c_int != 0 &&
           (*afl).afl_env.afl_custom_mutator_only as libc::c_int != 0 {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mUsing -M master with the AFL_CUSTOM_MUTATOR_ONLY mutator options will result in no deterministic mutations being done!\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    if (*afl).fixed_seed != 0 {
        printf(b"\x1b[1;92m[+] \x1b[0mRunning with fixed seed: %u\x00" as
                   *const u8 as *const libc::c_char,
               (*afl).init_seed as u32_0);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    srandom((*afl).init_seed as u32_0);
    srand((*afl).init_seed as u32_0);
    if (*afl).use_radamsa != 0 {
        if (*afl).limit_time_sig > 0 as libc::c_int {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMOpt and Radamsa are mutually exclusive unless you specify -L -1. We accept pull requests that integrates MOpt with the optional mutators (custom/radamsa/redqueen/...).\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
                   721 as libc::c_int);
            exit(1 as libc::c_int);
        }
        if (*afl).limit_time_sig != 0 &&
               (*afl).use_radamsa as libc::c_int > 1 as libc::c_int {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mRadamsa in radamsa-only mode can not run together with -L\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
                   724 as libc::c_int);
            exit(1 as libc::c_int);
        }
        printf(b"\x1b[1;92m[+] \x1b[0mUsing Radamsa add-on\x00" as *const u8
                   as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        let mut libradamsa_path: *mut u8_0 =
            get_libradamsa_path(*argv.offset(0 as libc::c_int as isize) as
                                    *mut u8_0);
        let mut handle: *mut libc::c_void =
            dlopen(libradamsa_path as *const libc::c_char,
                   0x2 as libc::c_int);
        DFL_ck_free(libradamsa_path as *mut libc::c_void);
        if handle.is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mFailed to dlopen() libradamsa\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
                   732 as libc::c_int);
            exit(1 as libc::c_int);
        }
        let mut radamsa_init_ptr: Option<unsafe extern "C" fn() -> ()> =
            ::std::mem::transmute::<*mut libc::c_void,
                                    Option<unsafe extern "C" fn()
                                               ->
                                                   ()>>(dlsym(handle,
                                                              b"radamsa_init\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char));
        (*afl).radamsa_mutate_ptr =
            ::std::mem::transmute::<*mut libc::c_void,
                                    Option<unsafe extern "C" fn(_: *mut u8_0,
                                                                _: size_t,
                                                                _: *mut u8_0,
                                                                _: size_t,
                                                                _: u32_0)
                                               ->
                                                   size_t>>(dlsym(handle,
                                                                  b"radamsa\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char));
        if radamsa_init_ptr.is_none() || (*afl).radamsa_mutate_ptr.is_none() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mFailed to dlsym() libradamsa\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
                   738 as libc::c_int);
            exit(1 as libc::c_int);
        }
        /* randamsa_init installs some signal hadlers, call it before
       setup_signal_handlers so that AFL++ can then replace those signal
       handlers */
        radamsa_init_ptr.expect("non-null function pointer")();
    }
    setup_signal_handlers();
    check_asan_opts();
    (*afl).power_name = power_names[(*afl).schedule as usize] as *mut u8_0;
    if !(*afl).sync_id.is_null() { fix_up_sync(afl); }
    if strcmp((*afl).in_dir as *const libc::c_char,
              (*afl).out_dir as *const libc::c_char) == 0 {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mInput and output directories can\'t be the same\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
               765 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if (*afl).dumb_mode != 0 {
        if (*afl).crash_mode != 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0m-C and -n are mutually exclusive\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
                   769 as libc::c_int);
            exit(1 as libc::c_int);
        }
        if (*afl).fsrv.qemu_mode != 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0m-Q and -n are mutually exclusive\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
                   770 as libc::c_int);
            exit(1 as libc::c_int);
        }
        if (*afl).unicorn_mode != 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0m-U and -n are mutually exclusive\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
                   771 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    if !get_afl_env(b"AFL_DISABLE_TRIM\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char).is_null() {
        (*afl).disable_trim = 1 as libc::c_int as u8_0
    }
    if !getenv(b"AFL_NO_UI\x00" as *const u8 as *const libc::c_char).is_null()
           &&
           !getenv(b"AFL_FORCE_UI\x00" as *const u8 as
                       *const libc::c_char).is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mAFL_NO_UI and AFL_FORCE_UI are mutually exclusive\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
               778 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if strchr(*argv.offset(optind as isize), '/' as i32).is_null() &&
           (*afl).unicorn_mode == 0 {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0m\x1b[1;91mTarget binary called without a prefixed path, make sure you are fuzzing the right binary: \x1b[0m%s\x00"
                   as *const u8 as *const libc::c_char,
               *argv.offset(optind as isize));
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    printf(b"\x1b[1;94m[*] \x1b[0mGetting to work...\x00" as *const u8 as
               *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    match (*afl).schedule as libc::c_int {
        1 => {
            printf(b"\x1b[1;92m[+] \x1b[0mUsing exponential power schedule (FAST)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
        2 => {
            printf(b"\x1b[1;92m[+] \x1b[0mUsing cut-off exponential power schedule (COE)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
        5 => {
            printf(b"\x1b[1;92m[+] \x1b[0mUsing exploitation-based constant power schedule (EXPLOIT)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
        3 => {
            printf(b"\x1b[1;92m[+] \x1b[0mUsing linear power schedule (LIN)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
        4 => {
            printf(b"\x1b[1;92m[+] \x1b[0mUsing quadratic power schedule (QUAD)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
        6 => {
            printf(b"\x1b[1;92m[+] \x1b[0mUsing modified MOpt power schedule (MMOPT)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
        7 => {
            printf(b"\x1b[1;92m[+] \x1b[0mUsing rare edge focus power schedule (RARE)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
        0 => {
            printf(b"\x1b[1;92m[+] \x1b[0mUsing exploration-based constant power schedule (EXPLORE, default)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
        _ => {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnknown power schedule\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
                   802 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    if !get_afl_env(b"AFL_NO_FORKSRV\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char).is_null() {
        (*afl).no_forkserver = 1 as libc::c_int as u8_0
    }
    if !get_afl_env(b"AFL_NO_CPU_RED\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char).is_null() {
        (*afl).no_cpu_meter_red = 1 as libc::c_int as u8_0
    }
    if !get_afl_env(b"AFL_NO_ARITH\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char).is_null() {
        (*afl).no_arith = 1 as libc::c_int as u8_0
    }
    if !get_afl_env(b"AFL_SHUFFLE_QUEUE\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char).is_null() {
        (*afl).shuffle_queue = 1 as libc::c_int as u8_0
    }
    if !get_afl_env(b"AFL_FAST_CAL\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char).is_null() {
        (*afl).fast_cal = 1 as libc::c_int as u8_0
    }
    if (*afl).afl_env.afl_autoresume != 0 {
        (*afl).autoresume = 1 as libc::c_int as u8_0;
        if (*afl).in_place_resume != 0 {
            printf(b"AFL_AUTORESUME has no effect for \'-i -\'\x00" as
                       *const u8 as *const libc::c_char);
        }
    }
    if !(*afl).afl_env.afl_hang_tmout.is_null() {
        (*afl).hang_tmout =
            atoi((*afl).afl_env.afl_hang_tmout as *const libc::c_char) as
                u32_0;
        if (*afl).hang_tmout == 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mInvalid value of AFL_HANG_TMOUT\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
                   822 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    if (*afl).dumb_mode as libc::c_int == 2 as libc::c_int &&
           (*afl).no_forkserver as libc::c_int != 0 {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mAFL_DUMB_FORKSRV and AFL_NO_FORKSRV are mutually exclusive\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
               827 as libc::c_int);
        exit(1 as libc::c_int);
    }
    (*afl).fsrv.use_fauxsrv =
        ((*afl).dumb_mode as libc::c_int == 1 as libc::c_int ||
             (*afl).no_forkserver as libc::c_int != 0) as libc::c_int as u8_0;
    if !getenv(b"LD_PRELOAD\x00" as *const u8 as
                   *const libc::c_char).is_null() {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mLD_PRELOAD is set, are you sure that is what to you want to do instead of using AFL_PRELOAD?\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    if !(*afl).afl_env.afl_preload.is_null() {
        if (*afl).fsrv.qemu_mode != 0 {
            let mut qemu_preload: *mut u8_0 =
                getenv(b"QEMU_SET_ENV\x00" as *const u8 as
                           *const libc::c_char) as *mut u8_0;
            let mut afl_preload: *mut u8_0 =
                getenv(b"AFL_PRELOAD\x00" as *const u8 as *const libc::c_char)
                    as *mut u8_0;
            let mut buf: *mut u8_0 = 0 as *mut u8_0;
            let mut i_0: s32 = 0;
            let mut afl_preload_size: s32 =
                strlen(afl_preload as *const libc::c_char) as s32;
            i_0 = 0 as libc::c_int;
            while i_0 < afl_preload_size {
                if *afl_preload.offset(i_0 as isize) as libc::c_int ==
                       ',' as i32 {
                    fflush(stdout);
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mComma (\',\') is not allowed in AFL_PRELOAD when -Q is specified!\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz.c\x00" as *const u8 as
                               *const libc::c_char, 850 as libc::c_int);
                    printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                               *const u8 as *const libc::c_char,
                           strerror(*__errno_location()));
                    exit(1 as libc::c_int);
                }
                i_0 += 1
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
                                    b"src/afl-fuzz.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    856 as libc::c_int);
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
                                    b"src/afl-fuzz.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    859 as libc::c_int);
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
    }
    if !getenv(b"AFL_LD_PRELOAD\x00" as *const u8 as
                   *const libc::c_char).is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUse AFL_PRELOAD instead of AFL_LD_PRELOAD\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
               875 as libc::c_int);
        exit(1 as libc::c_int);
    }
    save_cmdline(afl, argc as u32_0, argv);
    fix_up_banner(afl, *argv.offset(optind as isize) as *mut u8_0);
    check_if_tty(afl);
    if (*afl).afl_env.afl_force_ui != 0 {
        (*afl).not_on_tty = 0 as libc::c_int as u8_0
    }
    if (*afl).afl_env.afl_cal_fast != 0 {
        /* Use less calibration cycles, for slow applications */
        (*afl).cal_cycles = 3 as libc::c_int as u8_0;
        (*afl).cal_cycles_long = 5 as libc::c_int as u8_0
    }
    if (*afl).afl_env.afl_custom_mutator_only != 0 {
        /* This ensures we don't proceed to havoc/splice */
        (*afl).custom_only = 1 as libc::c_int as u8_0;
        /* Ensure we also skip all deterministic steps */
        (*afl).skip_deterministic = 1 as libc::c_int as u8_0
    }
    get_core_count(afl);
    bind_to_free_cpu(afl);
    /* HAVE_AFFINITY */
    check_crash_handling();
    check_cpu_governor(afl);
    (*afl).fsrv.trace_bits =
        afl_shm_init(&mut (*afl).shm,
                     ((1 as libc::c_int) << 16 as libc::c_int) as size_t,
                     (*afl).dumb_mode);
    setup_post(afl);
    if (*afl).in_bitmap.is_null() {
        memset((*afl).virgin_bits.as_mut_ptr() as *mut libc::c_void,
               255 as libc::c_int,
               ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong);
    }
    memset((*afl).virgin_tmout.as_mut_ptr() as *mut libc::c_void,
           255 as libc::c_int,
           ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong);
    memset((*afl).virgin_crash.as_mut_ptr() as *mut libc::c_void,
           255 as libc::c_int,
           ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong);
    init_count_class16();
    setup_dirs_fds(afl);
    setup_custom_mutator(afl);
    setup_cmdline_file(afl, argv.offset(optind as isize));
    read_testcases(afl);
    load_auto(afl);
    pivot_inputs(afl);
    if !extras_dir.is_null() { load_extras(afl, extras_dir); }
    if (*afl).timeout_given == 0 { find_timeout(afl); }
    (*afl).tmp_dir = (*afl).afl_env.afl_tmpdir;
    if !(*afl).tmp_dir.is_null() && (*afl).in_place_resume == 0 {
        let mut tmpfile: [libc::c_char; 4096] = [0; 4096];
        if !(*afl).file_extension.is_null() {
            snprintf(tmpfile.as_mut_ptr(),
                     4096 as libc::c_int as libc::c_ulong,
                     b"%s/.cur_input.%s\x00" as *const u8 as
                         *const libc::c_char, (*afl).tmp_dir,
                     (*afl).file_extension);
        } else {
            snprintf(tmpfile.as_mut_ptr(),
                     4096 as libc::c_int as libc::c_ulong,
                     b"%s/.cur_input\x00" as *const u8 as *const libc::c_char,
                     (*afl).tmp_dir);
        }
        /* there is still a race condition here, but well ... */
        if access(tmpfile.as_mut_ptr(), 0 as libc::c_int) !=
               -(1 as libc::c_int) {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mAFL_TMPDIR already has an existing temporary input file: %s - if this is not from another instance, then just remove the file.\x00"
                       as *const u8 as *const libc::c_char,
                   tmpfile.as_mut_ptr());
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
                   957 as libc::c_int);
            exit(1 as libc::c_int);
        }
    } else { (*afl).tmp_dir = (*afl).out_dir }
    /* If we don't have a file name chosen yet, use a safe default. */
    if (*afl).fsrv.out_file.is_null() {
        let mut i_1: u32_0 = (optind + 1 as libc::c_int) as u32_0;
        while !(*argv.offset(i_1 as isize)).is_null() {
            let mut aa_loc: *mut u8_0 =
                strstr(*argv.offset(i_1 as isize),
                       b"@@\x00" as *const u8 as *const libc::c_char) as
                    *mut u8_0;
            if !aa_loc.is_null() && (*afl).fsrv.out_file.is_null() {
                (*afl).fsrv.use_stdin = 0 as libc::c_int as u8_0;
                if !(*afl).file_extension.is_null() {
                    (*afl).fsrv.out_file =
                        ({
                             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                             let mut _len: s32 =
                                 snprintf(0 as *mut libc::c_char,
                                          0 as libc::c_int as libc::c_ulong,
                                          b"%s/.cur_input.%s\x00" as *const u8
                                              as *const libc::c_char,
                                          (*afl).tmp_dir,
                                          (*afl).file_extension);
                             if _len < 0 as libc::c_int {
                                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        b"func_unknown\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"src/afl-fuzz.c\x00" as *const u8 as
                                            *const libc::c_char,
                                        979 as libc::c_int);
                                 exit(1 as libc::c_int);
                             }
                             _tmp =
                                 DFL_ck_alloc((_len + 1 as libc::c_int) as
                                                  u32_0) as *mut u8_0;
                             snprintf(_tmp as *mut libc::c_char,
                                      (_len + 1 as libc::c_int) as
                                          libc::c_ulong,
                                      b"%s/.cur_input.%s\x00" as *const u8 as
                                          *const libc::c_char, (*afl).tmp_dir,
                                      (*afl).file_extension);
                             _tmp
                         })
                } else {
                    (*afl).fsrv.out_file =
                        ({
                             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                             let mut _len: s32 =
                                 snprintf(0 as *mut libc::c_char,
                                          0 as libc::c_int as libc::c_ulong,
                                          b"%s/.cur_input\x00" as *const u8 as
                                              *const libc::c_char,
                                          (*afl).tmp_dir);
                             if _len < 0 as libc::c_int {
                                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        b"func_unknown\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"src/afl-fuzz.c\x00" as *const u8 as
                                            *const libc::c_char,
                                        983 as libc::c_int);
                                 exit(1 as libc::c_int);
                             }
                             _tmp =
                                 DFL_ck_alloc((_len + 1 as libc::c_int) as
                                                  u32_0) as *mut u8_0;
                             snprintf(_tmp as *mut libc::c_char,
                                      (_len + 1 as libc::c_int) as
                                          libc::c_ulong,
                                      b"%s/.cur_input\x00" as *const u8 as
                                          *const libc::c_char,
                                      (*afl).tmp_dir);
                             _tmp
                         })
                }
                detect_file_args(argv.offset(optind as
                                                 isize).offset(1 as
                                                                   libc::c_int
                                                                   as isize),
                                 (*afl).fsrv.out_file,
                                 &mut (*afl).fsrv.use_stdin);
                break ;
            } else { i_1 = i_1.wrapping_add(1) }
        }
    }
    if (*afl).fsrv.out_file.is_null() { setup_stdio_file(afl); }
    if !(*afl).cmplog_binary.is_null() {
        if (*afl).limit_time_sig > 0 as libc::c_int {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMOpt and CmpLog are mutually exclusive unless you specify -L -1. We accept pull requests that integrates MOpt with the optional mutators (custom/radamsa/redqueen/...).\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
                   1007 as libc::c_int);
            exit(1 as libc::c_int);
        }
        if (*afl).unicorn_mode != 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mCmpLog and Unicorn mode are not compatible at the moment, sorry\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz.c\x00" as *const u8 as *const libc::c_char,
                   1010 as libc::c_int);
            exit(1 as libc::c_int);
        }
        if (*afl).fsrv.qemu_mode == 0 {
            check_binary(afl, (*afl).cmplog_binary as *mut u8_0);
        }
    }
    check_binary(afl, *argv.offset(optind as isize) as *mut u8_0);
    (*afl).start_time = get_cur_time();
    if (*afl).fsrv.qemu_mode != 0 {
        if (*afl).use_wine != 0 {
            use_argv =
                get_wine_argv(*argv.offset(0 as libc::c_int as isize) as
                                  *mut u8_0, &mut (*afl).fsrv.target_path,
                              argc - optind, argv.offset(optind as isize))
        } else {
            use_argv =
                get_qemu_argv(*argv.offset(0 as libc::c_int as isize) as
                                  *mut u8_0, &mut (*afl).fsrv.target_path,
                              argc - optind, argv.offset(optind as isize))
        }
    } else { use_argv = argv.offset(optind as isize) }
    (*afl).argv = use_argv;
    if !(*afl).cmplog_binary.is_null() {
        printf(b"\x1b[1;94m[*] \x1b[0mSpawning cmplog forkserver\x00" as
                   *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        afl_fsrv_init_dup(&mut (*afl).cmplog_fsrv, &mut (*afl).fsrv);
        // TODO: this is semi-nice
        (*afl).cmplog_fsrv.trace_bits = (*afl).fsrv.trace_bits;
        (*afl).cmplog_fsrv.qemu_mode = (*afl).fsrv.qemu_mode;
        (*afl).cmplog_fsrv.cmplog_binary = (*afl).cmplog_binary;
        (*afl).cmplog_fsrv.init_child_func =
            Some(cmplog_exec_child as
                     unsafe extern "C" fn(_: *mut afl_forkserver_t,
                                          _: *mut *mut libc::c_char) -> ());
        afl_fsrv_start(&mut (*afl).cmplog_fsrv, (*afl).argv,
                       &mut (*afl).stop_soon,
                       (*afl).afl_env.afl_debug_child_output);
    }
    perform_dry_run(afl);
    cull_queue(afl);
    show_init_stats(afl);
    seek_to = find_start_position(afl);
    write_stats_file(afl, 0 as libc::c_int as libc::c_double,
                     0 as libc::c_int as libc::c_double,
                     0 as libc::c_int as libc::c_double);
    maybe_update_plot_file(afl, 0 as libc::c_int as libc::c_double,
                           0 as libc::c_int as libc::c_double);
    save_auto(afl);
    if !((*afl).stop_soon != 0) {
        /* Woop woop woop */
        if (*afl).not_on_tty == 0 {
            sleep(4 as libc::c_int as libc::c_uint);
            (*afl).start_time =
                ((*afl).start_time as
                     libc::c_ulonglong).wrapping_add(4000 as libc::c_int as
                                                         libc::c_ulonglong) as
                    u64_0 as u64_0;
            if (*afl).stop_soon != 0 {
                current_block = 10583885991603710378;
            } else { current_block = 13779552533911551544; }
        } else { current_block = 13779552533911551544; }
        match current_block {
            10583885991603710378 => { }
            _ => {
                // real start time, we reset, so this works correctly with -V
                (*afl).start_time = get_cur_time();
                loop  {
                    let mut skipped_fuzz: u8_0 = 0;
                    cull_queue(afl);
                    if (*afl).queue_cur.is_null() {
                        (*afl).queue_cycle =
                            (*afl).queue_cycle.wrapping_add(1);
                        (*afl).current_entry = 0 as libc::c_int as u32_0;
                        (*afl).cur_skipped_paths = 0 as libc::c_int as u32_0;
                        (*afl).queue_cur = (*afl).queue;
                        while seek_to != 0 {
                            (*afl).current_entry =
                                (*afl).current_entry.wrapping_add(1);
                            seek_to = seek_to.wrapping_sub(1);
                            (*afl).queue_cur = (*(*afl).queue_cur).next
                        }
                        // show_stats(afl);
                        if (*afl).not_on_tty != 0 {
                            printf(b"\x1b[1;94m[*] \x1b[0mEntering queue cycle %llu.\x00"
                                       as *const u8 as *const libc::c_char,
                                   (*afl).queue_cycle);
                            printf(b"\x1b[0m\n\x00" as *const u8 as
                                       *const libc::c_char);
                            fflush(stdout);
                        }
                        /* If we had a full queue cycle with no new finds, try
         recombination strategies next. */
                        if (*afl).queued_paths as libc::c_ulonglong ==
                               prev_queued {
                            if (*afl).use_splicing != 0 {
                                (*afl).cycles_wo_finds =
                                    (*afl).cycles_wo_finds.wrapping_add(1)
                            } else {
                                (*afl).use_splicing = 1 as libc::c_int as u8_0
                            }
                        } else {
                            (*afl).cycles_wo_finds = 0 as libc::c_int as u64_0
                        } // ensure the screen is reprinted
                        prev_queued =
                            (*afl).queued_paths as
                                u64_0; // print the screen one last time
                        if !(*afl).sync_id.is_null() &&
                               (*afl).queue_cycle ==
                                   1 as libc::c_int as libc::c_ulonglong &&
                               (*afl).afl_env.afl_import_first as libc::c_int
                                   != 0 {
                            sync_fuzzers(afl);
                        }
                    }
                    skipped_fuzz = fuzz_one(afl);
                    if skipped_fuzz == 0 && (*afl).stop_soon == 0 &&
                           !(*afl).sync_id.is_null() {
                        let fresh0 = sync_interval_cnt;
                        sync_interval_cnt = sync_interval_cnt.wrapping_add(1);
                        if fresh0.wrapping_rem(5 as libc::c_int as
                                                   libc::c_uint) == 0 {
                            sync_fuzzers(afl);
                        }
                    }
                    if (*afl).stop_soon == 0 && exit_1 as libc::c_int != 0 {
                        ::std::ptr::write_volatile(&mut (*afl).stop_soon as
                                                       *mut u8_0,
                                                   2 as libc::c_int as u8_0)
                    }
                    if (*afl).stop_soon != 0 { break ; }
                    (*afl).queue_cur = (*(*afl).queue_cur).next;
                    (*afl).current_entry =
                        (*afl).current_entry.wrapping_add(1)
                }
                write_bitmap(afl);
                maybe_update_plot_file(afl,
                                       0 as libc::c_int as libc::c_double,
                                       0 as libc::c_int as libc::c_double);
                save_auto(afl);
            }
        }
    }
    write_stats_file(afl, 0 as libc::c_int as libc::c_double,
                     0 as libc::c_int as libc::c_double,
                     0 as libc::c_int as libc::c_double);
    (*afl).force_ui_update = 1 as libc::c_int as u64_0;
    show_stats(afl);
    printf(b"\x1b[?25h\x1b[1;91m\n\n+++ Testing aborted %s +++\n\x1b[0m\x00"
               as *const u8 as *const libc::c_char,
           if (*afl).stop_soon as libc::c_int == 2 as libc::c_int {
               b"programmatically\x00" as *const u8 as *const libc::c_char
           } else { b"by user\x00" as *const u8 as *const libc::c_char });
    if (*afl).most_time_key == 2 as libc::c_int as libc::c_ulonglong {
        printf(b"\x1b[1;93m[!] \x1b[0mTime limit was reached\n\x00" as
                   *const u8 as *const libc::c_char);
    }
    if (*afl).most_execs_key == 2 as libc::c_int as libc::c_ulonglong {
        printf(b"\x1b[1;93m[!] \x1b[0mExecution limit was reached\n\x00" as
                   *const u8 as *const libc::c_char);
    }
    /* Running for more than 30 minutes but still doing first cycle? */
    if (*afl).queue_cycle == 1 as libc::c_int as libc::c_ulonglong &&
           get_cur_time().wrapping_sub((*afl).start_time) >
               (30 as libc::c_int * 60 as libc::c_int * 1000 as libc::c_int)
                   as libc::c_ulonglong {
        printf(b"\n\x1b[1;93m[!] \x1b[0mStopped during the first cycle, results may be incomplete.\n    (For info on resuming, see %s/README.md)\n\x00"
                   as *const u8 as *const libc::c_char,
               doc_path); /* not tracked */
    }
    fclose((*afl).fsrv.plot_file);
    destroy_queue(afl);
    destroy_extras(afl);
    destroy_custom_mutator(afl);
    afl_shm_deinit(&mut (*afl).shm);
    afl_fsrv_deinit(&mut (*afl).fsrv);
    if !(*afl).orig_cmdline.is_null() {
        DFL_ck_free((*afl).orig_cmdline as *mut libc::c_void);
    }
    DFL_ck_free((*afl).fsrv.target_path as *mut libc::c_void);
    DFL_ck_free((*afl).fsrv.out_file as *mut libc::c_void);
    DFL_ck_free((*afl).sync_id as *mut libc::c_void);
    afl_state_deinit(afl);
    free(afl as *mut libc::c_void);
    argv_cpy_free(argv);
    printf(b"\x1b[1;92m[+] \x1b[0mWe\'re done here. Have a nice day!\n\x00" as
               *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
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
/* !AFL_LIB */
