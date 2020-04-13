use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type cmp_map;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
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
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn symlink(__from: *const libc::c_char, __to: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    /* path to documentation dir        */
    /* Get unix time in milliseconds */
    #[no_mangle]
    fn get_cur_time() -> u64_0;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn minimize_bits(_: *mut afl_state_t, _: *mut u8_0, _: *mut u8_0);
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
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
pub const POWER_SCHEDULES_NUM: C2RustUnnamed = 8;
pub const RARE: C2RustUnnamed = 7;
pub const MMOPT: C2RustUnnamed = 6;
pub const EXPLOIT: C2RustUnnamed = 5;
pub const QUAD: C2RustUnnamed = 4;
pub const LIN: C2RustUnnamed = 3;
pub const COE: C2RustUnnamed = 2;
pub const FAST: C2RustUnnamed = 1;
pub const EXPLORE: C2RustUnnamed = 0;
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
/* In non-debug mode, we just do straightforward aliasing of the above functions
   to user-visible names such as ck_alloc(). */
/* _WANT_ORIGINAL_AFL_ALLOC */
/* This function calculates the next power of 2 greater or equal its argument.
 @return The rounded up power of 2 (if no overflow) or 0 on overflow.
*/
#[inline]
unsafe extern "C" fn next_pow2(mut in_0: size_t) -> size_t {
    if in_0 == 0 as libc::c_int as libc::c_ulong ||
           in_0 > -(1 as libc::c_int) as size_t {
        return 0 as libc::c_int as size_t
    } /* avoid undefined behaviour under-/overflow */
    let mut out: size_t =
        in_0.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    out |= out >> 1 as libc::c_int;
    out |= out >> 2 as libc::c_int;
    out |= out >> 4 as libc::c_int;
    out |= out >> 8 as libc::c_int;
    out |= out >> 16 as libc::c_int;
    return out.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
/*
   american fuzzy lop++ - queue relates routines
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
/* Mark deterministic checks as done for a particular queue entry. We use the
   .state file to avoid repeating deterministic fuzzing when resuming aborted
   scans. */
#[no_mangle]
pub unsafe extern "C" fn mark_as_det_done(mut afl: *mut afl_state_t,
                                          mut q: *mut queue_entry) {
    let mut fn_0: [u8_0; 4096] = [0; 4096];
    let mut fd: s32 = 0;
    snprintf(fn_0.as_mut_ptr() as *mut libc::c_char,
             4096 as libc::c_int as libc::c_ulong,
             b"%s/queue/.state/deterministic_done/%s\x00" as *const u8 as
                 *const libc::c_char, (*afl).out_dir,
             strrchr((*q).fname as *const libc::c_char,
                     '/' as i32).offset(1 as libc::c_int as isize));
    fd =
        open(fn_0.as_mut_ptr() as *const libc::c_char,
             0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
             0o600 as libc::c_int);
    if fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, fn_0.as_mut_ptr());
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-queue.c\x00" as *const u8 as
                   *const libc::c_char, 41 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    close(fd);
    (*q).passed_det = 1 as libc::c_int as u8_0;
}
/* Mark as variable. Create symlinks if possible to make it easier to examine
   the files. */
#[no_mangle]
pub unsafe extern "C" fn mark_as_variable(mut afl: *mut afl_state_t,
                                          mut q: *mut queue_entry) {
    let mut fn_0: [u8_0; 4096] = [0; 4096];
    let mut ldest: [u8_0; 4096] = [0; 4096];
    let mut fn_name: *mut u8_0 =
        strrchr((*q).fname as *const libc::c_char,
                '/' as i32).offset(1 as libc::c_int as isize) as *mut u8_0;
    sprintf(ldest.as_mut_ptr() as *mut libc::c_char,
            b"../../%s\x00" as *const u8 as *const libc::c_char, fn_name);
    sprintf(fn_0.as_mut_ptr() as *mut libc::c_char,
            b"%s/queue/.state/variable_behavior/%s\x00" as *const u8 as
                *const libc::c_char, (*afl).out_dir, fn_name);
    if symlink(ldest.as_mut_ptr() as *const libc::c_char,
               fn_0.as_mut_ptr() as *const libc::c_char) != 0 {
        let mut fd: s32 =
            open(fn_0.as_mut_ptr() as *const libc::c_char,
                 0o1 as libc::c_int | 0o100 as libc::c_int |
                     0o200 as libc::c_int, 0o600 as libc::c_int);
        if fd < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                       as *const u8 as *const libc::c_char,
                   fn_0.as_mut_ptr());
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-queue.c\x00" as *const u8 as
                       *const libc::c_char, 64 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        close(fd);
    }
    (*q).var_behavior = 1 as libc::c_int as u8_0;
}
/* Mark / unmark as redundant (edge-only). This is not used for restoring state,
   but may be useful for post-processing datasets. */
#[no_mangle]
pub unsafe extern "C" fn mark_as_redundant(mut afl: *mut afl_state_t,
                                           mut q: *mut queue_entry,
                                           mut state: u8_0) {
    let mut fn_0: [u8_0; 4096] = [0; 4096];
    if state as libc::c_int == (*q).fs_redundant as libc::c_int { return }
    (*q).fs_redundant = state;
    sprintf(fn_0.as_mut_ptr() as *mut libc::c_char,
            b"%s/queue/.state/redundant_edges/%s\x00" as *const u8 as
                *const libc::c_char, (*afl).out_dir,
            strrchr((*q).fname as *const libc::c_char,
                    '/' as i32).offset(1 as libc::c_int as isize));
    if state != 0 {
        let mut fd: s32 = 0;
        fd =
            open(fn_0.as_mut_ptr() as *const libc::c_char,
                 0o1 as libc::c_int | 0o100 as libc::c_int |
                     0o200 as libc::c_int, 0o600 as libc::c_int);
        if fd < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                       as *const u8 as *const libc::c_char,
                   fn_0.as_mut_ptr());
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-queue.c\x00" as *const u8 as
                       *const libc::c_char, 92 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        close(fd);
    } else if unlink(fn_0.as_mut_ptr() as *const libc::c_char) != 0 {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to remove \'%s\'\x00"
                   as *const u8 as *const libc::c_char, fn_0.as_mut_ptr());
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-queue.c\x00" as *const u8 as
                   *const libc::c_char, 97 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    };
}
/* Append new test case to the queue. */
#[no_mangle]
pub unsafe extern "C" fn add_to_queue(mut afl: *mut afl_state_t,
                                      mut fname: *mut u8_0, mut len: u32_0,
                                      mut passed_det: u8_0) {
    let mut q: *mut queue_entry =
        DFL_ck_alloc(::std::mem::size_of::<queue_entry>() as libc::c_ulong as
                         u32_0) as *mut queue_entry;
    (*q).fname = fname;
    (*q).len = len;
    (*q).depth =
        (*afl).cur_depth.wrapping_add(1 as libc::c_int as libc::c_uint) as
            u64_0;
    (*q).passed_det = passed_det;
    (*q).n_fuzz = 1 as libc::c_int as u64_0;
    (*q).trace_mini = 0 as *mut u8_0;
    if (*q).depth > (*afl).max_depth as libc::c_ulonglong {
        (*afl).max_depth = (*q).depth as u32_0
    }
    if !(*afl).queue_top.is_null() {
        (*(*afl).queue_top).next = q;
        (*afl).queue_top = q
    } else {
        (*afl).queue_top = q;
        (*afl).queue = (*afl).queue_top;
        (*afl).q_prev100 = (*afl).queue
    }
    (*afl).queued_paths = (*afl).queued_paths.wrapping_add(1);
    (*afl).pending_not_fuzzed = (*afl).pending_not_fuzzed.wrapping_add(1);
    (*afl).cycles_wo_finds = 0 as libc::c_int as u64_0;
    if (*afl).queued_paths.wrapping_rem(100 as libc::c_int as libc::c_uint) ==
           0 {
        (*(*afl).q_prev100).next_100 = q;
        (*afl).q_prev100 = q
    }
    (*afl).last_path_time = get_cur_time();
    if !(*afl).mutator.is_null() &&
           (*(*afl).mutator).afl_custom_queue_new_entry.is_some() {
        let mut fname_orig: *mut u8_0 = 0 as *mut u8_0;
        /* At the initialization stage, queue_cur is NULL */
        if !(*afl).queue_cur.is_null() {
            fname_orig = (*(*afl).queue_cur).fname
        }
        (*(*afl).mutator).afl_custom_queue_new_entry.expect("non-null function pointer")((*(*afl).mutator).data,
                                                                                         fname,
                                                                                         fname_orig);
    };
}
/* Destroy the entire queue. */
#[no_mangle]
pub unsafe extern "C" fn destroy_queue(mut afl: *mut afl_state_t) {
    let mut q: *mut queue_entry = (*afl).queue;
    let mut n: *mut queue_entry = 0 as *mut queue_entry;
    while !q.is_null() {
        n = (*q).next;
        DFL_ck_free((*q).fname as *mut libc::c_void);
        DFL_ck_free((*q).trace_mini as *mut libc::c_void);
        DFL_ck_free(q as *mut libc::c_void);
        q = n
    };
}
/* When we bump into a new path, we call this to see if the path appears
   more "favorable" than any of the existing ones. The purpose of the
   "favorables" is to have a minimal set of paths that trigger all the bits
   seen in the bitmap so far, and focus on fuzzing them at the expense of
   the rest.

   The first step of the process is to maintain a list of afl->top_rated[]
   entries for every byte in the bitmap. We win that slot if there is no
   previous contender, or if the contender has a more favorable speed x size
   factor. */
#[no_mangle]
pub unsafe extern "C" fn update_bitmap_score(mut afl: *mut afl_state_t,
                                             mut q: *mut queue_entry) {
    let mut i: u32_0 = 0;
    let mut fav_factor: u64_0 = 0;
    let mut fuzz_p2: u64_0 = next_pow2((*q).n_fuzz as size_t) as u64_0;
    if (*afl).schedule as libc::c_int == MMOPT as libc::c_int ||
           (*afl).schedule as libc::c_int == RARE as libc::c_int ||
           (*afl).fixed_seed as libc::c_int != 0 {
        fav_factor = ((*q).len << 2 as libc::c_int) as u64_0
    } else {
        fav_factor = (*q).exec_us.wrapping_mul((*q).len as libc::c_ulonglong)
    }
    let mut current_block_19: u64;
    /* For every byte set in afl->fsrv.trace_bits[], see if there is a previous
     winner, and how it compares to us. */
    i = 0 as libc::c_int as u32_0;
    while i < (*afl).fsrv.map_size {
        if *(*afl).fsrv.trace_bits.offset(i as isize) != 0 {
            if !(*afl).top_rated[i as usize].is_null() {
                /* Faster-executing or smaller test cases are favored. */
                let mut top_rated_fav_factor: u64_0 = 0;
                let mut top_rated_fuzz_p2: u64_0 =
                    next_pow2((*(*afl).top_rated[i as usize]).n_fuzz as
                                  size_t) as u64_0;
                if (*afl).schedule as libc::c_int == MMOPT as libc::c_int ||
                       (*afl).schedule as libc::c_int == RARE as libc::c_int
                       || (*afl).fixed_seed as libc::c_int != 0 {
                    top_rated_fav_factor =
                        ((*(*afl).top_rated[i as usize]).len <<
                             2 as libc::c_int) as u64_0
                } else {
                    top_rated_fav_factor =
                        (*(*afl).top_rated[i as
                                               usize]).exec_us.wrapping_mul((*(*afl).top_rated[i
                                                                                                   as
                                                                                                   usize]).len
                                                                                as
                                                                                libc::c_ulonglong)
                }
                if fuzz_p2 > top_rated_fuzz_p2 {
                    current_block_19 = 7095457783677275021;
                } else {
                    if fuzz_p2 == top_rated_fuzz_p2 {
                        if fav_factor > top_rated_fav_factor {
                            current_block_19 = 7095457783677275021;
                        } else { current_block_19 = 12349973810996921269; }
                    } else { current_block_19 = 12349973810996921269; }
                    match current_block_19 {
                        7095457783677275021 => { }
                        _ => {
                            if (*afl).schedule as libc::c_int ==
                                   MMOPT as libc::c_int ||
                                   (*afl).schedule as libc::c_int ==
                                       RARE as libc::c_int ||
                                   (*afl).fixed_seed as libc::c_int != 0 {
                                if fav_factor >
                                       ((*(*afl).top_rated[i as usize]).len <<
                                            2 as libc::c_int) as
                                           libc::c_ulonglong {
                                    current_block_19 = 7095457783677275021;
                                } else {
                                    current_block_19 = 13242334135786603907;
                                }
                            } else if fav_factor >
                                          (*(*afl).top_rated[i as
                                                                 usize]).exec_us.wrapping_mul((*(*afl).top_rated[i
                                                                                                                     as
                                                                                                                     usize]).len
                                                                                                  as
                                                                                                  libc::c_ulonglong)
                             {
                                current_block_19 = 7095457783677275021;
                            } else {
                                current_block_19 = 13242334135786603907;
                            }
                            match current_block_19 {
                                7095457783677275021 => { }
                                _ => {
                                    /* Looks like we're going to win. Decrease ref count for the
           previous winner, discard its afl->fsrv.trace_bits[] if necessary. */
                                    (*(*afl).top_rated[i as usize]).tc_ref =
                                        (*(*afl).top_rated[i as
                                                               usize]).tc_ref.wrapping_sub(1);
                                    if (*(*afl).top_rated[i as usize]).tc_ref
                                           == 0 {
                                        DFL_ck_free((*(*afl).top_rated[i as
                                                                           usize]).trace_mini
                                                        as *mut libc::c_void);
                                        (*(*afl).top_rated[i as
                                                               usize]).trace_mini
                                            = 0 as *mut u8_0
                                    }
                                    current_block_19 = 11298138898191919651;
                                }
                            }
                        }
                    }
                }
            } else { current_block_19 = 11298138898191919651; }
            match current_block_19 {
                7095457783677275021 => { }
                _ => {
                    /* Insert ourselves as the new winner. */
                    (*afl).top_rated[i as usize] = q;
                    (*q).tc_ref = (*q).tc_ref.wrapping_add(1);
                    if (*q).trace_mini.is_null() {
                        let mut len: u32_0 =
                            (*afl).fsrv.map_size >> 3 as libc::c_int;
                        if len == 0 as libc::c_int as libc::c_uint {
                            len = 1 as libc::c_int as u32_0
                        }
                        (*q).trace_mini = DFL_ck_alloc(len) as *mut u8_0;
                        minimize_bits(afl, (*q).trace_mini,
                                      (*afl).fsrv.trace_bits);
                    }
                    (*afl).score_changed = 1 as libc::c_int as u8_0
                }
            }
        }
        i = i.wrapping_add(1)
    };
}
/* The second part of the mechanism discussed above is a routine that
   goes over afl->top_rated[] entries, and then sequentially grabs winners for
   previously-unseen bytes (temp_v) and marks them as favored, at least
   until the next run. The favored entries are given more air time during
   all fuzzing steps. */
#[no_mangle]
pub unsafe extern "C" fn cull_queue(mut afl: *mut afl_state_t) {
    let mut q: *mut queue_entry = 0 as *mut queue_entry;
    let mut len: u32_0 = (*afl).fsrv.map_size >> 3 as libc::c_int;
    let mut i: u32_0 = 0;
    let mut temp_v: [u8_0; 8192] = [0; 8192];
    if len == 0 as libc::c_int as libc::c_uint {
        len = 1 as libc::c_int as u32_0
    }
    if (*afl).dumb_mode as libc::c_int != 0 || (*afl).score_changed == 0 {
        return
    }
    (*afl).score_changed = 0 as libc::c_int as u8_0;
    memset(temp_v.as_mut_ptr() as *mut libc::c_void, 255 as libc::c_int,
           len as libc::c_ulong);
    (*afl).queued_favored = 0 as libc::c_int as u32_0;
    (*afl).pending_favored = 0 as libc::c_int as u32_0;
    q = (*afl).queue;
    while !q.is_null() {
        (*q).favored = 0 as libc::c_int as u8_0;
        q = (*q).next
    }
    /* Let's see if anything in the bitmap isn't captured in temp_v.
     If yes, and if it has a afl->top_rated[] contender, let's use it. */
    i = 0 as libc::c_int as u32_0;
    while i < (*afl).fsrv.map_size {
        if !(*afl).top_rated[i as usize].is_null() &&
               temp_v[(i >> 3 as libc::c_int) as usize] as libc::c_int &
                   (1 as libc::c_int) <<
                       (i & 7 as libc::c_int as libc::c_uint) != 0 {
            let mut j: u32_0 = len;
            loop 
                 /* Remove all bits belonging to the current entry from temp_v. */
                 {
                let fresh0 = j;
                j = j.wrapping_sub(1);
                if !(fresh0 != 0) { break ; }
                if *(*(*afl).top_rated[i as
                                           usize]).trace_mini.offset(j as
                                                                         isize)
                       != 0 {
                    temp_v[j as usize] =
                        (temp_v[j as usize] as libc::c_int &
                             !(*(*(*afl).top_rated[i as
                                                       usize]).trace_mini.offset(j
                                                                                     as
                                                                                     isize)
                                   as libc::c_int)) as u8_0
                }
            }
            (*(*afl).top_rated[i as usize]).favored =
                1 as libc::c_int as u8_0;
            (*afl).queued_favored = (*afl).queued_favored.wrapping_add(1);
            if (*(*afl).top_rated[i as usize]).fuzz_level ==
                   0 as libc::c_int as libc::c_uint ||
                   (*(*afl).top_rated[i as usize]).was_fuzzed == 0 {
                (*afl).pending_favored =
                    (*afl).pending_favored.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    }
    q = (*afl).queue;
    while !q.is_null() {
        mark_as_redundant(afl, q, ((*q).favored == 0) as libc::c_int as u8_0);
        q = (*q).next
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
/* Python */
/* Queue */
/* Calculate case desirability score to adjust the length of havoc fuzzing.
   A helper function for fuzz_one(). Maybe some of these constants should
   go into config.h. */
#[no_mangle]
pub unsafe extern "C" fn calculate_score(mut afl: *mut afl_state_t,
                                         mut q: *mut queue_entry) -> u32_0 {
    let mut avg_exec_us: u32_0 =
        (*afl).total_cal_us.wrapping_div((*afl).total_cal_cycles) as u32_0;
    let mut avg_bitmap_size: u32_0 =
        (*afl).total_bitmap_size.wrapping_div((*afl).total_bitmap_entries) as
            u32_0;
    let mut perf_score: u32_0 = 100 as libc::c_int as u32_0;
    /* Adjust score based on execution speed of this path, compared to the
     global average. Multiplier ranges from 0.1x to 3x. Fast inputs are
     less expensive to fuzz, so we're giving them more air time. */
    // TODO BUG FIXME: is this really a good idea?
  // This sounds like looking for lost keys under a street light just because
  // the light is better there.
  // Longer execution time means longer work on the input, the deeper in
  // coverage, the better the fuzzing, right? -mh
    if (*afl).schedule as libc::c_int != MMOPT as libc::c_int &&
           (*afl).schedule as libc::c_int != RARE as libc::c_int &&
           (*afl).fixed_seed == 0 {
        if (*q).exec_us as libc::c_double * 0.1f64 >
               avg_exec_us as libc::c_double {
            perf_score = 10 as libc::c_int as u32_0
        } else if (*q).exec_us as libc::c_double * 0.25f64 >
                      avg_exec_us as libc::c_double {
            perf_score = 25 as libc::c_int as u32_0
        } else if (*q).exec_us as libc::c_double * 0.5f64 >
                      avg_exec_us as libc::c_double {
            perf_score = 50 as libc::c_int as u32_0
        } else if (*q).exec_us as libc::c_double * 0.75f64 >
                      avg_exec_us as libc::c_double {
            perf_score = 75 as libc::c_int as u32_0
        } else if (*q).exec_us.wrapping_mul(4 as libc::c_int as
                                                libc::c_ulonglong) <
                      avg_exec_us as libc::c_ulonglong {
            perf_score = 300 as libc::c_int as u32_0
        } else if (*q).exec_us.wrapping_mul(3 as libc::c_int as
                                                libc::c_ulonglong) <
                      avg_exec_us as libc::c_ulonglong {
            perf_score = 200 as libc::c_int as u32_0
        } else if (*q).exec_us.wrapping_mul(2 as libc::c_int as
                                                libc::c_ulonglong) <
                      avg_exec_us as libc::c_ulonglong {
            perf_score = 150 as libc::c_int as u32_0
        }
    }
    /* Adjust score based on bitmap size. The working theory is that better
     coverage translates to better targets. Multiplier from 0.25x to 3x. */
    if (*q).bitmap_size as libc::c_double * 0.3f64 >
           avg_bitmap_size as libc::c_double {
        perf_score =
            (perf_score as
                 libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint)
                as u32_0 as u32_0
    } else if (*q).bitmap_size as libc::c_double * 0.5f64 >
                  avg_bitmap_size as libc::c_double {
        perf_score =
            (perf_score as
                 libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint)
                as u32_0 as u32_0
    } else if (*q).bitmap_size as libc::c_double * 0.75f64 >
                  avg_bitmap_size as libc::c_double {
        perf_score = (perf_score as libc::c_double * 1.5f64) as u32_0
    } else if (*q).bitmap_size.wrapping_mul(3 as libc::c_int as libc::c_uint)
                  < avg_bitmap_size {
        perf_score = (perf_score as libc::c_double * 0.25f64) as u32_0
    } else if (*q).bitmap_size.wrapping_mul(2 as libc::c_int as libc::c_uint)
                  < avg_bitmap_size {
        perf_score = (perf_score as libc::c_double * 0.5f64) as u32_0
    } else if (*q).bitmap_size as libc::c_double * 1.5f64 <
                  avg_bitmap_size as libc::c_double {
        perf_score = (perf_score as libc::c_double * 0.75f64) as u32_0
    }
    /* Adjust score based on handicap. Handicap is proportional to how late
     in the game we learned about this path. Latecomers are allowed to run
     for a bit longer until they catch up with the rest. */
    if (*q).handicap >= 4 as libc::c_int as libc::c_ulonglong {
        perf_score =
            (perf_score as
                 libc::c_uint).wrapping_mul(4 as libc::c_int as libc::c_uint)
                as u32_0 as u32_0;
        (*q).handicap =
            ((*q).handicap as
                 libc::c_ulonglong).wrapping_sub(4 as libc::c_int as
                                                     libc::c_ulonglong) as
                u64_0 as u64_0
    } else if (*q).handicap != 0 {
        perf_score =
            (perf_score as
                 libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint)
                as u32_0 as u32_0;
        (*q).handicap = (*q).handicap.wrapping_sub(1)
    }
    /* Final adjustment based on input depth, under the assumption that fuzzing
     deeper test cases is more likely to reveal stuff that can't be
     discovered with traditional fuzzers. */
    match (*q).depth {
        0 => { }
        4 => {
            perf_score =
                (perf_score as
                     libc::c_uint).wrapping_mul(2 as libc::c_int as
                                                    libc::c_uint) as u32_0 as
                    u32_0
        }
        8 => {
            perf_score =
                (perf_score as
                     libc::c_uint).wrapping_mul(3 as libc::c_int as
                                                    libc::c_uint) as u32_0 as
                    u32_0
        }
        14 => {
            perf_score =
                (perf_score as
                     libc::c_uint).wrapping_mul(4 as libc::c_int as
                                                    libc::c_uint) as u32_0 as
                    u32_0
        }
        _ => {
            perf_score =
                (perf_score as
                     libc::c_uint).wrapping_mul(5 as libc::c_int as
                                                    libc::c_uint) as u32_0 as
                    u32_0
        }
    }
    let mut fuzz: u64_0 = (*q).n_fuzz;
    let mut fuzz_total: u64_0 = 0;
    let mut n_paths: u32_0 = 0;
    let mut fuzz_mu: u32_0 = 0;
    let mut factor: u32_0 = 1 as libc::c_int as u32_0;
    let mut queue_it: *mut queue_entry = 0 as *mut queue_entry;
    match (*afl).schedule as libc::c_int {
        0 => { }
        5 => { factor = (1 as libc::c_int * 32 as libc::c_int) as u32_0 }
        2 => {
            fuzz_total = 0 as libc::c_int as u64_0;
            n_paths = 0 as libc::c_int as u32_0;
            queue_it = (*afl).queue;
            while !queue_it.is_null() {
                fuzz_total =
                    (fuzz_total as
                         libc::c_ulonglong).wrapping_add((*queue_it).n_fuzz)
                        as u64_0 as u64_0;
                n_paths = n_paths.wrapping_add(1);
                queue_it = (*queue_it).next
            }
            if n_paths == 0 {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mQueue state corrupt\x00"
                           as *const u8 as *const libc::c_char);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-queue.c\x00" as *const u8 as
                           *const libc::c_char, 441 as libc::c_int);
                exit(1 as libc::c_int);
            }
            fuzz_mu =
                fuzz_total.wrapping_div(n_paths as libc::c_ulonglong) as
                    u32_0;
            if fuzz <= fuzz_mu as libc::c_ulonglong {
                if (*q).fuzz_level < 16 as libc::c_int as libc::c_uint {
                    factor = ((1 as libc::c_int) << (*q).fuzz_level) as u32_0
                } else {
                    factor = (1 as libc::c_int * 32 as libc::c_int) as u32_0
                }
            } else { factor = 0 as libc::c_int as u32_0 }
        }
        1 => {
            if (*q).fuzz_level < 16 as libc::c_int as libc::c_uint {
                factor =
                    (((1 as libc::c_int) << (*q).fuzz_level) as u32_0 as
                         libc::c_ulonglong).wrapping_div((if fuzz ==
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulonglong
                                                             {
                                                              1 as libc::c_int
                                                                  as
                                                                  libc::c_ulonglong
                                                          } else { fuzz })) as
                        u32_0
            } else {
                factor =
                    ((1 as libc::c_int * 32 as libc::c_int) as
                         libc::c_ulong).wrapping_div((if fuzz ==
                                                             0 as libc::c_int
                                                                 as
                                                                 libc::c_ulonglong
                                                         {
                                                          1 as libc::c_int as
                                                              libc::c_ulong
                                                      } else {
                                                          next_pow2(fuzz as
                                                                        size_t)
                                                      })) as u32_0
            }
        }
        3 => {
            factor =
                ((*q).fuzz_level as
                     libc::c_ulonglong).wrapping_div((if fuzz ==
                                                             0 as libc::c_int
                                                                 as
                                                                 libc::c_ulonglong
                                                         {
                                                          1 as libc::c_int as
                                                              libc::c_ulonglong
                                                      } else { fuzz })) as
                    u32_0
        }
        4 => {
            factor =
                ((*q).fuzz_level.wrapping_mul((*q).fuzz_level) as
                     libc::c_ulonglong).wrapping_div((if fuzz ==
                                                             0 as libc::c_int
                                                                 as
                                                                 libc::c_ulonglong
                                                         {
                                                          1 as libc::c_int as
                                                              libc::c_ulonglong
                                                      } else { fuzz })) as
                    u32_0
        }
        6 => {
            /* -- this was a more complex setup, which is good, but competed with
         -- rare. the simpler algo however is good when rare is not.
        // the newer the entry, the higher the pref_score
        perf_score *= (1 + (double)((double)q->depth /
        (double)afl->queued_paths));
        // with special focus on the last 8 entries
        if (afl->max_depth - q->depth < 8) perf_score *= (1 + ((8 -
        (afl->max_depth - q->depth)) / 5));
      */
      // put focus on the last 5 entries
            if ((*afl).max_depth as
                    libc::c_ulonglong).wrapping_sub((*q).depth) <
                   5 as libc::c_int as libc::c_ulonglong {
                perf_score =
                    (perf_score as
                         libc::c_uint).wrapping_mul(2 as libc::c_int as
                                                        libc::c_uint) as u32_0
                        as u32_0
            }
        }
        7 => {
            // increase the score for every bitmap byte for which this entry
      // is the top contender
            perf_score =
                (perf_score as
                     libc::c_uint).wrapping_add((*q).tc_ref.wrapping_mul(10 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint))
                    as u32_0 as u32_0;
            // the more often fuzz result paths are equal to this queue entry,
      // reduce its value
            perf_score =
                (perf_score as libc::c_double *
                     (1 as libc::c_int as libc::c_double -
                          (*q).n_fuzz as libc::c_double /
                              (*afl).total_execs as libc::c_double)) as u32_0
        }
        _ => {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnknown Power Schedule\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-queue.c\x00" as *const u8 as
                       *const libc::c_char, 499 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
    }
    if factor > (1 as libc::c_int * 32 as libc::c_int) as libc::c_uint {
        factor = (1 as libc::c_int * 32 as libc::c_int) as u32_0
    }
    perf_score =
        (perf_score as
             libc::c_uint).wrapping_mul(factor.wrapping_div(1 as libc::c_int
                                                                as
                                                                libc::c_uint))
            as u32_0 as u32_0;
    // MOpt mode
    if (*afl).limit_time_sig != 0 as libc::c_int &&
           ((*afl).max_depth as libc::c_ulonglong).wrapping_sub((*q).depth) <
               3 as libc::c_int as libc::c_ulonglong {
        perf_score =
            (perf_score as
                 libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint)
                as u32_0 as u32_0
    } else if perf_score < 1 as libc::c_int as libc::c_uint {
        // Add a lower bound to AFLFast's energy assignment strategies
        perf_score = 1 as libc::c_int as u32_0
    }
    /* Make sure that we don't go over limit. */
    if perf_score >
           ((*afl).havoc_max_mult as libc::c_int * 100 as libc::c_int) as
               libc::c_uint {
        perf_score =
            ((*afl).havoc_max_mult as libc::c_int * 100 as libc::c_int) as
                u32_0
    }
    return perf_score;
}
