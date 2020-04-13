use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    /* Bitmap */
    /* Extras */
    /* Stats */
    /* Run */
    #[no_mangle]
    fn common_fuzz_stuff(_: *mut afl_state_t, _: *mut u8_0, _: u32_0) -> u8_0;
    #[no_mangle]
    fn maybe_add_auto(_: *mut libc::c_void, _: *mut u8_0, _: u32_0);
    /* CmpLog */
    #[no_mangle]
    fn common_fuzz_cmplog_stuff(afl: *mut afl_state_t, out_buf: *mut u8_0,
                                len: u32_0) -> u8_0;
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
/*
   american fuzzy lop++ - redqueen implementation on top of cmplog
   ---------------------------------------------------------------

   Originally written by Michal Zalewski

   Forkserver design by Jann Horn <jannhorn@googlemail.com>

   Now maintained by by Marc Heuse <mh@mh-sec.de>,
                        Heiko Eißfeldt <heiko.eissfeldt@hexco.de> and
                        Andrea Fioraldi <andreafioraldi@gmail.com>

   Copyright 2016, 2017 Google Inc. All rights reserved.
   Copyright 2019-2020 AFLplusplus Project. All rights reserved.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at:

     http://www.apache.org/licenses/LICENSE-2.0

   Shared code to handle the shared memory. This is used by the fuzzer
   as well the other components like afl-tmin, afl-showmap, etc...

 */
// /// Colorization
#[derive(Copy, Clone)]
#[repr(C)]
pub struct range {
    pub start: u32_0,
    pub end: u32_0,
    pub next: *mut range,
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
pub type C2RustUnnamed = libc::c_uint;
pub const STAGE_ITS: C2RustUnnamed = 21;
pub const STAGE_COLORIZATION: C2RustUnnamed = 20;
pub const STAGE_CUSTOM_MUTATOR: C2RustUnnamed = 19;
pub const STAGE_RADAMSA: C2RustUnnamed = 18;
pub const STAGE_PYTHON: C2RustUnnamed = 17;
pub const STAGE_SPLICE: C2RustUnnamed = 16;
pub const STAGE_HAVOC: C2RustUnnamed = 15;
pub const STAGE_EXTRAS_AO: C2RustUnnamed = 14;
pub const STAGE_EXTRAS_UI: C2RustUnnamed = 13;
pub const STAGE_EXTRAS_UO: C2RustUnnamed = 12;
pub const STAGE_INTEREST32: C2RustUnnamed = 11;
pub const STAGE_INTEREST16: C2RustUnnamed = 10;
pub const STAGE_INTEREST8: C2RustUnnamed = 9;
pub const STAGE_ARITH32: C2RustUnnamed = 8;
pub const STAGE_ARITH16: C2RustUnnamed = 7;
pub const STAGE_ARITH8: C2RustUnnamed = 6;
pub const STAGE_FLIP32: C2RustUnnamed = 5;
pub const STAGE_FLIP16: C2RustUnnamed = 4;
pub const STAGE_FLIP8: C2RustUnnamed = 3;
pub const STAGE_FLIP4: C2RustUnnamed = 2;
pub const STAGE_FLIP2: C2RustUnnamed = 1;
pub const STAGE_FLIP1: C2RustUnnamed = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmpfn_operands {
    pub v0: [u8_0; 32],
    pub v1: [u8_0; 32],
}
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
/* Free memory, checking for double free and corrupted heap. When DEBUG_BUILD
   is set, the old memory will be also clobbered with 0xFF. */
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
/* *** Inline routines ****/
/* Generate a random number (from 0 to limit - 1). This may
   have slight bias. */
#[inline]
unsafe extern "C" fn rand_below(mut afl: *mut afl_state_t, mut limit: u32_0)
 -> u32_0 {
    let fresh2 = (*afl).rand_cnt;
    (*afl).rand_cnt = (*afl).rand_cnt.wrapping_sub(1);
    if fresh2 == 0 && (*afl).fixed_seed == 0 {
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
#[no_mangle]
pub unsafe extern "C" fn add_range(mut ranges: *mut range, mut start: u32_0,
                                   mut end: u32_0) -> *mut range {
    let mut r: *mut range =
        DFL_ck_alloc_nozero(::std::mem::size_of::<range>() as libc::c_ulong as
                                u32_0) as *mut range;
    (*r).start = start;
    (*r).end = end;
    (*r).next = ranges;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn pop_biggest_range(mut ranges: *mut *mut range)
 -> *mut range {
    let mut r: *mut range = *ranges;
    let mut prev: *mut range = 0 as *mut range;
    let mut rmax: *mut range = 0 as *mut range;
    let mut prev_rmax: *mut range = 0 as *mut range;
    let mut max_size: u32_0 = 0 as libc::c_int as u32_0;
    while !r.is_null() {
        let mut s: u32_0 = (*r).end.wrapping_sub((*r).start);
        if s >= max_size { max_size = s; prev_rmax = prev; rmax = r }
        prev = r;
        r = (*r).next
    }
    if !rmax.is_null() {
        if !prev_rmax.is_null() {
            (*prev_rmax).next = (*rmax).next
        } else { *ranges = (*rmax).next }
    }
    return rmax;
}
unsafe extern "C" fn get_exec_checksum(mut afl: *mut afl_state_t,
                                       mut buf: *mut u8_0, mut len: u32_0,
                                       mut cksum: *mut u32_0) -> u8_0 {
    if common_fuzz_stuff(afl, buf, len) != 0 {
        return 1 as libc::c_int as u8_0
    }
    *cksum =
        hash32((*afl).fsrv.trace_bits as *const libc::c_void,
               (*afl).fsrv.map_size, 0xa5b35705 as libc::c_uint);
    return 0 as libc::c_int as u8_0;
}
unsafe extern "C" fn rand_replace(mut afl: *mut afl_state_t,
                                  mut buf: *mut u8_0, mut len: u32_0) {
    let mut i: u32_0 = 0;
    i = 0 as libc::c_int as u32_0;
    while i < len {
        *buf.offset(i as isize) =
            rand_below(afl, 256 as libc::c_int as u32_0) as u8_0;
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn colorization(mut afl: *mut afl_state_t,
                                  mut buf: *mut u8_0, mut len: u32_0,
                                  mut exec_cksum: u32_0) -> u8_0 {
    let mut cksum: u32_0 = 0;
    let mut current_block: u64;
    let mut ranges: *mut range =
        add_range(0 as *mut range, 0 as libc::c_int as u32_0, len);
    let mut backup: *mut u8_0 = DFL_ck_alloc_nozero(len) as *mut u8_0;
    let mut needs_write: u8_0 = 0 as libc::c_int as u8_0;
    let mut orig_hit_cnt: u64_0 = 0;
    let mut new_hit_cnt: u64_0 = 0;
    orig_hit_cnt =
        ((*afl).queued_paths as
             libc::c_ulonglong).wrapping_add((*afl).unique_crashes);
    (*afl).stage_name =
        b"colorization\x00" as *const u8 as *const libc::c_char as *mut u8_0;
    (*afl).stage_short =
        b"colorization\x00" as *const u8 as *const libc::c_char as *mut u8_0;
    (*afl).stage_max = 1000 as libc::c_int;
    let mut rng: *mut range = 0 as *mut range;
    (*afl).stage_cur = 0 as libc::c_int;
    loop  {
        rng = pop_biggest_range(&mut ranges);
        if !(!rng.is_null() && (*afl).stage_cur < (*afl).stage_max) {
            current_block = 5783071609795492627;
            break ;
        }
        let mut s: u32_0 = (*rng).end.wrapping_sub((*rng).start);
        if !(s == 0 as libc::c_int as libc::c_uint) {
            memcpy(backup as *mut libc::c_void,
                   buf.offset((*rng).start as isize) as *const libc::c_void,
                   s as libc::c_ulong);
            rand_replace(afl, buf.offset((*rng).start as isize), s);
            cksum = 0;
            if get_exec_checksum(afl, buf, len, &mut cksum) != 0 {
                current_block = 15377205530662800562;
                break ;
            }
            if cksum != exec_cksum {
                ranges =
                    add_range(ranges, (*rng).start,
                              (*rng).start.wrapping_add(s.wrapping_div(2 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)));
                ranges =
                    add_range(ranges,
                              (*rng).start.wrapping_add(s.wrapping_div(2 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)).wrapping_add(1
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_uint),
                              (*rng).end);
                memcpy(buf.offset((*rng).start as isize) as *mut libc::c_void,
                       backup as *const libc::c_void, s as libc::c_ulong);
            } else { needs_write = 1 as libc::c_int as u8_0 }
        }
        DFL_ck_free(rng as *mut libc::c_void);
        rng = 0 as *mut range;
        (*afl).stage_cur += 1
    }
    match current_block {
        15377205530662800562 => {
            DFL_ck_free(backup as *mut libc::c_void);
            while !ranges.is_null() {
                rng = ranges;
                ranges = (*ranges).next;
                DFL_ck_free(rng as *mut libc::c_void);
                rng = 0 as *mut range
            }
            // TODO: clang notices a _potential_ leak of mem pointed to by rng
            return 1 as libc::c_int as u8_0
        }
        _ => {
            if (*afl).stage_cur < (*afl).stage_max {
                (*(*afl).queue_cur).fully_colorized = 1 as libc::c_int as u8_0
            }
            new_hit_cnt =
                ((*afl).queued_paths as
                     libc::c_ulonglong).wrapping_add((*afl).unique_crashes);
            (*afl).stage_finds[STAGE_COLORIZATION as libc::c_int as usize] =
                ((*afl).stage_finds[STAGE_COLORIZATION as libc::c_int as
                                        usize] as
                     libc::c_ulonglong).wrapping_add(new_hit_cnt.wrapping_sub(orig_hit_cnt))
                    as u64_0 as u64_0;
            (*afl).stage_cycles[STAGE_COLORIZATION as libc::c_int as usize] =
                ((*afl).stage_cycles[STAGE_COLORIZATION as libc::c_int as
                                         usize] as
                     libc::c_ulonglong).wrapping_add((*afl).stage_cur as
                                                         libc::c_ulonglong) as
                    u64_0 as u64_0;
            DFL_ck_free(backup as *mut libc::c_void);
            while !ranges.is_null() {
                rng = ranges;
                ranges = (*ranges).next;
                DFL_ck_free(rng as *mut libc::c_void);
                rng = 0 as *mut range
            }
            // save the input with the high entropy
            if needs_write != 0 {
                let mut fd: s32 = 0; /* ignore errors */
                if (*afl).no_unlink != 0 {
                    fd =
                        open((*(*afl).queue_cur).fname as *const libc::c_char,
                             0o1 as libc::c_int | 0o100 as libc::c_int |
                                 0o1000 as libc::c_int, 0o600 as libc::c_int)
                } else {
                    unlink((*(*afl).queue_cur).fname as
                               *const libc::c_char); // no-op, just to be 100% safe
                    fd =
                        open((*(*afl).queue_cur).fname as *const libc::c_char,
                             0o1 as libc::c_int | 0o100 as libc::c_int |
                                 0o200 as libc::c_int, 0o600 as libc::c_int)
                }
                if fd < 0 as libc::c_int {
                    fflush(stdout);
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                               as *const u8 as *const libc::c_char,
                           (*(*afl).queue_cur).fname);
                    printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz-redqueen.c\x00" as *const u8 as
                               *const libc::c_char, 182 as libc::c_int);
                    printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                               *const u8 as *const libc::c_char,
                           strerror(*__errno_location()));
                    exit(1 as libc::c_int);
                }
                let mut _len: u32_0 = len;
                let mut _res: s32 =
                    write(fd, buf as *const libc::c_void, _len as size_t) as
                        s32;
                if _res as libc::c_uint != _len {
                    if _res < 0 as libc::c_int {
                        fflush(stdout);
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                                   as *const u8 as *const libc::c_char,
                               (*(*afl).queue_cur).fname);
                        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-fuzz-redqueen.c\x00" as *const u8 as
                                   *const libc::c_char, 184 as libc::c_int);
                        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00"
                                   as *const u8 as *const libc::c_char,
                               strerror(*__errno_location()));
                        exit(1 as libc::c_int);
                    } else {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                                   as *const u8 as *const libc::c_char,
                               (*(*afl).queue_cur).fname);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-fuzz-redqueen.c\x00" as *const u8 as
                                   *const libc::c_char, 184 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                }
                (*(*afl).queue_cur).len = len;
                close(fd);
            }
            return 0 as libc::c_int as u8_0
        }
    };
}
// /// Input to State replacement
unsafe extern "C" fn its_fuzz(mut afl: *mut afl_state_t, mut buf: *mut u8_0,
                              mut len: u32_0, mut status: *mut u8_0) -> u8_0 {
    let mut orig_hit_cnt: u64_0 = 0;
    let mut new_hit_cnt: u64_0 = 0;
    orig_hit_cnt =
        ((*afl).queued_paths as
             libc::c_ulonglong).wrapping_add((*afl).unique_crashes);
    if common_fuzz_stuff(afl, buf, len) != 0 {
        return 1 as libc::c_int as u8_0
    }
    new_hit_cnt =
        ((*afl).queued_paths as
             libc::c_ulonglong).wrapping_add((*afl).unique_crashes);
    if new_hit_cnt != orig_hit_cnt {
        *status = 1 as libc::c_int as u8_0
    } else { *status = 2 as libc::c_int as u8_0 }
    return 0 as libc::c_int as u8_0;
}
unsafe extern "C" fn cmp_extend_encoding(mut afl: *mut afl_state_t,
                                         mut h: *mut cmp_header,
                                         mut pattern: u64_0, mut repl: u64_0,
                                         mut idx: u32_0,
                                         mut orig_buf: *mut u8_0,
                                         mut buf: *mut u8_0, mut len: u32_0,
                                         mut do_reverse: u8_0,
                                         mut status: *mut u8_0) -> u8_0 {
    let mut buf_64: *mut u64_0 =
        &mut *buf.offset(idx as isize) as *mut u8_0 as *mut u64_0;
    let mut buf_32: *mut u32_0 =
        &mut *buf.offset(idx as isize) as *mut u8_0 as *mut u32_0;
    let mut buf_16: *mut u16_0 =
        &mut *buf.offset(idx as isize) as *mut u8_0 as *mut u16_0;
    // u8*  buf_8  = &buf[idx];
  // u64* o_buf_64 = (u64*)&orig_buf[idx];
  // u32* o_buf_32 = (u32*)&orig_buf[idx];
  // u16* o_buf_16 = (u16*)&orig_buf[idx];
  // u8*  o_buf_8  = &orig_buf[idx];
    let mut its_len: u32_0 = len.wrapping_sub(idx);
    *status = 0 as libc::c_int as u8_0;
    if (*h).shape() as libc::c_int + 1 as libc::c_int == 8 as libc::c_int {
        if its_len >= 8 as libc::c_int as libc::c_uint && *buf_64 == pattern {
            // && *o_buf_64 == pattern) {
            *buf_64 = repl;
            if its_fuzz(afl, buf, len, status) != 0 {
                return 1 as libc::c_int as u8_0
            }
            *buf_64 = pattern
        }
        // reverse encoding
        if do_reverse != 0 {
            if cmp_extend_encoding(afl, h,
                                   ({
                                        let mut _ret: u64_0 = pattern;
                                        _ret =
                                            (_ret &
                                                 0xffffffff as libc::c_uint as
                                                     libc::c_ulonglong) <<
                                                32 as libc::c_int |
                                                (_ret &
                                                     0xffffffff00000000 as
                                                         libc::c_ulong as
                                                         libc::c_ulonglong) >>
                                                    32 as libc::c_int;
                                        _ret =
                                            (_ret &
                                                 0xffff0000ffff as
                                                     libc::c_long as
                                                     libc::c_ulonglong) <<
                                                16 as libc::c_int |
                                                (_ret &
                                                     0xffff0000ffff0000 as
                                                         libc::c_ulong as
                                                         libc::c_ulonglong) >>
                                                    16 as libc::c_int;
                                        _ret =
                                            (_ret &
                                                 0xff00ff00ff00ff as
                                                     libc::c_long as
                                                     libc::c_ulonglong) <<
                                                8 as libc::c_int |
                                                (_ret &
                                                     0xff00ff00ff00ff00 as
                                                         libc::c_ulong as
                                                         libc::c_ulonglong) >>
                                                    8 as libc::c_int;
                                        _ret
                                    }),
                                   ({
                                        let mut _ret: u64_0 = repl;
                                        _ret =
                                            (_ret &
                                                 0xffffffff as libc::c_uint as
                                                     libc::c_ulonglong) <<
                                                32 as libc::c_int |
                                                (_ret &
                                                     0xffffffff00000000 as
                                                         libc::c_ulong as
                                                         libc::c_ulonglong) >>
                                                    32 as libc::c_int;
                                        _ret =
                                            (_ret &
                                                 0xffff0000ffff as
                                                     libc::c_long as
                                                     libc::c_ulonglong) <<
                                                16 as libc::c_int |
                                                (_ret &
                                                     0xffff0000ffff0000 as
                                                         libc::c_ulong as
                                                         libc::c_ulonglong) >>
                                                    16 as libc::c_int;
                                        _ret =
                                            (_ret &
                                                 0xff00ff00ff00ff as
                                                     libc::c_long as
                                                     libc::c_ulonglong) <<
                                                8 as libc::c_int |
                                                (_ret &
                                                     0xff00ff00ff00ff00 as
                                                         libc::c_ulong as
                                                         libc::c_ulonglong) >>
                                                    8 as libc::c_int;
                                        _ret
                                    }), idx, orig_buf, buf, len,
                                   0 as libc::c_int as u8_0, status) != 0 {
                return 1 as libc::c_int as u8_0
            }
        }
    }
    if (*h).shape() as libc::c_int + 1 as libc::c_int == 4 as libc::c_int ||
           *status as libc::c_int == 2 as libc::c_int {
        if its_len >= 4 as libc::c_int as libc::c_uint &&
               *buf_32 == pattern as u32_0 {
            // && *o_buf_32 == (u32)pattern) {
            *buf_32 = repl as u32_0;
            if its_fuzz(afl, buf, len, status) != 0 {
                return 1 as libc::c_int as u8_0
            }
            *buf_32 = pattern as u32_0
        }
        // reverse encoding
        if do_reverse != 0 {
            if cmp_extend_encoding(afl, h,
                                   ({
                                        let mut _ret: u32_0 =
                                            pattern as u32_0;
                                        (_ret << 24 as libc::c_int |
                                             _ret >> 24 as libc::c_int |
                                             _ret << 8 as libc::c_int &
                                                 0xff0000 as libc::c_int as
                                                     libc::c_uint) |
                                            _ret >> 8 as libc::c_int &
                                                0xff00 as libc::c_int as
                                                    libc::c_uint
                                    }) as u64_0,
                                   ({
                                        let mut _ret: u32_0 = repl as u32_0;
                                        (_ret << 24 as libc::c_int |
                                             _ret >> 24 as libc::c_int |
                                             _ret << 8 as libc::c_int &
                                                 0xff0000 as libc::c_int as
                                                     libc::c_uint) |
                                            _ret >> 8 as libc::c_int &
                                                0xff00 as libc::c_int as
                                                    libc::c_uint
                                    }) as u64_0, idx, orig_buf, buf, len,
                                   0 as libc::c_int as u8_0, status) != 0 {
                return 1 as libc::c_int as u8_0
            }
        }
    }
    if (*h).shape() as libc::c_int + 1 as libc::c_int == 2 as libc::c_int ||
           *status as libc::c_int == 2 as libc::c_int {
        if its_len >= 2 as libc::c_int as libc::c_uint &&
               *buf_16 as libc::c_int == pattern as u16_0 as libc::c_int {
            // && *o_buf_16 == (u16)pattern) {
            *buf_16 = repl as u16_0;
            if its_fuzz(afl, buf, len, status) != 0 {
                return 1 as libc::c_int as u8_0
            }
            *buf_16 = pattern as u16_0
        }
        // reverse encoding
        if do_reverse != 0 {
            if cmp_extend_encoding(afl, h,
                                   ({
                                        let mut _ret: u16_0 =
                                            pattern as u16_0;
                                        ((_ret as libc::c_int) <<
                                             8 as libc::c_int |
                                             _ret as libc::c_int >>
                                                 8 as libc::c_int) as u16_0
                                    }) as u64_0,
                                   ({
                                        let mut _ret: u16_0 = repl as u16_0;
                                        ((_ret as libc::c_int) <<
                                             8 as libc::c_int |
                                             _ret as libc::c_int >>
                                                 8 as libc::c_int) as u16_0
                                    }) as u64_0, idx, orig_buf, buf, len,
                                   0 as libc::c_int as u8_0, status) != 0 {
                return 1 as libc::c_int as u8_0
            }
        }
    }
    /*if (SHAPE_BYTES(h->shape) == 1 || *status == 2) {

    if (its_len >= 2 && *buf_8 == (u8)pattern) {// && *o_buf_8 == (u8)pattern) {

      *buf_8 = (u8)repl;
      if (unlikely(its_fuzz(afl, buf, len, status)))
        return 1;
      *buf_16 = (u16)pattern;

    }

  }*/
    return 0 as libc::c_int as u8_0;
}
unsafe extern "C" fn try_to_add_to_dict(mut afl: *mut afl_state_t,
                                        mut v: u64_0, mut shape: u8_0) {
    let mut b: *mut u8_0 = &mut v as *mut u64_0 as *mut u8_0;
    let mut k: u32_0 = 0;
    let mut cons_ff: u8_0 = 0 as libc::c_int as u8_0;
    let mut cons_0: u8_0 = 0 as libc::c_int as u8_0;
    k = 0 as libc::c_int as u32_0;
    while k < shape as libc::c_uint {
        if *b.offset(k as isize) as libc::c_int == 0 as libc::c_int {
            cons_0 = cons_0.wrapping_add(1)
        } else if *b.offset(k as isize) as libc::c_int == 0xff as libc::c_int
         {
            cons_0 = cons_0.wrapping_add(1)
        } else { cons_ff = 0 as libc::c_int as u8_0; cons_0 = cons_ff }
        if cons_0 as libc::c_int > 1 as libc::c_int ||
               cons_ff as libc::c_int > 1 as libc::c_int {
            return
        }
        k = k.wrapping_add(1)
    }
    maybe_add_auto(afl as *mut u8_0 as *mut libc::c_void,
                   &mut v as *mut u64_0 as *mut u8_0, shape as u32_0);
    let mut rev: u64_0 = 0;
    match shape as libc::c_int {
        2 => {
            rev =
                ({
                     let mut _ret: u16_0 = v as u16_0;
                     ((_ret as libc::c_int) << 8 as libc::c_int |
                          _ret as libc::c_int >> 8 as libc::c_int) as u16_0
                 }) as u64_0;
            maybe_add_auto(afl as *mut u8_0 as *mut libc::c_void,
                           &mut rev as *mut u64_0 as *mut u8_0,
                           shape as u32_0);
        }
        4 => {
            rev =
                ({
                     let mut _ret: u32_0 = v as u32_0;
                     (_ret << 24 as libc::c_int | _ret >> 24 as libc::c_int |
                          _ret << 8 as libc::c_int &
                              0xff0000 as libc::c_int as libc::c_uint) |
                         _ret >> 8 as libc::c_int &
                             0xff00 as libc::c_int as libc::c_uint
                 }) as u64_0;
            maybe_add_auto(afl as *mut u8_0 as *mut libc::c_void,
                           &mut rev as *mut u64_0 as *mut u8_0,
                           shape as u32_0);
        }
        8 => {
            rev =
                ({
                     let mut _ret: u64_0 = v;
                     _ret =
                         (_ret &
                              0xffffffff as libc::c_uint as libc::c_ulonglong)
                             << 32 as libc::c_int |
                             (_ret &
                                  0xffffffff00000000 as libc::c_ulong as
                                      libc::c_ulonglong) >> 32 as libc::c_int;
                     _ret =
                         (_ret &
                              0xffff0000ffff as libc::c_long as
                                  libc::c_ulonglong) << 16 as libc::c_int |
                             (_ret &
                                  0xffff0000ffff0000 as libc::c_ulong as
                                      libc::c_ulonglong) >> 16 as libc::c_int;
                     _ret =
                         (_ret &
                              0xff00ff00ff00ff as libc::c_long as
                                  libc::c_ulonglong) << 8 as libc::c_int |
                             (_ret &
                                  0xff00ff00ff00ff00 as libc::c_ulong as
                                      libc::c_ulonglong) >> 8 as libc::c_int;
                     _ret
                 });
            maybe_add_auto(afl as *mut u8_0 as *mut libc::c_void,
                           &mut rev as *mut u64_0 as *mut u8_0,
                           shape as u32_0);
        }
        1 | _ => { }
    };
}
unsafe extern "C" fn cmp_fuzz(mut afl: *mut afl_state_t, mut key: u32_0,
                              mut orig_buf: *mut u8_0, mut buf: *mut u8_0,
                              mut len: u32_0) -> u8_0 {
    let mut h: *mut cmp_header =
        &mut *(*(*afl).shm.cmp_map).headers.as_mut_ptr().offset(key as isize)
            as *mut cmp_header;
    let mut i: u32_0 = 0;
    let mut j: u32_0 = 0;
    let mut idx: u32_0 = 0;
    let mut loggeds: u32_0 = (*h).hits();
    if (*h).hits() as libc::c_int > 256 as libc::c_int {
        loggeds = 256 as libc::c_int as u32_0
    }
    let mut status: u8_0 = 0;
    // opt not in the paper
    let mut fails: u32_0 = 0 as libc::c_int as u32_0;
    i = 0 as libc::c_int as u32_0;
    while i < loggeds {
        let mut current_block_16: u64;
        let mut o: *mut cmp_operands =
            &mut *(*(*(*afl).shm.cmp_map).log.as_mut_ptr().offset(key as
                                                                      isize)).as_mut_ptr().offset(i
                                                                                                      as
                                                                                                      isize)
                as *mut cmp_operands;
        // opt not in the paper
        j = 0 as libc::c_int as u32_0;
        loop  {
            if !(j < i) { current_block_16 = 13109137661213826276; break ; }
            if (*(*afl).shm.cmp_map).log[key as usize][j as usize].v0 ==
                   (*o).v0 &&
                   (*(*afl).shm.cmp_map).log[key as usize][i as usize].v1 ==
                       (*o).v1 {
                current_block_16 = 10272526490169719578;
                break ;
            }
            j = j.wrapping_add(1)
        }
        match current_block_16 {
            13109137661213826276 => {
                idx = 0 as libc::c_int as u32_0;
                while idx < len && fails < 8 as libc::c_int as libc::c_uint {
                    if cmp_extend_encoding(afl, h, (*o).v0, (*o).v1, idx,
                                           orig_buf, buf, len,
                                           1 as libc::c_int as u8_0,
                                           &mut status) != 0 {
                        return 1 as libc::c_int as u8_0
                    }
                    if status as libc::c_int == 2 as libc::c_int {
                        fails = fails.wrapping_add(1)
                    } else if status as libc::c_int == 1 as libc::c_int {
                        break ;
                    }
                    if cmp_extend_encoding(afl, h, (*o).v1, (*o).v0, idx,
                                           orig_buf, buf, len,
                                           1 as libc::c_int as u8_0,
                                           &mut status) != 0 {
                        return 1 as libc::c_int as u8_0
                    }
                    if status as libc::c_int == 2 as libc::c_int {
                        fails = fails.wrapping_add(1)
                    } else if status as libc::c_int == 1 as libc::c_int {
                        break ;
                    }
                    idx = idx.wrapping_add(1)
                }
                // If failed, add to dictionary
                if fails == 8 as libc::c_int as libc::c_uint {
                    try_to_add_to_dict(afl, (*o).v0,
                                       ((*h).shape() as libc::c_int +
                                            1 as libc::c_int) as u8_0);
                    try_to_add_to_dict(afl, (*o).v1,
                                       ((*h).shape() as libc::c_int +
                                            1 as libc::c_int) as u8_0);
                }
            }
            _ => { }
        }
        (*afl).stage_cur += 1;
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int as u8_0;
}
unsafe extern "C" fn rtn_extend_encoding(mut afl: *mut afl_state_t,
                                         mut h: *mut cmp_header,
                                         mut pattern: *mut u8_0,
                                         mut repl: *mut u8_0, mut idx: u32_0,
                                         mut orig_buf: *mut u8_0,
                                         mut buf: *mut u8_0, mut len: u32_0,
                                         mut status: *mut u8_0) -> u8_0 {
    let mut i: u32_0 = 0;
    let mut its_len: u32_0 =
        ({
             let mut _a: libc::c_int = 32 as libc::c_int;
             let mut _b: libc::c_uint = len.wrapping_sub(idx);
             if (_a as libc::c_uint) < _b { _a as libc::c_uint } else { _b }
         });
    let mut save: [u8_0; 32] = [0; 32];
    memcpy(save.as_mut_ptr() as *mut libc::c_void,
           &mut *buf.offset(idx as isize) as *mut u8_0 as *const libc::c_void,
           its_len as libc::c_ulong);
    *status = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int as u32_0;
    while i < its_len {
        if *pattern.offset(idx.wrapping_add(i) as isize) as libc::c_int !=
               *buf.offset(idx.wrapping_add(i) as isize) as libc::c_int ||
               *status as libc::c_int == 1 as libc::c_int {
            break ;
        }
        *buf.offset(idx.wrapping_add(i) as isize) =
            *repl.offset(idx.wrapping_add(i) as isize);
        if its_fuzz(afl, buf, len, status) != 0 {
            return 1 as libc::c_int as u8_0
        }
        i = i.wrapping_add(1)
    }
    memcpy(&mut *buf.offset(idx as isize) as *mut u8_0 as *mut libc::c_void,
           save.as_mut_ptr() as *const libc::c_void, i as libc::c_ulong);
    return 0 as libc::c_int as u8_0;
}
unsafe extern "C" fn rtn_fuzz(mut afl: *mut afl_state_t, mut key: u32_0,
                              mut orig_buf: *mut u8_0, mut buf: *mut u8_0,
                              mut len: u32_0) -> u8_0 {
    let mut h: *mut cmp_header =
        &mut *(*(*afl).shm.cmp_map).headers.as_mut_ptr().offset(key as isize)
            as *mut cmp_header;
    let mut i: u32_0 = 0;
    let mut j: u32_0 = 0;
    let mut idx: u32_0 = 0;
    let mut loggeds: u32_0 = (*h).hits();
    if (*h).hits() as libc::c_int > 256 as libc::c_int / 4 as libc::c_int {
        loggeds = (256 as libc::c_int / 4 as libc::c_int) as u32_0
    }
    let mut status: u8_0 = 0;
    // opt not in the paper
    let mut fails: u32_0 = 0 as libc::c_int as u32_0;
    i = 0 as libc::c_int as u32_0;
    while i < loggeds {
        let mut current_block_16: u64;
        let mut o: *mut cmpfn_operands =
            &mut *((*(*(*afl).shm.cmp_map).log.as_mut_ptr().offset(key as
                                                                       isize)).as_mut_ptr()
                       as *mut cmpfn_operands).offset(i as isize) as
                *mut cmpfn_operands;
        // opt not in the paper
        j = 0 as libc::c_int as u32_0;
        loop  {
            if !(j < i) { current_block_16 = 13109137661213826276; break ; }
            if memcmp(&mut *((*(*(*afl).shm.cmp_map).log.as_mut_ptr().offset(key
                                                                                 as
                                                                                 isize)).as_mut_ptr()
                                 as *mut cmpfn_operands).offset(j as isize) as
                          *mut cmpfn_operands as *const libc::c_void,
                      o as *const libc::c_void,
                      ::std::mem::size_of::<cmpfn_operands>() as
                          libc::c_ulong) == 0 {
                current_block_16 = 14705049148686875255;
                break ;
            }
            j = j.wrapping_add(1)
        }
        match current_block_16 {
            13109137661213826276 => {
                idx = 0 as libc::c_int as u32_0;
                while idx < len && fails < 8 as libc::c_int as libc::c_uint {
                    if rtn_extend_encoding(afl, h, (*o).v0.as_mut_ptr(),
                                           (*o).v1.as_mut_ptr(), idx,
                                           orig_buf, buf, len, &mut status) !=
                           0 {
                        return 1 as libc::c_int as u8_0
                    }
                    if status as libc::c_int == 2 as libc::c_int {
                        fails = fails.wrapping_add(1)
                    } else if status as libc::c_int == 1 as libc::c_int {
                        break ;
                    }
                    if rtn_extend_encoding(afl, h, (*o).v1.as_mut_ptr(),
                                           (*o).v0.as_mut_ptr(), idx,
                                           orig_buf, buf, len, &mut status) !=
                           0 {
                        return 1 as libc::c_int as u8_0
                    }
                    if status as libc::c_int == 2 as libc::c_int {
                        fails = fails.wrapping_add(1)
                    } else if status as libc::c_int == 1 as libc::c_int {
                        break ;
                    }
                    idx = idx.wrapping_add(1)
                }
                // If failed, add to dictionary
                if fails == 8 as libc::c_int as libc::c_uint {
                    maybe_add_auto(afl as *mut u8_0 as *mut libc::c_void,
                                   (*o).v0.as_mut_ptr(),
                                   ((*h).shape() as libc::c_int +
                                        1 as libc::c_int) as u32_0);
                    maybe_add_auto(afl as *mut u8_0 as *mut libc::c_void,
                                   (*o).v1.as_mut_ptr(),
                                   ((*h).shape() as libc::c_int +
                                        1 as libc::c_int) as u32_0);
                }
            }
            _ => { }
        }
        (*afl).stage_cur += 1;
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int as u8_0;
}
/* RedQueen */
// /// Input to State stage
// afl->queue_cur->exec_cksum
#[no_mangle]
pub unsafe extern "C" fn input_to_state_stage(mut afl: *mut afl_state_t,
                                              mut orig_buf: *mut u8_0,
                                              mut buf: *mut u8_0,
                                              mut len: u32_0,
                                              mut exec_cksum: u32_0) -> u8_0 {
    let mut current_block: u64;
    let mut r: u8_0 = 1 as libc::c_int as u8_0;
    if colorization(afl, buf, len, exec_cksum) != 0 {
        return 1 as libc::c_int as u8_0
    }
    // do it manually, forkserver clear only afl->fsrv.trace_bits
    memset((*(*afl).shm.cmp_map).headers.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[cmp_header; 65536]>() as libc::c_ulong);
    if common_fuzz_cmplog_stuff(afl, buf, len) != 0 {
        return 1 as libc::c_int as u8_0
    }
    let mut orig_hit_cnt: u64_0 = 0;
    let mut new_hit_cnt: u64_0 = 0;
    let mut orig_execs: u64_0 = (*afl).total_execs;
    orig_hit_cnt =
        ((*afl).queued_paths as
             libc::c_ulonglong).wrapping_add((*afl).unique_crashes);
    (*afl).stage_name =
        b"input-to-state\x00" as *const u8 as *const libc::c_char as
            *mut u8_0;
    (*afl).stage_short =
        b"its\x00" as *const u8 as *const libc::c_char as *mut u8_0;
    (*afl).stage_max = 0 as libc::c_int;
    (*afl).stage_cur = 0 as libc::c_int;
    let mut k: u32_0 = 0;
    k = 0 as libc::c_int as u32_0;
    while k < 65536 as libc::c_int as libc::c_uint {
        if !((*(*afl).shm.cmp_map).headers[k as usize].hits() == 0) {
            if (*(*afl).shm.cmp_map).headers[k as usize].type_0() as
                   libc::c_int == 0 as libc::c_int {
                (*afl).stage_max =
                    ((*afl).stage_max as
                         libc::c_uint).wrapping_add(({
                                                         let mut _a: u32_0 =
                                                             (*(*afl).shm.cmp_map).headers[k
                                                                                               as
                                                                                               usize].hits();
                                                         let mut _b:
                                                                 libc::c_int =
                                                             256 as
                                                                 libc::c_int;
                                                         if _a <
                                                                _b as
                                                                    libc::c_uint
                                                            {
                                                             _a
                                                         } else {
                                                             _b as
                                                                 libc::c_uint
                                                         }
                                                     })) as s32 as s32
            } else {
                (*afl).stage_max =
                    ((*afl).stage_max as
                         libc::c_uint).wrapping_add(({
                                                         let mut _a: u32_0 =
                                                             (*(*afl).shm.cmp_map).headers[k
                                                                                               as
                                                                                               usize].hits();
                                                         let mut _b:
                                                                 libc::c_int =
                                                             256 as
                                                                 libc::c_int /
                                                                 4 as
                                                                     libc::c_int;
                                                         if _a <
                                                                _b as
                                                                    libc::c_uint
                                                            {
                                                             _a
                                                         } else {
                                                             _b as
                                                                 libc::c_uint
                                                         }
                                                     })) as s32 as s32
            }
        }
        k = k.wrapping_add(1)
    }
    k = 0 as libc::c_int as u32_0;
    loop  {
        if !(k < 65536 as libc::c_int as libc::c_uint) {
            current_block = 18386322304582297246;
            break ;
        }
        if !((*(*afl).shm.cmp_map).headers[k as usize].hits() == 0) {
            if (*(*afl).shm.cmp_map).headers[k as usize].type_0() as
                   libc::c_int == 0 as libc::c_int {
                if cmp_fuzz(afl, k, orig_buf, buf, len) != 0 {
                    current_block = 13999925517074022731;
                    break ;
                }
            } else if rtn_fuzz(afl, k, orig_buf, buf, len) != 0 {
                current_block = 13999925517074022731;
                break ;
            }
        }
        k = k.wrapping_add(1)
    }
    match current_block {
        18386322304582297246 => { r = 0 as libc::c_int as u8_0 }
        _ => { }
    }
    memcpy(orig_buf as *mut libc::c_void, buf as *const libc::c_void,
           len as libc::c_ulong);
    new_hit_cnt =
        ((*afl).queued_paths as
             libc::c_ulonglong).wrapping_add((*afl).unique_crashes);
    (*afl).stage_finds[STAGE_ITS as libc::c_int as usize] =
        ((*afl).stage_finds[STAGE_ITS as libc::c_int as usize] as
             libc::c_ulonglong).wrapping_add(new_hit_cnt.wrapping_sub(orig_hit_cnt))
            as u64_0 as u64_0;
    (*afl).stage_cycles[STAGE_ITS as libc::c_int as usize] =
        ((*afl).stage_cycles[STAGE_ITS as libc::c_int as usize] as
             libc::c_ulonglong).wrapping_add((*afl).total_execs.wrapping_sub(orig_execs))
            as u64_0 as u64_0;
    return r;
}
