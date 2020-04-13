use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type cmp_map;
    #[no_mangle]
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn get_afl_env(env: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn maybe_add_auto(_: *mut libc::c_void, _: *mut u8_0, _: u32_0);
    #[no_mangle]
    static mut afl_environment_variables: [*mut libc::c_char; 0];
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
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
/* ^__x86_64__ */
pub type s8 = int8_t;
pub type s16 = int16_t;
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
pub struct list {
    pub element_prealloc_buf: [element_t; 64],
    pub element_prealloc_count: s32,
}
pub type list_t = list;
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
/* Free memory, checking for double free and corrupted heap. When DEBUG_BUILD
   is set, the old memory will be also clobbered with 0xFF. */
#[inline]
unsafe extern "C" fn DFL_ck_free(mut mem: *mut libc::c_void) {
    if mem.is_null() { return }
    free(mem);
}
#[inline]
unsafe extern "C" fn get_head(mut list: *mut list_t) -> *mut element_t {
    /* The first element is the head */
    return (*list).element_prealloc_buf.as_mut_ptr();
}
#[inline]
unsafe extern "C" fn list_free_el(mut list: *mut list_t,
                                  mut el: *mut element_t) {
    match (*el).pre_status as libc::c_uint {
        1 => {
            (*el).pre_status = PRE_STATUS_UNUSED;
            (*list).element_prealloc_count -= 1;
            if (*list).element_prealloc_count < 0 as libc::c_int {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mInconsistent data in PRE_FREE\x00"
                           as *const u8 as *const libc::c_char);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"include/list.h\x00" as *const u8 as
                           *const libc::c_char, 66 as libc::c_int);
                exit(1 as libc::c_int);
            }
        }
        2 => {
            (*el).pre_status = PRE_STATUS_UNUSED;
            DFL_ck_free(el as *mut libc::c_void);
        }
        _ => {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mDouble Free Detected\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"include/list.h\x00" as *const u8 as *const libc::c_char,
                   66 as libc::c_int);
            exit(1 as libc::c_int);
        }
    };
}
#[inline]
unsafe extern "C" fn list_append(mut list: *mut list_t,
                                 mut el: *mut libc::c_void) {
    let mut head: *mut element_t = get_head(list);
    if (*head).next.is_null() {
        /* initialize */
        memset(list as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<list_t>() as libc::c_ulong);
        if (*head).pre_status as libc::c_uint !=
               PRE_STATUS_UNUSED as libc::c_int as libc::c_uint {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mPRE_ALLOC_FORCE element already allocated\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"include/list.h\x00" as *const u8 as *const libc::c_char,
                   78 as libc::c_int);
            exit(1 as libc::c_int);
        }
        (*head).pre_status = PRE_STATUS_USED;
        (*list).element_prealloc_count += 1;
        (*head).prev = head;
        (*head).next = (*head).prev
    }
    let mut el_box: *mut element_t = 0 as *mut element_t;
    if (*list).element_prealloc_count >= 64 as libc::c_int {
        el_box =
            malloc(::std::mem::size_of::<element_t>() as libc::c_ulong) as
                *mut element_t;
        (*el_box).pre_status = PRE_STATUS_MALLOC
    } else {
        let mut i: u32_0 = 0;
        i = 0 as libc::c_int as u32_0;
        while i < 64 as libc::c_int as libc::c_uint {
            el_box =
                &mut *(*list).element_prealloc_buf.as_mut_ptr().offset(i as
                                                                           isize)
                    as *mut element_t;
            if (*el_box).pre_status as libc::c_uint ==
                   PRE_STATUS_UNUSED as libc::c_int as libc::c_uint {
                (*list).element_prealloc_count += 1;
                (*el_box).pre_status = PRE_STATUS_USED;
                break ;
            } else { i = i.wrapping_add(1) }
        }
    }
    if el_box.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBUG in list.h -> no element found or allocated!\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"include/list.h\x00" as *const u8 as *const libc::c_char,
               85 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if el_box.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mfailed to allocate list element\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"include/list.h\x00" as *const u8 as *const libc::c_char,
               86 as libc::c_int);
        exit(1 as libc::c_int);
    }
    (*el_box).data = el;
    (*el_box).next = head;
    (*el_box).prev = (*head).prev;
    (*(*head).prev).next = el_box;
    (*head).prev = el_box;
}
/* Simple foreach.
   Pointer to the current element is in `el`,
   casted to (a pointer) of the given `type`.
   A return from this block will return from calling func.
*/
/* get next so el_box can be unlinked */
/* In foreach: remove the current el from the list */
/* Same as foreach, but will clear list in the process */
/* remove an item from the list */
#[inline]
unsafe extern "C" fn list_remove(mut list: *mut list_t,
                                 mut remove_me: *mut libc::c_void) {
    let mut li: *mut list_t = list;
    let mut head: *mut element_t = get_head(li);
    let mut el_box: *mut element_t = (*head).next;
    if el_box.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mforeach over uninitialized list\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"include/list.h\x00" as *const u8 as *const libc::c_char,
               161 as libc::c_int);
        exit(1 as libc::c_int);
    }
    while el_box != head {
        let mut el: *mut libc::c_void = (*el_box).data;
        let mut next: *mut element_t = (*el_box).next;
        if el == remove_me {
            (*(*el_box).prev).next = (*el_box).next;
            (*(*el_box).next).prev = (*el_box).prev;
            (*el_box).data = 0 as *mut libc::c_void;
            list_free_el(list, el_box);
            return
        }
        el_box = next
    }
    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mList item to be removed not in list\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00" as
               *const u8 as *const libc::c_char,
           b"func_unknown\x00" as *const u8 as *const libc::c_char,
           b"include/list.h\x00" as *const u8 as *const libc::c_char,
           163 as libc::c_int);
    exit(1 as libc::c_int);
}
/*
   american fuzzy lop++ - globals declarations
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

   This is the real deal: the program takes an instrumented binary and
   attempts a variety of basic fuzzing tricks, paying close attention to
   how they affect the execution path.

 */
#[no_mangle]
pub static mut interesting_8: [s8; 9] =
    [-(128 as libc::c_int) as s8, -(1 as libc::c_int) as s8,
     0 as libc::c_int as s8, 1 as libc::c_int as s8, 16 as libc::c_int as s8,
     32 as libc::c_int as s8, 64 as libc::c_int as s8,
     100 as libc::c_int as s8, 127 as libc::c_int as s8];
#[no_mangle]
pub static mut interesting_16: [s16; 19] =
    [-(128 as libc::c_int) as s16, -(1 as libc::c_int) as s16,
     0 as libc::c_int as s16, 1 as libc::c_int as s16,
     16 as libc::c_int as s16, 32 as libc::c_int as s16,
     64 as libc::c_int as s16, 100 as libc::c_int as s16,
     127 as libc::c_int as s16, -(32768 as libc::c_int) as s16,
     -(129 as libc::c_int) as s16, 128 as libc::c_int as s16,
     255 as libc::c_int as s16, 256 as libc::c_int as s16,
     512 as libc::c_int as s16, 1000 as libc::c_int as s16,
     1024 as libc::c_int as s16, 4096 as libc::c_int as s16,
     32767 as libc::c_int as s16];
#[no_mangle]
pub static mut interesting_32: [s32; 27] =
    [-(128 as libc::c_int), -(1 as libc::c_int), 0 as libc::c_int,
     1 as libc::c_int, 16 as libc::c_int, 32 as libc::c_int,
     64 as libc::c_int, 100 as libc::c_int, 127 as libc::c_int,
     -(32768 as libc::c_int), -(129 as libc::c_int), 128 as libc::c_int,
     255 as libc::c_int, 256 as libc::c_int, 512 as libc::c_int,
     1000 as libc::c_int, 1024 as libc::c_int, 4096 as libc::c_int,
     32767 as libc::c_int, -(2147483648 as libc::c_longlong) as s32,
     -(100663046 as libc::c_int), -(32769 as libc::c_int),
     32768 as libc::c_int, 65535 as libc::c_int, 65536 as libc::c_int,
     100663045 as libc::c_int, 2147483647 as libc::c_int];
#[no_mangle]
pub static mut power_names: [*mut libc::c_char; 8] =
    [b"explore\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"fast\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"coe\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"lin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"quad\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"exploit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"mmopt\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"rare\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
/* Initialize MOpt "globals" for this afl state */
unsafe extern "C" fn init_mopt_globals(mut afl: *mut afl_state_t) {
    let mut core: *mut MOpt_globals_t = &mut (*afl).mopt_globals_core;
    (*core).finds = (*afl).core_operator_finds_puppet.as_mut_ptr();
    (*core).finds_v2 = (*afl).core_operator_finds_puppet_v2.as_mut_ptr();
    (*core).cycles = (*afl).core_operator_cycles_puppet.as_mut_ptr();
    (*core).cycles_v2 = (*afl).core_operator_cycles_puppet_v2.as_mut_ptr();
    (*core).cycles_v3 = (*afl).core_operator_cycles_puppet_v3.as_mut_ptr();
    (*core).is_pilot_mode = 0 as libc::c_int as u32_0;
    (*core).pTime = &mut (*afl).tmp_core_time;
    (*core).period = 500000 as libc::c_int as u64_0;
    (*core).havoc_stagename =
        b"MOpt-core-havoc\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    (*core).splice_stageformat =
        b"MOpt-core-splice %u\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    (*core).havoc_stagenameshort =
        b"MOpt_core_havoc\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    (*core).splice_stagenameshort =
        b"MOpt_core_splice\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    let mut pilot: *mut MOpt_globals_t = &mut (*afl).mopt_globals_pilot;
    (*pilot).finds =
        (*afl).stage_finds_puppet[0 as libc::c_int as usize].as_mut_ptr();
    (*pilot).finds_v2 =
        (*afl).stage_finds_puppet_v2[0 as libc::c_int as usize].as_mut_ptr();
    (*pilot).cycles =
        (*afl).stage_cycles_puppet[0 as libc::c_int as usize].as_mut_ptr();
    (*pilot).cycles_v2 =
        (*afl).stage_cycles_puppet_v2[0 as libc::c_int as usize].as_mut_ptr();
    (*pilot).cycles_v3 =
        (*afl).stage_cycles_puppet_v3[0 as libc::c_int as usize].as_mut_ptr();
    (*pilot).is_pilot_mode = 1 as libc::c_int as u32_0;
    (*pilot).pTime = &mut (*afl).tmp_pilot_time;
    (*pilot).period = 50000 as libc::c_int as u64_0;
    (*pilot).havoc_stagename =
        b"MOpt-havoc\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    (*pilot).splice_stageformat =
        b"MOpt-splice %u\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    (*pilot).havoc_stagenameshort =
        b"MOpt_havoc\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    (*pilot).splice_stagenameshort =
        b"MOpt_splice\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
}
/* A global pointer to all instances is needed (for now) for signals to arrive
 */
#[no_mangle]
pub static mut afl_states: list_t =
    {
        let mut init =
            list{element_prealloc_buf:
                     [element_t{pre_status: PRE_STATUS_UNUSED,
                                prev:
                                    0 as *const list_element as
                                        *mut list_element,
                                next:
                                    0 as *const list_element as
                                        *mut list_element,
                                data:
                                    0 as *const libc::c_void as
                                        *mut libc::c_void,}; 64],
                 element_prealloc_count: 0 as libc::c_int,};
        init
    };
/* Initializes an afl_state_t. */
#[no_mangle]
pub unsafe extern "C" fn afl_state_init(mut afl: *mut afl_state_t) {
    /* thanks to this memset, growing vars like out_buf
  and out_size are NULL/0 by default. */
    memset(afl as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<afl_state_t>() as
               libc::c_ulong); /* Power schedule (default: EXPLORE)*/
    (*afl).w_init = 0.9f64; /* Window resized?                  */
    (*afl).w_end = 0.3f64; /* Cycle count divisor for havoc    */
    (*afl).g_max = 5000 as libc::c_int; /* Name of the current fuzz stage   */
    (*afl).period_pilot_tmp =
        5000.0f64; /* Splicing with which test case?   */
    (*afl).schedule =
        EXPLORE as libc::c_int as u8_0; /* Selected CPU core                */
    (*afl).havoc_max_mult = 16 as libc::c_int as u8_0;
    ::std::ptr::write_volatile(&mut (*afl).clear_screen as *mut u8_0,
                               1 as libc::c_int as u8_0);
    (*afl).havoc_div = 1 as libc::c_int as u32_0;
    (*afl).stage_name =
        b"init\x00" as *const u8 as *const libc::c_char as *mut u8_0;
    (*afl).splicing_with = -(1 as libc::c_int);
    (*afl).cpu_aff = -(1 as libc::c_int);
    /* HAVE_AFFINITY */
    (*afl).fsrv.use_stdin = 1 as libc::c_int as u8_0;
    (*afl).fsrv.map_size = ((1 as libc::c_int) << 16 as libc::c_int) as u32_0;
    (*afl).fsrv.function_opt = afl as *mut u8_0;
    (*afl).fsrv.function_ptr =
        Some(maybe_add_auto as
                 unsafe extern "C" fn(_: *mut libc::c_void, _: *mut u8_0,
                                      _: u32_0) -> ());
    (*afl).cal_cycles = 8 as libc::c_int as u8_0;
    (*afl).cal_cycles_long = 40 as libc::c_int as u8_0;
    (*afl).fsrv.exec_tmout = 1000 as libc::c_int as u32_0;
    (*afl).hang_tmout = 1000 as libc::c_int as u32_0;
    (*afl).fsrv.mem_limit = 50 as libc::c_int as u64_0;
    (*afl).stats_update_freq = 1 as libc::c_int as u32_0;
    (*afl).fsrv.dev_urandom_fd = -(1 as libc::c_int);
    (*afl).fsrv.dev_null_fd = -(1 as libc::c_int);
    (*afl).fsrv.child_pid = -(1 as libc::c_int);
    (*afl).fsrv.out_dir_fd = -(1 as libc::c_int);
    (*afl).cmplog_prev_timed_out = 0 as libc::c_int as u32_0;
    /* statis file */
    (*afl).last_bitmap_cvg = 0 as libc::c_int as libc::c_double;
    (*afl).last_stability = 0 as libc::c_int as libc::c_double;
    (*afl).last_eps = 0 as libc::c_int as libc::c_double;
    /* plot file saves from last run */
    (*afl).plot_prev_qp = 0 as libc::c_int as u32_0;
    (*afl).plot_prev_pf = 0 as libc::c_int as u32_0;
    (*afl).plot_prev_pnf = 0 as libc::c_int as u32_0;
    (*afl).plot_prev_ce = 0 as libc::c_int as u32_0;
    (*afl).plot_prev_md = 0 as libc::c_int as u32_0;
    (*afl).plot_prev_qc = 0 as libc::c_int as u64_0;
    (*afl).plot_prev_uc = 0 as libc::c_int as u64_0;
    (*afl).plot_prev_uh = 0 as libc::c_int as u64_0;
    (*afl).stats_last_stats_ms = 0 as libc::c_int as u64_0;
    (*afl).stats_last_plot_ms = 0 as libc::c_int as u64_0;
    (*afl).stats_last_ms = 0 as libc::c_int as u64_0;
    (*afl).stats_last_execs = 0 as libc::c_int as u64_0;
    (*afl).stats_avg_exec = -(1 as libc::c_int) as libc::c_double;
    init_mopt_globals(afl);
    list_append(&mut afl_states, afl as *mut libc::c_void);
}
/*This sets up the environment variables for afl-fuzz into the afl_state
 * struct*/
#[no_mangle]
pub unsafe extern "C" fn read_afl_environment(mut afl: *mut afl_state_t,
                                              mut envp:
                                                  *mut *mut libc::c_char) {
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
    loop  {
        let fresh0 = index;
        index = index + 1;
        env = *envp.offset(fresh0 as isize);
        if env.is_null() { break ; }
        if strncmp(env, b"ALF_\x00" as *const u8 as *const libc::c_char,
                   4 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
            printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mPotentially mistyped AFL environment variable: %s\x00"
                       as *const u8 as *const libc::c_char, env);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
            found += 1
        } else if strncmp(env,
                          b"AFL_\x00" as *const u8 as *const libc::c_char,
                          4 as libc::c_int as libc::c_ulong) ==
                      0 as libc::c_int {
            let mut i: libc::c_int = 0 as libc::c_int;
            let mut match_0: libc::c_int = 0 as libc::c_int;
            while match_0 == 0 as libc::c_int &&
                      !(*afl_environment_variables.as_mut_ptr().offset(i as
                                                                           isize)).is_null()
                  {
                let mut afl_environment_variable_len: size_t =
                    strlen(*afl_environment_variables.as_mut_ptr().offset(i as
                                                                              isize));
                if strncmp(env,
                           *afl_environment_variables.as_mut_ptr().offset(i as
                                                                              isize),
                           afl_environment_variable_len) == 0 as libc::c_int
                       &&
                       *env.offset(afl_environment_variable_len as isize) as
                           libc::c_int == '=' as i32 {
                    match_0 = 1 as libc::c_int;
                    if strncmp(env,
                               b"AFL_SKIP_CPUFREQ\x00" as *const u8 as
                                   *const libc::c_char,
                               afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_skip_cpufreq =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_EXIT_WHEN_DONE\x00" as *const u8
                                          as *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_exit_when_done =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_NO_AFFINITY\x00" as *const u8 as
                                          *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_no_affinity =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_SKIP_CRASHES\x00" as *const u8 as
                                          *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_skip_crashes =
                            get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                           as
                                                                                           isize))
                                as *mut u8_0
                    } else if strncmp(env,
                                      b"AFL_HANG_TMOUT\x00" as *const u8 as
                                          *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_hang_tmout =
                            get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                           as
                                                                                           isize))
                                as *mut u8_0
                    } else if strncmp(env,
                                      b"AFL_SKIP_BIN_CHECK\x00" as *const u8
                                          as *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_skip_bin_check =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_DUMB_FORKSRV\x00" as *const u8 as
                                          *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_dumb_forksrv =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_IMPORT_FIRST\x00" as *const u8 as
                                          *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_import_first =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_CUSTOM_MUTATOR_ONLY\x00" as
                                          *const u8 as *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_custom_mutator_only =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_NO_UI\x00" as *const u8 as
                                          *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_no_ui =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_FORCE_UI\x00" as *const u8 as
                                          *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_force_ui =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_I_DONT_CARE_ABOUT_MISSING_CRASHES\x00"
                                          as *const u8 as *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_i_dont_care_about_missing_crashes =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_BENCH_JUST_ONE\x00" as *const u8
                                          as *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_bench_just_one =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_BENCH_UNTIL_CRASH\x00" as
                                          *const u8 as *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_bench_until_crash =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_DEBUG_CHILD_OUTPUT\x00" as
                                          *const u8 as *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_debug_child_output =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_AUTORESUME\x00" as *const u8 as
                                          *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_autoresume =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_CAL_FAST\x00" as *const u8 as
                                          *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_cal_fast =
                            if !get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).is_null()
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int } as u8_0
                    } else if strncmp(env,
                                      b"AFL_TMPDIR\x00" as *const u8 as
                                          *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_tmpdir =
                            get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                           as
                                                                                           isize))
                                as *mut u8_0
                    } else if strncmp(env,
                                      b"AFL_POST_LIBRARY\x00" as *const u8 as
                                          *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_post_library =
                            get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                           as
                                                                                           isize))
                                as *mut u8_0
                    } else if strncmp(env,
                                      b"AFL_CUSTOM_MUTATOR_LIBRARY\x00" as
                                          *const u8 as *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_custom_mutator_library =
                            get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                           as
                                                                                           isize))
                                as *mut u8_0
                    } else if strncmp(env,
                                      b"AFL_PYTHON_MODULE\x00" as *const u8 as
                                          *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_python_module =
                            get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                           as
                                                                                           isize))
                                as *mut u8_0
                    } else if strncmp(env,
                                      b"AFL_PATH\x00" as *const u8 as
                                          *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_path =
                            get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                           as
                                                                                           isize))
                                as *mut u8_0
                    } else if strncmp(env,
                                      b"AFL_PRELOAD\x00" as *const u8 as
                                          *const libc::c_char,
                                      afl_environment_variable_len) == 0 {
                        (*afl).afl_env.afl_preload =
                            get_afl_env(*afl_environment_variables.as_mut_ptr().offset(i
                                                                                           as
                                                                                           isize))
                                as *mut u8_0
                    }
                } else { i += 1 }
            }
            if match_0 == 0 as libc::c_int {
                printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mMistyped AFL environment variable: %s\x00"
                           as *const u8 as *const libc::c_char, env);
                printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
                found += 1
            }
        }
    }
    if found != 0 { sleep(2 as libc::c_int as libc::c_uint); };
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
/* Removes this afl_state instance and frees it. */
#[no_mangle]
pub unsafe extern "C" fn afl_state_deinit(mut afl: *mut afl_state_t) {
    if (*afl).post_deinit.is_some() {
        (*afl).post_deinit.expect("non-null function pointer")((*afl).post_data);
    }
    if (*afl).in_place_resume != 0 {
        DFL_ck_free((*afl).in_dir as *mut libc::c_void);
    }
    if !(*afl).sync_id.is_null() {
        DFL_ck_free((*afl).out_dir as *mut libc::c_void);
    }
    free((*afl).out_buf as *mut libc::c_void);
    free((*afl).out_scratch_buf as *mut libc::c_void);
    free((*afl).eff_buf as *mut libc::c_void);
    free((*afl).in_buf as *mut libc::c_void);
    free((*afl).in_scratch_buf as *mut libc::c_void);
    free((*afl).ex_buf as *mut libc::c_void);
    list_remove(&mut afl_states, afl as *mut libc::c_void);
}
