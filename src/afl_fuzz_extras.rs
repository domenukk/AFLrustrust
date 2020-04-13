use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type cmp_map;
    pub type __dirstream;
    #[no_mangle]
    fn access(__name: *const libc::c_char, __type: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn random() -> libc::c_long;
    #[no_mangle]
    fn srandom(__seed: libc::c_uint);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    /* Describe integer as memory size. */
    #[no_mangle]
    fn stringify_mem_size(buf: *mut u8_0, len: size_t, val: u64_0)
     -> *mut u8_0;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    static mut interesting_16: [s16; 19];
    #[no_mangle]
    static mut interesting_32: [s32; 27];
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
/* Re-allocate a buffer, checking for issues and zeroing any newly-added tail.
   With DEBUG_BUILD, the buffer is always reallocated to a new addresses and the
   old memory is clobbered with 0xFF. */
#[inline]
unsafe extern "C" fn DFL_ck_realloc(mut orig: *mut libc::c_void,
                                    mut size: u32_0) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 { DFL_ck_free(orig); return 0 as *mut libc::c_void }
    if size > 0x40000000 as libc::c_int as libc::c_uint {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBad alloc request: %u bytes\x00"
                   as *const u8 as *const libc::c_char, size);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"include/alloc-inl.h\x00" as *const u8 as *const libc::c_char,
               139 as libc::c_int);
        abort();
    }
    /* Catch pointer issues sooner: force relocation and make sure that the
     original buffer is wiped. */
    ret = realloc(orig, size as libc::c_ulong);
    if ret.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mOut of memory: can\'t allocate %u bytes\x00"
                   as *const u8 as *const libc::c_char, size);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"include/alloc-inl.h\x00" as *const u8 as *const libc::c_char,
               146 as libc::c_int);
        abort();
    }
    return ret;
}
/* Re-allocate a buffer with ALLOC_BLK_INC increments (used to speed up
   repeated small reallocs without complicating the user code). */
#[inline]
unsafe extern "C" fn DFL_ck_realloc_block(mut orig: *mut libc::c_void,
                                          mut size: u32_0)
 -> *mut libc::c_void {
    return DFL_ck_realloc(orig, size);
}
/* Create a buffer with a copy of a memory block. Returns NULL for zero-sized
   or NULL inputs. */
#[inline]
unsafe extern "C" fn DFL_ck_memdup(mut mem: *mut libc::c_void,
                                   mut size: u32_0) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if mem.is_null() || size == 0 { return 0 as *mut libc::c_void }
    if size > 0x40000000 as libc::c_int as libc::c_uint {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBad alloc request: %u bytes\x00"
                   as *const u8 as *const libc::c_char, size);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"include/alloc-inl.h\x00" as *const u8 as *const libc::c_char,
               189 as libc::c_int);
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
               191 as libc::c_int);
        abort();
    }
    return memcpy(ret, mem, size as libc::c_ulong);
}
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
/*
   american fuzzy lop++ - extras relates routines
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
/* Helper function for load_extras. */
unsafe extern "C" fn compare_extras_len(mut p1: *const libc::c_void,
                                        mut p2: *const libc::c_void)
 -> libc::c_int {
    let mut e1: *mut extra_data = p1 as *mut extra_data;
    let mut e2: *mut extra_data = p2 as *mut extra_data;
    return (*e1).len.wrapping_sub((*e2).len) as libc::c_int;
}
unsafe extern "C" fn compare_extras_use_d(mut p1: *const libc::c_void,
                                          mut p2: *const libc::c_void)
 -> libc::c_int {
    let mut e1: *mut extra_data = p1 as *mut extra_data;
    let mut e2: *mut extra_data = p2 as *mut extra_data;
    return (*e2).hit_cnt.wrapping_sub((*e1).hit_cnt) as libc::c_int;
}
/* Read extras from a file, sort by size. */
#[no_mangle]
pub unsafe extern "C" fn load_extras_file(mut afl: *mut afl_state_t,
                                          mut fname: *mut u8_0,
                                          mut min_len: *mut u32_0,
                                          mut max_len: *mut u32_0,
                                          mut dict_level: u32_0) {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut buf: [u8_0; 8192] = [0; 8192];
    let mut lptr: *mut u8_0 = 0 as *mut u8_0;
    let mut cur_line: u32_0 = 0 as libc::c_int as u32_0;
    let mut val_bufs: [[u8_0; 16]; 2] = [[0; 16]; 2];
    f =
        fopen(fname as *const libc::c_char,
              b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                   as *const u8 as *const libc::c_char, fname);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-extras.c\x00" as *const u8 as
                   *const libc::c_char, 62 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    loop  {
        lptr =
            fgets(buf.as_mut_ptr() as *mut libc::c_char, 8192 as libc::c_int,
                  f) as *mut u8_0;
        if lptr.is_null() { break ; }
        let mut rptr: *mut u8_0 = 0 as *mut u8_0;
        let mut wptr: *mut u8_0 = 0 as *mut u8_0;
        let mut klen: u32_0 = 0 as libc::c_int as u32_0;
        cur_line = cur_line.wrapping_add(1);
        /* Trim on left and right. */
        while *(*__ctype_b_loc()).offset(*lptr as libc::c_int as isize) as
                  libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
            lptr = lptr.offset(1)
        }
        rptr =
            lptr.offset(strlen(lptr as *const libc::c_char) as
                            isize).offset(-(1 as libc::c_int as isize));
        while rptr >= lptr &&
                  *(*__ctype_b_loc()).offset(*rptr as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
            rptr = rptr.offset(-1)
        }
        rptr = rptr.offset(1);
        *rptr = 0 as libc::c_int as u8_0;
        /* Skip empty lines and comments. */
        if *lptr == 0 || *lptr as libc::c_int == '#' as i32 { continue ; }
        /* All other lines must end with '"', which we can consume. */
        rptr = rptr.offset(-1);
        if rptr < lptr || *rptr as libc::c_int != '\"' as i32 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMalformed name=\"value\" pair in line %u.\x00"
                       as *const u8 as *const libc::c_char, cur_line);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-extras.c\x00" as *const u8 as
                       *const libc::c_char, 91 as libc::c_int);
            exit(1 as libc::c_int);
        }
        *rptr = 0 as libc::c_int as u8_0;
        /* Skip alphanumerics and dashes (label). */
        while *(*__ctype_b_loc()).offset(*lptr as libc::c_int as isize) as
                  libc::c_int &
                  _ISalnum as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 || *lptr as libc::c_int == '_' as i32 {
            lptr = lptr.offset(1)
        }
        /* If @number follows, parse that. */
        if *lptr as libc::c_int == '@' as i32 {
            lptr = lptr.offset(1);
            if atoi(lptr as *const libc::c_char) as libc::c_uint > dict_level
               {
                continue ;
            }
            while *(*__ctype_b_loc()).offset(*lptr as libc::c_int as isize) as
                      libc::c_int &
                      _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
                lptr = lptr.offset(1)
            }
        }
        /* Skip whitespace and = signs. */
        while *(*__ctype_b_loc()).offset(*lptr as libc::c_int as isize) as
                  libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 || *lptr as libc::c_int == '=' as i32 {
            lptr = lptr.offset(1)
        }
        /* Consume opening '"'. */
        if *lptr as libc::c_int != '\"' as i32 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMalformed name=\"keyword\" pair in line %u.\x00"
                       as *const u8 as *const libc::c_char, cur_line);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-extras.c\x00" as *const u8 as
                       *const libc::c_char, 119 as libc::c_int);
            exit(1 as libc::c_int);
        }
        lptr = lptr.offset(1);
        if *lptr == 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mEmpty keyword in line %u.\x00"
                       as *const u8 as *const libc::c_char, cur_line);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-extras.c\x00" as *const u8 as
                       *const libc::c_char, 123 as libc::c_int);
            exit(1 as libc::c_int);
        }
        /* Okay, let's allocate memory and copy data between "...", handling
       \xNN escaping, \\, and \". */
        (*afl).extras =
            DFL_ck_realloc_block((*afl).extras as *mut libc::c_void,
                                 ((*afl).extras_cnt.wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                      as
                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<extra_data>()
                                                                      as
                                                                      libc::c_ulong)
                                     as u32_0) as *mut extra_data;
        let ref mut fresh1 =
            (*(*afl).extras.offset((*afl).extras_cnt as isize)).data;
        *fresh1 =
            DFL_ck_alloc(rptr.wrapping_offset_from(lptr) as libc::c_long as
                             u32_0) as *mut u8_0;
        wptr = *fresh1;
        if wptr.is_null() {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mno mem for data\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-extras.c\x00" as *const u8 as
                       *const libc::c_char, 133 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        while *lptr != 0 {
            let mut hexdigits: *mut libc::c_char =
                b"0123456789abcdef\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            match *lptr as libc::c_int {
                1 | 128 => {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mNon-printable characters in line %u.\x00"
                               as *const u8 as *const libc::c_char, cur_line);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz-extras.c\x00" as *const u8 as
                               *const libc::c_char, 143 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                92 => {
                    lptr = lptr.offset(1);
                    if *lptr as libc::c_int == '\\' as i32 ||
                           *lptr as libc::c_int == '\"' as i32 {
                        let fresh2 = lptr;
                        lptr = lptr.offset(1);
                        let fresh3 = wptr;
                        wptr = wptr.offset(1);
                        *fresh3 = *fresh2;
                        klen = klen.wrapping_add(1)
                    } else {
                        if *lptr as libc::c_int != 'x' as i32 ||
                               *(*__ctype_b_loc()).offset(*lptr.offset(1 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                                              as libc::c_int
                                                              as isize) as
                                   libc::c_int &
                                   _ISxdigit as libc::c_int as libc::c_ushort
                                       as libc::c_int == 0 ||
                               *(*__ctype_b_loc()).offset(*lptr.offset(2 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                                              as libc::c_int
                                                              as isize) as
                                   libc::c_int &
                                   _ISxdigit as libc::c_int as libc::c_ushort
                                       as libc::c_int == 0 {
                            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mInvalid escaping (not \\xNN) in line %u.\x00"
                                       as *const u8 as *const libc::c_char,
                                   cur_line);
                            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   b"func_unknown\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"src/afl-fuzz-extras.c\x00" as *const u8
                                       as *const libc::c_char,
                                   158 as libc::c_int);
                            exit(1 as libc::c_int);
                        }
                        let fresh4 = wptr;
                        wptr = wptr.offset(1);
                        *fresh4 =
                            ((strchr(hexdigits,
                                     tolower(*lptr.offset(1 as libc::c_int as
                                                              isize) as
                                                 libc::c_int)).wrapping_offset_from(hexdigits)
                                  as libc::c_long) << 4 as libc::c_int |
                                 strchr(hexdigits,
                                        tolower(*lptr.offset(2 as libc::c_int
                                                                 as isize) as
                                                    libc::c_int)).wrapping_offset_from(hexdigits)
                                     as libc::c_long) as u8_0;
                        lptr = lptr.offset(3 as libc::c_int as isize);
                        klen = klen.wrapping_add(1)
                    }
                }
                _ => {
                    let fresh5 = lptr;
                    lptr = lptr.offset(1);
                    let fresh6 = wptr;
                    wptr = wptr.offset(1);
                    *fresh6 = *fresh5;
                    klen = klen.wrapping_add(1)
                }
            }
        }
        (*(*afl).extras.offset((*afl).extras_cnt as isize)).len = klen;
        if (*(*afl).extras.offset((*afl).extras_cnt as isize)).len >
               128 as libc::c_int as libc::c_uint {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mKeyword too big in line %u (%s, limit is %s)\x00"
                       as *const u8 as *const libc::c_char, cur_line,
                   stringify_mem_size(val_bufs[0 as libc::c_int as
                                                   usize].as_mut_ptr(),
                                      ::std::mem::size_of::<[u8_0; 16]>() as
                                          libc::c_ulong, klen as u64_0),
                   stringify_mem_size(val_bufs[1 as libc::c_int as
                                                   usize].as_mut_ptr(),
                                      ::std::mem::size_of::<[u8_0; 16]>() as
                                          libc::c_ulong,
                                      128 as libc::c_int as u64_0));
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-extras.c\x00" as *const u8 as
                       *const libc::c_char, 180 as libc::c_int);
            exit(1 as libc::c_int);
        }
        if *min_len > klen { *min_len = klen }
        if *max_len < klen { *max_len = klen }
        (*afl).extras_cnt = (*afl).extras_cnt.wrapping_add(1)
    }
    fclose(f);
}
/* Read extras from the extras directory and sort them by size. */
#[no_mangle]
pub unsafe extern "C" fn load_extras(mut afl: *mut afl_state_t,
                                     mut dir: *mut u8_0) {
    let mut d: *mut DIR = 0 as *mut DIR;
    let mut de: *mut dirent = 0 as *mut dirent;
    let mut min_len: u32_0 = 128 as libc::c_int as u32_0;
    let mut max_len: u32_0 = 0 as libc::c_int as u32_0;
    let mut dict_level: u32_0 = 0 as libc::c_int as u32_0;
    let mut x: *mut u8_0 = 0 as *mut u8_0;
    let mut val_bufs: [[u8_0; 16]; 2] = [[0; 16]; 2];
    /* If the name ends with @, extract level and continue. */
    x = strchr(dir as *const libc::c_char, '@' as i32) as *mut u8_0;
    if !x.is_null() {
        *x = 0 as libc::c_int as u8_0;
        dict_level =
            atoi(x.offset(1 as libc::c_int as isize) as *const libc::c_char)
                as u32_0
    }
    printf(b"\x1b[1;94m[*] \x1b[0mLoading extra dictionary from \'%s\' (level %u)...\x00"
               as *const u8 as *const libc::c_char, dir, dict_level);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    d = opendir(dir as *const libc::c_char);
    if d.is_null() {
        if *__errno_location() == 20 as libc::c_int {
            load_extras_file(afl, dir, &mut min_len, &mut max_len,
                             dict_level);
        } else {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                       as *const u8 as *const libc::c_char, dir);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-extras.c\x00" as *const u8 as
                       *const libc::c_char, 226 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
    } else {
        if !x.is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mDictionary levels not supported for directories.\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-extras.c\x00" as *const u8 as
                       *const libc::c_char, 230 as libc::c_int);
            exit(1 as libc::c_int);
        }
        loop  {
            de = readdir(d);
            if de.is_null() { break ; }
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
            let mut fn_0: *mut u8_0 =
                ({
                     let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                     let mut _len: s32 =
                         snprintf(0 as *mut libc::c_char,
                                  0 as libc::c_int as libc::c_ulong,
                                  b"%s/%s\x00" as *const u8 as
                                      *const libc::c_char, dir,
                                  (*de).d_name.as_mut_ptr());
                     if _len < 0 as libc::c_int {
                         printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                    as *const u8 as *const libc::c_char);
                         printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                    as *const u8 as *const libc::c_char,
                                b"func_unknown\x00" as *const u8 as
                                    *const libc::c_char,
                                b"src/afl-fuzz-extras.c\x00" as *const u8 as
                                    *const libc::c_char, 235 as libc::c_int);
                         exit(1 as libc::c_int);
                     }
                     _tmp =
                         DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                             *mut u8_0;
                     snprintf(_tmp as *mut libc::c_char,
                              (_len + 1 as libc::c_int) as libc::c_ulong,
                              b"%s/%s\x00" as *const u8 as
                                  *const libc::c_char, dir,
                              (*de).d_name.as_mut_ptr());
                     _tmp
                 });
            let mut fd: s32 = 0;
            if lstat(fn_0 as *const libc::c_char, &mut st) != 0 ||
                   access(fn_0 as *const libc::c_char, 4 as libc::c_int) != 0
               {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to access \'%s\'\x00"
                           as *const u8 as *const libc::c_char, fn_0);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-extras.c\x00" as *const u8 as
                           *const libc::c_char, 238 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            }
            /* This also takes care of . and .. */
            if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
                     0o100000 as libc::c_int as libc::c_uint) ||
                   st.st_size == 0 {
                DFL_ck_free(fn_0 as *mut libc::c_void);
            } else {
                if st.st_size > 128 as libc::c_int as libc::c_long {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mExtra \'%s\' is too big (%s, limit is %s)\x00"
                               as *const u8 as *const libc::c_char, fn_0,
                           stringify_mem_size(val_bufs[0 as libc::c_int as
                                                           usize].as_mut_ptr(),
                                              ::std::mem::size_of::<[u8_0; 16]>()
                                                  as libc::c_ulong,
                                              st.st_size as u64_0),
                           stringify_mem_size(val_bufs[1 as libc::c_int as
                                                           usize].as_mut_ptr(),
                                              ::std::mem::size_of::<[u8_0; 16]>()
                                                  as libc::c_ulong,
                                              128 as libc::c_int as u64_0));
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz-extras.c\x00" as *const u8 as
                               *const libc::c_char, 252 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if min_len as libc::c_long > st.st_size {
                    min_len = st.st_size as u32_0
                }
                if (max_len as libc::c_long) < st.st_size {
                    max_len = st.st_size as u32_0
                }
                (*afl).extras =
                    DFL_ck_realloc_block((*afl).extras as *mut libc::c_void,
                                         ((*afl).extras_cnt.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                              as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<extra_data>()
                                                                              as
                                                                              libc::c_ulong)
                                             as u32_0) as *mut extra_data;
                let ref mut fresh7 =
                    (*(*afl).extras.offset((*afl).extras_cnt as isize)).data;
                *fresh7 = DFL_ck_alloc(st.st_size as u32_0) as *mut u8_0;
                (*(*afl).extras.offset((*afl).extras_cnt as isize)).len =
                    st.st_size as u32_0;
                fd = open(fn_0 as *const libc::c_char, 0 as libc::c_int);
                if fd < 0 as libc::c_int {
                    fflush(stdout);
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                               as *const u8 as *const libc::c_char, fn_0);
                    printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz-extras.c\x00" as *const u8 as
                               *const libc::c_char, 265 as libc::c_int);
                    printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                               *const u8 as *const libc::c_char,
                           strerror(*__errno_location()));
                    exit(1 as libc::c_int);
                }
                let mut _len: u32_0 = st.st_size as u32_0;
                let mut _res: s32 =
                    read(fd,
                         (*(*afl).extras.offset((*afl).extras_cnt as
                                                    isize)).data as
                             *mut libc::c_void, _len as size_t) as s32;
                if _res as libc::c_uint != _len {
                    if _res < 0 as libc::c_int {
                        fflush(stdout);
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort read from %s\x00"
                                   as *const u8 as *const libc::c_char, fn_0);
                        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-fuzz-extras.c\x00" as *const u8 as
                                   *const libc::c_char, 267 as libc::c_int);
                        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00"
                                   as *const u8 as *const libc::c_char,
                               strerror(*__errno_location()));
                        exit(1 as libc::c_int);
                    } else {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort read from %s\x00"
                                   as *const u8 as *const libc::c_char, fn_0);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-fuzz-extras.c\x00" as *const u8 as
                                   *const libc::c_char, 267 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                }
                close(fd);
                DFL_ck_free(fn_0 as *mut libc::c_void);
                (*afl).extras_cnt = (*afl).extras_cnt.wrapping_add(1)
            }
        }
        closedir(d);
    }
    if (*afl).extras_cnt == 0 {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mNo usable files in \'%s\'\x00"
                   as *const u8 as *const libc::c_char, dir);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-extras.c\x00" as *const u8 as
                   *const libc::c_char, 280 as libc::c_int);
        exit(1 as libc::c_int);
    }
    qsort((*afl).extras as *mut libc::c_void, (*afl).extras_cnt as size_t,
          ::std::mem::size_of::<extra_data>() as libc::c_ulong,
          Some(compare_extras_len as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
    printf(b"\x1b[1;92m[+] \x1b[0mLoaded %u extra tokens, size range %s to %s.\x00"
               as *const u8 as *const libc::c_char, (*afl).extras_cnt,
           stringify_mem_size(val_bufs[0 as libc::c_int as
                                           usize].as_mut_ptr(),
                              ::std::mem::size_of::<[u8_0; 16]>() as
                                  libc::c_ulong, min_len as u64_0),
           stringify_mem_size(val_bufs[1 as libc::c_int as
                                           usize].as_mut_ptr(),
                              ::std::mem::size_of::<[u8_0; 16]>() as
                                  libc::c_ulong, max_len as u64_0));
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    if max_len > 32 as libc::c_int as libc::c_uint {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mSome tokens are relatively large (%s) - consider trimming.\x00"
                   as *const u8 as *const libc::c_char,
               stringify_mem_size(val_bufs[0 as libc::c_int as
                                               usize].as_mut_ptr(),
                                  ::std::mem::size_of::<[u8_0; 16]>() as
                                      libc::c_ulong, max_len as u64_0));
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    if (*afl).extras_cnt > 200 as libc::c_int as libc::c_uint {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mMore than %d tokens - will use them probabilistically.\x00"
                   as *const u8 as *const libc::c_char, 200 as libc::c_int);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    };
}
/* Helper function for maybe_add_auto(afl, ) */
#[inline]
unsafe extern "C" fn memcmp_nocase(mut m1: *mut u8_0, mut m2: *mut u8_0,
                                   mut len: u32_0) -> u8_0 {
    loop  {
        let fresh8 = len;
        len = len.wrapping_sub(1);
        if !(fresh8 != 0) { break ; }
        let fresh9 = m1;
        m1 = m1.offset(1);
        let fresh10 = m2;
        m2 = m2.offset(1);
        if tolower(*fresh9 as libc::c_int) ^ tolower(*fresh10 as libc::c_int)
               != 0 {
            return 1 as libc::c_int as u8_0
        }
    }
    return 0 as libc::c_int as u8_0;
}
/* Maybe add automatic extra. */
/* Ugly hack: afl state is transfered as u8* because we import data via
   afl-forkserver.c - which is shared with other afl tools that do not
   have the afl state struct */
#[no_mangle]
pub unsafe extern "C" fn maybe_add_auto(mut afl_tmp: *mut libc::c_void,
                                        mut mem: *mut u8_0, mut len: u32_0) {
    let mut current_block: u64;
    let mut afl: *mut afl_state_t = afl_tmp as *mut afl_state_t;
    let mut i: u32_0 = 0;
    /* Allow users to specify that they don't want auto dictionaries. */
    if 128 as libc::c_int * 64 as libc::c_int == 0 || 128 as libc::c_int == 0
       {
        return
    }
    /* Skip runs of identical bytes. */
    i = 1 as libc::c_int as u32_0;
    while i < len {
        if *mem.offset(0 as libc::c_int as isize) as libc::c_int ^
               *mem.offset(i as isize) as libc::c_int != 0 {
            break ;
        }
        i = i.wrapping_add(1)
    }
    if i == len { return }
    /* Reject builtin interesting values. */
    if len == 2 as libc::c_int as libc::c_uint {
        i =
            (::std::mem::size_of::<[s16; 19]>() as libc::c_ulong >>
                 1 as libc::c_int) as u32_0;
        loop  {
            let fresh11 = i;
            i = i.wrapping_sub(1);
            if !(fresh11 != 0) { break ; }
            if *(mem as *mut u16_0) as libc::c_int ==
                   interesting_16[i as usize] as libc::c_int ||
                   *(mem as *mut u16_0) as libc::c_int ==
                       ({
                            let mut _ret: u16_0 =
                                interesting_16[i as usize] as u16_0;
                            ((_ret as libc::c_int) << 8 as libc::c_int |
                                 _ret as libc::c_int >> 8 as libc::c_int) as
                                u16_0
                        }) as libc::c_int {
                return
            }
        }
    }
    if len == 4 as libc::c_int as libc::c_uint {
        i =
            (::std::mem::size_of::<[s32; 27]>() as libc::c_ulong >>
                 2 as libc::c_int) as u32_0;
        loop  {
            let fresh12 = i;
            i = i.wrapping_sub(1);
            if !(fresh12 != 0) { break ; }
            if *(mem as *mut u32_0) ==
                   interesting_32[i as usize] as libc::c_uint ||
                   *(mem as *mut u32_0) ==
                       ({
                            let mut _ret: u32_0 =
                                interesting_32[i as usize] as u32_0;
                            (_ret << 24 as libc::c_int |
                                 _ret >> 24 as libc::c_int |
                                 _ret << 8 as libc::c_int &
                                     0xff0000 as libc::c_int as libc::c_uint)
                                |
                                _ret >> 8 as libc::c_int &
                                    0xff00 as libc::c_int as libc::c_uint
                        }) {
                return
            }
        }
    }
    /* Reject anything that matches existing extras. Do a case-insensitive
     match. We optimize by exploiting the fact that extras[] are sorted
     by size. */
    i = 0 as libc::c_int as u32_0;
    while i < (*afl).extras_cnt {
        if (*(*afl).extras.offset(i as isize)).len >= len { break ; }
        i = i.wrapping_add(1)
    }
    while i < (*afl).extras_cnt &&
              (*(*afl).extras.offset(i as isize)).len == len {
        if memcmp_nocase((*(*afl).extras.offset(i as isize)).data, mem, len)
               == 0 {
            return
        }
        i = i.wrapping_add(1)
    }
    /* Last but not least, check afl->a_extras[] for matches. There are no
     guarantees of a particular sort order. */
    (*afl).auto_changed = 1 as libc::c_int as u8_0;
    i = 0 as libc::c_int as u32_0;
    loop  {
        if !(i < (*afl).a_extras_cnt) {
            current_block = 15897653523371991391;
            break ;
        }
        if (*(*afl).a_extras.offset(i as isize)).len == len &&
               memcmp_nocase((*(*afl).a_extras.offset(i as isize)).data, mem,
                             len) == 0 {
            let ref mut fresh13 =
                (*(*afl).a_extras.offset(i as isize)).hit_cnt;
            *fresh13 = (*fresh13).wrapping_add(1);
            current_block = 9967254363505006861;
            break ;
        } else { i = i.wrapping_add(1) }
    }
    match current_block {
        15897653523371991391 => {
            /* At this point, looks like we're dealing with a new entry. So, let's
     append it if we have room. Otherwise, let's randomly evict some other
     entry from the bottom half of the list. */
            if (*afl).a_extras_cnt <
                   (128 as libc::c_int * 64 as libc::c_int) as libc::c_uint {
                (*afl).a_extras =
                    DFL_ck_realloc_block((*afl).a_extras as *mut libc::c_void,
                                         ((*afl).a_extras_cnt.wrapping_add(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
                                              as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<extra_data>()
                                                                              as
                                                                              libc::c_ulong)
                                             as u32_0) as *mut extra_data;
                let ref mut fresh14 =
                    (*(*afl).a_extras.offset((*afl).a_extras_cnt as
                                                 isize)).data;
                *fresh14 =
                    DFL_ck_memdup(mem as *mut libc::c_void, len) as *mut u8_0;
                (*(*afl).a_extras.offset((*afl).a_extras_cnt as isize)).len =
                    len;
                (*afl).a_extras_cnt = (*afl).a_extras_cnt.wrapping_add(1)
            } else {
                i =
                    ((128 as libc::c_int * 64 as libc::c_int /
                          2 as libc::c_int) as
                         libc::c_uint).wrapping_add(rand_below(afl,
                                                               ((128 as
                                                                     libc::c_int
                                                                     *
                                                                     64 as
                                                                         libc::c_int
                                                                     +
                                                                     1 as
                                                                         libc::c_int)
                                                                    /
                                                                    2 as
                                                                        libc::c_int)
                                                                   as u32_0));
                DFL_ck_free((*(*afl).a_extras.offset(i as isize)).data as
                                *mut libc::c_void);
                let ref mut fresh15 =
                    (*(*afl).a_extras.offset(i as isize)).data;
                *fresh15 =
                    DFL_ck_memdup(mem as *mut libc::c_void, len) as *mut u8_0;
                (*(*afl).a_extras.offset(i as isize)).len = len;
                (*(*afl).a_extras.offset(i as isize)).hit_cnt =
                    0 as libc::c_int as u32_0
            }
        }
        _ => { }
    }
    /* First, sort all auto extras by use count, descending order. */
    qsort((*afl).a_extras as *mut libc::c_void, (*afl).a_extras_cnt as size_t,
          ::std::mem::size_of::<extra_data>() as libc::c_ulong,
          Some(compare_extras_use_d as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
    /* Then, sort the top USE_AUTO_EXTRAS entries by size. */
    qsort((*afl).a_extras as *mut libc::c_void,
          ({
               let mut _a: libc::c_int = 128 as libc::c_int;
               let mut _b: u32_0 = (*afl).a_extras_cnt;
               if (_a as libc::c_uint) < _b { _a as libc::c_uint } else { _b }
           }) as size_t, ::std::mem::size_of::<extra_data>() as libc::c_ulong,
          Some(compare_extras_len as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
}
/* Save automatically generated extras. */
#[no_mangle]
pub unsafe extern "C" fn save_auto(mut afl: *mut afl_state_t) {
    let mut i: u32_0 = 0;
    if (*afl).auto_changed == 0 { return }
    (*afl).auto_changed = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int as u32_0;
    while i <
              ({
                   let mut _a: libc::c_int = 128 as libc::c_int;
                   let mut _b: u32_0 = (*afl).a_extras_cnt;
                   (if (_a as libc::c_uint) < _b {
                        _a as libc::c_uint
                    } else { _b })
               }) {
        let mut fn_0: *mut u8_0 =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/queue/.state/auto_extras/auto_%06u\x00" as
                                  *const u8 as *const libc::c_char,
                              (*afl).out_dir, i);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-extras.c\x00" as *const u8 as
                                *const libc::c_char, 432 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/queue/.state/auto_extras/auto_%06u\x00" as
                              *const u8 as *const libc::c_char,
                          (*afl).out_dir, i);
                 _tmp
             });
        let mut fd: s32 = 0;
        fd =
            open(fn_0 as *const libc::c_char,
                 0o1 as libc::c_int | 0o100 as libc::c_int |
                     0o1000 as libc::c_int, 0o600 as libc::c_int);
        if fd < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                       as *const u8 as *const libc::c_char, fn_0);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-extras.c\x00" as *const u8 as
                       *const libc::c_char, 437 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        let mut _len: u32_0 = (*(*afl).a_extras.offset(i as isize)).len;
        let mut _res: s32 =
            write(fd,
                  (*(*afl).a_extras.offset(i as isize)).data as
                      *const libc::c_void, _len as size_t) as s32;
        if _res as libc::c_uint != _len {
            if _res < 0 as libc::c_int {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char, fn_0);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-extras.c\x00" as *const u8 as
                           *const libc::c_char, 439 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            } else {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char, fn_0);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-extras.c\x00" as *const u8 as
                           *const libc::c_char, 439 as libc::c_int);
                exit(1 as libc::c_int);
            }
        }
        close(fd);
        DFL_ck_free(fn_0 as *mut libc::c_void);
        i = i.wrapping_add(1)
    };
}
/* Load automatically generated extras. */
#[no_mangle]
pub unsafe extern "C" fn load_auto(mut afl: *mut afl_state_t) {
    let mut i: u32_0 = 0;
    i = 0 as libc::c_int as u32_0;
    while i < 128 as libc::c_int as libc::c_uint {
        let mut tmp: [u8_0; 33] = [0; 33];
        let mut fn_0: *mut u8_0 =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/.state/auto_extras/auto_%06u\x00" as
                                  *const u8 as *const libc::c_char,
                              (*afl).in_dir, i);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-extras.c\x00" as *const u8 as
                                *const libc::c_char, 457 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/.state/auto_extras/auto_%06u\x00" as *const u8
                              as *const libc::c_char, (*afl).in_dir, i);
                 _tmp
             });
        let mut fd: s32 = 0;
        let mut len: s32 = 0;
        fd = open(fn_0 as *const libc::c_char, 0 as libc::c_int);
        if fd < 0 as libc::c_int {
            if *__errno_location() != 2 as libc::c_int {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                           as *const u8 as *const libc::c_char, fn_0);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-extras.c\x00" as *const u8 as
                           *const libc::c_char, 464 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            }
            DFL_ck_free(fn_0 as *mut libc::c_void);
            break ;
        } else {
            /* We read one byte more to cheaply detect tokens that are too
       long (and skip them). */
            len =
                read(fd, tmp.as_mut_ptr() as *mut libc::c_void,
                     (32 as libc::c_int + 1 as libc::c_int) as size_t) as s32;
            if len < 0 as libc::c_int {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to read from \'%s\'\x00"
                           as *const u8 as *const libc::c_char, fn_0);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-extras.c\x00" as *const u8 as
                           *const libc::c_char, 475 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            }
            if len >= 3 as libc::c_int && len <= 32 as libc::c_int {
                maybe_add_auto(afl as *mut u8_0 as *mut libc::c_void,
                               tmp.as_mut_ptr(), len as u32_0);
            }
            close(fd);
            DFL_ck_free(fn_0 as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
    }
    if i != 0 {
        printf(b"\x1b[1;92m[+] \x1b[0mLoaded %u auto-discovered dictionary tokens.\x00"
                   as *const u8 as *const libc::c_char, i);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1b[1;92m[+] \x1b[0mNo auto-generated dictionary tokens to reuse.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
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
/* Bitmap */
/* Extras */
/* Destroy extras. */
#[no_mangle]
pub unsafe extern "C" fn destroy_extras(mut afl: *mut afl_state_t) {
    let mut i: u32_0 = 0;
    i = 0 as libc::c_int as u32_0;
    while i < (*afl).extras_cnt {
        DFL_ck_free((*(*afl).extras.offset(i as isize)).data as
                        *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    DFL_ck_free((*afl).extras as *mut libc::c_void);
    i = 0 as libc::c_int as u32_0;
    while i < (*afl).a_extras_cnt {
        DFL_ck_free((*(*afl).a_extras.offset(i as isize)).data as
                        *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    DFL_ck_free((*afl).a_extras as *mut libc::c_void);
}
