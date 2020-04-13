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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn system(__command: *const libc::c_char) -> libc::c_int;
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
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    /* path to documentation dir        */
    /* Get unix time in milliseconds */
    #[no_mangle]
    fn get_cur_time() -> u64_0;
    /* Describe integer as memory size. */
    #[no_mangle]
    fn stringify_mem_size(buf: *mut u8_0, len: size_t, val: u64_0)
     -> *mut u8_0;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn add_to_queue(_: *mut afl_state_t, _: *mut u8_0, _: u32_0, _: u8_0);
    #[no_mangle]
    fn run_target(_: *mut afl_state_t, fsrv: *mut afl_forkserver_t, _: u32_0)
     -> u8_0;
    #[no_mangle]
    fn write_to_testcase(_: *mut afl_state_t, _: *mut libc::c_void, _: u32_0);
    #[no_mangle]
    fn calibrate_case(_: *mut afl_state_t, _: *mut queue_entry, _: *mut u8_0,
                      _: u32_0, _: u8_0) -> u8_0;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
pub type u16_0 = uint16_t;
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
pub const STAGE_VAL_BE: C2RustUnnamed = 2;
pub const STAGE_VAL_LE: C2RustUnnamed = 1;
pub const STAGE_VAL_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const FAULT_NOBITS: C2RustUnnamed_0 = 5;
pub const FAULT_NOINST: C2RustUnnamed_0 = 4;
pub const FAULT_ERROR: C2RustUnnamed_0 = 3;
pub const FAULT_CRASH: C2RustUnnamed_0 = 2;
pub const FAULT_TMOUT: C2RustUnnamed_0 = 1;
pub const FAULT_NONE: C2RustUnnamed_0 = 0;
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
   american fuzzy lop++ - bitmap related routines
   ----------------------------------------------

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
/* Write bitmap to file. The bitmap is useful mostly for the secret
   -B option, to focus a separate fuzzing session on a particular
   interesting input without rediscovering all the others. */
#[no_mangle]
pub unsafe extern "C" fn write_bitmap(mut afl: *mut afl_state_t) {
    let mut fname: [u8_0; 4096] = [0; 4096];
    let mut fd: s32 = 0;
    if (*afl).bitmap_changed == 0 { return }
    (*afl).bitmap_changed = 0 as libc::c_int as u8_0;
    snprintf(fname.as_mut_ptr() as *mut libc::c_char,
             4096 as libc::c_int as libc::c_ulong,
             b"%s/fuzz_bitmap\x00" as *const u8 as *const libc::c_char,
             (*afl).out_dir);
    fd =
        open(fname.as_mut_ptr() as *const libc::c_char,
             0o1 as libc::c_int | 0o100 as libc::c_int |
                 0o1000 as libc::c_int, 0o600 as libc::c_int);
    if fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                   as *const u8 as *const libc::c_char, fname.as_mut_ptr());
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                   *const libc::c_char, 44 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    let mut _len: u32_0 = ((1 as libc::c_int) << 16 as libc::c_int) as u32_0;
    let mut _res: s32 =
        write(fd, (*afl).virgin_bits.as_mut_ptr() as *const libc::c_void,
              _len as size_t) as s32;
    if _res as libc::c_uint != _len {
        if _res < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                       as *const u8 as *const libc::c_char,
                   fname.as_mut_ptr());
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                       *const libc::c_char, 46 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        } else {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                       as *const u8 as *const libc::c_char,
                   fname.as_mut_ptr());
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                       *const libc::c_char, 46 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    close(fd);
}
/* Read bitmap from file. This is for the -B option again. */
#[no_mangle]
pub unsafe extern "C" fn read_bitmap(mut afl: *mut afl_state_t,
                                     mut fname: *mut u8_0) {
    let mut fd: s32 = open(fname as *const libc::c_char, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                   as *const u8 as *const libc::c_char, fname);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                   *const libc::c_char, 58 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    let mut _len: u32_0 = ((1 as libc::c_int) << 16 as libc::c_int) as u32_0;
    let mut _res: s32 =
        read(fd, (*afl).virgin_bits.as_mut_ptr() as *mut libc::c_void,
             _len as size_t) as s32;
    if _res as libc::c_uint != _len {
        if _res < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort read from %s\x00"
                       as *const u8 as *const libc::c_char, fname);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                       *const libc::c_char, 60 as libc::c_int);
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
                   b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                       *const libc::c_char, 60 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    close(fd);
}
/* Check if the current execution path brings anything new to the table.
   Update virgin bits to reflect the finds. Returns 1 if the only change is
   the hit-count for a particular tuple; 2 if there are new tuples seen.
   Updates the map, so subsequent calls will always return 0.

   This function is called after every exec() on a fairly large buffer, so
   it needs to be fast. We do this in 32-bit and 64-bit flavors. */
#[no_mangle]
pub unsafe extern "C" fn has_new_bits(mut afl: *mut afl_state_t,
                                      mut virgin_map: *mut u8_0) -> u8_0 {
    let mut current: *mut u64_0 = (*afl).fsrv.trace_bits as *mut u64_0;
    let mut virgin: *mut u64_0 = virgin_map as *mut u64_0;
    let mut i: u32_0 = (*afl).fsrv.map_size >> 3 as libc::c_int;
    /* ^WORD_SIZE_64 */
    if i == 0 as libc::c_int as libc::c_uint { i = 1 as libc::c_int as u32_0 }
    let mut ret: u8_0 = 0 as libc::c_int as u8_0;
    loop  {
        let fresh2 = i;
        i = i.wrapping_sub(1);
        if !(fresh2 != 0) { break ; }
        /* Optimize for (*current & *virgin) == 0 - i.e., no bits in current bitmap
       that have not been already cleared from the virgin map - since this will
       almost always be the case. */
        if *current != 0 && *current & *virgin != 0 {
            if (ret as libc::c_int) < 2 as libc::c_int {
                let mut cur: *mut u8_0 = current as *mut u8_0;
                let mut vir: *mut u8_0 = virgin as *mut u8_0;
                /* ^WORD_SIZE_64 */
                if *cur.offset(0 as libc::c_int as isize) as libc::c_int != 0
                       &&
                       *vir.offset(0 as libc::c_int as isize) as libc::c_int
                           == 0xff as libc::c_int ||
                       *cur.offset(1 as libc::c_int as isize) as libc::c_int
                           != 0 &&
                           *vir.offset(1 as libc::c_int as isize) as
                               libc::c_int == 0xff as libc::c_int ||
                       *cur.offset(2 as libc::c_int as isize) as libc::c_int
                           != 0 &&
                           *vir.offset(2 as libc::c_int as isize) as
                               libc::c_int == 0xff as libc::c_int ||
                       *cur.offset(3 as libc::c_int as isize) as libc::c_int
                           != 0 &&
                           *vir.offset(3 as libc::c_int as isize) as
                               libc::c_int == 0xff as libc::c_int ||
                       *cur.offset(4 as libc::c_int as isize) as libc::c_int
                           != 0 &&
                           *vir.offset(4 as libc::c_int as isize) as
                               libc::c_int == 0xff as libc::c_int ||
                       *cur.offset(5 as libc::c_int as isize) as libc::c_int
                           != 0 &&
                           *vir.offset(5 as libc::c_int as isize) as
                               libc::c_int == 0xff as libc::c_int ||
                       *cur.offset(6 as libc::c_int as isize) as libc::c_int
                           != 0 &&
                           *vir.offset(6 as libc::c_int as isize) as
                               libc::c_int == 0xff as libc::c_int ||
                       *cur.offset(7 as libc::c_int as isize) as libc::c_int
                           != 0 &&
                           *vir.offset(7 as libc::c_int as isize) as
                               libc::c_int == 0xff as libc::c_int {
                    ret = 2 as libc::c_int as u8_0
                } else { ret = 1 as libc::c_int as u8_0 }
            }
            *virgin &= !*current
        }
        current = current.offset(1);
        virgin = virgin.offset(1)
    }
    if ret as libc::c_int != 0 &&
           virgin_map == (*afl).virgin_bits.as_mut_ptr() {
        (*afl).bitmap_changed = 1 as libc::c_int as u8_0
    }
    return ret;
}
/* Looks like we have not found any new bytes yet; see if any non-zero
           bytes in current[] are pristine in virgin[]. */
/* Count the number of bits set in the provided bitmap. Used for the status
   screen several times every second, does not have to be fast. */
#[no_mangle]
pub unsafe extern "C" fn count_bits(mut afl: *mut afl_state_t,
                                    mut mem: *mut u8_0) -> u32_0 {
    let mut ptr: *mut u32_0 = mem as *mut u32_0;
    let mut i: u32_0 = (*afl).fsrv.map_size >> 2 as libc::c_int;
    let mut ret: u32_0 = 0 as libc::c_int as u32_0;
    if i == 0 as libc::c_int as libc::c_uint { i = 1 as libc::c_int as u32_0 }
    loop  {
        let fresh3 = i;
        i = i.wrapping_sub(1);
        if !(fresh3 != 0) { break ; }
        let fresh4 = ptr;
        ptr = ptr.offset(1);
        let mut v: u32_0 = *fresh4;
        /* This gets called on the inverse, virgin bitmap; optimize for sparse
       data. */
        if v == 0xffffffff as libc::c_uint {
            ret =
                (ret as
                     libc::c_uint).wrapping_add(32 as libc::c_int as
                                                    libc::c_uint) as u32_0 as
                    u32_0
        } else {
            v =
                (v as
                     libc::c_uint).wrapping_sub(v >> 1 as libc::c_int &
                                                    0x55555555 as libc::c_int
                                                        as libc::c_uint) as
                    u32_0 as u32_0;
            v =
                (v &
                     0x33333333 as libc::c_int as
                         libc::c_uint).wrapping_add(v >> 2 as libc::c_int &
                                                        0x33333333 as
                                                            libc::c_int as
                                                            libc::c_uint);
            ret =
                (ret as
                     libc::c_uint).wrapping_add((v.wrapping_add(v >>
                                                                    4 as
                                                                        libc::c_int)
                                                     &
                                                     0xf0f0f0f as libc::c_int
                                                         as
                                                         libc::c_uint).wrapping_mul(0x1010101
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint)
                                                    >> 24 as libc::c_int) as
                    u32_0 as u32_0
        }
    }
    return ret;
}
/* Count the number of bytes set in the bitmap. Called fairly sporadically,
   mostly to update the status screen or calibrate and examine confirmed
   new paths. */
#[no_mangle]
pub unsafe extern "C" fn count_bytes(mut afl: *mut afl_state_t,
                                     mut mem: *mut u8_0) -> u32_0 {
    let mut ptr: *mut u32_0 = mem as *mut u32_0;
    let mut i: u32_0 = (*afl).fsrv.map_size >> 2 as libc::c_int;
    let mut ret: u32_0 = 0 as libc::c_int as u32_0;
    if i == 0 as libc::c_int as libc::c_uint { i = 1 as libc::c_int as u32_0 }
    loop  {
        let fresh5 = i;
        i = i.wrapping_sub(1);
        if !(fresh5 != 0) { break ; }
        let fresh6 = ptr;
        ptr = ptr.offset(1);
        let mut v: u32_0 = *fresh6;
        if v == 0 { continue ; }
        if v & 0xff as libc::c_int as libc::c_uint != 0 {
            ret = ret.wrapping_add(1)
        }
        if v & 0xff00 as libc::c_int as libc::c_uint != 0 {
            ret = ret.wrapping_add(1)
        }
        if v & 0xff0000 as libc::c_int as libc::c_uint != 0 {
            ret = ret.wrapping_add(1)
        }
        if v & 0xff000000 as libc::c_uint != 0 { ret = ret.wrapping_add(1) }
    }
    return ret;
}
/* Count the number of non-255 bytes set in the bitmap. Used strictly for the
   status screen, several calls per second or so. */
#[no_mangle]
pub unsafe extern "C" fn count_non_255_bytes(mut afl: *mut afl_state_t,
                                             mut mem: *mut u8_0) -> u32_0 {
    let mut ptr: *mut u32_0 = mem as *mut u32_0;
    let mut i: u32_0 = (*afl).fsrv.map_size >> 2 as libc::c_int;
    let mut ret: u32_0 = 0 as libc::c_int as u32_0;
    if i == 0 as libc::c_int as libc::c_uint { i = 1 as libc::c_int as u32_0 }
    loop  {
        let fresh7 = i;
        i = i.wrapping_sub(1);
        if !(fresh7 != 0) { break ; }
        let fresh8 = ptr;
        ptr = ptr.offset(1);
        let mut v: u32_0 = *fresh8;
        /* This is called on the virgin bitmap, so optimize for the most likely
       case. */
        if v == 0xffffffff as libc::c_uint { continue ; }
        if v & 0xff as libc::c_int as libc::c_uint !=
               0xff as libc::c_int as libc::c_uint {
            ret = ret.wrapping_add(1)
        }
        if v & 0xff00 as libc::c_int as libc::c_uint !=
               0xff00 as libc::c_int as libc::c_uint {
            ret = ret.wrapping_add(1)
        }
        if v & 0xff0000 as libc::c_int as libc::c_uint !=
               0xff0000 as libc::c_int as libc::c_uint {
            ret = ret.wrapping_add(1)
        }
        if v & 0xff000000 as libc::c_uint != 0xff000000 as libc::c_uint {
            ret = ret.wrapping_add(1)
        }
    }
    return ret;
}
/* Destructively simplify trace by eliminating hit count information
   and replacing it with 0x80 or 0x01 depending on whether the tuple
   is hit or not. Called on every new crash or timeout, should be
   reasonably fast. */
#[no_mangle]
pub static mut simplify_lookup: [u8_0; 256] =
    [1 as libc::c_int as u8_0, 128 as libc::c_int as u8_0,
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
#[no_mangle]
pub unsafe extern "C" fn simplify_trace(mut afl: *mut afl_state_t,
                                        mut mem: *mut u64_0) {
    let mut i: u32_0 = (*afl).fsrv.map_size >> 3 as libc::c_int;
    if i == 0 as libc::c_int as libc::c_uint { i = 1 as libc::c_int as u32_0 }
    loop  {
        let fresh9 = i;
        i = i.wrapping_sub(1);
        if !(fresh9 != 0) { break ; }
        /* Optimize for sparse bitmaps. */
        if *mem != 0 {
            let mut mem8: *mut u8_0 = mem as *mut u8_0;
            *mem8.offset(0 as libc::c_int as isize) =
                simplify_lookup[*mem8.offset(0 as libc::c_int as isize) as
                                    usize];
            *mem8.offset(1 as libc::c_int as isize) =
                simplify_lookup[*mem8.offset(1 as libc::c_int as isize) as
                                    usize];
            *mem8.offset(2 as libc::c_int as isize) =
                simplify_lookup[*mem8.offset(2 as libc::c_int as isize) as
                                    usize];
            *mem8.offset(3 as libc::c_int as isize) =
                simplify_lookup[*mem8.offset(3 as libc::c_int as isize) as
                                    usize];
            *mem8.offset(4 as libc::c_int as isize) =
                simplify_lookup[*mem8.offset(4 as libc::c_int as isize) as
                                    usize];
            *mem8.offset(5 as libc::c_int as isize) =
                simplify_lookup[*mem8.offset(5 as libc::c_int as isize) as
                                    usize];
            *mem8.offset(6 as libc::c_int as isize) =
                simplify_lookup[*mem8.offset(6 as libc::c_int as isize) as
                                    usize];
            *mem8.offset(7 as libc::c_int as isize) =
                simplify_lookup[*mem8.offset(7 as libc::c_int as isize) as
                                    usize]
        } else { *mem = 0x101010101010101 as libc::c_ulonglong }
        mem = mem.offset(1)
    };
}
/* ^WORD_SIZE_64 */
/* Destructively classify execution counts in a trace. This is used as a
   preprocessing step for any newly acquired traces. Called on every exec,
   must be fast. */
static mut count_class_lookup8: [u8_0; 256] =
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
static mut count_class_lookup16: [u16_0; 65536] = [0; 65536];
#[no_mangle]
pub unsafe extern "C" fn init_count_class16() {
    let mut b1: u32_0 = 0;
    let mut b2: u32_0 = 0;
    b1 = 0 as libc::c_int as u32_0;
    while b1 < 256 as libc::c_int as libc::c_uint {
        b2 = 0 as libc::c_int as u32_0;
        while b2 < 256 as libc::c_int as libc::c_uint {
            count_class_lookup16[(b1 << 8 as libc::c_int).wrapping_add(b2) as
                                     usize] =
                ((count_class_lookup8[b1 as usize] as libc::c_int) <<
                     8 as libc::c_int |
                     count_class_lookup8[b2 as usize] as libc::c_int) as
                    u16_0;
            b2 = b2.wrapping_add(1)
        }
        b1 = b1.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn classify_counts(mut afl: *mut afl_state_t,
                                         mut mem: *mut u64_0) {
    let mut i: u32_0 = (*afl).fsrv.map_size >> 3 as libc::c_int;
    if i == 0 as libc::c_int as libc::c_uint { i = 1 as libc::c_int as u32_0 }
    loop  {
        let fresh10 = i;
        i = i.wrapping_sub(1);
        if !(fresh10 != 0) { break ; }
        /* Optimize for sparse bitmaps. */
        if *mem != 0 {
            let mut mem16: *mut u16_0 = mem as *mut u16_0;
            *mem16.offset(0 as libc::c_int as isize) =
                count_class_lookup16[*mem16.offset(0 as libc::c_int as isize)
                                         as usize];
            *mem16.offset(1 as libc::c_int as isize) =
                count_class_lookup16[*mem16.offset(1 as libc::c_int as isize)
                                         as usize];
            *mem16.offset(2 as libc::c_int as isize) =
                count_class_lookup16[*mem16.offset(2 as libc::c_int as isize)
                                         as usize];
            *mem16.offset(3 as libc::c_int as isize) =
                count_class_lookup16[*mem16.offset(3 as libc::c_int as isize)
                                         as usize]
        }
        mem = mem.offset(1)
    };
}
/* ^WORD_SIZE_64 */
/* Compact trace bytes into a smaller bitmap. We effectively just drop the
   count information here. This is called only sporadically, for some
   new paths. */
#[no_mangle]
pub unsafe extern "C" fn minimize_bits(mut afl: *mut afl_state_t,
                                       mut dst: *mut u8_0,
                                       mut src: *mut u8_0) {
    let mut i: u32_0 = 0 as libc::c_int as u32_0;
    while i < (*afl).fsrv.map_size {
        let fresh11 = src;
        src = src.offset(1);
        if *fresh11 != 0 {
            let ref mut fresh12 =
                *dst.offset((i >> 3 as libc::c_int) as isize);
            *fresh12 =
                (*fresh12 as libc::c_int |
                     (1 as libc::c_int) <<
                         (i & 7 as libc::c_int as libc::c_uint)) as u8_0
        }
        i = i.wrapping_add(1)
    };
}
/* Construct a file name for a new test case, capturing the operation
   that led to its discovery. Returns a ptr to afl->describe_op_buf_256. */
#[no_mangle]
pub unsafe extern "C" fn describe_op(mut afl: *mut afl_state_t, mut hnb: u8_0)
 -> *mut u8_0 {
    let mut ret: *mut u8_0 = (*afl).describe_op_buf_256.as_mut_ptr();
    if !(*afl).syncing_party.is_null() {
        sprintf(ret as *mut libc::c_char,
                b"sync:%s,src:%06u\x00" as *const u8 as *const libc::c_char,
                (*afl).syncing_party, (*afl).syncing_case);
    } else {
        sprintf(ret as *mut libc::c_char,
                b"src:%06u\x00" as *const u8 as *const libc::c_char,
                (*afl).current_entry);
        sprintf(ret.offset(strlen(ret as *const libc::c_char) as isize) as
                    *mut libc::c_char,
                b",time:%llu\x00" as *const u8 as *const libc::c_char,
                get_cur_time().wrapping_sub((*afl).start_time));
        if (*afl).splicing_with >= 0 as libc::c_int {
            sprintf(ret.offset(strlen(ret as *const libc::c_char) as isize) as
                        *mut libc::c_char,
                    b"+%06d\x00" as *const u8 as *const libc::c_char,
                    (*afl).splicing_with);
        }
        sprintf(ret.offset(strlen(ret as *const libc::c_char) as isize) as
                    *mut libc::c_char,
                b",op:%s\x00" as *const u8 as *const libc::c_char,
                (*afl).stage_short);
        if (*afl).stage_cur_byte >= 0 as libc::c_int {
            sprintf(ret.offset(strlen(ret as *const libc::c_char) as isize) as
                        *mut libc::c_char,
                    b",pos:%d\x00" as *const u8 as *const libc::c_char,
                    (*afl).stage_cur_byte);
            if (*afl).stage_val_type as libc::c_int !=
                   STAGE_VAL_NONE as libc::c_int {
                sprintf(ret.offset(strlen(ret as *const libc::c_char) as
                                       isize) as *mut libc::c_char,
                        b",val:%s%+d\x00" as *const u8 as *const libc::c_char,
                        if (*afl).stage_val_type as libc::c_int ==
                               STAGE_VAL_BE as libc::c_int {
                            b"be:\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        }, (*afl).stage_cur_val);
            }
        } else {
            sprintf(ret.offset(strlen(ret as *const libc::c_char) as isize) as
                        *mut libc::c_char,
                    b",rep:%d\x00" as *const u8 as *const libc::c_char,
                    (*afl).stage_cur_val);
        }
    }
    if hnb as libc::c_int == 2 as libc::c_int {
        strcat(ret as *mut libc::c_char,
               b",+cov\x00" as *const u8 as *const libc::c_char);
    }
    return ret;
}
/* !SIMPLE_FILES */
/* Write a message accompanying the crash directory :-) */
unsafe extern "C" fn write_crash_readme(mut afl: *mut afl_state_t) {
    let mut fn_0: [u8_0; 4096] = [0; 4096];
    let mut fd: s32 = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut val_buf: [u8_0; 16] = [0; 16];
    sprintf(fn_0.as_mut_ptr() as *mut libc::c_char,
            b"%s/crashes/README.txt\x00" as *const u8 as *const libc::c_char,
            (*afl).out_dir);
    fd =
        open(fn_0.as_mut_ptr() as *const libc::c_char,
             0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
             0o600 as libc::c_int);
    /* Do not die on errors here - that would be impolite. */
    if fd < 0 as libc::c_int { return } /* ignore errors */
    f = fdopen(fd, b"w\x00" as *const u8 as *const libc::c_char);
    if f.is_null() { close(fd); return }
    fprintf(f,
            b"Command line used to find this crash:\n\n%s\n\nIf you can\'t reproduce a bug outside of afl-fuzz, be sure to set the same\nmemory limit. The limit used for this fuzzing session was %s.\n\nNeed a tool to minimize test cases before investigating the crashes or sending\nthem to a vendor? Check out the afl-tmin that comes with the fuzzer!\n\nFound any cool bugs in open-source tools using afl-fuzz? If yes, please drop\nan mail at <afl-users@googlegroups.com> once the issues are fixed\n\n  https://github.com/AFLplusplus/AFLplusplus\n\n\x00"
                as *const u8 as *const libc::c_char, (*afl).orig_cmdline,
            stringify_mem_size(val_buf.as_mut_ptr(),
                               ::std::mem::size_of::<[u8_0; 16]>() as
                                   libc::c_ulong,
                               (*afl).fsrv.mem_limit << 20 as libc::c_int));
    fclose(f);
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
/* Check if the result of an execve() during routine fuzzing is interesting,
   save or queue the input test case for further analysis if so. Returns 1 if
   entry is saved, 0 otherwise. */
#[no_mangle]
pub unsafe extern "C" fn save_if_interesting(mut afl: *mut afl_state_t,
                                             mut mem: *mut libc::c_void,
                                             mut len: u32_0, mut fault: u8_0)
 -> u8_0 {
    if len == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as u8_0
    }
    let mut queue_fn: *mut u8_0 =
        b"\x00" as *const u8 as *const libc::c_char as *mut u8_0;
    let mut hnb: u8_0 = '\u{0}' as i32 as u8_0;
    let mut fd: s32 = 0;
    let mut keeping: u8_0 = 0 as libc::c_int as u8_0;
    let mut res: u8_0 = 0;
    let mut fn_0: [u8_0; 4096] = [0; 4096];
    /* Update path frequency. */
    let mut cksum: u32_0 =
        hash32((*afl).fsrv.trace_bits as *const libc::c_void,
               (*afl).fsrv.map_size, 0xa5b35705 as libc::c_uint);
    let mut q: *mut queue_entry = (*afl).queue;
    while !q.is_null() {
        if (*q).exec_cksum == cksum {
            (*q).n_fuzz =
                (*q).n_fuzz.wrapping_add(1 as libc::c_int as
                                             libc::c_ulonglong);
            break ;
        } else { q = (*q).next }
    }
    if fault as libc::c_int == (*afl).crash_mode as libc::c_int {
        /* Keep only if there are new bits in the map, add to queue for
       future fuzzing, etc. */
        hnb = has_new_bits(afl, (*afl).virgin_bits.as_mut_ptr());
        if hnb == 0 {
            if (*afl).crash_mode != 0 {
                (*afl).total_crashes = (*afl).total_crashes.wrapping_add(1)
            }
            return 0 as libc::c_int as u8_0
        }
        queue_fn =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/queue/id:%06u,%s\x00" as *const u8 as
                                  *const libc::c_char, (*afl).out_dir,
                              (*afl).queued_paths, describe_op(afl, hnb));
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                                *const libc::c_char, 576 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/queue/id:%06u,%s\x00" as *const u8 as
                              *const libc::c_char, (*afl).out_dir,
                          (*afl).queued_paths, describe_op(afl, hnb));
                 _tmp
             });
        /* ^!SIMPLE_FILES */
        add_to_queue(afl, queue_fn, len, 0 as libc::c_int as u8_0);
        if hnb as libc::c_int == 2 as libc::c_int {
            (*(*afl).queue_top).has_new_cov = 1 as libc::c_int as u8_0;
            (*afl).queued_with_cov = (*afl).queued_with_cov.wrapping_add(1)
        }
        (*(*afl).queue_top).exec_cksum = cksum;
        /* Try to calibrate inline; this also calls update_bitmap_score() when
       successful. */
        res =
            calibrate_case(afl, (*afl).queue_top, mem as *mut u8_0,
                           (*afl).queue_cycle.wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulonglong)
                               as u32_0, 0 as libc::c_int as u8_0);
        if res as libc::c_int == FAULT_ERROR as libc::c_int {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to execute target application\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                       *const libc::c_char, 602 as libc::c_int);
            exit(1 as libc::c_int);
        }
        fd =
            open(queue_fn as *const libc::c_char,
                 0o1 as libc::c_int | 0o100 as libc::c_int |
                     0o200 as libc::c_int, 0o600 as libc::c_int);
        if fd < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                       as *const u8 as *const libc::c_char, queue_fn);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                       *const libc::c_char, 605 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        let mut _len: u32_0 = len;
        let mut _res: s32 = write(fd, mem, _len as size_t) as s32;
        if _res as libc::c_uint != _len {
            if _res < 0 as libc::c_int {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char, queue_fn);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                           *const libc::c_char, 606 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            } else {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char, queue_fn);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                           *const libc::c_char, 606 as libc::c_int);
                exit(1 as libc::c_int);
            }
        }
        close(fd);
        keeping = 1 as libc::c_int as u8_0
    }
    let mut current_block_93: u64;
    match fault as libc::c_int {
        1 => {
            /* Timeouts are not very interesting, but we're still obliged to keep
         a handful of samples. We use the presence of new bits in the
         hang-specific bitmap as a signal of uniqueness. In "dumb" mode, we
         just keep everything. */
            (*afl).total_tmouts = (*afl).total_tmouts.wrapping_add(1);
            if (*afl).unique_hangs >= 500 as libc::c_int as libc::c_ulonglong
               {
                return keeping
            }
            if (*afl).dumb_mode == 0 {
                simplify_trace(afl, (*afl).fsrv.trace_bits as *mut u64_0);
                /* ^WORD_SIZE_64 */
                if has_new_bits(afl, (*afl).virgin_tmout.as_mut_ptr()) == 0 {
                    return keeping
                }
            }
            (*afl).unique_tmouts = (*afl).unique_tmouts.wrapping_add(1);
            /* Before saving, we make sure that it's a genuine hang by re-running
         the target with a more generous timeout (unless the default timeout
         is already generous). */
            if (*afl).fsrv.exec_tmout < (*afl).hang_tmout {
                let mut new_fault: u8_0 = 0;
                write_to_testcase(afl, mem, len);
                new_fault =
                    run_target(afl, &mut (*afl).fsrv, (*afl).hang_tmout);
                /* A corner case that one user reported bumping into: increasing the
           timeout actually uncovers a crash. Make sure we don't discard it if
           so. */
                if (*afl).stop_soon == 0 &&
                       new_fault as libc::c_int == FAULT_CRASH as libc::c_int
                   {
                    current_block_93 = 5475558340279042137;
                } else {
                    if (*afl).stop_soon as libc::c_int != 0 ||
                           new_fault as libc::c_int !=
                               FAULT_TMOUT as libc::c_int {
                        return keeping
                    }
                    current_block_93 = 129780949503461575;
                }
            } else { current_block_93 = 129780949503461575; }
            match current_block_93 {
                5475558340279042137 => { }
                _ => {
                    snprintf(fn_0.as_mut_ptr() as *mut libc::c_char,
                             4096 as libc::c_int as libc::c_ulong,
                             b"%s/hangs/id:%06llu,%s\x00" as *const u8 as
                                 *const libc::c_char, (*afl).out_dir,
                             (*afl).unique_hangs,
                             describe_op(afl, 0 as libc::c_int as u8_0));
                    /* ^!SIMPLE_FILES */
                    (*afl).unique_hangs = (*afl).unique_hangs.wrapping_add(1);
                    (*afl).last_hang_time = get_cur_time();
                    current_block_93 = 16313536926714486912;
                }
            }
        }
        2 => { current_block_93 = 5475558340279042137; }
        3 => {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to execute target application\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                       *const libc::c_char, 737 as libc::c_int);
            exit(1 as libc::c_int);
        }
        _ => { return keeping }
    }
    match current_block_93 {
        5475558340279042137 => {
            /* This is handled in a manner roughly similar to timeouts,
         except for slightly different limits and no need to re-run test
         cases. */
            (*afl).total_crashes = (*afl).total_crashes.wrapping_add(1);
            if (*afl).unique_crashes >=
                   5000 as libc::c_int as libc::c_ulonglong {
                return keeping
            }
            if (*afl).dumb_mode == 0 {
                simplify_trace(afl, (*afl).fsrv.trace_bits as *mut u64_0);
                /* ^WORD_SIZE_64 */
                if has_new_bits(afl, (*afl).virgin_crash.as_mut_ptr()) == 0 {
                    return keeping
                }
            }
            if (*afl).unique_crashes == 0 { write_crash_readme(afl); }
            snprintf(fn_0.as_mut_ptr() as *mut libc::c_char,
                     4096 as libc::c_int as libc::c_ulong,
                     b"%s/crashes/id:%06llu,sig:%02u,%s\x00" as *const u8 as
                         *const libc::c_char, (*afl).out_dir,
                     (*afl).unique_crashes, (*afl).kill_signal as libc::c_int,
                     describe_op(afl, 0 as libc::c_int as u8_0));
            /* ^!SIMPLE_FILES */
            (*afl).unique_crashes = (*afl).unique_crashes.wrapping_add(1);
            if !(*afl).infoexec.is_null() {
                // if the user wants to be informed on new crashes - do that
                // we dont care if system errors, but we dont want a
        // compiler warning either
        // See
        // https://stackoverflow.com/questions/11888594/ignoring-return-values-in-c
                system((*afl).infoexec as *const libc::c_char);
            }
            (*afl).last_crash_time = get_cur_time();
            (*afl).last_crash_execs = (*afl).total_execs
        }
        _ => { }
    }
    /* If we're here, we apparently want to save the crash or hang
     test case, too. */
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
               b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                   *const libc::c_char, 747 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    let mut _len_0: u32_0 = len;
    let mut _res_0: s32 = write(fd, mem, _len_0 as size_t) as s32;
    if _res_0 as libc::c_uint != _len_0 {
        if _res_0 < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                       as *const u8 as *const libc::c_char,
                   fn_0.as_mut_ptr());
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                       *const libc::c_char, 748 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        } else {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                       as *const u8 as *const libc::c_char,
                   fn_0.as_mut_ptr());
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-bitmap.c\x00" as *const u8 as
                       *const libc::c_char, 748 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    close(fd);
    return keeping;
}
