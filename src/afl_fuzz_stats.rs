use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type cmp_map;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn get_runnable_processes() -> libc::c_double;
    #[no_mangle]
    fn save_auto(_: *mut afl_state_t);
    #[no_mangle]
    fn count_non_255_bytes(_: *mut afl_state_t, _: *mut u8_0) -> u32_0;
    #[no_mangle]
    fn count_bits(_: *mut afl_state_t, _: *mut u8_0) -> u32_0;
    #[no_mangle]
    fn write_bitmap(_: *mut afl_state_t);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    static mut doc_path: *mut u8_0;
    /* path to documentation dir        */
    /* Get unix time in milliseconds */
    #[no_mangle]
    fn get_cur_time() -> u64_0;
    /* Describe integer. The buf should be
   at least 6 bytes to fit all ints we randomly see.
   Will return buf for convenience. */
    #[no_mangle]
    fn stringify_int(buf: *mut u8_0, len: size_t, val: u64_0) -> *mut u8_0;
    /* Describe integer as memory size. */
    #[no_mangle]
    fn stringify_mem_size(buf: *mut u8_0, len: size_t, val: u64_0)
     -> *mut u8_0;
    /* Unsafe Describe integer. The buf sizes are not checked.
   This is unsafe but fast.
   Will return buf for convenience. */
    #[no_mangle]
    fn u_stringify_int(buf: *mut u8_0, val: u64_0) -> *mut u8_0;
    /* Unsafe describe float. Similar as unsafe int. */
    #[no_mangle]
    fn u_stringify_float(buf: *mut u8_0, val: libc::c_double) -> *mut u8_0;
    /* Unsafe describe time delta as string.
   Returns a pointer to buf for convenience. */
    #[no_mangle]
    fn u_stringify_time_diff(buf: *mut u8_0, cur_ms: u64_0, event_ms: u64_0)
     -> *mut u8_0;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> libc::c_int;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub c2rust_unnamed_0: C2RustUnnamed_11,
    pub c2rust_unnamed_1: C2RustUnnamed_10,
    pub c2rust_unnamed_2: C2RustUnnamed_9,
    pub c2rust_unnamed_3: C2RustUnnamed_8,
    pub c2rust_unnamed_4: C2RustUnnamed_7,
    pub c2rust_unnamed_5: C2RustUnnamed_6,
    pub c2rust_unnamed_6: C2RustUnnamed_5,
    pub c2rust_unnamed_7: C2RustUnnamed_4,
    pub c2rust_unnamed_8: C2RustUnnamed_3,
    pub c2rust_unnamed_9: C2RustUnnamed_2,
    pub c2rust_unnamed_10: C2RustUnnamed_1,
    pub c2rust_unnamed_11: C2RustUnnamed_0,
    pub c2rust_unnamed_12: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ru_nivcsw: libc::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ru_nvcsw: libc::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ru_nsignals: libc::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ru_msgrcv: libc::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ru_msgsnd: libc::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ru_oublock: libc::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ru_inblock: libc::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ru_nswap: libc::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub ru_majflt: libc::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ru_minflt: libc::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub ru_isrss: libc::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub ru_idrss: libc::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ru_ixrss: libc::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub ru_maxrss: libc::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
pub type __rusage_who = libc::c_int;
pub const RUSAGE_THREAD: __rusage_who = 1;
pub const RUSAGE_CHILDREN: __rusage_who = -1;
pub const RUSAGE_SELF: __rusage_who = 0;
pub type __rusage_who_t = __rusage_who;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
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
pub type C2RustUnnamed_13 = libc::c_uint;
pub const STAGE_ITS: C2RustUnnamed_13 = 21;
pub const STAGE_COLORIZATION: C2RustUnnamed_13 = 20;
pub const STAGE_CUSTOM_MUTATOR: C2RustUnnamed_13 = 19;
pub const STAGE_RADAMSA: C2RustUnnamed_13 = 18;
pub const STAGE_PYTHON: C2RustUnnamed_13 = 17;
pub const STAGE_SPLICE: C2RustUnnamed_13 = 16;
pub const STAGE_HAVOC: C2RustUnnamed_13 = 15;
pub const STAGE_EXTRAS_AO: C2RustUnnamed_13 = 14;
pub const STAGE_EXTRAS_UI: C2RustUnnamed_13 = 13;
pub const STAGE_EXTRAS_UO: C2RustUnnamed_13 = 12;
pub const STAGE_INTEREST32: C2RustUnnamed_13 = 11;
pub const STAGE_INTEREST16: C2RustUnnamed_13 = 10;
pub const STAGE_INTEREST8: C2RustUnnamed_13 = 9;
pub const STAGE_ARITH32: C2RustUnnamed_13 = 8;
pub const STAGE_ARITH16: C2RustUnnamed_13 = 7;
pub const STAGE_ARITH8: C2RustUnnamed_13 = 6;
pub const STAGE_FLIP32: C2RustUnnamed_13 = 5;
pub const STAGE_FLIP16: C2RustUnnamed_13 = 4;
pub const STAGE_FLIP8: C2RustUnnamed_13 = 3;
pub const STAGE_FLIP4: C2RustUnnamed_13 = 2;
pub const STAGE_FLIP2: C2RustUnnamed_13 = 1;
pub const STAGE_FLIP1: C2RustUnnamed_13 = 0;
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
   american fuzzy lop++ - stats related routines
   ---------------------------------------------

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
/* Update stats file for unattended monitoring. */
#[no_mangle]
pub unsafe extern "C" fn write_stats_file(mut afl: *mut afl_state_t,
                                          mut bitmap_cvg: libc::c_double,
                                          mut stability: libc::c_double,
                                          mut eps: libc::c_double) {
    let mut rus: rusage =
        rusage{ru_utime: timeval{tv_sec: 0, tv_usec: 0,},
               ru_stime: timeval{tv_sec: 0, tv_usec: 0,},
               c2rust_unnamed: C2RustUnnamed_12{ru_maxrss: 0,},
               c2rust_unnamed_0: C2RustUnnamed_11{ru_ixrss: 0,},
               c2rust_unnamed_1: C2RustUnnamed_10{ru_idrss: 0,},
               c2rust_unnamed_2: C2RustUnnamed_9{ru_isrss: 0,},
               c2rust_unnamed_3: C2RustUnnamed_8{ru_minflt: 0,},
               c2rust_unnamed_4: C2RustUnnamed_7{ru_majflt: 0,},
               c2rust_unnamed_5: C2RustUnnamed_6{ru_nswap: 0,},
               c2rust_unnamed_6: C2RustUnnamed_5{ru_inblock: 0,},
               c2rust_unnamed_7: C2RustUnnamed_4{ru_oublock: 0,},
               c2rust_unnamed_8: C2RustUnnamed_3{ru_msgsnd: 0,},
               c2rust_unnamed_9: C2RustUnnamed_2{ru_msgrcv: 0,},
               c2rust_unnamed_10: C2RustUnnamed_1{ru_nsignals: 0,},
               c2rust_unnamed_11: C2RustUnnamed_0{ru_nvcsw: 0,},
               c2rust_unnamed_12: C2RustUnnamed{ru_nivcsw: 0,},};
    let mut cur_time: libc::c_ulonglong = get_cur_time();
    let mut fn_0: [u8_0; 4096] = [0; 4096];
    let mut fd: s32 = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut t_bytes: uint32_t =
        count_non_255_bytes(afl, (*afl).virgin_bits.as_mut_ptr());
    snprintf(fn_0.as_mut_ptr() as *mut libc::c_char,
             4096 as libc::c_int as libc::c_ulong,
             b"%s/fuzzer_stats\x00" as *const u8 as *const libc::c_char,
             (*afl).out_dir);
    fd =
        open(fn_0.as_mut_ptr() as *const libc::c_char,
             0o1 as libc::c_int | 0o100 as libc::c_int |
                 0o1000 as libc::c_int, 0o600 as libc::c_int);
    if fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, fn_0.as_mut_ptr());
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-stats.c\x00" as *const u8 as
                   *const libc::c_char, 46 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    f = fdopen(fd, b"w\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mfdopen() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-stats.c\x00" as *const u8 as
                   *const libc::c_char, 50 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    /* Keep last values in case we're called from another context
     where exec/sec stats and such are not readily available. */
    if bitmap_cvg == 0. && stability == 0. && eps == 0. {
        bitmap_cvg = (*afl).last_bitmap_cvg;
        stability = (*afl).last_stability
    } else {
        (*afl).last_bitmap_cvg = bitmap_cvg;
        (*afl).last_stability = stability;
        (*afl).last_eps = eps
    }
    if getrusage(RUSAGE_CHILDREN, &mut rus) != 0 {
        rus.c2rust_unnamed.ru_maxrss = 0 as libc::c_int as libc::c_long
    }
    fprintf(f,
            b"start_time        : %llu\nlast_update       : %llu\nrun_time          : %llu\nfuzzer_pid        : %d\ncycles_done       : %llu\ncycles_wo_finds   : %llu\nexecs_done        : %llu\nexecs_per_sec     : %0.02f\npaths_total       : %u\npaths_favored     : %u\npaths_found       : %u\npaths_imported    : %u\nmax_depth         : %u\ncur_path          : %u\npending_favs      : %u\npending_total     : %u\nvariable_paths    : %u\nstability         : %0.02f%%\nbitmap_cvg        : %0.02f%%\nunique_crashes    : %llu\nunique_hangs      : %llu\nlast_path         : %llu\nlast_crash        : %llu\nlast_hang         : %llu\nexecs_since_crash : %llu\nexec_timeout      : %u\nslowest_exec_ms   : %u\npeak_rss_mb       : %lu\nedges_found       : %u\nvar_byte_count    : %u\nafl_banner        : %s\nafl_version       : ++2.63d\ntarget_mode       : %s%s%s%s%s%s%s%s\ncommand_line      : %s\n\x00"
                as *const u8 as *const libc::c_char,
            (*afl).start_time.wrapping_div(1000 as libc::c_int as
                                               libc::c_ulonglong),
            cur_time.wrapping_div(1000 as libc::c_int as libc::c_ulonglong),
            cur_time.wrapping_sub((*afl).start_time).wrapping_div(1000 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulonglong),
            getpid(),
            if (*afl).queue_cycle != 0 {
                (*afl).queue_cycle.wrapping_sub(1 as libc::c_int as
                                                    libc::c_ulonglong)
            } else { 0 as libc::c_int as libc::c_ulonglong },
            (*afl).cycles_wo_finds, (*afl).total_execs,
            (*afl).total_execs as libc::c_double /
                (get_cur_time().wrapping_sub((*afl).start_time) as
                     libc::c_double / 1000 as libc::c_int as libc::c_double),
            (*afl).queued_paths, (*afl).queued_favored,
            (*afl).queued_discovered, (*afl).queued_imported,
            (*afl).max_depth, (*afl).current_entry, (*afl).pending_favored,
            (*afl).pending_not_fuzzed, (*afl).queued_variable, stability,
            bitmap_cvg, (*afl).unique_crashes, (*afl).unique_hangs,
            (*afl).last_path_time.wrapping_div(1000 as libc::c_int as
                                                   libc::c_ulonglong),
            (*afl).last_crash_time.wrapping_div(1000 as libc::c_int as
                                                    libc::c_ulonglong),
            (*afl).last_hang_time.wrapping_div(1000 as libc::c_int as
                                                   libc::c_ulonglong),
            (*afl).total_execs.wrapping_sub((*afl).last_crash_execs),
            (*afl).fsrv.exec_tmout, (*afl).slowest_exec_ms,
            (rus.c2rust_unnamed.ru_maxrss >> 10 as libc::c_int) as
                libc::c_ulong, t_bytes, (*afl).var_byte_count,
            (*afl).use_banner,
            if (*afl).unicorn_mode as libc::c_int != 0 {
                b"unicorn\x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char },
            if (*afl).fsrv.qemu_mode as libc::c_int != 0 {
                b"qemu \x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char },
            if (*afl).dumb_mode as libc::c_int != 0 {
                b" dumb \x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char },
            if (*afl).no_forkserver as libc::c_int != 0 {
                b"no_fsrv \x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char },
            if (*afl).crash_mode as libc::c_int != 0 {
                b"crash \x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char },
            if (*afl).persistent_mode as libc::c_int != 0 {
                b"persistent \x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char },
            if (*afl).deferred_mode as libc::c_int != 0 {
                b"deferred \x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char },
            if (*afl).unicorn_mode as libc::c_int != 0 ||
                   (*afl).fsrv.qemu_mode as libc::c_int != 0 ||
                   (*afl).dumb_mode as libc::c_int != 0 ||
                   (*afl).no_forkserver as libc::c_int != 0 ||
                   (*afl).crash_mode as libc::c_int != 0 ||
                   (*afl).persistent_mode as libc::c_int != 0 ||
                   (*afl).deferred_mode as libc::c_int != 0 {
                b"\x00" as *const u8 as *const libc::c_char
            } else { b"default\x00" as *const u8 as *const libc::c_char },
            (*afl).orig_cmdline);
    /* ignore errors */
    fclose(f);
}
/* Update the plot file if there is a reason to. */
#[no_mangle]
pub unsafe extern "C" fn maybe_update_plot_file(mut afl: *mut afl_state_t,
                                                mut bitmap_cvg:
                                                    libc::c_double,
                                                mut eps: libc::c_double) {
    if (*afl).plot_prev_qp == (*afl).queued_paths &&
           (*afl).plot_prev_pf == (*afl).pending_favored &&
           (*afl).plot_prev_pnf == (*afl).pending_not_fuzzed &&
           (*afl).plot_prev_ce == (*afl).current_entry &&
           (*afl).plot_prev_qc == (*afl).queue_cycle &&
           (*afl).plot_prev_uc == (*afl).unique_crashes &&
           (*afl).plot_prev_uh == (*afl).unique_hangs &&
           (*afl).plot_prev_md == (*afl).max_depth {
        return
    }
    (*afl).plot_prev_qp = (*afl).queued_paths;
    (*afl).plot_prev_pf = (*afl).pending_favored;
    (*afl).plot_prev_pnf = (*afl).pending_not_fuzzed;
    (*afl).plot_prev_ce = (*afl).current_entry;
    (*afl).plot_prev_qc = (*afl).queue_cycle;
    (*afl).plot_prev_uc = (*afl).unique_crashes;
    (*afl).plot_prev_uh = (*afl).unique_hangs;
    (*afl).plot_prev_md = (*afl).max_depth;
    /* Fields in the file:

     unix_time, afl->cycles_done, cur_path, paths_total, paths_not_fuzzed,
     favored_not_fuzzed, afl->unique_crashes, afl->unique_hangs, afl->max_depth,
     execs_per_sec */
    fprintf((*afl).fsrv.plot_file,
            b"%llu, %llu, %u, %u, %u, %u, %0.02f%%, %llu, %llu, %u, %0.02f\n\x00"
                as *const u8 as *const libc::c_char,
            get_cur_time().wrapping_div(1000 as libc::c_int as
                                            libc::c_ulonglong),
            (*afl).queue_cycle.wrapping_sub(1 as libc::c_int as
                                                libc::c_ulonglong),
            (*afl).current_entry, (*afl).queued_paths,
            (*afl).pending_not_fuzzed, (*afl).pending_favored, bitmap_cvg,
            (*afl).unique_crashes, (*afl).unique_hangs, (*afl).max_depth,
            eps); /* ignore errors */
    fflush((*afl).fsrv.plot_file);
}
/* Check terminal dimensions after resize. */
unsafe extern "C" fn check_term_size(mut afl: *mut afl_state_t) {
    let mut ws: winsize =
        winsize{ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0,};
    (*afl).term_too_small = 0 as libc::c_int as u8_0;
    if ioctl(1 as libc::c_int, 0x5413 as libc::c_int as libc::c_ulong,
             &mut ws as *mut winsize) != 0 {
        return
    }
    if ws.ws_row as libc::c_int == 0 as libc::c_int ||
           ws.ws_col as libc::c_int == 0 as libc::c_int {
        return
    }
    if (ws.ws_row as libc::c_int) < 24 as libc::c_int ||
           (ws.ws_col as libc::c_int) < 79 as libc::c_int {
        (*afl).term_too_small = 1 as libc::c_int as u8_0
    };
}
/* A spiffy retro stats screen! This is called every afl->stats_update_freq
   execve() calls, plus in several other circumstances. */
#[no_mangle]
pub unsafe extern "C" fn show_stats(mut afl: *mut afl_state_t) {
    let mut t_byte_ratio: libc::c_double = 0.;
    let mut stab_ratio: libc::c_double = 0.;
    let mut cur_ms: u64_0 = 0;
    let mut t_bytes: u32_0 = 0;
    let mut t_bits: u32_0 = 0;
    let mut banner_len: u32_0 = 0;
    let mut banner_pad: u32_0 = 0;
    let mut tmp: [u8_0; 256] = [0; 256];
    let mut time_tmp: [u8_0; 64] = [0; 64];
    let mut val_buf: [[u8_0; 16]; 8] = [[0; 16]; 8];
    cur_ms = get_cur_time();
    if (*afl).most_time_key != 0 {
        if (*afl).most_time.wrapping_mul(1000 as libc::c_int as
                                             libc::c_ulonglong) <
               cur_ms.wrapping_sub((*afl).start_time) {
            (*afl).most_time_key = 2 as libc::c_int as u64_0;
            ::std::ptr::write_volatile(&mut (*afl).stop_soon as *mut u8_0,
                                       2 as libc::c_int as u8_0)
        }
    }
    if (*afl).most_execs_key == 1 as libc::c_int as libc::c_ulonglong {
        if (*afl).most_execs <= (*afl).total_execs {
            (*afl).most_execs_key = 2 as libc::c_int as u64_0;
            ::std::ptr::write_volatile(&mut (*afl).stop_soon as *mut u8_0,
                                       2 as libc::c_int as u8_0)
        }
    }
    /* If not enough time has passed since last UI update, bail out. */
    if cur_ms.wrapping_sub((*afl).stats_last_ms) <
           (1000 as libc::c_int / 5 as libc::c_int) as libc::c_ulonglong &&
           (*afl).force_ui_update == 0 {
        return
    }
    /* Check if we're past the 10 minute mark. */
    if cur_ms.wrapping_sub((*afl).start_time) >
           (10 as libc::c_int * 60 as libc::c_int * 1000 as libc::c_int) as
               libc::c_ulonglong {
        (*afl).run_over10m = 1 as libc::c_int as u8_0
    }
    /* Calculate smoothed exec speed stats. */
    if (*afl).stats_last_execs == 0 {
        (*afl).stats_avg_exec =
            (*afl).total_execs as libc::c_double *
                1000 as libc::c_int as libc::c_double /
                cur_ms.wrapping_sub((*afl).start_time) as libc::c_double
    } else {
        let mut cur_avg: libc::c_double =
            (*afl).total_execs.wrapping_sub((*afl).stats_last_execs) as
                libc::c_double * 1000 as libc::c_int as libc::c_double /
                cur_ms.wrapping_sub((*afl).stats_last_ms) as libc::c_double;
        /* If there is a dramatic (5x+) jump in speed, reset the indicator
       more quickly. */
        if (cur_avg * 5 as libc::c_int as libc::c_double) <
               (*afl).stats_avg_exec ||
               cur_avg / 5 as libc::c_int as libc::c_double >
                   (*afl).stats_avg_exec {
            (*afl).stats_avg_exec = cur_avg
        }
        (*afl).stats_avg_exec =
            (*afl).stats_avg_exec *
                (1.0f64 - 1.0f64 / 16 as libc::c_int as libc::c_double) +
                cur_avg * (1.0f64 / 16 as libc::c_int as libc::c_double)
    }
    (*afl).stats_last_ms = cur_ms;
    (*afl).stats_last_execs = (*afl).total_execs;
    /* Tell the callers when to contact us (as measured in execs). */
    (*afl).stats_update_freq =
        ((*afl).stats_avg_exec /
             (5 as libc::c_int * 10 as libc::c_int) as libc::c_double) as
            u32_0;
    if (*afl).stats_update_freq == 0 {
        (*afl).stats_update_freq = 1 as libc::c_int as u32_0
    }
    /* Do some bitmap stats. */
    t_bytes = count_non_255_bytes(afl, (*afl).virgin_bits.as_mut_ptr());
    t_byte_ratio =
        t_bytes as libc::c_double * 100 as libc::c_int as libc::c_double /
            (*afl).fsrv.map_size as libc::c_double;
    if t_bytes != 0 && (*afl).var_byte_count != 0 {
        stab_ratio =
            100 as libc::c_int as libc::c_double -
                (*afl).var_byte_count as libc::c_double *
                    100 as libc::c_int as libc::c_double /
                    t_bytes as libc::c_double
    } else { stab_ratio = 100 as libc::c_int as libc::c_double }
    /* Roughly every minute, update fuzzer stats and save auto tokens. */
    if cur_ms.wrapping_sub((*afl).stats_last_stats_ms) >
           (60 as libc::c_int * 1000 as libc::c_int) as libc::c_ulonglong {
        (*afl).stats_last_stats_ms = cur_ms;
        write_stats_file(afl, t_byte_ratio, stab_ratio,
                         (*afl).stats_avg_exec);
        save_auto(afl);
        write_bitmap(afl);
    }
    /* Every now and then, write plot data. */
    if cur_ms.wrapping_sub((*afl).stats_last_plot_ms) >
           (5 as libc::c_int * 1000 as libc::c_int) as libc::c_ulonglong {
        (*afl).stats_last_plot_ms = cur_ms;
        maybe_update_plot_file(afl, t_byte_ratio, (*afl).stats_avg_exec);
    }
    /* Honor AFL_EXIT_WHEN_DONE and AFL_BENCH_UNTIL_CRASH. */
    if (*afl).dumb_mode == 0 &&
           (*afl).cycles_wo_finds > 100 as libc::c_int as libc::c_ulonglong &&
           (*afl).pending_not_fuzzed == 0 &&
           (*afl).afl_env.afl_exit_when_done as libc::c_int != 0 {
        ::std::ptr::write_volatile(&mut (*afl).stop_soon as *mut u8_0,
                                   2 as libc::c_int as u8_0)
    }
    if (*afl).total_crashes != 0 &&
           (*afl).afl_env.afl_bench_until_crash as libc::c_int != 0 {
        ::std::ptr::write_volatile(&mut (*afl).stop_soon as *mut u8_0,
                                   2 as libc::c_int as u8_0)
    }
    /* If we're not on TTY, bail out. */
    if (*afl).not_on_tty != 0 { return }
    /* If we haven't started doing things, bail out. */
    if (*afl).queue_cur.is_null() { return }
    /* Compute some mildly useful bitmap stats. */
    t_bits =
        ((*afl).fsrv.map_size <<
             3 as
                 libc::c_int).wrapping_sub(count_bits(afl,
                                                      (*afl).virgin_bits.as_mut_ptr()));
    /* Now, for the visuals... */
    if (*afl).clear_screen != 0 {
        printf(b"\x1b[H\x1b[2J\x1b[?25l\x00" as *const u8 as
                   *const libc::c_char);
        ::std::ptr::write_volatile(&mut (*afl).clear_screen as *mut u8_0,
                                   0 as libc::c_int as u8_0);
        check_term_size(afl);
    }
    printf(b"\x1b[H\x00" as *const u8 as *const libc::c_char);
    if (*afl).term_too_small != 0 {
        printf(b"\x1b[1;97mYour terminal is too small to display the UI.\nPlease resize terminal window to at least 79x24.\n\x1b[0m\x00"
                   as *const u8 as *const libc::c_char);
        return
    }
    /* Let's start by drawing a centered banner. */
    banner_len =
        ((if (*afl).crash_mode as libc::c_int != 0 {
              24 as libc::c_int
          } else { 22 as libc::c_int }) as
             libc::c_ulong).wrapping_add(strlen(b"++2.63d\x00" as *const u8 as
                                                    *const libc::c_char)).wrapping_add(strlen((*afl).use_banner
                                                                                                  as
                                                                                                  *const libc::c_char)).wrapping_add(strlen((*afl).power_name
                                                                                                                                                as
                                                                                                                                                *const libc::c_char)).wrapping_add(3
                                                                                                                                                                                       as
                                                                                                                                                                                       libc::c_int
                                                                                                                                                                                       as
                                                                                                                                                                                       libc::c_ulong).wrapping_add(5
                                                                                                                                                                                                                       as
                                                                                                                                                                                                                       libc::c_int
                                                                                                                                                                                                                       as
                                                                                                                                                                                                                       libc::c_ulong)
            as u32_0;
    banner_pad =
        (79 as libc::c_int as
             libc::c_uint).wrapping_sub(banner_len).wrapping_div(2 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint);
    memset(tmp.as_mut_ptr() as *mut libc::c_void, ' ' as i32,
           banner_pad as libc::c_ulong);
    sprintf(tmp.as_mut_ptr().offset(banner_pad as isize) as *mut libc::c_char,
            b"%s \x1b[1;96m++2.63d\x1b[1;92m (%s) \x1b[1;95m[%s]\x1b[0;34m {%d}\x00"
                as *const u8 as *const libc::c_char,
            if (*afl).crash_mode as libc::c_int != 0 {
                b"\x1b[1;95mperuvian were-rabbit\x00" as *const u8 as
                    *const libc::c_char
            } else {
                b"\x1b[1;93mamerican fuzzy lop\x00" as *const u8 as
                    *const libc::c_char
            }, (*afl).use_banner, (*afl).power_name, (*afl).cpu_aff);
    /* HAVE_AFFINITY */
    printf(b"\n%s\n\x00" as *const u8 as *const libc::c_char,
           tmp.as_mut_ptr());
    /* "Handy" shortcuts for drawing boxes... */
    /* Lord, forgive me this. */
    printf(b"\x1b)0\x0e\x1b[1;90mlq\x0f\x1b[0;36m process timing \x0e\x1b[1;90mqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqwq\x0f\x1b[0;36m overall results \x0e\x1b[1;90mqqqqk\n\x00"
               as *const u8 as *const libc::c_char);
    if (*afl).dumb_mode != 0 {
        strcpy(tmp.as_mut_ptr() as *mut libc::c_char,
               b"\x1b[0m\x00" as *const u8 as *const libc::c_char);
    } else {
        let mut min_wo_finds: u64_0 =
            cur_ms.wrapping_sub((*afl).last_path_time).wrapping_div(1000 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulonglong).wrapping_div(60
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulonglong);
        /* First queue cycle: don't stop now! */
        if (*afl).queue_cycle == 1 as libc::c_int as libc::c_ulonglong ||
               min_wo_finds < 15 as libc::c_int as libc::c_ulonglong {
            strcpy(tmp.as_mut_ptr() as *mut libc::c_char,
                   b"\x1b[0;35m\x00" as *const u8 as *const libc::c_char);
        } else if (*afl).cycles_wo_finds <
                      25 as libc::c_int as libc::c_ulonglong ||
                      min_wo_finds < 30 as libc::c_int as libc::c_ulonglong {
            strcpy(tmp.as_mut_ptr() as *mut libc::c_char,
                   b"\x1b[1;93m\x00" as *const u8 as *const libc::c_char);
        } else if (*afl).cycles_wo_finds >
                      100 as libc::c_int as libc::c_ulonglong &&
                      (*afl).pending_not_fuzzed == 0 &&
                      min_wo_finds > 120 as libc::c_int as libc::c_ulonglong {
            strcpy(tmp.as_mut_ptr() as *mut libc::c_char,
                   b"\x1b[1;92m\x00" as *const u8 as *const libc::c_char);
        } else {
            /* Subsequent cycles, but we're still making finds. */
            /* No finds for a long time and no test cases to try. */
            /* Default: cautiously OK to stop? */
            strcpy(tmp.as_mut_ptr() as *mut libc::c_char,
                   b"\x1b[1;94m\x00" as *const u8 as *const libc::c_char);
        }
    }
    u_stringify_time_diff(time_tmp.as_mut_ptr(), cur_ms, (*afl).start_time);
    printf(b"x\x0f        run time : \x1b[0m%-33s \x0e\x1b[1;90mx\x0f  cycles done : %s%-5s \x0e\x1b[1;90mx\n\x00"
               as *const u8 as *const libc::c_char, time_tmp.as_mut_ptr(),
           tmp.as_mut_ptr(),
           u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                           (*afl).queue_cycle.wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulonglong)));
    /* We want to warn people about not seeing new paths after a full cycle,
     except when resuming fuzzing or running in non-instrumented mode. */
    if (*afl).dumb_mode == 0 &&
           ((*afl).last_path_time != 0 ||
                (*afl).resuming_fuzz as libc::c_int != 0 ||
                (*afl).queue_cycle == 1 as libc::c_int as libc::c_ulonglong ||
                !(*afl).in_bitmap.is_null() ||
                (*afl).crash_mode as libc::c_int != 0) {
        u_stringify_time_diff(time_tmp.as_mut_ptr(), cur_ms,
                              (*afl).last_path_time);
        printf(b"x\x0f   last new path : \x1b[0m%-33s \x00" as *const u8 as
                   *const libc::c_char, time_tmp.as_mut_ptr());
    } else if (*afl).dumb_mode != 0 {
        printf(b"x\x0f   last new path : \x1b[1;95mn/a\x1b[0m (non-instrumented mode)       \x00"
                   as *const u8 as *const libc::c_char);
    } else {
        printf(b"x\x0f   last new path : \x1b[0mnone yet \x1b[1;91m(odd, check syntax!)     \x00"
                   as *const u8 as *const libc::c_char);
    }
    printf(b"\x0e\x1b[1;90mx\x0f  total paths : \x1b[0m%-5s \x0e\x1b[1;90mx\n\x00"
               as *const u8 as *const libc::c_char,
           u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                           (*afl).queued_paths as u64_0));
    /* Highlight crashes in red if found, denote going over the KEEP_UNIQUE_CRASH
     limit with a '+' appended to the count. */
    sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
            b"%s%s\x00" as *const u8 as *const libc::c_char,
            u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).unique_crashes),
            if (*afl).unique_crashes >=
                   5000 as libc::c_int as libc::c_ulonglong {
                b"+\x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char });
    u_stringify_time_diff(time_tmp.as_mut_ptr(), cur_ms,
                          (*afl).last_crash_time);
    printf(b"x\x0f last uniq crash : \x1b[0m%-33s \x0e\x1b[1;90mx\x0f uniq crashes : %s%-6s\x0e\x1b[1;90mx\n\x00"
               as *const u8 as *const libc::c_char, time_tmp.as_mut_ptr(),
           if (*afl).unique_crashes != 0 {
               b"\x1b[1;91m\x00" as *const u8 as *const libc::c_char
           } else { b"\x1b[0m\x00" as *const u8 as *const libc::c_char },
           tmp.as_mut_ptr());
    sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
            b"%s%s\x00" as *const u8 as *const libc::c_char,
            u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).unique_hangs),
            if (*afl).unique_hangs >= 500 as libc::c_int as libc::c_ulonglong
               {
                b"+\x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char });
    u_stringify_time_diff(time_tmp.as_mut_ptr(), cur_ms,
                          (*afl).last_hang_time);
    printf(b"x\x0f  last uniq hang : \x1b[0m%-33s \x0e\x1b[1;90mx\x0f   uniq hangs : \x1b[0m%-6s\x0e\x1b[1;90mx\n\x00"
               as *const u8 as *const libc::c_char, time_tmp.as_mut_ptr(),
           tmp.as_mut_ptr());
    printf(b"tq\x0f\x1b[0;36m cycle progress \x0e\x1b[1;90mqqqqqqqqqqqqqqqqqqqwq\x0f\x1b[0;36m map coverage \x0e\x1b[1;90mqvqqqqqqqqqqqqqqqqqqqqqqu\n\x00"
               as *const u8 as *const libc::c_char);
    /* This gets funny because we want to print several variable-length variables
     together, but then cram them into a fixed-width field - so we need to
     put them in a temporary buffer first. */
    sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
            b"%s%s%u (%0.01f%%)\x00" as *const u8 as *const libc::c_char,
            u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).current_entry as u64_0),
            if (*(*afl).queue_cur).favored as libc::c_int != 0 {
                b".\x00" as *const u8 as *const libc::c_char
            } else { b"*\x00" as *const u8 as *const libc::c_char },
            (*(*afl).queue_cur).fuzz_level,
            (*afl).current_entry as libc::c_double *
                100 as libc::c_int as libc::c_double /
                (*afl).queued_paths as libc::c_double);
    printf(b"x\x0f  now processing : \x1b[0m%-16s \x0e\x1b[1;90mx\x0f\x00" as
               *const u8 as *const libc::c_char, tmp.as_mut_ptr());
    sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
            b"%0.02f%% / %0.02f%%\x00" as *const u8 as *const libc::c_char,
            (*(*afl).queue_cur).bitmap_size as libc::c_double *
                100 as libc::c_int as libc::c_double /
                (*afl).fsrv.map_size as libc::c_double, t_byte_ratio);
    printf(b"    map density : %s%-21s\x0e\x1b[1;90mx\n\x00" as *const u8 as
               *const libc::c_char,
           if t_byte_ratio > 70 as libc::c_int as libc::c_double {
               b"\x1b[1;91m\x00" as *const u8 as *const libc::c_char
           } else if t_bytes < 200 as libc::c_int as libc::c_uint &&
                         (*afl).dumb_mode == 0 {
               b"\x1b[1;95m\x00" as *const u8 as *const libc::c_char
           } else { b"\x1b[0m\x00" as *const u8 as *const libc::c_char },
           tmp.as_mut_ptr());
    sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
            b"%s (%0.02f%%)\x00" as *const u8 as *const libc::c_char,
            u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).cur_skipped_paths as u64_0),
            (*afl).cur_skipped_paths as libc::c_double *
                100 as libc::c_int as libc::c_double /
                (*afl).queued_paths as libc::c_double);
    printf(b"x\x0f paths timed out : \x1b[0m%-16s \x0e\x1b[1;90mx\x00" as
               *const u8 as *const libc::c_char, tmp.as_mut_ptr());
    sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
            b"%0.02f bits/tuple\x00" as *const u8 as *const libc::c_char,
            if t_bytes != 0 {
                (t_bits as libc::c_double) / t_bytes as libc::c_double
            } else { 0 as libc::c_int as libc::c_double });
    printf(b"\x0f count coverage : \x1b[0m%-21s\x0e\x1b[1;90mx\n\x00" as
               *const u8 as *const libc::c_char, tmp.as_mut_ptr());
    printf(b"tq\x0f\x1b[0;36m stage progress \x0e\x1b[1;90mqqqqqqqqqqqqqqqqqqqnq\x0f\x1b[0;36m findings in depth \x0e\x1b[1;90mqqqqqqqqqqqqqqqqqqqu\n\x00"
               as *const u8 as *const libc::c_char);
    sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
            b"%s (%0.02f%%)\x00" as *const u8 as *const libc::c_char,
            u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).queued_favored as u64_0),
            (*afl).queued_favored as libc::c_double *
                100 as libc::c_int as libc::c_double /
                (*afl).queued_paths as libc::c_double);
    /* Yeah... it's still going on... halp? */
    printf(b"x\x0f  now trying : \x1b[0m%-20s \x0e\x1b[1;90mx\x0f favored paths : \x1b[0m%-22s\x0e\x1b[1;90mx\n\x00"
               as *const u8 as *const libc::c_char, (*afl).stage_name,
           tmp.as_mut_ptr());
    if (*afl).stage_max == 0 {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"%s/-\x00" as *const u8 as *const libc::c_char,
                u_stringify_int(val_buf[0 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cur as u64_0));
    } else {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"%s/%s (%0.02f%%)\x00" as *const u8 as *const libc::c_char,
                u_stringify_int(val_buf[0 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cur as u64_0),
                u_stringify_int(val_buf[1 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_max as u64_0),
                (*afl).stage_cur as libc::c_double *
                    100 as libc::c_int as libc::c_double /
                    (*afl).stage_max as libc::c_double);
    }
    printf(b"x\x0f stage execs : \x1b[0m%-21s\x0e\x1b[1;90mx\x0f\x00" as
               *const u8 as *const libc::c_char, tmp.as_mut_ptr());
    sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
            b"%s (%0.02f%%)\x00" as *const u8 as *const libc::c_char,
            u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).queued_with_cov as u64_0),
            (*afl).queued_with_cov as libc::c_double *
                100 as libc::c_int as libc::c_double /
                (*afl).queued_paths as libc::c_double);
    printf(b"  new edges on : \x1b[0m%-22s\x0e\x1b[1;90mx\n\x00" as *const u8
               as *const libc::c_char, tmp.as_mut_ptr());
    sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
            b"%s (%s%s unique)\x00" as *const u8 as *const libc::c_char,
            u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).total_crashes),
            u_stringify_int(val_buf[1 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).unique_crashes),
            if (*afl).unique_crashes >=
                   5000 as libc::c_int as libc::c_ulonglong {
                b"+\x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char });
    if (*afl).crash_mode != 0 {
        printf(b"x\x0f total execs : \x1b[0m%-20s \x0e\x1b[1;90mx\x0f   new crashes : %s%-22s\x0e\x1b[1;90mx\n\x00"
                   as *const u8 as *const libc::c_char,
               u_stringify_int(val_buf[0 as libc::c_int as
                                           usize].as_mut_ptr(),
                               (*afl).total_execs),
               if (*afl).unique_crashes != 0 {
                   b"\x1b[1;91m\x00" as *const u8 as *const libc::c_char
               } else { b"\x1b[0m\x00" as *const u8 as *const libc::c_char },
               tmp.as_mut_ptr());
    } else {
        printf(b"x\x0f total execs : \x1b[0m%-20s \x0e\x1b[1;90mx\x0f total crashes : %s%-22s\x0e\x1b[1;90mx\n\x00"
                   as *const u8 as *const libc::c_char,
               u_stringify_int(val_buf[0 as libc::c_int as
                                           usize].as_mut_ptr(),
                               (*afl).total_execs),
               if (*afl).unique_crashes != 0 {
                   b"\x1b[1;91m\x00" as *const u8 as *const libc::c_char
               } else { b"\x1b[0m\x00" as *const u8 as *const libc::c_char },
               tmp.as_mut_ptr());
    }
    /* Show a warning about slow execution. */
    if (*afl).stats_avg_exec < 100 as libc::c_int as libc::c_double {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"%s/sec (%s)\x00" as *const u8 as *const libc::c_char,
                u_stringify_float(val_buf[0 as libc::c_int as
                                              usize].as_mut_ptr(),
                                  (*afl).stats_avg_exec),
                if (*afl).stats_avg_exec < 20 as libc::c_int as libc::c_double
                   {
                    b"zzzz...\x00" as *const u8 as *const libc::c_char
                } else { b"slow!\x00" as *const u8 as *const libc::c_char });
        printf(b"x\x0f  exec speed : \x1b[1;91m%-20s \x00" as *const u8 as
                   *const libc::c_char, tmp.as_mut_ptr());
    } else {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"%s/sec\x00" as *const u8 as *const libc::c_char,
                u_stringify_float(val_buf[0 as libc::c_int as
                                              usize].as_mut_ptr(),
                                  (*afl).stats_avg_exec));
        printf(b"x\x0f  exec speed : \x1b[0m%-20s \x00" as *const u8 as
                   *const libc::c_char, tmp.as_mut_ptr());
    }
    sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
            b"%s (%s%s unique)\x00" as *const u8 as *const libc::c_char,
            u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).total_tmouts),
            u_stringify_int(val_buf[1 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).unique_tmouts),
            if (*afl).unique_hangs >= 500 as libc::c_int as libc::c_ulonglong
               {
                b"+\x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char });
    printf(b"\x0e\x1b[1;90mx\x0f  total tmouts : \x1b[0m%-22s\x0e\x1b[1;90mx\n\x00"
               as *const u8 as *const libc::c_char, tmp.as_mut_ptr());
    /* Aaaalmost there... hold on! */
    printf(b"tq\x1b[0;36m\x0f fuzzing strategy yields \x0e\x1b[1;90mqqqqqqqqqqvqqqqqqqqqqqqqqqwq\x0f\x1b[0;36m path geometry \x0e\x1b[1;90mqqqqqqqu\n\x00"
               as *const u8 as *const libc::c_char);
    if (*afl).skip_deterministic != 0 {
        strcpy(tmp.as_mut_ptr() as *mut libc::c_char,
               b"n/a, n/a, n/a\x00" as *const u8 as *const libc::c_char);
    } else {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"%s/%s, %s/%s, %s/%s\x00" as *const u8 as
                    *const libc::c_char,
                u_stringify_int(val_buf[0 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_FLIP1 as libc::c_int
                                                       as usize]),
                u_stringify_int(val_buf[1 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_FLIP1 as libc::c_int
                                                        as usize]),
                u_stringify_int(val_buf[2 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_FLIP2 as libc::c_int
                                                       as usize]),
                u_stringify_int(val_buf[3 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_FLIP2 as libc::c_int
                                                        as usize]),
                u_stringify_int(val_buf[4 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_FLIP4 as libc::c_int
                                                       as usize]),
                u_stringify_int(val_buf[5 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_FLIP4 as libc::c_int
                                                        as usize]));
    }
    printf(b"x\x0f   bit flips : \x1b[0m%-36s \x0e\x1b[1;90mx\x0f    levels : \x1b[0m%-10s\x0e\x1b[1;90mx\n\x00"
               as *const u8 as *const libc::c_char, tmp.as_mut_ptr(),
           u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                           (*afl).max_depth as u64_0));
    if (*afl).skip_deterministic == 0 {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"%s/%s, %s/%s, %s/%s\x00" as *const u8 as
                    *const libc::c_char,
                u_stringify_int(val_buf[0 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_FLIP8 as libc::c_int
                                                       as usize]),
                u_stringify_int(val_buf[1 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_FLIP8 as libc::c_int
                                                        as usize]),
                u_stringify_int(val_buf[2 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_FLIP16 as libc::c_int
                                                       as usize]),
                u_stringify_int(val_buf[3 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_FLIP16 as
                                                        libc::c_int as
                                                        usize]),
                u_stringify_int(val_buf[4 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_FLIP32 as libc::c_int
                                                       as usize]),
                u_stringify_int(val_buf[5 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_FLIP32 as
                                                        libc::c_int as
                                                        usize]));
    }
    printf(b"x\x0f  byte flips : \x1b[0m%-36s \x0e\x1b[1;90mx\x0f   pending : \x1b[0m%-10s\x0e\x1b[1;90mx\n\x00"
               as *const u8 as *const libc::c_char, tmp.as_mut_ptr(),
           u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                           (*afl).pending_not_fuzzed as u64_0));
    if (*afl).skip_deterministic == 0 {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"%s/%s, %s/%s, %s/%s\x00" as *const u8 as
                    *const libc::c_char,
                u_stringify_int(val_buf[0 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_ARITH8 as libc::c_int
                                                       as usize]),
                u_stringify_int(val_buf[1 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_ARITH8 as
                                                        libc::c_int as
                                                        usize]),
                u_stringify_int(val_buf[2 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_ARITH16 as
                                                       libc::c_int as usize]),
                u_stringify_int(val_buf[3 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_ARITH16 as
                                                        libc::c_int as
                                                        usize]),
                u_stringify_int(val_buf[4 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_ARITH32 as
                                                       libc::c_int as usize]),
                u_stringify_int(val_buf[5 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_ARITH32 as
                                                        libc::c_int as
                                                        usize]));
    }
    printf(b"x\x0f arithmetics : \x1b[0m%-36s \x0e\x1b[1;90mx\x0f  pend fav : \x1b[0m%-10s\x0e\x1b[1;90mx\n\x00"
               as *const u8 as *const libc::c_char, tmp.as_mut_ptr(),
           u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                           (*afl).pending_favored as u64_0));
    if (*afl).skip_deterministic == 0 {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"%s/%s, %s/%s, %s/%s\x00" as *const u8 as
                    *const libc::c_char,
                u_stringify_int(val_buf[0 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_INTEREST8 as
                                                       libc::c_int as usize]),
                u_stringify_int(val_buf[1 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_INTEREST8 as
                                                        libc::c_int as
                                                        usize]),
                u_stringify_int(val_buf[2 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_INTEREST16 as
                                                       libc::c_int as usize]),
                u_stringify_int(val_buf[3 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_INTEREST16 as
                                                        libc::c_int as
                                                        usize]),
                u_stringify_int(val_buf[4 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_INTEREST32 as
                                                       libc::c_int as usize]),
                u_stringify_int(val_buf[5 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_INTEREST32 as
                                                        libc::c_int as
                                                        usize]));
    }
    printf(b"x\x0f  known ints : \x1b[0m%-36s \x0e\x1b[1;90mx\x0f own finds : \x1b[0m%-10s\x0e\x1b[1;90mx\n\x00"
               as *const u8 as *const libc::c_char, tmp.as_mut_ptr(),
           u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                           (*afl).queued_discovered as u64_0));
    if (*afl).skip_deterministic == 0 {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"%s/%s, %s/%s, %s/%s\x00" as *const u8 as
                    *const libc::c_char,
                u_stringify_int(val_buf[0 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_EXTRAS_UO as
                                                       libc::c_int as usize]),
                u_stringify_int(val_buf[1 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_EXTRAS_UO as
                                                        libc::c_int as
                                                        usize]),
                u_stringify_int(val_buf[2 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_EXTRAS_UI as
                                                       libc::c_int as usize]),
                u_stringify_int(val_buf[3 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_EXTRAS_UI as
                                                        libc::c_int as
                                                        usize]),
                u_stringify_int(val_buf[4 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_EXTRAS_AO as
                                                       libc::c_int as usize]),
                u_stringify_int(val_buf[5 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_EXTRAS_AO as
                                                        libc::c_int as
                                                        usize]));
    }
    printf(b"x\x0f  dictionary : \x1b[0m%-36s \x0e\x1b[1;90mx\x0f  imported : \x1b[0m%-10s\x0e\x1b[1;90mx\n\x00"
               as *const u8 as *const libc::c_char, tmp.as_mut_ptr(),
           if !(*afl).sync_id.is_null() {
               u_stringify_int(val_buf[0 as libc::c_int as
                                           usize].as_mut_ptr(),
                               (*afl).queued_imported as u64_0)
           } else {
               b"n/a\x00" as *const u8 as *const libc::c_char as *mut u8_0
           });
    sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
            b"%s/%s, %s/%s, %s/%s\x00" as *const u8 as *const libc::c_char,
            u_stringify_int(val_buf[0 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).stage_finds[STAGE_HAVOC as libc::c_int as
                                                   usize]),
            u_stringify_int(val_buf[2 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).stage_cycles[STAGE_HAVOC as libc::c_int as
                                                    usize]),
            u_stringify_int(val_buf[3 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).stage_finds[STAGE_SPLICE as libc::c_int as
                                                   usize]),
            u_stringify_int(val_buf[4 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).stage_cycles[STAGE_SPLICE as libc::c_int as
                                                    usize]),
            u_stringify_int(val_buf[5 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).stage_finds[STAGE_RADAMSA as libc::c_int as
                                                   usize]),
            u_stringify_int(val_buf[6 as libc::c_int as usize].as_mut_ptr(),
                            (*afl).stage_cycles[STAGE_RADAMSA as libc::c_int
                                                    as usize]));
    printf(b"x\x0f   havoc/rad : \x1b[0m%-36s \x0e\x1b[1;90mx\x0f\x00" as
               *const u8 as *const libc::c_char, tmp.as_mut_ptr());
    if t_bytes != 0 {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"%0.02f%%\x00" as *const u8 as *const libc::c_char,
                stab_ratio);
    } else {
        strcpy(tmp.as_mut_ptr() as *mut libc::c_char,
               b"n/a\x00" as *const u8 as *const libc::c_char);
    }
    printf(b" stability : %s%-10s\x0e\x1b[1;90mx\n\x00" as *const u8 as
               *const libc::c_char,
           if stab_ratio < 85 as libc::c_int as libc::c_double &&
                  (*afl).var_byte_count > 40 as libc::c_int as libc::c_uint {
               b"\x1b[1;91m\x00" as *const u8 as *const libc::c_char
           } else if (*afl).queued_variable != 0 &&
                         ((*afl).persistent_mode == 0 ||
                              (*afl).var_byte_count >
                                  20 as libc::c_int as libc::c_uint) {
               b"\x1b[0;35m\x00" as *const u8 as *const libc::c_char
           } else { b"\x1b[0m\x00" as *const u8 as *const libc::c_char },
           tmp.as_mut_ptr());
    if (*afl).shm.cmplog_mode != 0 {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"%s/%s, %s/%s, %s/%s, %s/%s\x00" as *const u8 as
                    *const libc::c_char,
                u_stringify_int(val_buf[0 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_PYTHON as libc::c_int
                                                       as usize]),
                u_stringify_int(val_buf[1 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_PYTHON as
                                                        libc::c_int as
                                                        usize]),
                u_stringify_int(val_buf[2 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_CUSTOM_MUTATOR as
                                                       libc::c_int as usize]),
                u_stringify_int(val_buf[3 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_CUSTOM_MUTATOR as
                                                        libc::c_int as
                                                        usize]),
                u_stringify_int(val_buf[4 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_COLORIZATION as
                                                       libc::c_int as usize]),
                u_stringify_int(val_buf[5 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_COLORIZATION as
                                                        libc::c_int as
                                                        usize]),
                u_stringify_int(val_buf[6 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_ITS as libc::c_int as
                                                       usize]),
                u_stringify_int(val_buf[7 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_ITS as libc::c_int
                                                        as usize]));
        printf(b"x\x0f   custom/rq : \x1b[0m%-36s \x0e\x1b[1;90mtqqqqqqqqqqqqqqqqqqqqqqqj\n\x00"
                   as *const u8 as *const libc::c_char, tmp.as_mut_ptr());
    } else {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"%s/%s, %s/%s\x00" as *const u8 as *const libc::c_char,
                u_stringify_int(val_buf[0 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_PYTHON as libc::c_int
                                                       as usize]),
                u_stringify_int(val_buf[1 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_PYTHON as
                                                        libc::c_int as
                                                        usize]),
                u_stringify_int(val_buf[2 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_CUSTOM_MUTATOR as
                                                       libc::c_int as usize]),
                u_stringify_int(val_buf[3 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_CUSTOM_MUTATOR as
                                                        libc::c_int as
                                                        usize]));
        printf(b"x\x0f   py/custom : \x1b[0m%-36s \x0e\x1b[1;90mtqqqqqqqqqqqqqqqqqqqqqqqj\n\x00"
                   as *const u8 as *const libc::c_char, tmp.as_mut_ptr());
    }
    if (*afl).bytes_trim_out == 0 {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"n/a, \x00" as *const u8 as *const libc::c_char);
    } else {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"%0.02f%%/%s, \x00" as *const u8 as *const libc::c_char,
                (*afl).bytes_trim_in.wrapping_sub((*afl).bytes_trim_out) as
                    libc::c_double * 100 as libc::c_int as libc::c_double /
                    (*afl).bytes_trim_in as libc::c_double,
                u_stringify_int(val_buf[0 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).trim_execs));
    }
    if (*afl).blocks_eff_total == 0 {
        let mut tmp2: [u8_0; 128] = [0; 128];
        sprintf(tmp2.as_mut_ptr() as *mut libc::c_char,
                b"n/a\x00" as *const u8 as *const libc::c_char);
        strcat(tmp.as_mut_ptr() as *mut libc::c_char,
               tmp2.as_mut_ptr() as *const libc::c_char);
    } else {
        let mut tmp2_0: [u8_0; 128] = [0; 128];
        sprintf(tmp2_0.as_mut_ptr() as *mut libc::c_char,
                b"%0.02f%%\x00" as *const u8 as *const libc::c_char,
                (*afl).blocks_eff_total.wrapping_sub((*afl).blocks_eff_select)
                    as libc::c_double * 100 as libc::c_int as libc::c_double /
                    (*afl).blocks_eff_total as libc::c_double);
        strcat(tmp.as_mut_ptr() as *mut libc::c_char,
               tmp2_0.as_mut_ptr() as *const libc::c_char);
    }
    if !(*afl).mutator.is_null() {
        sprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                b"%s/%s\x00" as *const u8 as *const libc::c_char,
                u_stringify_int(val_buf[0 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_finds[STAGE_CUSTOM_MUTATOR as
                                                       libc::c_int as usize]),
                u_stringify_int(val_buf[1 as libc::c_int as
                                            usize].as_mut_ptr(),
                                (*afl).stage_cycles[STAGE_CUSTOM_MUTATOR as
                                                        libc::c_int as
                                                        usize]));
        printf(b"x\x0f custom mut. : \x1b[0m%-36s \x0e\x1b[1;90mx\x1b)B\x00"
                   as *const u8 as *const libc::c_char, tmp.as_mut_ptr());
    } else {
        printf(b"x\x0f        trim : \x1b[0m%-36s \x0e\x1b[1;90mx\x1b)B\x00"
                   as *const u8 as *const libc::c_char, tmp.as_mut_ptr());
    }
    /* Provide some CPU utilization stats. */
    if (*afl).cpu_core_count != 0 {
        let mut spacing: *mut libc::c_char =
            b"          \x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        let mut snap: [libc::c_char; 24] =
            *::std::mem::transmute::<&[u8; 24],
                                     &mut [libc::c_char; 24]>(b" \x1b[1;92msnapshot\x1b[0m \x00\x00\x00");
        let mut cur_runnable: libc::c_double = get_runnable_processes();
        let mut cur_utilization: u32_0 =
            (cur_runnable * 100 as libc::c_int as libc::c_double /
                 (*afl).cpu_core_count as libc::c_double) as u32_0;
        let mut cpu_color: *mut u8_0 =
            b"\x1b[0;36m\x00" as *const u8 as *const libc::c_char as
                *mut u8_0;
        /* ^HAVE_AFFINITY */
        if (*afl).cpu_core_count > 1 as libc::c_int &&
               cur_runnable + 1 as libc::c_int as libc::c_double <=
                   (*afl).cpu_core_count as libc::c_double {
            cpu_color =
                b"\x1b[1;92m\x00" as *const u8 as *const libc::c_char as
                    *mut u8_0
        }
        if (*afl).no_cpu_meter_red == 0 &&
               cur_utilization >= 150 as libc::c_int as libc::c_uint {
            cpu_color =
                b"\x1b[1;91m\x00" as *const u8 as *const libc::c_char as
                    *mut u8_0
        }
        if (*afl).fsrv.snapshot != 0 { spacing = snap.as_mut_ptr() }
        if (*afl).cpu_aff >= 0 as libc::c_int {
            printf(b"%s\x1b[1;90m[cpu%03u:%s%3u%%\x1b[1;90m]\r\x1b[0m\x00" as
                       *const u8 as *const libc::c_char, spacing,
                   ({
                        let mut _a: s32 = (*afl).cpu_aff;
                        let mut _b: libc::c_int = 999 as libc::c_int;
                        if _a < _b { _a } else { _b }
                    }), cpu_color,
                   ({
                        let mut _a: u32_0 = cur_utilization;
                        let mut _b: libc::c_int = 999 as libc::c_int;
                        if _a < _b as libc::c_uint {
                            _a
                        } else { _b as libc::c_uint }
                    }));
        } else {
            printf(b"%s\x1b[1;90m   [cpu:%s%3u%%\x1b[1;90m]\r\x1b[0m\x00" as
                       *const u8 as *const libc::c_char, spacing, cpu_color,
                   ({
                        let mut _a: u32_0 = cur_utilization;
                        let mut _b: libc::c_int = 999 as libc::c_int;
                        if _a < _b as libc::c_uint {
                            _a
                        } else { _b as libc::c_uint }
                    }));
        }
    } else { printf(b"\r\x00" as *const u8 as *const libc::c_char); }
    /* If we could still run one or more processes, use green. */
    /* If we're clearly oversubscribed, use red. */
    /* Last line */
    printf(b"\x1b)0\n\x0e\x1b[1;90mmqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqj\x0f\x1b[0m\x1b)B\x00"
               as *const u8 as *const libc::c_char);
    /* Hallelujah! */
    fflush(0 as *mut FILE);
}
/*
   american fuzzy lop++ - fuzzer header
   ------------------------------------

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

   This is the real deal: the program takes an instrumented binary and
   attempts a variety of basic fuzzing tricks, paying close attention to
   how they affect the execution path.

 */
/* __APPLE__ || __FreeBSD__ || __OpenBSD__ */
/* For systems that have sched_setaffinity; right now just Linux, but one
   can hope... */
/* __linux__ */
/* clashes with FreeBSD */
/* ^!SIMPLE_FILES */
/* usable size for stage name buf in afl_state */
/* File name for the test case      */
/* Input length                     */
/* Calibration failed?              */
/* Trimmed?                         */
/* historical, but needed for MOpt  */
/* Deterministic stages passed?     */
/* Triggers new coverage?           */
/* Variable behavior?               */
/* Currently favored?               */
/* Marked as redundant in the fs?   */
/* Do not run redqueen stage again  */
/* Number of bits set in bitmap     */
/* Number of fuzzing iterations     */
/* Checksum of the execution trace  */
/* Execution time (us)              */
/* Number of queue cycles behind    */
/* Number of fuzz, does not overflow */
/* Path depth                       */
/* Trace bytes, if kept             */
/* Trace bytes ref count            */
/* Next element, if any             */
/* 100 elements ahead               */
/* Dictionary token data            */
/* Dictionary token length          */
/* Use count in the corpus          */
/* Fuzzing stages */
/* 00 */
/* 01 */
/* 02 */
/* 03 */
/* 04 */
/* 05 */
/* 06 */
/* 07 */
/* 08 */
/* 09 */
/* 10 */
/* 11 */
/* 12 */
/* 13 */
/* 14 */
/* 15 */
/* 16 */
/* 17 */
/* 18 */
/* 19 */
/* 20 */
/* 21 */
/* Stage value types */
/* 00 */
/* 01 */
/* 02 */
/* Execution status fault codes */
/* 00 */
/* 01 */
/* 02 */
/* 03 */
/* 04 */
/* 05 */
/* 00 */
/* AFL default, Exploration-based constant schedule */
/* 01 */
/* Exponential schedule             */
/* 02 */
/* Cut-Off Exponential schedule     */
/* 03 */
/* Linear schedule                  */
/* 04 */
/* Quadratic schedule               */
/* 05 */
/* AFL's exploitation-based const.  */
/* 06 */
/* Modified MOPT schedule           */
/* 07 */
/* Rare edges                       */
/* Python stuff */
/* Position of this state in the global states list */
/* argv if needed */
/* MOpt:
    Lots of globals, but mostly for the status UI and other things where it
    really makes no sense to haul them around as function parameters. */
/* Patterns found per
                                                            fuzz stage    */
/* Execs per fuzz stage */
/* Input directory with test cases  */
/* Working & output directory       */
/* Temporary directory for input    */
/* Synchronization directory        */
/* Fuzzer ID                        */
/* Power schedule name              */
/* Display banner                   */
/* Input bitmap                     */
/* File extension                   */
/* Original command line            */
/* Command to execute on a new crash */
/* Timeout used for hang det (ms)   */
/* Calibration cycles defaults      */
/* Calibration cycles defaults      */
/* do not unlink cur_input          */
/* Debug mode                       */
/* Custom mutator only mode         */
/* Python-only mode                 */
/* Stats update frequency (execs)   */
/* Power schedule (default: EXPLORE)*/
/* Skip deterministic stages?       */
/* Force deterministic stages?      */
/* Recombine input files?           */
/* Run in non-instrumented mode?    */
/* Scoring for favorites changed?   */
/* Signal that killed the child     */
/* Resuming an older fuzzing job?   */
/* Specific timeout given?          */
/* stdout is not a tty              */
/* terminal dimensions too small    */
/* Disable forkserver?              */
/* Crash mode! Yeah!                */
/* Attempt in-place resume?         */
/* Resume if afl->out_dir exists?   */
/* Auto-generated tokens changed?   */
/* Feng shui on the status screen   */
/* Skip most arithmetic ops         */
/* Shuffle input queue?             */
/* Time to update bitmap?           */
/* Running in Unicorn mode?         */
/* Use WINE with QEMU mode          */
/* Skip request, via SIGUSR1        */
/* Run time over 10 minutes?        */
/* Running in persistent mode?      */
/* Deferred forkserver mode?        */
/* do not reseed                    */
/* Try to calibrate faster?         */
/* Never trim in fuzz_one           */
/* Regions yet untouched by fuzzing */
/* Bits we haven't seen in tmouts   */
/* Bits we haven't seen in crashes  */
/* Bytes that appear to be variable */
/* Ctrl-C pressed?                  */
/* Window resized?                  */
/* Total number of queued testcases */
/* Testcases with variable behavior */
/* Total number of initial inputs   */
/* Items discovered during this run */
/* Items imported via -S            */
/* Paths deemed favorable           */
/* Paths with new coverage bytes    */
/* Queued but not done yet          */
/* Pending favored paths            */
/* Abandoned inputs in cur cycle    */
/* Current path depth               */
/* Max path depth                   */
/* Number of useless starting paths */
/* Bitmap bytes with var behavior   */
/* Current queue entry ID           */
/* Cycle count divisor for havoc    */
/* Total number of crashes          */
/* Crashes with unique signatures   */
/* Total number of timeouts         */
/* Timeouts with unique signatures  */
/* Hangs with unique signatures     */
/* Total execve() calls             */
/* Exec counter at last crash       */
/* Queue round counter              */
/* Cycles without any new paths     */
/* Execs done to trim input files   */
/* Bytes coming into the trimmer    */
/* Bytes coming outa the trimmer    */
/* Blocks subject to effector maps  */
/* Blocks selected as fuzzable      */
/* Unix start time (ms)             */
/* Time for most recent path (ms)   */
/* Time for most recent crash (ms)  */
/* Time for most recent hang (ms)   */
/* Slowest testcase non hang in ms  */
/* Number of timeouts in a row      */
/* Name of the current fuzz stage   */
/* Short stage name                 */
/* Currently syncing with...        */
/* reused stagename buf with len 64 */
/* Stage progression                */
/* Splicing with which test case?   */
/* Master instance job splitting    */
/* Syncing with case #...           */
/* Byte offset of current stage op  */
/* Value used for stage op          */
/* Value type (STAGE_VAL_*)         */
/* Patterns found per fuzz stage    */
/* Execs per fuzz stage             */
/* Random number counter            */
/* Total calibration time (us)      */
/* Total calibration cycles         */
/* Total bit count for all bitmaps  */
/* Number of bitmaps counted        */
/* CPU core count                   */
/* Selected CPU core                */
/* HAVE_AFFINITY */
/* Fuzzing queue (linked list)      */
/* Current offset within the queue  */
/* Top of the list                  */
/* Previous 100 marker              */
/* Top entries for bitmap bytes */
/* Extra tokens to fuzz with        */
/* Total number of tokens read      */
/* Automatically selected extras    */
/* Total number of tokens available */
/* afl_postprocess API */
/* CmpLog */
/* cmplog has its own little forkserver */
/* Custom mutators */
/* cmplog forkserver ids */
/* describe_op will use this to return a string
                                  up to 256 */
/* statistics file */
/* plot file saves from last run */
/*needed for afl_fuzz_one */
  // TODO: see which we can reuse
/* A global pointer to all instances is needed (for now) for signals to arrive
 */
/* custom mutator data ptr */
/* hooks for the custom mutator function */
/* *
   * Initialize the custom mutator.
   *
   * @param afl AFL instance.
   * @param seed Seed used for the mutation.
   * @return pointer to internal data or NULL on error
   */
/* *
   * Perform custom mutations on a given input
   *
   * (Optional for now. Required in the future)
   *
   * @param data pointer returned in afl_custom_init for this fuzz case
   * @param[in] buf Pointer to the input data to be mutated and the mutated
   *     output
   * @param[in] buf_size Size of the input/output data
   * @param[out] out_buf the new buffer. We may reuse *buf if large enough.
   *             *out_buf = NULL is treated as FATAL.
   * @param[in] add_buf Buffer containing the additional test case
   * @param[in] add_buf_size Size of the additional test case
   * @param[in] max_size Maximum size of the mutated output. The mutation must
   * not produce data larger than max_size.
   * @return Size of the mutated output.
   */
/* *
   * A post-processing function to use right before AFL writes the test case to
   * disk in order to execute the target.
   *
   * (Optional) If this functionality is not needed, simply don't define this
   * function.
   *
   * @param[in] data pointer returned in afl_custom_init for this fuzz case
   * @param[in] buf Buffer containing the test case to be executed
   * @param[in] buf_size Size of the test case
   * @param[out] out_buf Pointer to the buffer storing the test case after
   *     processing. External library should allocate memory for out_buf.
   *     It can chose to alter buf in-place, if the space is large enough.
   * @return Size of the output buffer.
   */
/* *
   * This method is called at the start of each trimming operation and receives
   * the initial buffer. It should return the amount of iteration steps possible
   * on this input (e.g. if your input has n elements and you want to remove
   * them one by one, return n, if you do a binary search, return log(n),
   * and so on...).
   *
   * If your trimming algorithm doesn't allow you to determine the amount of
   * (remaining) steps easily (esp. while running), then you can alternatively
   * return 1 here and always return 0 in post_trim until you are finished and
   * no steps remain. In that case, returning 1 in post_trim will end the
   * trimming routine. The whole current index/max iterations stuff is only used
   * to show progress.
   *
   * (Optional)
   *
   * @param data pointer returned in afl_custom_init for this fuzz case
   * @param buf Buffer containing the test case
   * @param buf_size Size of the test case
   * @return The amount of possible iteration steps to trim the input.
   *        Negative on error.
   */
/* *
   * This method is called for each trimming operation. It doesn't have any
   * arguments because we already have the initial buffer from init_trim and we
   * can memorize the current state in global variables. This can also save
   * reparsing steps for each iteration. It should return the trimmed input
   * buffer, where the returned data must not exceed the initial input data in
   * length. Returning anything that is larger than the original data (passed
   * to init_trim) will result in a fatal abort of AFLFuzz.
   *
   * (Optional)
   *
   * @param data pointer returned in afl_custom_init for this fuzz case
   * @param[out] out_buf Pointer to the buffer containing the trimmed test case.
   *             The library can reuse a buffer for each call
   *             and will have to free the buf (for example in deinit)
   * @return the size of the trimmed test case
   */
/* *
   * This method is called after each trim operation to inform you if your
   * trimming step was successful or not (in terms of coverage). If you receive
   * a failure here, you should reset your input to the last known good state.
   *
   * (Optional)
   *
   * @param data pointer returned in afl_custom_init for this fuzz case
   * @param success Indicates if the last trim operation was successful.
   * @return The next trim iteration index (from 0 to the maximum amount of
   *     steps returned in init_trim). Negative on error.
   */
/* *
   * Perform a single custom mutation on a given input.
   * This mutation is stacked with the other muatations in havoc.
   *
   * (Optional)
   *
   * @param[in] data pointer returned in afl_custom_init for this fuzz case
   * @param[in] buf Pointer to the input data to be mutated and the mutated
   *     output
   * @param[in] buf_size Size of input data
   * @param[out] out_buf The new buffer. It's legal to reuse *buf if it's <
   * buf_size.
   * @param[in] max_size Maximum size of the mutated output. The mutation must
   *     not produce data larger than max_size.
   * @return Size of the mutated output (out_size).
   */
/* *
   * Return the probability (in percentage) that afl_custom_havoc_mutation
   * is called in havoc. By default it is 6 %.
   *
   * (Optional)
   *
   * @param data pointer returned in afl_custom_init for this fuzz case
   * @return The probability (0-100).
   */
/* *
   * Determine whether the fuzzer should fuzz the current queue entry or not.
   *
   * (Optional)
   *
   * @param data pointer returned in afl_custom_init for this fuzz case
   * @param filename File name of the test case in the queue entry
   * @return Return True(1) if the fuzzer will fuzz the queue entry, and
   *     False(0) otherwise.
   */
/* *
   * Allow for additional analysis (e.g. calling a different tool that does a
   * different kind of coverage and saves this for the custom mutator).
   *
   * (Optional)
   *
   * @param data pointer returned in afl_custom_init for this fuzz case
   * @param filename_new_queue File name of the new queue entry
   * @param filename_orig_queue File name of the original queue entry. This
   *     argument can be NULL while initializing the fuzzer
   */
/* *
   * Deinitialize the custom mutator.
   *
   * @param data pointer returned in afl_custom_init for this fuzz case
   */
/* *** Prototypes ****/
/* Custom mutators */
/* Python */
/* Queue */
/* Bitmap */
/* Extras */
/* Stats */
/* Display quick statistics at the end of processing the input directory,
   plus a bunch of warnings. Some calibration stuff also ended up here,
   along with several hardcoded constants. Maybe clean up eventually. */
#[no_mangle]
pub unsafe extern "C" fn show_init_stats(mut afl: *mut afl_state_t) {
    let mut q: *mut queue_entry = (*afl).queue;
    let mut min_bits: u32_0 = 0 as libc::c_int as u32_0;
    let mut max_bits: u32_0 = 0 as libc::c_int as u32_0;
    let mut min_us: u64_0 = 0 as libc::c_int as u64_0;
    let mut max_us: u64_0 = 0 as libc::c_int as u64_0;
    let mut avg_us: u64_0 = 0 as libc::c_int as u64_0;
    let mut max_len: u32_0 = 0 as libc::c_int as u32_0;
    let mut val_bufs: [[u8_0; 16]; 4] = [[0; 16]; 4];
    if (*afl).total_cal_cycles != 0 {
        avg_us = (*afl).total_cal_us.wrapping_div((*afl).total_cal_cycles)
    }
    while !q.is_null() {
        if min_us == 0 || (*q).exec_us < min_us { min_us = (*q).exec_us }
        if (*q).exec_us > max_us { max_us = (*q).exec_us }
        if min_bits == 0 || (*q).bitmap_size < min_bits {
            min_bits = (*q).bitmap_size
        }
        if (*q).bitmap_size > max_bits { max_bits = (*q).bitmap_size }
        if (*q).len > max_len { max_len = (*q).len }
        q = (*q).next
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    if avg_us >
           (if (*afl).fsrv.qemu_mode as libc::c_int != 0 ||
                   (*afl).unicorn_mode as libc::c_int != 0 {
                50000 as libc::c_int
            } else { 10000 as libc::c_int }) as libc::c_ulonglong {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0m\x1b[1;91mThe target binary is pretty slow! See %s/perf_tips.md.\x00"
                   as *const u8 as *const libc::c_char, doc_path);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    /* Let's keep things moving with slow binaries. */
    if avg_us > 50000 as libc::c_int as libc::c_ulonglong
       { /* 50-100 execs/sec */
        (*afl).havoc_div = 10 as libc::c_int as u32_0
    } else if avg_us > 20000 as libc::c_int as libc::c_ulonglong
     { /* 0-19 execs/sec   */
        (*afl).havoc_div = 5 as libc::c_int as u32_0
    } else if avg_us > 10000 as libc::c_int as libc::c_ulonglong {
        (*afl).havoc_div = 2 as libc::c_int as u32_0
    } /* 20-49 execs/sec  */
    if (*afl).resuming_fuzz == 0 {
        if max_len > (50 as libc::c_int * 1024 as libc::c_int) as libc::c_uint
           {
            printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0m\x1b[1;91mSome test cases are huge (%s) - see %s/perf_tips.md!\x00"
                       as *const u8 as *const libc::c_char,
                   stringify_mem_size(val_bufs[0 as libc::c_int as
                                                   usize].as_mut_ptr(),
                                      ::std::mem::size_of::<[u8_0; 16]>() as
                                          libc::c_ulong, max_len as u64_0),
                   doc_path);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        } else if max_len >
                      (10 as libc::c_int * 1024 as libc::c_int) as
                          libc::c_uint {
            printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mSome test cases are big (%s) - see %s/perf_tips.md.\x00"
                       as *const u8 as *const libc::c_char,
                   stringify_mem_size(val_bufs[0 as libc::c_int as
                                                   usize].as_mut_ptr(),
                                      ::std::mem::size_of::<[u8_0; 16]>() as
                                          libc::c_ulong, max_len as u64_0),
                   doc_path);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
        if (*afl).useless_at_start != 0 && (*afl).in_bitmap.is_null() {
            printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0m\x1b[1;91mSome test cases look useless. Consider using a smaller set.\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
        if (*afl).queued_paths > 100 as libc::c_int as libc::c_uint {
            printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0m\x1b[1;91mYou probably have far too many input files! Consider trimming down.\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        } else if (*afl).queued_paths > 20 as libc::c_int as libc::c_uint {
            printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mYou have lots of input files; try starting small.\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
    }
    printf(b"\x1b[1;92m[+] \x1b[0mHere are some useful stats:\n\n\x1b[1;90m    Test case count : \x1b[0m%u favored, %u variable, %u total\n\x1b[1;90m       Bitmap range : \x1b[0m%u to %u bits (average: %0.02f bits)\n\x1b[1;90m        Exec timing : \x1b[0m%s to %s us (average: %s us)\n\x00"
               as *const u8 as *const libc::c_char, (*afl).queued_favored,
           (*afl).queued_variable, (*afl).queued_paths, min_bits, max_bits,
           (*afl).total_bitmap_size as libc::c_double /
               (if (*afl).total_bitmap_entries != 0 {
                    (*afl).total_bitmap_entries
                } else { 1 as libc::c_int as libc::c_ulonglong }) as
                   libc::c_double,
           stringify_int(val_bufs[0 as libc::c_int as usize].as_mut_ptr(),
                         ::std::mem::size_of::<[u8_0; 16]>() as libc::c_ulong,
                         min_us),
           stringify_int(val_bufs[1 as libc::c_int as usize].as_mut_ptr(),
                         ::std::mem::size_of::<[u8_0; 16]>() as libc::c_ulong,
                         max_us),
           stringify_int(val_bufs[2 as libc::c_int as usize].as_mut_ptr(),
                         ::std::mem::size_of::<[u8_0; 16]>() as libc::c_ulong,
                         avg_us));
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    if (*afl).timeout_given == 0 {
        /* Figure out the appropriate timeout. The basic idea is: 5x average or
       1x max, rounded up to EXEC_TM_ROUND ms and capped at 1 second.

       If the program is slow, the multiplier is lowered to 2x or 3x, because
       random scheduler jitter is less likely to have any impact, and because
       our patience is wearing thin =) */
        if avg_us > 50000 as libc::c_int as libc::c_ulonglong {
            (*afl).fsrv.exec_tmout =
                avg_us.wrapping_mul(2 as libc::c_int as
                                        libc::c_ulonglong).wrapping_div(1000
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulonglong)
                    as u32_0
        } else if avg_us > 10000 as libc::c_int as libc::c_ulonglong {
            (*afl).fsrv.exec_tmout =
                avg_us.wrapping_mul(3 as libc::c_int as
                                        libc::c_ulonglong).wrapping_div(1000
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulonglong)
                    as u32_0
        } else {
            (*afl).fsrv.exec_tmout =
                avg_us.wrapping_mul(5 as libc::c_int as
                                        libc::c_ulonglong).wrapping_div(1000
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulonglong)
                    as u32_0
        }
        (*afl).fsrv.exec_tmout =
            ({
                 let mut _a: u32_0 = (*afl).fsrv.exec_tmout;
                 let mut _b: libc::c_ulonglong =
                     max_us.wrapping_div(1000 as libc::c_int as
                                             libc::c_ulonglong);
                 if _a as libc::c_ulonglong > _b {
                     _a as libc::c_ulonglong
                 } else { _b }
             }) as u32_0;
        (*afl).fsrv.exec_tmout =
            (*afl).fsrv.exec_tmout.wrapping_add(20 as libc::c_int as
                                                    libc::c_uint).wrapping_div(20
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint).wrapping_mul(20
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint);
        if (*afl).fsrv.exec_tmout > 1000 as libc::c_int as libc::c_uint {
            (*afl).fsrv.exec_tmout = 1000 as libc::c_int as u32_0
        }
        printf(b"\x1b[1;94m[*] \x1b[0mNo -t option specified, so I\'ll use exec timeout of %u ms.\x00"
                   as *const u8 as *const libc::c_char,
               (*afl).fsrv.exec_tmout);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        (*afl).timeout_given = 1 as libc::c_int as u8_0
    } else if (*afl).timeout_given as libc::c_int == 3 as libc::c_int {
        printf(b"\x1b[1;94m[*] \x1b[0mApplying timeout settings from resumed session (%u ms).\x00"
                   as *const u8 as *const libc::c_char,
               (*afl).fsrv.exec_tmout);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    /* In dumb mode, re-running every timing out test case with a generous time
     limit is very expensive, so let's select a more conservative default. */
    if (*afl).dumb_mode as libc::c_int != 0 &&
           (*afl).afl_env.afl_hang_tmout.is_null() {
        (*afl).hang_tmout =
            ({
                 let mut _a: libc::c_int = 1000 as libc::c_int;
                 let mut _b: libc::c_uint =
                     (*afl).fsrv.exec_tmout.wrapping_mul(2 as libc::c_int as
                                                             libc::c_uint).wrapping_add(100
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint);
                 if (_a as libc::c_uint) < _b {
                     _a as libc::c_uint
                 } else { _b }
             })
    }
    printf(b"\x1b[1;92m[+] \x1b[0mAll set and ready to roll!\x00" as *const u8
               as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
}
