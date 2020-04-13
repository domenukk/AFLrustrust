use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int)
     -> __off64_t;
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
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn afl_fsrv_start(fsrv: *mut afl_forkserver_t,
                      argv: *mut *mut libc::c_char, stop_soon_p: *mut u8_0,
                      debug_child_output: u8_0);
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn ftruncate(__fd: libc::c_int, __length: __off64_t) -> libc::c_int;
    /* Get unix time in microseconds */
    #[no_mangle]
    fn get_cur_time_us() -> u64_0;
    /* Unsafe Describe integer. The buf sizes are not checked.
   This is unsafe but fast.
   Will return buf for convenience. */
    #[no_mangle]
    fn u_stringify_int(buf: *mut u8_0, val: u64_0) -> *mut u8_0;
    /* Wrapper for select() and read(), reading exactly len bytes.
  Returns the time passed to read.
  stop_soon should point to a variable indicating ctrl+c was pressed.
  If the wait times out, returns timeout_ms + 1;
  Returns 0 if an error occurred (fd closed, signal, ...); */
    #[no_mangle]
    fn read_timed(fd: s32, buf: *mut libc::c_void, len: size_t,
                  timeout_ms: u32_0, stop_soon_p: *mut u8_0) -> u32_0;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn mmap(__addr: *mut libc::c_void, __len: size_t, __prot: libc::c_int,
            __flags: libc::c_int, __fd: libc::c_int, __offset: __off64_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    #[no_mangle]
    fn trim_case_custom(_: *mut afl_state_t, q: *mut queue_entry,
                        in_buf: *mut u8_0) -> u8_0;
    #[no_mangle]
    fn mark_as_variable(_: *mut afl_state_t, _: *mut queue_entry);
    #[no_mangle]
    fn update_bitmap_score(_: *mut afl_state_t, _: *mut queue_entry);
    #[no_mangle]
    fn count_bytes(_: *mut afl_state_t, _: *mut u8_0) -> u32_0;
    #[no_mangle]
    fn classify_counts(_: *mut afl_state_t, _: *mut u64_0);
    #[no_mangle]
    fn save_if_interesting(_: *mut afl_state_t, _: *mut libc::c_void,
                           _: u32_0, _: u8_0) -> u8_0;
    #[no_mangle]
    fn has_new_bits(_: *mut afl_state_t, _: *mut u8_0) -> u8_0;
    #[no_mangle]
    fn show_stats(_: *mut afl_state_t);
    /* Execs the child */
    #[no_mangle]
    fn cmplog_exec_child(fsrv: *mut afl_forkserver_t,
                         argv: *mut *mut libc::c_char);
}
pub type __uint8_t = libc::c_uchar;
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
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
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
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
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
pub type C2RustUnnamed = libc::c_uint;
pub const FAULT_NOBITS: C2RustUnnamed = 5;
pub const FAULT_NOINST: C2RustUnnamed = 4;
pub const FAULT_ERROR: C2RustUnnamed = 3;
pub const FAULT_CRASH: C2RustUnnamed = 2;
pub const FAULT_TMOUT: C2RustUnnamed = 1;
pub const FAULT_NONE: C2RustUnnamed = 0;
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
   american fuzzy lop++ - target execution related routines
   --------------------------------------------------------

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
/* Execute target application, monitoring for timeouts. Return status
   information. The called program will update afl->fsrv->trace_bits. */
#[no_mangle]
pub unsafe extern "C" fn run_target(mut afl: *mut afl_state_t,
                                    mut fsrv: *mut afl_forkserver_t,
                                    mut timeout: u32_0) -> u8_0 {
    let mut res: s32 = 0;
    let mut exec_ms: u32_0 = 0;
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut tb4: u32_0 = 0;
    (*fsrv).child_timed_out = 0 as libc::c_int as u8_0;
    /* After this memset, fsrv->trace_bits[] are effectively volatile, so we
     must prevent any earlier operations from venturing into that
     territory. */
    memset((*fsrv).trace_bits as *mut libc::c_void, 0 as libc::c_int,
           (*fsrv).map_size as libc::c_ulong);
    asm!("" : : : "memory" : "volatile");
    /* we have the fork server (or faux server) up and running, so simply
      tell it to have at it, and then read back PID. */
    res =
        write((*fsrv).fsrv_ctl_fd,
              &mut (*fsrv).prev_timed_out as *mut u32_0 as
                  *const libc::c_void, 4 as libc::c_int as size_t) as s32;
    if res != 4 as libc::c_int {
        if (*afl).stop_soon != 0 { return 0 as libc::c_int as u8_0 }
        if res < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to request new process from fork server (OOM?)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                       *const libc::c_char, 59 as libc::c_int);
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
                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                       *const libc::c_char, 59 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    res =
        read((*fsrv).fsrv_st_fd,
             &mut (*fsrv).child_pid as *mut s32 as *mut libc::c_void,
             4 as libc::c_int as size_t) as s32;
    if res != 4 as libc::c_int {
        if (*afl).stop_soon != 0 { return 0 as libc::c_int as u8_0 }
        if res < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to request new process from fork server (OOM?)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                       *const libc::c_char, 66 as libc::c_int);
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
                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                       *const libc::c_char, 66 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    if (*fsrv).child_pid <= 0 as libc::c_int {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mFork server is misbehaving (OOM?)\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-run.c\x00" as *const u8 as *const libc::c_char,
               70 as libc::c_int);
        exit(1 as libc::c_int);
    }
    exec_ms =
        read_timed((*fsrv).fsrv_st_fd,
                   &mut status as *mut libc::c_int as *mut libc::c_void,
                   4 as libc::c_int as size_t, timeout,
                   &mut (*afl).stop_soon);
    if exec_ms > timeout {
        /* If there was no response from forkserver after timeout seconds,
    we kill the child. The forkserver should inform us afterwards */
        kill((*fsrv).child_pid, 9 as libc::c_int);
        (*fsrv).child_timed_out = 1 as libc::c_int as u8_0;
        if read((*fsrv).fsrv_st_fd,
                &mut status as *mut libc::c_int as *mut libc::c_void,
                4 as libc::c_int as size_t) < 4 as libc::c_int as libc::c_long
           {
            exec_ms = 0 as libc::c_int as u32_0
        }
    }
    if exec_ms == 0 {
        if (*afl).stop_soon != 0 { return 0 as libc::c_int as u8_0 }
        printf(b"\n\x1b[1;91m[-] \x1b[0mUnable to communicate with fork server. Some possible reasons:\n\n    - You\'ve run out of memory. Use -m to increase the the memory limit\n      to something higher than %lld.\n    - The binary or one of the libraries it uses manages to create\n      threads before the forkserver initializes.\n    - The binary, at least in some circumstances, exits in a way that\n      also kills the parent process - raise() could be the culprit.\n    - If using persistent mode with QEMU, AFL_QEMU_PERSISTENT_ADDR is\n      probably not valid (hint: add the base address in case of PIE)\n\nIf all else fails you can disable the fork server via AFL_NO_FORKSRV=1.\n\x00"
                   as *const u8 as *const libc::c_char, (*fsrv).mem_limit);
        if res < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to communicate with fork server\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                       *const libc::c_char, 109 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        } else {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to communicate with fork server\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                       *const libc::c_char, 109 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    if !(status & 0xff as libc::c_int == 0x7f as libc::c_int) {
        (*fsrv).child_pid = 0 as libc::c_int
    }
    (*afl).total_execs = (*afl).total_execs.wrapping_add(1);
    /* Any subsequent operations on fsrv->trace_bits must not be moved by the
     compiler below this point. Past this location, fsrv->trace_bits[]
     behave very normally and do not have to be treated as volatile. */
    asm!("" : : : "memory" : "volatile");
    tb4 = *((*fsrv).trace_bits as *mut u32_0);
    classify_counts(afl, (*fsrv).trace_bits as *mut u64_0);
    /* ^WORD_SIZE_64 */
    (*fsrv).prev_timed_out = (*fsrv).child_timed_out as u32_0;
    /* Report outcome to caller. */
    if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar as
           libc::c_int >> 1 as libc::c_int > 0 as libc::c_int &&
           (*afl).stop_soon == 0 {
        (*afl).kill_signal = (status & 0x7f as libc::c_int) as u8_0;
        if (*fsrv).child_timed_out as libc::c_int != 0 &&
               (*afl).kill_signal as libc::c_int == 9 as libc::c_int {
            return FAULT_TMOUT as libc::c_int as u8_0
        }
        return FAULT_CRASH as libc::c_int as u8_0
    }
    /* A somewhat nasty hack for MSAN, which doesn't support abort_on_error and
     must use a special exit code. */
    if (*fsrv).uses_asan as libc::c_int != 0 &&
           (status & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
               86 as libc::c_int {
        (*afl).kill_signal = 0 as libc::c_int as u8_0;
        return FAULT_CRASH as libc::c_int as u8_0
    }
    if ((*afl).dumb_mode as libc::c_int == 1 as libc::c_int ||
            (*afl).no_forkserver as libc::c_int != 0) &&
           tb4 == 0xfee1dead as libc::c_uint {
        return FAULT_ERROR as libc::c_int as u8_0
    }
    return FAULT_NONE as libc::c_int as u8_0;
}
/* Write modified data to file for testing. If afl->fsrv.out_file is set, the
   old file is unlinked and a new one is created. Otherwise, afl->fsrv.out_fd is
   rewound and truncated. */
#[no_mangle]
pub unsafe extern "C" fn write_to_testcase(mut afl: *mut afl_state_t,
                                           mut mem: *mut libc::c_void,
                                           mut len: u32_0) {
    let mut fd: s32 = (*afl).fsrv.out_fd; /* Ignore errors. */
    if !(*afl).fsrv.out_file.is_null() {
        if (*afl).no_unlink != 0 {
            fd =
                open((*afl).fsrv.out_file as *const libc::c_char,
                     0o1 as libc::c_int | 0o100 as libc::c_int |
                         0o1000 as libc::c_int, 0o600 as libc::c_int)
        } else {
            unlink((*afl).fsrv.out_file as *const libc::c_char);
            fd =
                open((*afl).fsrv.out_file as *const libc::c_char,
                     0o1 as libc::c_int | 0o100 as libc::c_int |
                         0o200 as libc::c_int, 0o600 as libc::c_int)
        }
        if fd < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                       as *const u8 as *const libc::c_char,
                   (*afl).fsrv.out_file);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                       *const libc::c_char, 200 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
    } else { lseek(fd, 0 as libc::c_int as __off64_t, 0 as libc::c_int); }
    if !(*afl).mutator.is_null() &&
           (*(*afl).mutator).afl_custom_pre_save.is_some() {
        let mut new_buf: *mut u8_0 = 0 as *mut u8_0;
        let mut new_size: size_t =
            (*(*afl).mutator).afl_custom_pre_save.expect("non-null function pointer")((*(*afl).mutator).data,
                                                                                      mem
                                                                                          as
                                                                                          *mut u8_0,
                                                                                      len
                                                                                          as
                                                                                          size_t,
                                                                                      &mut new_buf);
        if new_buf.is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mCustom_pre_save failed (ret: %lu)\x00"
                       as *const u8 as *const libc::c_char, new_size);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                       *const libc::c_char, 214 as libc::c_int);
            exit(1 as libc::c_int);
        }
        /* everything as planned. use the new data. */
        let mut _len: u32_0 = new_size as u32_0;
        let mut _res: s32 =
            write(fd, new_buf as *const libc::c_void, _len as size_t) as s32;
        if _res as libc::c_uint != _len {
            if _res < 0 as libc::c_int {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char,
                       (*afl).fsrv.out_file);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-run.c\x00" as *const u8 as
                           *const libc::c_char, 217 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            } else {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char,
                       (*afl).fsrv.out_file);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-run.c\x00" as *const u8 as
                           *const libc::c_char, 217 as libc::c_int);
                exit(1 as libc::c_int);
            }
        }
    } else {
        /* boring uncustom. */
        let mut _len_0: u32_0 = len;
        let mut _res_0: s32 = write(fd, mem, _len_0 as size_t) as s32;
        if _res_0 as libc::c_uint != _len_0 {
            if _res_0 < 0 as libc::c_int {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char,
                       (*afl).fsrv.out_file);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-run.c\x00" as *const u8 as
                           *const libc::c_char, 222 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            } else {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char,
                       (*afl).fsrv.out_file);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-run.c\x00" as *const u8 as
                           *const libc::c_char, 222 as libc::c_int);
                exit(1 as libc::c_int);
            }
        }
    }
    if (*afl).fsrv.out_file.is_null() {
        if ftruncate(fd, len as __off64_t) != 0 {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mftruncate() failed\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                       *const libc::c_char, 228 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        lseek(fd, 0 as libc::c_int as __off64_t, 0 as libc::c_int);
    } else { close(fd); };
}
/* The same, but with an adjustable gap. Used for trimming. */
unsafe extern "C" fn write_with_gap(mut afl: *mut afl_state_t,
                                    mut mem: *mut libc::c_void,
                                    mut len: u32_0, mut skip_at: u32_0,
                                    mut skip_len: u32_0) {
    let mut fd: s32 = (*afl).fsrv.out_fd; /* Ignore errors. */
    let mut tail_len: u32_0 =
        len.wrapping_sub(skip_at).wrapping_sub(skip_len);
    if !(*afl).fsrv.out_file.is_null() {
        if (*afl).no_unlink != 0 {
            fd =
                open((*afl).fsrv.out_file as *const libc::c_char,
                     0o1 as libc::c_int | 0o100 as libc::c_int |
                         0o1000 as libc::c_int, 0o600 as libc::c_int)
        } else {
            unlink((*afl).fsrv.out_file as *const libc::c_char);
            fd =
                open((*afl).fsrv.out_file as *const libc::c_char,
                     0o1 as libc::c_int | 0o100 as libc::c_int |
                         0o200 as libc::c_int, 0o600 as libc::c_int)
        }
        if fd < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                       as *const u8 as *const libc::c_char,
                   (*afl).fsrv.out_file);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                       *const libc::c_char, 258 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
    } else { lseek(fd, 0 as libc::c_int as __off64_t, 0 as libc::c_int); }
    if skip_at != 0 {
        let mut _len: u32_0 = skip_at;
        let mut _res: s32 = write(fd, mem, _len as size_t) as s32;
        if _res as libc::c_uint != _len {
            if _res < 0 as libc::c_int {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char,
                       (*afl).fsrv.out_file);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-run.c\x00" as *const u8 as
                           *const libc::c_char, 264 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            } else {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char,
                       (*afl).fsrv.out_file);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-run.c\x00" as *const u8 as
                           *const libc::c_char, 264 as libc::c_int);
                exit(1 as libc::c_int);
            }
        }
    }
    let mut memu8: *mut u8_0 = mem as *mut u8_0;
    if tail_len != 0 {
        let mut _len_0: u32_0 = tail_len;
        let mut _res_0: s32 =
            write(fd,
                  memu8.offset(skip_at as isize).offset(skip_len as isize) as
                      *const libc::c_void, _len_0 as size_t) as s32;
        if _res_0 as libc::c_uint != _len_0 {
            if _res_0 < 0 as libc::c_int {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char,
                       (*afl).fsrv.out_file);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-run.c\x00" as *const u8 as
                           *const libc::c_char, 268 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            } else {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char,
                       (*afl).fsrv.out_file);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-run.c\x00" as *const u8 as
                           *const libc::c_char, 268 as libc::c_int);
                exit(1 as libc::c_int);
            }
        }
    }
    if (*afl).fsrv.out_file.is_null() {
        if ftruncate(fd, len.wrapping_sub(skip_len) as __off64_t) != 0 {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mftruncate() failed\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                       *const libc::c_char, 272 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        lseek(fd, 0 as libc::c_int as __off64_t, 0 as libc::c_int);
    } else { close(fd); };
}
/* Calibrate a new test case. This is done when processing the input directory
   to warn about flaky or otherwise problematic test cases early on; and when
   new paths are discovered to detect variable behavior and so on. */
#[no_mangle]
pub unsafe extern "C" fn calibrate_case(mut afl: *mut afl_state_t,
                                        mut q: *mut queue_entry,
                                        mut use_mem: *mut u8_0,
                                        mut handicap: u32_0,
                                        mut from_queue: u8_0) -> u8_0 {
    let mut current_block: u64;
    let mut fault: u8_0 = 0 as libc::c_int as u8_0;
    let mut new_bits: u8_0 = 0 as libc::c_int as u8_0;
    let mut var_detected: u8_0 = 0 as libc::c_int as u8_0;
    let mut first_run: u8_0 =
        ((*q).exec_cksum == 0 as libc::c_int as libc::c_uint) as libc::c_int
            as u8_0;
    let mut start_us: u64_0 = 0;
    let mut stop_us: u64_0 = 0;
    let mut old_sc: s32 = (*afl).stage_cur;
    let mut old_sm: s32 = (*afl).stage_max;
    let mut use_tmout: u32_0 = (*afl).fsrv.exec_tmout;
    let mut old_sn: *mut u8_0 = (*afl).stage_name;
    /* Be a bit more generous about timeouts when resuming sessions, or when
     trying to calibrate already-added finds. This helps avoid trouble due
     to intermittent latency. */
    if from_queue == 0 || (*afl).resuming_fuzz as libc::c_int != 0 {
        use_tmout =
            ({
                 let mut _a: libc::c_uint =
                     (*afl).fsrv.exec_tmout.wrapping_add(50 as libc::c_int as
                                                             libc::c_uint);
                 let mut _b: libc::c_uint =
                     (*afl).fsrv.exec_tmout.wrapping_mul(125 as libc::c_int as
                                                             libc::c_uint).wrapping_div(100
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint);
                 if _a > _b { _a } else { _b }
             })
    }
    (*q).cal_failed = (*q).cal_failed.wrapping_add(1);
    (*afl).stage_name =
        b"calibration\x00" as *const u8 as *const libc::c_char as *mut u8_0;
    (*afl).stage_max =
        if (*afl).fast_cal as libc::c_int != 0 {
            3 as libc::c_int
        } else { 8 as libc::c_int };
    /* Make sure the forkserver is up before we do anything, and let's not
     count its spin-up time toward binary calibration. */
    if (*afl).fsrv.fsrv_pid == 0 {
        if !(*afl).fsrv.cmplog_binary.is_null() &&
               (*afl).fsrv.init_child_func !=
                   Some(cmplog_exec_child as
                            unsafe extern "C" fn(_: *mut afl_forkserver_t,
                                                 _: *mut *mut libc::c_char)
                                -> ()) {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBUG in afl-fuzz detected. Cmplog mode not set correctly.\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                       *const libc::c_char, 318 as libc::c_int);
            exit(1 as libc::c_int);
        }
        afl_fsrv_start(&mut (*afl).fsrv, (*afl).argv, &mut (*afl).stop_soon,
                       (*afl).afl_env.afl_debug_child_output);
    }
    if (*q).exec_cksum != 0 {
        memcpy((*afl).first_trace.as_mut_ptr() as *mut libc::c_void,
               (*afl).fsrv.trace_bits as *const libc::c_void,
               (*afl).fsrv.map_size as libc::c_ulong);
    }
    start_us = get_cur_time_us();
    (*afl).stage_cur = 0 as libc::c_int;
    loop  {
        if !((*afl).stage_cur < (*afl).stage_max) {
            current_block = 11626999923138678822;
            break ;
        }
        let mut cksum: u32_0 = 0;
        if first_run == 0 &&
               ((*afl).stage_cur as
                    libc::c_uint).wrapping_rem((*afl).stats_update_freq) == 0
           {
            show_stats(afl);
        }
        write_to_testcase(afl, use_mem as *mut libc::c_void, (*q).len);
        fault = run_target(afl, &mut (*afl).fsrv, use_tmout);
        /* afl->stop_soon is set by the handler for Ctrl+C. When it's pressed,
       we want to bail out quickly. */
        if (*afl).stop_soon as libc::c_int != 0 ||
               fault as libc::c_int != (*afl).crash_mode as libc::c_int {
            current_block = 1726299285531216109;
            break ;
        }
        if (*afl).dumb_mode == 0 && (*afl).stage_cur == 0 &&
               count_bytes(afl, (*afl).fsrv.trace_bits) == 0 {
            fault = FAULT_NOINST as libc::c_int as u8_0;
            current_block = 1726299285531216109;
            break ;
        } else {
            cksum =
                hash32((*afl).fsrv.trace_bits as *const libc::c_void,
                       (*afl).fsrv.map_size, 0xa5b35705 as libc::c_uint);
            if (*q).exec_cksum != cksum {
                let mut hnb: u8_0 =
                    has_new_bits(afl, (*afl).virgin_bits.as_mut_ptr());
                if hnb as libc::c_int > new_bits as libc::c_int {
                    new_bits = hnb
                }
                if (*q).exec_cksum != 0 {
                    let mut i: u32_0 = 0;
                    i = 0 as libc::c_int as u32_0;
                    while i < (*afl).fsrv.map_size {
                        if (*afl).var_bytes[i as usize] == 0 &&
                               (*afl).first_trace[i as usize] as libc::c_int
                                   !=
                                   *(*afl).fsrv.trace_bits.offset(i as isize)
                                       as libc::c_int {
                            (*afl).var_bytes[i as usize] =
                                1 as libc::c_int as u8_0
                        }
                        i = i.wrapping_add(1)
                    }
                    var_detected = 1 as libc::c_int as u8_0;
                    (*afl).stage_max = 40 as libc::c_int
                } else {
                    (*q).exec_cksum = cksum;
                    memcpy((*afl).first_trace.as_mut_ptr() as
                               *mut libc::c_void,
                           (*afl).fsrv.trace_bits as *const libc::c_void,
                           (*afl).fsrv.map_size as libc::c_ulong);
                }
            }
            (*afl).stage_cur += 1
        }
    }
    match current_block {
        11626999923138678822 => {
            stop_us = get_cur_time_us();
            (*afl).total_cal_us =
                ((*afl).total_cal_us as
                     libc::c_ulonglong).wrapping_add(stop_us.wrapping_sub(start_us))
                    as u64_0 as u64_0;
            (*afl).total_cal_cycles =
                ((*afl).total_cal_cycles as
                     libc::c_ulonglong).wrapping_add((*afl).stage_max as
                                                         libc::c_ulonglong) as
                    u64_0 as u64_0;
            /* OK, let's collect some stats about the performance of this test case.
     This is used for fuzzing air time calculations in calculate_score(). */
            (*q).exec_us =
                stop_us.wrapping_sub(start_us).wrapping_div((*afl).stage_max
                                                                as
                                                                libc::c_ulonglong);
            (*q).bitmap_size = count_bytes(afl, (*afl).fsrv.trace_bits);
            (*q).handicap = handicap as u64_0;
            (*q).cal_failed = 0 as libc::c_int as u8_0;
            (*afl).total_bitmap_size =
                ((*afl).total_bitmap_size as
                     libc::c_ulonglong).wrapping_add((*q).bitmap_size as
                                                         libc::c_ulonglong) as
                    u64_0 as u64_0;
            (*afl).total_bitmap_entries =
                (*afl).total_bitmap_entries.wrapping_add(1);
            update_bitmap_score(afl, q);
            /* If this case didn't result in new output from the instrumentation, tell
     parent. This is a non-critical problem, but something to warn the user
     about. */
            if (*afl).dumb_mode == 0 && first_run as libc::c_int != 0 &&
                   fault == 0 && new_bits == 0 {
                fault = FAULT_NOBITS as libc::c_int as u8_0
            }
        }
        _ => { }
    }
    if new_bits as libc::c_int == 2 as libc::c_int && (*q).has_new_cov == 0 {
        (*q).has_new_cov = 1 as libc::c_int as u8_0;
        (*afl).queued_with_cov = (*afl).queued_with_cov.wrapping_add(1)
    }
    /* Mark variable paths. */
    if var_detected != 0 {
        (*afl).var_byte_count =
            count_bytes(afl, (*afl).var_bytes.as_mut_ptr());
        if (*q).var_behavior == 0 {
            mark_as_variable(afl, q);
            (*afl).queued_variable = (*afl).queued_variable.wrapping_add(1)
        }
    }
    (*afl).stage_name = old_sn;
    (*afl).stage_cur = old_sc;
    (*afl).stage_max = old_sm;
    if first_run == 0 { show_stats(afl); }
    return fault;
}
/* Grab interesting test cases from other fuzzers. */
#[no_mangle]
pub unsafe extern "C" fn sync_fuzzers(mut afl: *mut afl_state_t) {
    let mut sd: *mut DIR = 0 as *mut DIR;
    let mut sd_ent: *mut dirent = 0 as *mut dirent;
    let mut sync_cnt: u32_0 = 0 as libc::c_int as u32_0;
    sd = opendir((*afl).sync_dir as *const libc::c_char);
    if sd.is_null() {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                   as *const u8 as *const libc::c_char, (*afl).sync_dir);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-run.c\x00" as *const u8 as *const libc::c_char,
               456 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    (*afl).stage_cur = 0 as libc::c_int;
    (*afl).stage_max = (*afl).stage_cur;
    (*afl).cur_depth = 0 as libc::c_int as u32_0;
    let mut current_block_82: u64;
    loop 
         /* Look at the entries created for every other fuzzer in the sync directory.
   */
         {
        sd_ent = readdir(sd);
        if sd_ent.is_null() { break ; }
        let mut qd: *mut DIR = 0 as *mut DIR;
        let mut qd_ent: *mut dirent = 0 as *mut dirent;
        let mut qd_path: *mut u8_0 = 0 as *mut u8_0;
        let mut qd_synced_path: *mut u8_0 = 0 as *mut u8_0;
        let mut min_accept: u32_0 = 0 as libc::c_int as u32_0;
        let mut next_min_accept: u32_0 = 0;
        let mut id_fd: s32 = 0;
        /* Skip dot files and our own output directory. */
        if (*sd_ent).d_name[0 as libc::c_int as usize] as libc::c_int ==
               '.' as i32 ||
               strcmp((*afl).sync_id as *const libc::c_char,
                      (*sd_ent).d_name.as_mut_ptr()) == 0 {
            continue ;
        }
        /* Skip anything that doesn't have a queue/ subdirectory. */
        qd_path =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/%s/queue\x00" as *const u8 as
                                  *const libc::c_char, (*afl).sync_dir,
                              (*sd_ent).d_name.as_mut_ptr());
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-run.c\x00" as *const u8 as
                                *const libc::c_char, 480 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/%s/queue\x00" as *const u8 as
                              *const libc::c_char, (*afl).sync_dir,
                          (*sd_ent).d_name.as_mut_ptr());
                 _tmp
             });
        qd = opendir(qd_path as *const libc::c_char);
        if qd.is_null() {
            DFL_ck_free(qd_path as *mut libc::c_void);
        } else {
            /* Retrieve the ID of the last seen test case. */
            qd_synced_path =
                ({
                     let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                     let mut _len: s32 =
                         snprintf(0 as *mut libc::c_char,
                                  0 as libc::c_int as libc::c_ulong,
                                  b"%s/.synced/%s\x00" as *const u8 as
                                      *const libc::c_char, (*afl).out_dir,
                                  (*sd_ent).d_name.as_mut_ptr());
                     if _len < 0 as libc::c_int {
                         printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                    as *const u8 as *const libc::c_char);
                         printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                    as *const u8 as *const libc::c_char,
                                b"func_unknown\x00" as *const u8 as
                                    *const libc::c_char,
                                b"src/afl-fuzz-run.c\x00" as *const u8 as
                                    *const libc::c_char, 492 as libc::c_int);
                         exit(1 as libc::c_int);
                     }
                     _tmp =
                         DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                             *mut u8_0;
                     snprintf(_tmp as *mut libc::c_char,
                              (_len + 1 as libc::c_int) as libc::c_ulong,
                              b"%s/.synced/%s\x00" as *const u8 as
                                  *const libc::c_char, (*afl).out_dir,
                              (*sd_ent).d_name.as_mut_ptr());
                     _tmp
                 });
            id_fd =
                open(qd_synced_path as *const libc::c_char,
                     0o2 as libc::c_int | 0o100 as libc::c_int,
                     0o600 as libc::c_int);
            if id_fd < 0 as libc::c_int {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                           as *const u8 as *const libc::c_char,
                       qd_synced_path);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-run.c\x00" as *const u8 as
                           *const libc::c_char, 496 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            }
            if read(id_fd, &mut min_accept as *mut u32_0 as *mut libc::c_void,
                    ::std::mem::size_of::<u32_0>() as libc::c_ulong) >
                   0 as libc::c_int as libc::c_long {
                lseek(id_fd, 0 as libc::c_int as __off64_t, 0 as libc::c_int);
            }
            next_min_accept = min_accept;
            /* Show stats */
            sync_cnt = sync_cnt.wrapping_add(1);
            snprintf((*afl).stage_name_buf.as_mut_ptr() as *mut libc::c_char,
                     64 as libc::c_int as libc::c_ulong,
                     b"sync %u\x00" as *const u8 as *const libc::c_char,
                     sync_cnt);
            (*afl).stage_name = (*afl).stage_name_buf.as_mut_ptr();
            (*afl).stage_cur = 0 as libc::c_int;
            (*afl).stage_max = 0 as libc::c_int;
            loop 
                 /* For every file queued by this fuzzer, parse ID and see if we have
       looked at it before; exec a test case if not. */
                 {
                qd_ent = readdir(qd);
                if qd_ent.is_null() {
                    current_block_82 = 7189308829251266000;
                    break ;
                }
                let mut path: *mut u8_0 = 0 as *mut u8_0;
                let mut fd: s32 = 0;
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
                if (*qd_ent).d_name[0 as libc::c_int as usize] as libc::c_int
                       == '.' as i32 ||
                       sscanf((*qd_ent).d_name.as_mut_ptr(),
                              b"id:%06u\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut (*afl).syncing_case as *mut u32_0) !=
                           1 as libc::c_int ||
                       (*afl).syncing_case < min_accept {
                    continue ;
                }
                /* OK, sounds like a new one. Let's give it a try. */
                if (*afl).syncing_case >= next_min_accept {
                    next_min_accept =
                        (*afl).syncing_case.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint)
                }
                path =
                    ({
                         let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                         let mut _len: s32 =
                             snprintf(0 as *mut libc::c_char,
                                      0 as libc::c_int as libc::c_ulong,
                                      b"%s/%s\x00" as *const u8 as
                                          *const libc::c_char, qd_path,
                                      (*qd_ent).d_name.as_mut_ptr());
                         if _len < 0 as libc::c_int {
                             printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                        as *const u8 as *const libc::c_char);
                             printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    b"func_unknown\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"src/afl-fuzz-run.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    529 as libc::c_int);
                             exit(1 as libc::c_int);
                         }
                         _tmp =
                             DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0)
                                 as *mut u8_0;
                         snprintf(_tmp as *mut libc::c_char,
                                  (_len + 1 as libc::c_int) as libc::c_ulong,
                                  b"%s/%s\x00" as *const u8 as
                                      *const libc::c_char, qd_path,
                                  (*qd_ent).d_name.as_mut_ptr());
                         _tmp
                     });
                /* Allow this to fail in case the other fuzzer is resuming or so... */
                fd = open(path as *const libc::c_char, 0 as libc::c_int);
                if fd < 0 as libc::c_int {
                    DFL_ck_free(path as *mut libc::c_void);
                } else {
                    if fstat(fd, &mut st) != 0 {
                        fflush(stdout);
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mfstat() failed\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-fuzz-run.c\x00" as *const u8 as
                                   *const libc::c_char, 542 as libc::c_int);
                        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00"
                                   as *const u8 as *const libc::c_char,
                               strerror(*__errno_location()));
                        exit(1 as libc::c_int);
                    }
                    /* Ignore zero-sized or oversized files. */
                    if st.st_size != 0 &&
                           st.st_size <=
                               (1 as libc::c_int * 1024 as libc::c_int *
                                    1024 as libc::c_int) as libc::c_long {
                        let mut fault: u8_0 = 0;
                        let mut mem: *mut u8_0 =
                            mmap(0 as *mut libc::c_void, st.st_size as size_t,
                                 0x1 as libc::c_int, 0x2 as libc::c_int, fd,
                                 0 as libc::c_int as __off64_t) as *mut u8_0;
                        if mem ==
                               -(1 as libc::c_int) as *mut libc::c_void as
                                   *mut u8_0 {
                            fflush(stdout);
                            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to mmap \'%s\'\x00"
                                       as *const u8 as *const libc::c_char,
                                   path);
                            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   b"func_unknown\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   551 as libc::c_int);
                            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   strerror(*__errno_location()));
                            exit(1 as libc::c_int);
                        }
                        /* See what happens. We rely on save_if_interesting() to catch major
           errors and save the test case. */
                        write_to_testcase(afl, mem as *mut libc::c_void,
                                          st.st_size as u32_0);
                        fault =
                            run_target(afl, &mut (*afl).fsrv,
                                       (*afl).fsrv.exec_tmout);
                        if (*afl).stop_soon != 0 {
                            current_block_82 = 5211265003979548643;
                            break ;
                        }
                        (*afl).syncing_party =
                            (*sd_ent).d_name.as_mut_ptr() as *mut u8_0;
                        (*afl).queued_imported =
                            ((*afl).queued_imported as
                                 libc::c_uint).wrapping_add(save_if_interesting(afl,
                                                                                mem
                                                                                    as
                                                                                    *mut libc::c_void,
                                                                                st.st_size
                                                                                    as
                                                                                    u32_0,
                                                                                fault)
                                                                as
                                                                libc::c_uint)
                                as u32_0 as u32_0;
                        (*afl).syncing_party = 0 as *mut u8_0;
                        munmap(mem as *mut libc::c_void,
                               st.st_size as size_t);
                        let fresh2 = (*afl).stage_cur;
                        (*afl).stage_cur = (*afl).stage_cur + 1;
                        if (fresh2 as
                                libc::c_uint).wrapping_rem((*afl).stats_update_freq)
                               == 0 {
                            show_stats(afl);
                        }
                    }
                    DFL_ck_free(path as *mut libc::c_void);
                    close(fd);
                }
            }
            match current_block_82 {
                7189308829251266000 => {
                    let mut _len: u32_0 =
                        ::std::mem::size_of::<u32_0>() as libc::c_ulong as
                            u32_0;
                    let mut _res: s32 =
                        write(id_fd,
                              &mut next_min_accept as *mut u32_0 as
                                  *const libc::c_void, _len as size_t) as s32;
                    if _res as libc::c_uint != _len {
                        if _res < 0 as libc::c_int {
                            fflush(stdout);
                            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                                       as *const u8 as *const libc::c_char,
                                   qd_synced_path);
                            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   b"func_unknown\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   578 as libc::c_int);
                            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   strerror(*__errno_location()));
                            exit(1 as libc::c_int);
                        } else {
                            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                                       as *const u8 as *const libc::c_char,
                                   qd_synced_path);
                            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   b"func_unknown\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"src/afl-fuzz-run.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   578 as libc::c_int);
                            exit(1 as libc::c_int);
                        }
                    }
                }
                _ => { }
            }
            close(id_fd);
            closedir(qd);
            DFL_ck_free(qd_path as *mut libc::c_void);
            DFL_ck_free(qd_synced_path as *mut libc::c_void);
        }
    }
    closedir(sd);
}
/* Trim all new test cases to save cycles when doing deterministic checks. The
   trimmer uses power-of-two increments somewhere between 1/16 and 1/1024 of
   file size, to keep the stage short and sweet. */
#[no_mangle]
pub unsafe extern "C" fn trim_case(mut afl: *mut afl_state_t,
                                   mut q: *mut queue_entry,
                                   mut in_buf: *mut u8_0) -> u8_0 {
    let mut current_block: u64;
    /* Custom mutator trimmer */
    if !(*afl).mutator.is_null() &&
           (*(*afl).mutator).afl_custom_trim.is_some() {
        return trim_case_custom(afl, q, in_buf)
    }
    let mut needs_write: u8_0 = 0 as libc::c_int as u8_0;
    let mut fault: u8_0 = 0 as libc::c_int as u8_0;
    let mut trim_exec: u32_0 = 0 as libc::c_int as u32_0;
    let mut remove_len: u32_0 = 0;
    let mut len_p2: u32_0 = 0;
    let mut val_bufs: [[u8_0; 16]; 2] = [[0; 16]; 2];
    /* Although the trimmer will be less useful when variable behavior is
     detected, it will still work to some extent, so we don't check for
     this. */
    if (*q).len < 5 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as u8_0
    }
    (*afl).stage_name = (*afl).stage_name_buf.as_mut_ptr();
    (*afl).bytes_trim_in =
        ((*afl).bytes_trim_in as
             libc::c_ulonglong).wrapping_add((*q).len as libc::c_ulonglong) as
            u64_0 as u64_0;
    /* Select initial chunk len, starting with large steps. */
    len_p2 = next_pow2((*q).len as size_t) as u32_0;
    remove_len =
        ({
             let mut _a: libc::c_uint =
                 len_p2.wrapping_div(16 as libc::c_int as libc::c_uint);
             let mut _b: libc::c_int = 4 as libc::c_int;
             if _a > _b as libc::c_uint { _a } else { _b as libc::c_uint }
         });
    's_49:
        loop 
             /* Continue until the number of steps gets too high or the stepover
     gets too small. */
             {
            if !(remove_len >=
                     ({
                          let mut _a: libc::c_uint =
                              len_p2.wrapping_div(1024 as libc::c_int as
                                                      libc::c_uint);
                          let mut _b: libc::c_int = 4 as libc::c_int;
                          (if _a > _b as libc::c_uint {
                               _a
                           } else { _b as libc::c_uint })
                      })) {
                current_block = 18153031941552419006;
                break ;
            }
            let mut remove_pos: u32_0 = remove_len;
            sprintf((*afl).stage_name_buf.as_mut_ptr() as *mut libc::c_char,
                    b"trim %s/%s\x00" as *const u8 as *const libc::c_char,
                    u_stringify_int(val_bufs[0 as libc::c_int as
                                                 usize].as_mut_ptr(),
                                    remove_len as u64_0),
                    u_stringify_int(val_bufs[1 as libc::c_int as
                                                 usize].as_mut_ptr(),
                                    remove_len as u64_0));
            (*afl).stage_cur = 0 as libc::c_int;
            (*afl).stage_max = (*q).len.wrapping_div(remove_len) as s32;
            while remove_pos < (*q).len {
                let mut trim_avail: u32_0 =
                    ({
                         let mut _a: u32_0 = remove_len;
                         let mut _b: libc::c_uint =
                             (*q).len.wrapping_sub(remove_pos);
                         if _a < _b { _a } else { _b }
                     });
                let mut cksum: u32_0 = 0;
                write_with_gap(afl, in_buf as *mut libc::c_void, (*q).len,
                               remove_pos, trim_avail);
                fault =
                    run_target(afl, &mut (*afl).fsrv, (*afl).fsrv.exec_tmout);
                (*afl).trim_execs = (*afl).trim_execs.wrapping_add(1);
                if (*afl).stop_soon as libc::c_int != 0 ||
                       fault as libc::c_int == FAULT_ERROR as libc::c_int {
                    current_block = 10120536549943927944;
                    break 's_49 ;
                }
                /* Note that we don't keep track of crashes or hangs here; maybe TODO?
       */
                cksum =
                    hash32((*afl).fsrv.trace_bits as *const libc::c_void,
                           (*afl).fsrv.map_size, 0xa5b35705 as libc::c_uint);
                /* If the deletion had no impact on the trace, make it permanent. This
         isn't perfect for variable-path inputs, but we're just making a
         best-effort pass, so it's not a big deal if we end up with false
         negatives every now and then. */
                if cksum == (*q).exec_cksum {
                    let mut move_tail: u32_0 =
                        (*q).len.wrapping_sub(remove_pos).wrapping_sub(trim_avail);
                    (*q).len =
                        ((*q).len as libc::c_uint).wrapping_sub(trim_avail) as
                            u32_0 as u32_0;
                    len_p2 = next_pow2((*q).len as size_t) as u32_0;
                    memmove(in_buf.offset(remove_pos as isize) as
                                *mut libc::c_void,
                            in_buf.offset(remove_pos as
                                              isize).offset(trim_avail as
                                                                isize) as
                                *const libc::c_void,
                            move_tail as libc::c_ulong);
                    /* Let's save a clean trace, which will be needed by
           update_bitmap_score once we're done with the trimming stuff. */
                    if needs_write == 0 {
                        needs_write = 1 as libc::c_int as u8_0;
                        memcpy((*afl).clean_trace.as_mut_ptr() as
                                   *mut libc::c_void,
                               (*afl).fsrv.trace_bits as *const libc::c_void,
                               (*afl).fsrv.map_size as libc::c_ulong);
                    }
                } else {
                    remove_pos =
                        (remove_pos as libc::c_uint).wrapping_add(remove_len)
                            as u32_0 as u32_0
                }
                /* Since this can be slow, update the screen every now and then. */
                let fresh3 = trim_exec;
                trim_exec = trim_exec.wrapping_add(1);
                if fresh3.wrapping_rem((*afl).stats_update_freq) == 0 {
                    show_stats(afl);
                }
                (*afl).stage_cur += 1
            }
            remove_len >>= 1 as libc::c_int
        }
    match current_block {
        18153031941552419006 => {
            /* If we have made changes to in_buf, we also need to update the on-disk
     version of the test case. */
            if needs_write != 0 {
                let mut fd: s32 = 0; /* ignore errors */
                if (*afl).no_unlink != 0 {
                    fd =
                        open((*q).fname as *const libc::c_char,
                             0o1 as libc::c_int | 0o100 as libc::c_int |
                                 0o1000 as libc::c_int, 0o600 as libc::c_int)
                } else {
                    unlink((*q).fname as *const libc::c_char);
                    fd =
                        open((*q).fname as *const libc::c_char,
                             0o1 as libc::c_int | 0o100 as libc::c_int |
                                 0o200 as libc::c_int, 0o600 as libc::c_int)
                }
                if fd < 0 as libc::c_int {
                    fflush(stdout);
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                               as *const u8 as *const libc::c_char,
                           (*q).fname);
                    printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz-run.c\x00" as *const u8 as
                               *const libc::c_char, 713 as libc::c_int);
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
                               b"src/afl-fuzz-run.c\x00" as *const u8 as
                                   *const libc::c_char, 715 as libc::c_int);
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
                               b"src/afl-fuzz-run.c\x00" as *const u8 as
                                   *const libc::c_char, 715 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                }
                close(fd);
                memcpy((*afl).fsrv.trace_bits as *mut libc::c_void,
                       (*afl).clean_trace.as_mut_ptr() as *const libc::c_void,
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
/* Run */
/* Write a modified test case, run program, process results. Handle
   error conditions, returning 1 if it's time to bail out. This is
   a helper function for fuzz_one(). */
#[no_mangle]
pub unsafe extern "C" fn common_fuzz_stuff(mut afl: *mut afl_state_t,
                                           mut out_buf: *mut u8_0,
                                           mut len: u32_0) -> u8_0 {
    let mut fault: u8_0 = 0;
    if (*afl).post_handler.is_some() {
        let mut post_buf: *mut u8_0 = 0 as *mut u8_0;
        let mut post_len: size_t =
            (*afl).post_handler.expect("non-null function pointer")((*afl).post_data,
                                                                    out_buf,
                                                                    len,
                                                                    &mut post_buf);
        if post_buf.is_null() || post_len == 0 {
            return 0 as libc::c_int as u8_0
        }
        out_buf = post_buf;
        len = post_len as u32_0
    }
    write_to_testcase(afl, out_buf as *mut libc::c_void, len);
    fault = run_target(afl, &mut (*afl).fsrv, (*afl).fsrv.exec_tmout);
    if (*afl).stop_soon != 0 { return 1 as libc::c_int as u8_0 }
    if fault as libc::c_int == FAULT_TMOUT as libc::c_int {
        let fresh4 = (*afl).subseq_tmouts;
        (*afl).subseq_tmouts = (*afl).subseq_tmouts.wrapping_add(1);
        if fresh4 > 250 as libc::c_int as libc::c_uint {
            (*afl).cur_skipped_paths =
                (*afl).cur_skipped_paths.wrapping_add(1);
            return 1 as libc::c_int as u8_0
        }
    } else { (*afl).subseq_tmouts = 0 as libc::c_int as u32_0 }
    /* Users can hit us with SIGUSR1 to request the current input
     to be abandoned. */
    if (*afl).skip_requested != 0 {
        (*afl).skip_requested = 0 as libc::c_int as u8_0;
        (*afl).cur_skipped_paths = (*afl).cur_skipped_paths.wrapping_add(1);
        return 1 as libc::c_int as u8_0
    }
    /* This handles FAULT_ERROR for us: */
    (*afl).queued_discovered =
        ((*afl).queued_discovered as
             libc::c_uint).wrapping_add(save_if_interesting(afl,
                                                            out_buf as
                                                                *mut libc::c_void,
                                                            len, fault) as
                                            libc::c_uint) as u32_0 as u32_0;
    if ((*afl).stage_cur as
            libc::c_uint).wrapping_rem((*afl).stats_update_freq) == 0 ||
           (*afl).stage_cur + 1 as libc::c_int == (*afl).stage_max {
        show_stats(afl);
    }
    return 0 as libc::c_int as u8_0;
}
