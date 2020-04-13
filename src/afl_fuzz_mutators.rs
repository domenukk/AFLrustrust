use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type cmp_map;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn random() -> libc::c_long;
    #[no_mangle]
    fn srandom(__seed: libc::c_uint);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn write_to_testcase(_: *mut afl_state_t, _: *mut libc::c_void, _: u32_0);
    #[no_mangle]
    fn run_target(_: *mut afl_state_t, fsrv: *mut afl_forkserver_t, _: u32_0)
     -> u8_0;
    #[no_mangle]
    fn show_stats(_: *mut afl_state_t);
    #[no_mangle]
    fn update_bitmap_score(_: *mut afl_state_t, _: *mut queue_entry);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    /* Unsafe Describe integer. The buf sizes are not checked.
   This is unsafe but fast.
   Will return buf for convenience. */
    #[no_mangle]
    fn u_stringify_int(buf: *mut u8_0, val: u64_0) -> *mut u8_0;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char)
     -> *mut libc::c_void;
    #[no_mangle]
    fn dlerror() -> *mut libc::c_char;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
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
pub struct extra_data {
    pub data: *mut u8_0,
    pub len: u32_0,
    pub hit_cnt: u32_0,
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
pub type MOpt_globals_t = MOpt_globals;
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
pub type afl_env_vars_t = afl_env_vars;
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
pub type C2RustUnnamed = libc::c_uint;
pub const FAULT_NOBITS: C2RustUnnamed = 5;
pub const FAULT_NOINST: C2RustUnnamed = 4;
pub const FAULT_ERROR: C2RustUnnamed = 3;
pub const FAULT_CRASH: C2RustUnnamed = 2;
pub const FAULT_TMOUT: C2RustUnnamed = 1;
pub const FAULT_NONE: C2RustUnnamed = 0;
#[inline]
unsafe extern "C" fn rand_below(mut afl: *mut afl_state_t, mut limit: u32_0)
 -> u32_0 {
    let fresh0 = (*afl).rand_cnt;
    (*afl).rand_cnt = (*afl).rand_cnt.wrapping_sub(1);
    if fresh0 == 0 && (*afl).fixed_seed == 0 {
        let mut _len: u32_0 =
            ::std::mem::size_of::<[u32_0; 2]>() as libc::c_ulong as u32_0;
        let mut _res: s32 =
            read((*afl).fsrv.dev_urandom_fd,
                 &mut (*afl).rand_seed as *mut [u32_0; 2] as
                     *mut libc::c_void, _len as size_t) as s32;
        if _res as libc::c_uint != _len {
            if _res < 0 as libc::c_int {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort read from %s\x00"
                           as *const u8 as *const libc::c_char,
                       b"/dev/urandom\x00" as *const u8 as
                           *const libc::c_char);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"include/afl-fuzz.h\x00" as *const u8 as
                           *const libc::c_char, 948 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            } else {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort read from %s\x00"
                           as *const u8 as *const libc::c_char,
                       b"/dev/urandom\x00" as *const u8 as
                           *const libc::c_char);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"include/afl-fuzz.h\x00" as *const u8 as
                           *const libc::c_char, 948 as libc::c_int);
                exit(1 as libc::c_int);
            }
        }
        srandom((*afl).rand_seed[0 as libc::c_int as usize]);
        (*afl).rand_cnt =
            ((10000 as libc::c_int / 2 as libc::c_int) as
                 libc::c_uint).wrapping_add((*afl).rand_seed[1 as libc::c_int
                                                                 as
                                                                 usize].wrapping_rem(10000
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint))
    }
    return (random() % limit as libc::c_long) as u32_0;
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
        let fresh1 = len;
        len = len.wrapping_sub(1);
        if !(fresh1 != 0) { break ; }
        let fresh2 = data;
        data = data.offset(1);
        let mut k1: u64_0 = *fresh2;
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
#[no_mangle]
pub unsafe extern "C" fn setup_custom_mutator(mut afl: *mut afl_state_t) {
    /* Try mutator library first */
    let mut fn_0: *mut u8_0 = (*afl).afl_env.afl_custom_mutator_library;
    if !fn_0.is_null() {
        if (*afl).limit_time_sig != 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMOpt and custom mutator are mutually exclusive. We accept pull requests that integrates MOpt with the optional mutators (custom/radamsa/redquenn/...).\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-mutators.c\x00" as *const u8 as
                       *const libc::c_char, 45 as libc::c_int);
            exit(1 as libc::c_int);
        }
        load_custom_mutator(afl, fn_0 as *const libc::c_char);
        return
    }
    /* Try Python module */
    if !(*afl).afl_env.afl_python_module.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mYour AFL binary was built without Python support\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-mutators.c\x00" as *const u8 as
                   *const libc::c_char, 71 as libc::c_int);
        exit(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn destroy_custom_mutator(mut afl: *mut afl_state_t) {
    if !(*afl).mutator.is_null() {
        (*(*afl).mutator).afl_custom_deinit.expect("non-null function pointer")((*(*afl).mutator).data);
        if !(*(*afl).mutator).dh.is_null() { dlclose((*(*afl).mutator).dh); }
        if !(*(*afl).mutator).pre_save_buf.is_null() {
            DFL_ck_free((*(*afl).mutator).pre_save_buf as *mut libc::c_void);
            (*(*afl).mutator).pre_save_buf = 0 as *mut u8_0;
            (*(*afl).mutator).pre_save_size = 0 as libc::c_int as size_t
        }
        DFL_ck_free((*afl).mutator as *mut libc::c_void);
        (*afl).mutator = 0 as *mut custom_mutator
    };
}
/*
   american fuzzy lop++ - custom mutators related routines
   -------------------------------------------------------

   Originally written by Shengtuo Hu

   Now maintained by  Marc Heuse <mh@mh-sec.de>,
                        Heiko Eißfeldt <heiko.eissfeldt@hexco.de> and
                        Andrea Fioraldi <andreafioraldi@gmail.com>
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
#[no_mangle]
pub unsafe extern "C" fn load_custom_mutator(mut afl: *mut afl_state_t,
                                             mut fn_0: *const libc::c_char) {
    let mut dh: *mut libc::c_void = 0 as *mut libc::c_void;
    (*afl).mutator =
        DFL_ck_alloc(::std::mem::size_of::<custom_mutator>() as libc::c_ulong
                         as u32_0) as *mut custom_mutator;
    (*(*afl).mutator).pre_save_buf = 0 as *mut u8_0;
    (*(*afl).mutator).pre_save_size = 0 as libc::c_int as size_t;
    (*(*afl).mutator).name = fn_0;
    printf(b"\x1b[1;94m[*] \x1b[0mLoading custom mutator library from \'%s\'...\x00"
               as *const u8 as *const libc::c_char, fn_0);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    dh = dlopen(fn_0, 0x2 as libc::c_int);
    if dh.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0m%s\x00"
                   as *const u8 as *const libc::c_char, dlerror());
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-mutators.c\x00" as *const u8 as
                   *const libc::c_char, 110 as libc::c_int);
        exit(1 as libc::c_int);
    }
    (*(*afl).mutator).dh = dh;
    /* Mutator */
  /* "afl_custom_init", required */
    (*(*afl).mutator).afl_custom_init =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut afl_state_t,
                                                            _: libc::c_uint)
                                           ->
                                               *mut libc::c_void>>(dlsym(dh,
                                                                         b"afl_custom_init\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char));
    if (*(*afl).mutator).afl_custom_init.is_none() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mSymbol \'afl_custom_init\' not found.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-mutators.c\x00" as *const u8 as
                   *const libc::c_char, 117 as libc::c_int);
        exit(1 as libc::c_int);
    }
    /* "afl_custom_deinit", required */
    (*(*afl).mutator).afl_custom_deinit =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void)
                                           ->
                                               ()>>(dlsym(dh,
                                                          b"afl_custom_deinit\x00"
                                                              as *const u8 as
                                                              *const libc::c_char));
    if (*(*afl).mutator).afl_custom_deinit.is_none() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mSymbol \'afl_custom_deinit\' not found.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-mutators.c\x00" as *const u8 as
                   *const libc::c_char, 122 as libc::c_int);
        exit(1 as libc::c_int);
    }
    /* "afl_custom_fuzz" or "afl_custom_mutator", required */
    (*(*afl).mutator).afl_custom_fuzz =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _: *mut u8_0,
                                                            _: size_t,
                                                            _: *mut *mut u8_0,
                                                            _: *mut u8_0,
                                                            _: size_t,
                                                            _: size_t)
                                           ->
                                               size_t>>(dlsym(dh,
                                                              b"afl_custom_fuzz\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char));
    if (*(*afl).mutator).afl_custom_fuzz.is_none() {
        /* Try "afl_custom_mutator" for backward compatibility */
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mSymbol \'afl_custom_fuzz\' not found. Try \'afl_custom_mutator\'.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        (*(*afl).mutator).afl_custom_fuzz =
            ::std::mem::transmute::<*mut libc::c_void,
                                    Option<unsafe extern "C" fn(_:
                                                                    *mut libc::c_void,
                                                                _: *mut u8_0,
                                                                _: size_t,
                                                                _:
                                                                    *mut *mut u8_0,
                                                                _: *mut u8_0,
                                                                _: size_t,
                                                                _: size_t)
                                               ->
                                                   size_t>>(dlsym(dh,
                                                                  b"afl_custom_mutator\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char));
        if (*(*afl).mutator).afl_custom_fuzz.is_none() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mSymbol \'afl_custom_mutator\' not found.\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-mutators.c\x00" as *const u8 as
                       *const libc::c_char, 133 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    /* "afl_custom_pre_save", optional */
    (*(*afl).mutator).afl_custom_pre_save =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _: *mut u8_0,
                                                            _: size_t,
                                                            _: *mut *mut u8_0)
                                           ->
                                               size_t>>(dlsym(dh,
                                                              b"afl_custom_pre_save\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char));
    if (*(*afl).mutator).afl_custom_pre_save.is_none() {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mSymbol \'afl_custom_pre_save\' not found.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    let mut notrim: u8_0 = 0 as libc::c_int as u8_0;
    /* "afl_custom_init_trim", optional */
    (*(*afl).mutator).afl_custom_init_trim =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _: *mut u8_0,
                                                            _: size_t)
                                           ->
                                               s32>>(dlsym(dh,
                                                           b"afl_custom_init_trim\x00"
                                                               as *const u8 as
                                                               *const libc::c_char));
    if (*(*afl).mutator).afl_custom_init_trim.is_none() {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mSymbol \'afl_custom_init_trim\' not found.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    /* "afl_custom_trim", optional */
    (*(*afl).mutator).afl_custom_trim =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _: *mut *mut u8_0)
                                           ->
                                               size_t>>(dlsym(dh,
                                                              b"afl_custom_trim\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char));
    if (*(*afl).mutator).afl_custom_trim.is_none() {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mSymbol \'afl_custom_trim\' not found.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    /* "afl_custom_post_trim", optional */
    (*(*afl).mutator).afl_custom_post_trim =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _: u8_0)
                                           ->
                                               s32>>(dlsym(dh,
                                                           b"afl_custom_post_trim\x00"
                                                               as *const u8 as
                                                               *const libc::c_char));
    if (*(*afl).mutator).afl_custom_post_trim.is_none() {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mSymbol \'afl_custom_post_trim\' not found.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    if notrim != 0 {
        (*(*afl).mutator).afl_custom_init_trim = None;
        (*(*afl).mutator).afl_custom_trim = None;
        (*(*afl).mutator).afl_custom_post_trim = None;
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mCustom mutator does not implement all three trim APIs, standard trimming will be used.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    /* "afl_custom_havoc_mutation", optional */
    (*(*afl).mutator).afl_custom_havoc_mutation =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _: *mut u8_0,
                                                            _: size_t,
                                                            _: *mut *mut u8_0,
                                                            _: size_t)
                                           ->
                                               size_t>>(dlsym(dh,
                                                              b"afl_custom_havoc_mutation\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char));
    if (*(*afl).mutator).afl_custom_havoc_mutation.is_none() {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mSymbol \'afl_custom_havoc_mutation\' not found.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    /* "afl_custom_havoc_mutation", optional */
    (*(*afl).mutator).afl_custom_havoc_mutation_probability =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void)
                                           ->
                                               u8_0>>(dlsym(dh,
                                                            b"afl_custom_havoc_mutation_probability\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char));
    if (*(*afl).mutator).afl_custom_havoc_mutation_probability.is_none() {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mSymbol \'afl_custom_havoc_mutation_probability\' not found.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    /* "afl_custom_queue_get", optional */
    (*(*afl).mutator).afl_custom_queue_get =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _: *const u8_0)
                                           ->
                                               u8_0>>(dlsym(dh,
                                                            b"afl_custom_queue_get\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char));
    if (*(*afl).mutator).afl_custom_queue_get.is_none() {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mSymbol \'afl_custom_queue_get\' not found.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    /* "afl_custom_queue_new_entry", optional */
    (*(*afl).mutator).afl_custom_queue_new_entry =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _: *const u8_0,
                                                            _: *const u8_0)
                                           ->
                                               ()>>(dlsym(dh,
                                                          b"afl_custom_queue_new_entry\x00"
                                                              as *const u8 as
                                                              *const libc::c_char));
    if (*(*afl).mutator).afl_custom_queue_new_entry.is_none() {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mSymbol \'afl_custom_queue_new_entry\' not found\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    printf(b"\x1b[1;92m[+] \x1b[0mCustom mutator \'%s\' installed successfully.\x00"
               as *const u8 as *const libc::c_char, fn_0);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    /* Initialize the custom mutator */
    if (*(*afl).mutator).afl_custom_init.is_some() {
        (*(*afl).mutator).data =
            (*(*afl).mutator).afl_custom_init.expect("non-null function pointer")(afl,
                                                                                  rand_below(afl,
                                                                                             0xffffffff
                                                                                                 as
                                                                                                 libc::c_uint))
    };
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
#[no_mangle]
pub unsafe extern "C" fn trim_case_custom(mut afl: *mut afl_state_t,
                                          mut q: *mut queue_entry,
                                          mut in_buf: *mut u8_0) -> u8_0 {
    let mut current_block: u64;
    let mut needs_write: u8_0 = 0 as libc::c_int as u8_0;
    let mut fault: u8_0 = 0 as libc::c_int as u8_0;
    let mut trim_exec: u32_0 = 0 as libc::c_int as u32_0;
    let mut orig_len: u32_0 = (*q).len;
    let mut val_buf: [u8_0; 16] = [0; 16];
    (*afl).stage_name = (*afl).stage_name_buf.as_mut_ptr();
    (*afl).bytes_trim_in =
        ((*afl).bytes_trim_in as
             libc::c_ulonglong).wrapping_add((*q).len as libc::c_ulonglong) as
            u64_0 as u64_0;
    /* Initialize trimming in the custom mutator */
    (*afl).stage_cur = 0 as libc::c_int;
    (*afl).stage_max =
        (*(*afl).mutator).afl_custom_init_trim.expect("non-null function pointer")((*(*afl).mutator).data,
                                                                                   in_buf,
                                                                                   (*q).len
                                                                                       as
                                                                                       size_t);
    if (*afl).stage_max < 0 as libc::c_int {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mcustom_init_trim error ret: %d\x00"
                   as *const u8 as *const libc::c_char, (*afl).stage_max);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-mutators.c\x00" as *const u8 as
                   *const libc::c_char, 217 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if (*afl).not_on_tty as libc::c_int != 0 &&
           (*afl).debug as libc::c_int != 0 {
        printf(b"[Custom Trimming] START: Max %d iterations, %u bytes\x00" as
                   *const u8 as *const libc::c_char, (*afl).stage_max,
               (*q).len);
    }
    loop  {
        if !((*afl).stage_cur < (*afl).stage_max) {
            current_block = 11777552016271000781;
            break ;
        }
        let mut retbuf: *mut u8_0 = 0 as *mut u8_0;
        sprintf((*afl).stage_name_buf.as_mut_ptr() as *mut libc::c_char,
                b"ptrim %s\x00" as *const u8 as *const libc::c_char,
                u_stringify_int(val_buf.as_mut_ptr(), trim_exec as u64_0));
        let mut cksum: u32_0 = 0;
        let mut retlen: size_t =
            (*(*afl).mutator).afl_custom_trim.expect("non-null function pointer")((*(*afl).mutator).data,
                                                                                  &mut retbuf);
        if retbuf.is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mcustom_trim failed (ret %zd)\x00"
                       as *const u8 as *const libc::c_char, retlen);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-mutators.c\x00" as *const u8 as
                       *const libc::c_char, 234 as libc::c_int);
            exit(1 as libc::c_int);
        } else {
            if retlen > orig_len as libc::c_ulong {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mTrimmed data returned by custom mutator is larger than original data\x00"
                           as *const u8 as *const libc::c_char);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-mutators.c\x00" as *const u8 as
                           *const libc::c_char, 238 as libc::c_int);
                exit(1 as libc::c_int);
            }
        }
        write_to_testcase(afl, retbuf as *mut libc::c_void, retlen as u32_0);
        fault = run_target(afl, &mut (*afl).fsrv, (*afl).fsrv.exec_tmout);
        (*afl).trim_execs = (*afl).trim_execs.wrapping_add(1);
        if (*afl).stop_soon as libc::c_int != 0 ||
               fault as libc::c_int == FAULT_ERROR as libc::c_int {
            current_block = 14904789583098922708;
            break ;
        }
        cksum =
            hash32((*afl).fsrv.trace_bits as *const libc::c_void,
                   (*afl).fsrv.map_size, 0xa5b35705 as libc::c_uint);
        if cksum == (*q).exec_cksum {
            (*q).len = retlen as u32_0;
            memcpy(in_buf as *mut libc::c_void, retbuf as *const libc::c_void,
                   retlen);
            /* Let's save a clean trace, which will be needed by
         update_bitmap_score once we're done with the trimming stuff. */
            if needs_write == 0 {
                needs_write = 1 as libc::c_int as u8_0;
                memcpy((*afl).clean_trace_custom.as_mut_ptr() as
                           *mut libc::c_void,
                       (*afl).fsrv.trace_bits as *const libc::c_void,
                       (*afl).fsrv.map_size as libc::c_ulong);
            }
            /* Tell the custom mutator that the trimming was successful */
            (*afl).stage_cur =
                (*(*afl).mutator).afl_custom_post_trim.expect("non-null function pointer")((*(*afl).mutator).data,
                                                                                           1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               u8_0);
            if (*afl).not_on_tty as libc::c_int != 0 &&
                   (*afl).debug as libc::c_int != 0 {
                printf(b"[Custom Trimming] SUCCESS: %d/%d iterations (now at %u bytes)\x00"
                           as *const u8 as *const libc::c_char,
                       (*afl).stage_cur, (*afl).stage_max, (*q).len);
            }
        } else {
            /* Tell the custom mutator that the trimming was unsuccessful */
            (*afl).stage_cur =
                (*(*afl).mutator).afl_custom_post_trim.expect("non-null function pointer")((*(*afl).mutator).data,
                                                                                           0
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               u8_0);
            if (*afl).stage_cur < 0 as libc::c_int {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mError ret in custom_post_trim: %d\x00"
                           as *const u8 as *const libc::c_char,
                       (*afl).stage_cur);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-mutators.c\x00" as *const u8 as
                           *const libc::c_char, 279 as libc::c_int);
                exit(1 as libc::c_int);
            }
            if (*afl).not_on_tty as libc::c_int != 0 &&
                   (*afl).debug as libc::c_int != 0 {
                printf(b"[Custom Trimming] FAILURE: %d/%d iterations\x00" as
                           *const u8 as *const libc::c_char, (*afl).stage_cur,
                       (*afl).stage_max);
            }
        }
        /* Since this can be slow, update the screen every now and then. */
        let fresh3 = trim_exec;
        trim_exec = trim_exec.wrapping_add(1);
        if fresh3.wrapping_rem((*afl).stats_update_freq) == 0 {
            show_stats(afl);
        }
    }
    match current_block {
        11777552016271000781 => {
            if (*afl).not_on_tty as libc::c_int != 0 &&
                   (*afl).debug as libc::c_int != 0 {
                printf(b"[Custom Trimming] DONE: %u bytes -> %u bytes\x00" as
                           *const u8 as *const libc::c_char, orig_len,
                       (*q).len);
            }
            /* If we have made changes to in_buf, we also need to update the on-disk
     version of the test case. */
            if needs_write != 0 {
                let mut fd: s32 = 0; /* ignore errors */
                unlink((*q).fname as *const libc::c_char);
                fd =
                    open((*q).fname as *const libc::c_char,
                         0o1 as libc::c_int | 0o100 as libc::c_int |
                             0o200 as libc::c_int, 0o600 as libc::c_int);
                if fd < 0 as libc::c_int {
                    fflush(stdout);
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                               as *const u8 as *const libc::c_char,
                           (*q).fname);
                    printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz-mutators.c\x00" as *const u8 as
                               *const libc::c_char, 306 as libc::c_int);
                    printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                               *const u8 as *const libc::c_char,
                           strerror(*__errno_location()));
                    exit(1 as libc::c_int);
                }
                let mut _len: u32_0 = (*q).len;
                let mut _res: s32 =
                    write(fd, in_buf as *const libc::c_void, _len as size_t)
                        as s32;
                if _res as libc::c_uint != _len {
                    if _res < 0 as libc::c_int {
                        fflush(stdout);
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                                   as *const u8 as *const libc::c_char,
                               (*q).fname);
                        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-fuzz-mutators.c\x00" as *const u8 as
                                   *const libc::c_char, 308 as libc::c_int);
                        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00"
                                   as *const u8 as *const libc::c_char,
                               strerror(*__errno_location()));
                        exit(1 as libc::c_int);
                    } else {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                                   as *const u8 as *const libc::c_char,
                               (*q).fname);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-fuzz-mutators.c\x00" as *const u8 as
                                   *const libc::c_char, 308 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                }
                close(fd);
                memcpy((*afl).fsrv.trace_bits as *mut libc::c_void,
                       (*afl).clean_trace_custom.as_mut_ptr() as
                           *const libc::c_void,
                       (*afl).fsrv.map_size as libc::c_ulong);
                update_bitmap_score(afl, q);
            }
        }
        _ => { }
    }
    (*afl).bytes_trim_out =
        ((*afl).bytes_trim_out as
             libc::c_ulonglong).wrapping_add((*q).len as libc::c_ulonglong) as
            u64_0 as u64_0;
    return fault;
}
