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
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    #[no_mangle]
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
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
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn setenv(__name: *const libc::c_char, __value: *const libc::c_char,
              __replace: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
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
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memmem(__haystack: *const libc::c_void, __haystacklen: size_t,
              __needle: *const libc::c_void, __needlelen: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn link(__from: *const libc::c_char, __to: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn get_afl_env(env: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    static mut doc_path: *mut u8_0;
    /* Describe integer as memory size. */
    #[no_mangle]
    fn stringify_mem_size(buf: *mut u8_0, len: size_t, val: u64_0)
     -> *mut u8_0;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                 __oact: *mut sigaction) -> libc::c_int;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn scandir(__dir: *const libc::c_char, __namelist: *mut *mut *mut dirent,
               __selector:
                   Option<unsafe extern "C" fn(_: *const dirent)
                              -> libc::c_int>,
               __cmp:
                   Option<unsafe extern "C" fn(_: *mut *const dirent,
                                               _: *mut *const dirent)
                              -> libc::c_int>) -> libc::c_int;
    #[no_mangle]
    fn alphasort(__e1: *mut *const dirent, __e2: *mut *const dirent)
     -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char)
     -> *mut libc::c_void;
    #[no_mangle]
    fn dlerror() -> *mut libc::c_char;
    #[no_mangle]
    fn sched_setaffinity(__pid: __pid_t, __cpusetsize: size_t,
                         __cpuset: *const cpu_set_t) -> libc::c_int;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn mmap(__addr: *mut libc::c_void, __len: size_t, __prot: libc::c_int,
            __flags: libc::c_int, __fd: libc::c_int, __offset: __off64_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn flock(__fd: libc::c_int, __operation: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut afl_states: list_t;
    #[no_mangle]
    fn mark_as_det_done(_: *mut afl_state_t, _: *mut queue_entry);
    #[no_mangle]
    fn add_to_queue(_: *mut afl_state_t, _: *mut u8_0, _: u32_0, _: u8_0);
    #[no_mangle]
    fn count_bytes(_: *mut afl_state_t, _: *mut u8_0) -> u32_0;
    #[no_mangle]
    fn calibrate_case(_: *mut afl_state_t, _: *mut queue_entry, _: *mut u8_0,
                      _: u32_0, _: u8_0) -> u8_0;
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
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
    pub _sifields: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_9,
    pub _timer: C2RustUnnamed_8,
    pub _rt: C2RustUnnamed_7,
    pub _sigchld: C2RustUnnamed_6,
    pub _sigfault: C2RustUnnamed_3,
    pub _sigpoll: C2RustUnnamed_2,
    pub _sigsys: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _addr_bnd: C2RustUnnamed_5,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut siginfo_t,
                                                  _: *mut libc::c_void)
                                 -> ()>,
}
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
pub type C2RustUnnamed_11 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_11 = 8;
pub const _ISpunct: C2RustUnnamed_11 = 4;
pub const _IScntrl: C2RustUnnamed_11 = 2;
pub const _ISblank: C2RustUnnamed_11 = 1;
pub const _ISgraph: C2RustUnnamed_11 = 32768;
pub const _ISprint: C2RustUnnamed_11 = 16384;
pub const _ISspace: C2RustUnnamed_11 = 8192;
pub const _ISxdigit: C2RustUnnamed_11 = 4096;
pub const _ISdigit: C2RustUnnamed_11 = 2048;
pub const _ISalpha: C2RustUnnamed_11 = 1024;
pub const _ISlower: C2RustUnnamed_11 = 512;
pub const _ISupper: C2RustUnnamed_11 = 256;
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
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
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
pub type C2RustUnnamed_12 = libc::c_uint;
pub const FAULT_NOBITS: C2RustUnnamed_12 = 5;
pub const FAULT_NOINST: C2RustUnnamed_12 = 4;
pub const FAULT_ERROR: C2RustUnnamed_12 = 3;
pub const FAULT_CRASH: C2RustUnnamed_12 = 2;
pub const FAULT_TMOUT: C2RustUnnamed_12 = 1;
pub const FAULT_NONE: C2RustUnnamed_12 = 0;
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
#[inline]
unsafe extern "C" fn get_head(mut list: *mut list_t) -> *mut element_t {
    /* The first element is the head */
    return (*list).element_prealloc_buf.as_mut_ptr();
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
   american fuzzy lop++ - initialization related routines
   ------------------------------------------------------

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
/* Build a list of processes bound to specific cores. Returns -1 if nothing
   can be found. Assumes an upper bound of 4k CPUs. */
#[no_mangle]
pub unsafe extern "C" fn bind_to_free_cpu(mut afl: *mut afl_state_t) {
    let mut c: cpu_set_t = cpu_set_t{__bits: [0; 16],};
    let mut cpu_used: [u8_0; 4096] =
        [0 as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut i: u32_0 = 0;
    if (*afl).cpu_core_count < 2 as libc::c_int { return }
    if (*afl).afl_env.afl_no_affinity != 0 {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mNot binding to a CPU core (AFL_NO_AFFINITY set).\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        return
    }
    let mut d: *mut DIR = 0 as *mut DIR;
    let mut de: *mut dirent = 0 as *mut dirent;
    d = opendir(b"/proc\x00" as *const u8 as *const libc::c_char);
    if d.is_null() {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mUnable to access /proc - can\'t scan for free CPU cores.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        return
    }
    printf(b"\x1b[1;94m[*] \x1b[0mChecking CPU core loadout...\x00" as
               *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    /* Introduce some jitter, in case multiple AFL tasks are doing the same
     thing at the same time... */
    usleep((random() % 1000 as libc::c_int as libc::c_long *
                250 as libc::c_int as libc::c_long) as __useconds_t);
    loop 
         /* Scan all /proc/<pid>/status entries, checking for Cpus_allowed_list.
     Flag all processes bound to a specific CPU using cpu_used[]. This will
     fail for some exotic binding setups, but is likely good enough in almost
     all real-world use cases. */
         {
        de = readdir(d);
        if de.is_null() { break ; }
        let mut fn_0: [u8_0; 4096] = [0; 4096];
        let mut f: *mut FILE = 0 as *mut FILE;
        let mut tmp: [u8_0; 8192] = [0; 8192];
        let mut has_vmsize: u8_0 = 0 as libc::c_int as u8_0;
        if *(*__ctype_b_loc()).offset((*de).d_name[0 as libc::c_int as usize]
                                          as libc::c_int as isize) as
               libc::c_int &
               _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0 {
            continue ;
        }
        snprintf(fn_0.as_mut_ptr() as *mut libc::c_char,
                 4096 as libc::c_int as libc::c_ulong,
                 b"/proc/%s/status\x00" as *const u8 as *const libc::c_char,
                 (*de).d_name.as_mut_ptr());
        f =
            fopen(fn_0.as_mut_ptr() as *const libc::c_char,
                  b"r\x00" as *const u8 as *const libc::c_char);
        if f.is_null() { continue ; }
        while !fgets(tmp.as_mut_ptr() as *mut libc::c_char,
                     8192 as libc::c_int, f).is_null() {
            let mut hval: u32_0 = 0;
            /* Processes without VmSize are probably kernel tasks. */
            if strncmp(tmp.as_mut_ptr() as *const libc::c_char,
                       b"VmSize:\t\x00" as *const u8 as *const libc::c_char,
                       8 as libc::c_int as libc::c_ulong) == 0 {
                has_vmsize = 1 as libc::c_int as u8_0
            }
            if !(strncmp(tmp.as_mut_ptr() as *const libc::c_char,
                         b"Cpus_allowed_list:\t\x00" as *const u8 as
                             *const libc::c_char,
                         19 as libc::c_int as libc::c_ulong) == 0 &&
                     strchr(tmp.as_mut_ptr() as *const libc::c_char,
                            '-' as i32).is_null() &&
                     strchr(tmp.as_mut_ptr() as *const libc::c_char,
                            ',' as i32).is_null() &&
                     sscanf(tmp.as_mut_ptr().offset(19 as libc::c_int as
                                                        isize) as
                                *const libc::c_char,
                            b"%u\x00" as *const u8 as *const libc::c_char,
                            &mut hval as *mut u32_0) == 1 as libc::c_int &&
                     (hval as libc::c_ulong) <
                         ::std::mem::size_of::<[u8_0; 4096]>() as
                             libc::c_ulong && has_vmsize as libc::c_int != 0)
               {
                continue ;
            }
            cpu_used[hval as usize] = 1 as libc::c_int as u8_0;
            break ;
        }
        fclose(f);
    }
    closedir(d);
    let mut cpu_start: size_t = 0 as libc::c_int as size_t;
    loop  {
        i = cpu_start as u32_0;
        while i < (*afl).cpu_core_count as libc::c_uint {
            if cpu_used[i as usize] == 0 { break ; }
            i = i.wrapping_add(1)
        }
        if i == (*afl).cpu_core_count as libc::c_uint {
            printf(b"\n\x1b[1;91m[-] \x1b[0mUh-oh, looks like all %d CPU cores on your system are allocated to\n    other instances of afl-fuzz (or similar CPU-locked tasks). Starting\n    another fuzzer on this machine is probably a bad plan, but if you are\n    absolutely sure, you can set AFL_NO_AFFINITY and try again.\n\x00"
                       as *const u8 as *const libc::c_char,
                   (*afl).cpu_core_count);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mNo more free CPU cores\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 212 as libc::c_int);
            exit(1 as libc::c_int);
        }
        printf(b"\x1b[1;92m[+] \x1b[0mFound a free CPU core, try binding to #%u.\x00"
                   as *const u8 as *const libc::c_char, i);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        (*afl).cpu_aff = i as s32;
        libc::memset(&mut c as *mut cpu_set_t as *mut libc::c_void,
                     '\u{0}' as i32,
                     ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong as
                         libc::size_t);
        let mut __cpu: size_t = i as size_t;
        if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong) <
               ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong {
            let ref mut fresh1 =
                *c.__bits.as_mut_ptr().offset(__cpu.wrapping_div((8 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<__cpu_mask>()
                                                                                                      as
                                                                                                      libc::c_ulong))
                                                  as isize);
            *fresh1 |=
                (1 as libc::c_int as __cpu_mask) <<
                    __cpu.wrapping_rem((8 as libc::c_int as
                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<__cpu_mask>()
                                                                            as
                                                                            libc::c_ulong))
        } else { };
        if !(sched_setaffinity(0 as libc::c_int,
                               ::std::mem::size_of::<cpu_set_t>() as
                                   libc::c_ulong, &mut c) != 0) {
            break ;
        }
        if cpu_start == (*afl).cpu_core_count as libc::c_ulong {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0msched_setaffinity failed for CPU %d, exit\x00"
                       as *const u8 as *const libc::c_char, i);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 233 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0msched_setaffinity failed to CPU %d, trying next CPU\x00"
                   as *const u8 as *const libc::c_char, i);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        cpu_start = cpu_start.wrapping_add(1)
    };
}
/* HAVE_AFFINITY */
/* Load postprocessor, if available. */
#[no_mangle]
pub unsafe extern "C" fn setup_post(mut afl: *mut afl_state_t) {
    let mut dh: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut fn_0: *mut u8_0 = (*afl).afl_env.afl_post_library;
    let mut tbuf: [u8_0; 6] = [0; 6];
    let mut tlen: u32_0 = 6 as libc::c_int as u32_0;
    strncpy(tbuf.as_mut_ptr() as *mut libc::c_char,
            b"hello\x00" as *const u8 as *const libc::c_char,
            tlen as libc::c_ulong);
    if fn_0.is_null() { return }
    printf(b"\x1b[1;94m[*] \x1b[0mLoading postprocessor from \'%s\'...\x00" as
               *const u8 as *const libc::c_char, fn_0);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    dh = dlopen(fn_0 as *const libc::c_char, 0x2 as libc::c_int);
    if dh.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0m%s\x00"
                   as *const u8 as *const libc::c_char, dlerror());
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               290 as libc::c_int);
        exit(1 as libc::c_int);
    }
    (*afl).post_handler =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _: *mut u8_0,
                                                            _: u32_0,
                                                            _: *mut *mut u8_0)
                                           ->
                                               size_t>>(dlsym(dh,
                                                              b"afl_postprocess\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char));
    if (*afl).post_handler.is_none() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mSymbol \'afl_postprocess\' not found.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               293 as libc::c_int);
        exit(1 as libc::c_int);
    }
    (*afl).post_init =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: *mut afl_state)
                                           ->
                                               *mut libc::c_void>>(dlsym(dh,
                                                                         b"afl_postprocess_init\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char));
    if (*afl).post_init.is_none() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mSymbol \'afl_postprocess_init\' not found.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               295 as libc::c_int);
        exit(1 as libc::c_int);
    }
    (*afl).post_deinit =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void)
                                           ->
                                               *mut libc::c_void>>(dlsym(dh,
                                                                         b"afl_postprocess_deinit\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char));
    if (*afl).post_deinit.is_none() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mSymbol \'afl_postprocess_deinit\' not found.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               297 as libc::c_int);
        exit(1 as libc::c_int);
    }
    /* Do a quick test. It's better to segfault now than later =) */
    let mut post_buf: *mut u8_0 = 0 as *mut u8_0;
    (*afl).post_data =
        (*afl).post_init.expect("non-null function pointer")(afl);
    if (*afl).post_data.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mCould not initialize post handler.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               303 as libc::c_int);
        exit(1 as libc::c_int);
    }
    let mut post_len: size_t =
        (*afl).post_handler.expect("non-null function pointer")((*afl).post_data,
                                                                tbuf.as_mut_ptr(),
                                                                tlen,
                                                                &mut post_buf);
    if post_len == 0 || post_buf.is_null() {
        printf(b"Empty return in test post handler for buf=\"hello\\0\".\x00"
                   as *const u8 as *const libc::c_char);
    }
    printf(b"\x1b[1;92m[+] \x1b[0mPostprocessor installed successfully.\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
}
/* Shuffle an array of pointers. Might be slightly biased. */
unsafe extern "C" fn shuffle_ptrs(mut afl: *mut afl_state_t,
                                  mut ptrs: *mut *mut libc::c_void,
                                  mut cnt: u32_0) {
    let mut i: u32_0 = 0;
    i = 0 as libc::c_int as u32_0;
    while i < cnt.wrapping_sub(2 as libc::c_int as libc::c_uint) {
        let mut j: u32_0 =
            i.wrapping_add(rand_below(afl, cnt.wrapping_sub(i)));
        let mut s: *mut libc::c_void = *ptrs.offset(i as isize);
        let ref mut fresh2 = *ptrs.offset(i as isize);
        *fresh2 = *ptrs.offset(j as isize);
        let ref mut fresh3 = *ptrs.offset(j as isize);
        *fresh3 = s;
        i = i.wrapping_add(1)
    };
}
/* Read all testcases from the input directory, then queue them for testing.
   Called at startup. */
#[no_mangle]
pub unsafe extern "C" fn read_testcases(mut afl: *mut afl_state_t) {
    let mut nl: *mut *mut dirent = 0 as *mut *mut dirent;
    let mut nl_cnt: s32 = 0;
    let mut i: u32_0 = 0;
    let mut fn1: *mut u8_0 = 0 as *mut u8_0;
    let mut val_buf: [[u8_0; 16]; 2] = [[0; 16]; 2];
    /* Auto-detect non-in-place resumption attempts. */
    fn1 =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%s/queue\x00" as *const u8 as *const libc::c_char,
                          (*afl).in_dir);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-fuzz-init.c\x00" as *const u8 as
                            *const libc::c_char, 344 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%s/queue\x00" as *const u8 as *const libc::c_char,
                      (*afl).in_dir);
             _tmp
         });
    if access(fn1 as *const libc::c_char, 0 as libc::c_int) == 0 {
        (*afl).in_dir = fn1
    } else { DFL_ck_free(fn1 as *mut libc::c_void); }
    printf(b"\x1b[1;94m[*] \x1b[0mScanning \'%s\'...\x00" as *const u8 as
               *const libc::c_char, (*afl).in_dir);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    /* We use scandir() + alphasort() rather than readdir() because otherwise,
     the ordering  of test cases would vary somewhat randomly and would be
     difficult to control. */
    nl_cnt =
        scandir((*afl).in_dir as *const libc::c_char, &mut nl, None,
                Some(alphasort as
                         unsafe extern "C" fn(_: *mut *const dirent,
                                              _: *mut *const dirent)
                             -> libc::c_int)); /* not tracked */
    if nl_cnt < 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int ||
               *__errno_location() == 20 as libc::c_int {
            printf(b"\n\x1b[1;91m[-] \x1b[0mThe input directory does not seem to be valid - try again. The fuzzer needs\n    one or more test case to start with - ideally, a small file under 1 kB\n    or so. The cases must be stored as regular files directly in the input\n    directory.\n\x00"
                       as *const u8 as *const libc::c_char);
        }
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                   as *const u8 as *const libc::c_char, (*afl).in_dir);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               371 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    if (*afl).shuffle_queue as libc::c_int != 0 && nl_cnt > 1 as libc::c_int {
        printf(b"\x1b[1;94m[*] \x1b[0mShuffling queue...\x00" as *const u8 as
                   *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        shuffle_ptrs(afl, nl as *mut *mut libc::c_void, nl_cnt as u32_0);
    }
    i = 0 as libc::c_int as u32_0;
    while i < nl_cnt as libc::c_uint {
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
        let mut dfn: [u8_0; 4096] = [0; 4096];
        snprintf(dfn.as_mut_ptr() as *mut libc::c_char,
                 4096 as libc::c_int as libc::c_ulong,
                 b"%s/.state/deterministic_done/%s\x00" as *const u8 as
                     *const libc::c_char, (*afl).in_dir,
                 (**nl.offset(i as isize)).d_name.as_mut_ptr());
        let mut fn2: *mut u8_0 =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/%s\x00" as *const u8 as
                                  *const libc::c_char, (*afl).in_dir,
                              (**nl.offset(i as isize)).d_name.as_mut_ptr());
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-init.c\x00" as *const u8 as
                                *const libc::c_char, 389 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/%s\x00" as *const u8 as *const libc::c_char,
                          (*afl).in_dir,
                          (**nl.offset(i as isize)).d_name.as_mut_ptr());
                 _tmp
             });
        let mut passed_det: u8_0 = 0 as libc::c_int as u8_0;
        free(*nl.offset(i as isize) as *mut libc::c_void);
        if lstat(fn2 as *const libc::c_char, &mut st) != 0 ||
               access(fn2 as *const libc::c_char, 4 as libc::c_int) != 0 {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to access \'%s\'\x00"
                       as *const u8 as *const libc::c_char, fn2);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 396 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        /* This also takes care of . and .. */
        if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
                 0o100000 as libc::c_int as libc::c_uint) || st.st_size == 0
               ||
               !strstr(fn2 as *const libc::c_char,
                       b"/README.txt\x00" as *const u8 as
                           *const libc::c_char).is_null() {
            DFL_ck_free(fn2 as *mut libc::c_void);
        } else {
            if st.st_size >
                   (1 as libc::c_int * 1024 as libc::c_int *
                        1024 as libc::c_int) as libc::c_long {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mTest case \'%s\' is too big (%s, limit is %s)\x00"
                           as *const u8 as *const libc::c_char, fn2,
                       stringify_mem_size(val_buf[0 as libc::c_int as
                                                      usize].as_mut_ptr(),
                                          ::std::mem::size_of::<[u8_0; 16]>()
                                              as libc::c_ulong,
                                          st.st_size as u64_0),
                       stringify_mem_size(val_buf[1 as libc::c_int as
                                                      usize].as_mut_ptr(),
                                          ::std::mem::size_of::<[u8_0; 16]>()
                                              as libc::c_ulong,
                                          (1 as libc::c_int *
                                               1024 as libc::c_int *
                                               1024 as libc::c_int) as
                                              u64_0));
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-init.c\x00" as *const u8 as
                           *const libc::c_char, 410 as libc::c_int);
                exit(1 as libc::c_int);
            }
            /* Check for metadata that indicates that deterministic fuzzing
       is complete for this entry. We don't want to repeat deterministic
       fuzzing when resuming aborted scans, because it would be pointless
       and probably very time-consuming. */
            if access(dfn.as_mut_ptr() as *const libc::c_char,
                      0 as libc::c_int) == 0 {
                passed_det = 1 as libc::c_int as u8_0
            } /* not tracked */
            add_to_queue(afl, fn2, st.st_size as u32_0, passed_det);
        }
        i = i.wrapping_add(1)
    }
    free(nl as *mut libc::c_void);
    if (*afl).queued_paths == 0 {
        printf(b"\n\x1b[1;91m[-] \x1b[0mLooks like there are no valid test cases in the input directory! The fuzzer\n    needs one or more test case to start with - ideally, a small file under\n    1 kB or so. The cases must be stored as regular files directly in the\n    input directory.\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mNo usable test cases in \'%s\'\x00"
                   as *const u8 as *const libc::c_char, (*afl).in_dir);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               436 as libc::c_int);
        exit(1 as libc::c_int);
    }
    (*afl).last_path_time = 0 as libc::c_int as u64_0;
    (*afl).queued_at_start = (*afl).queued_paths;
}
/* Examine map coverage. Called once, for first test case. */
unsafe extern "C" fn check_map_coverage(mut afl: *mut afl_state_t) {
    let mut i: u32_0 = 0;
    if count_bytes(afl, (*afl).fsrv.trace_bits) <
           100 as libc::c_int as libc::c_uint {
        return
    }
    i = ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int) as u32_0;
    while i < ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint {
        if *(*afl).fsrv.trace_bits.offset(i as isize) != 0 { return }
        i = i.wrapping_add(1)
    }
    if (*afl).fsrv.map_size !=
           ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint {
        return
    }
    printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mRecompile binary with newer version of afl to improve coverage!\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
}
/* Perform dry run of all test cases to confirm that the app is working as
   expected. This is done only for the initial inputs, and only once. */
#[no_mangle]
pub unsafe extern "C" fn perform_dry_run(mut afl: *mut afl_state_t) {
    let mut q: *mut queue_entry = (*afl).queue;
    let mut cal_failures: u32_0 = 0 as libc::c_int as u32_0;
    let mut skip_crashes: *mut u8_0 = (*afl).afl_env.afl_skip_crashes;
    while !q.is_null() {
        let mut use_mem: *mut u8_0 = 0 as *mut u8_0;
        let mut res: u8_0 = 0;
        let mut fd: s32 = 0;
        let mut fn_0: *mut u8_0 =
            strrchr((*q).fname as *const libc::c_char,
                    '/' as i32).offset(1 as libc::c_int as isize) as
                *mut u8_0;
        printf(b"\x1b[1;94m[*] \x1b[0mAttempting dry run with \'%s\'...\x00"
                   as *const u8 as *const libc::c_char, fn_0);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        fd = open((*q).fname as *const libc::c_char, 0 as libc::c_int);
        if fd < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                       as *const u8 as *const libc::c_char, (*q).fname);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 482 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        use_mem = DFL_ck_alloc_nozero((*q).len) as *mut u8_0;
        if read(fd, use_mem as *mut libc::c_void, (*q).len as size_t) !=
               (*q).len as libc::c_long {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort read from \'%s\'\x00"
                       as *const u8 as *const libc::c_char, (*q).fname);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 487 as libc::c_int);
            exit(1 as libc::c_int);
        }
        close(fd);
        res =
            calibrate_case(afl, q, use_mem, 0 as libc::c_int as u32_0,
                           1 as libc::c_int as u8_0);
        DFL_ck_free(use_mem as *mut libc::c_void);
        if (*afl).stop_soon != 0 { return }
        if res as libc::c_int == (*afl).crash_mode as libc::c_int ||
               res as libc::c_int == FAULT_NOBITS as libc::c_int {
            printf(b"\x1b[1;90m    len = %u, map size = %u, exec speed = %llu us\n\x1b[0m\x00"
                       as *const u8 as *const libc::c_char, (*q).len,
                   (*q).bitmap_size, (*q).exec_us);
        }
        match res as libc::c_int {
            0 => {
                if q == (*afl).queue { check_map_coverage(afl); }
                if (*afl).crash_mode != 0 {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mTest case \'%s\' does *NOT* crash\x00"
                               as *const u8 as *const libc::c_char, fn_0);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz-init.c\x00" as *const u8 as
                               *const libc::c_char, 506 as libc::c_int);
                    exit(1 as libc::c_int);
                }
            }
            1 => {
                if (*afl).timeout_given != 0 {
                    /* The -t nn+ syntax in the command line sets afl->timeout_given to
             '2' and instructs afl-fuzz to tolerate but skip queue entries that
             time out. */
                    if (*afl).timeout_given as libc::c_int > 1 as libc::c_int
                       {
                        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mTest case results in a timeout (skipping)\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[0m\n\x00" as *const u8 as
                                   *const libc::c_char);
                        (*q).cal_failed = 3 as libc::c_int as u8_0;
                        cal_failures = cal_failures.wrapping_add(1)
                    } else {
                        printf(b"\n\x1b[1;91m[-] \x1b[0mThe program took more than %u ms to process one of the initial test cases.\n    Usually, the right thing to do is to relax the -t option - or to delete it\n    altogether and allow the fuzzer to auto-calibrate. That said, if you know\n    what you are doing and want to simply skip the unruly test cases, append\n    \'+\' at the end of the value passed to -t (\'-t %u+\').\n\x00"
                                   as *const u8 as *const libc::c_char,
                               (*afl).fsrv.exec_tmout,
                               (*afl).fsrv.exec_tmout);
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mTest case \'%s\' results in a timeout\x00"
                                   as *const u8 as *const libc::c_char, fn_0);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-fuzz-init.c\x00" as *const u8 as
                                   *const libc::c_char, 539 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                } else {
                    printf(b"\n\x1b[1;91m[-] \x1b[0mThe program took more than %u ms to process one of the initial test cases.\n    This is bad news; raising the limit with the -t option is possible, but\n    will probably make the fuzzing process extremely slow.\n\n    If this test case is just a fluke, the other option is to just avoid it\n    altogether, and find one that is less of a CPU hog.\n\x00"
                               as *const u8 as *const libc::c_char,
                           (*afl).fsrv.exec_tmout);
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mTest case \'%s\' results in a timeout\x00"
                               as *const u8 as *const libc::c_char, fn_0);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-fuzz-init.c\x00" as *const u8 as
                               *const libc::c_char, 555 as libc::c_int);
                    exit(1 as libc::c_int);
                }
            }
            2 => {
                if !((*afl).crash_mode != 0) {
                    if !skip_crashes.is_null() {
                        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mTest case results in a crash (skipping)\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[0m\n\x00" as *const u8 as
                                   *const libc::c_char);
                        (*q).cal_failed = 3 as libc::c_int as u8_0;
                        cal_failures = cal_failures.wrapping_add(1)
                    } else {
                        if (*afl).fsrv.mem_limit != 0 {
                            let mut val_buf: [u8_0; 16] = [0; 16];
                            printf(b"\n\x1b[1;91m[-] \x1b[0mOops, the program crashed with one of the test cases provided. There are\n    several possible explanations:\n\n    - The test case causes known crashes under normal working conditions. If\n      so, please remove it. The fuzzer should be seeded with interesting\n      inputs - but not ones that cause an outright crash.\n\n    - The current memory limit (%s) is too low for this program, causing\n      it to die due to OOM when parsing valid files. To fix this, try\n      bumping it up with the -m setting in the command line. If in doubt,\n      try something along the lines of:\n\n      ( ulimit -Sd $[%llu << 10]; /path/to/binary [...] <testcase )\n\n      Tip: you can use http://jwilk.net/software/recidivm to quickly\n      estimate the required amount of virtual memory for the binary. Also,\n      if you are using ASAN, see %s/notes_for_asan.md.\n\n    - In QEMU persistent mode the selected address(es) for the loop are not\n      properly cleaning up variables and memory. Try adding\n      AFL_QEMU_PERSISTENT_GPR=1 or select better addresses in the binary.\n\n    - Least likely, there is a horrible bug in the fuzzer. If other options\n      fail, poke <afl-users@googlegroups.com> for troubleshooting tips.\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   stringify_mem_size(val_buf.as_mut_ptr(),
                                                      ::std::mem::size_of::<[u8_0; 16]>()
                                                          as libc::c_ulong,
                                                      (*afl).fsrv.mem_limit <<
                                                          20 as libc::c_int),
                                   (*afl).fsrv.mem_limit.wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulonglong),
                                   doc_path);
                        } else {
                            printf(b"\n\x1b[1;91m[-] \x1b[0mOops, the program crashed with one of the test cases provided. There are\n    several possible explanations:\n\n    - The test case causes known crashes under normal working conditions. If\n      so, please remove it. The fuzzer should be seeded with interesting\n      inputs - but not ones that cause an outright crash.\n\n    - In QEMU persistent mode the selected address(es) for the loop are not\n      properly cleaning up variables and memory. Try adding\n      AFL_QEMU_PERSISTENT_GPR=1 or select better addresses in the binary.\n\n    - Least likely, there is a horrible bug in the fuzzer. If other options\n      fail, poke <afl-users@googlegroups.com> for troubleshooting tips.\n\x00"
                                       as *const u8 as *const libc::c_char);
                        }
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mTest case \'%s\' results in a crash\x00"
                                   as *const u8 as *const libc::c_char, fn_0);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-fuzz-init.c\x00" as *const u8 as
                                   *const libc::c_char, 651 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                }
            }
            3 => {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to execute target application (\'%s\')\x00"
                           as *const u8 as *const libc::c_char,
                       *(*afl).argv.offset(0 as libc::c_int as isize));
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-init.c\x00" as *const u8 as
                           *const libc::c_char, 655 as libc::c_int);
                exit(1 as libc::c_int);
            }
            4 => {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mNo instrumentation detected\x00"
                           as *const u8 as *const libc::c_char);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-init.c\x00" as *const u8 as
                           *const libc::c_char, 657 as libc::c_int);
                exit(1 as libc::c_int);
            }
            5 => {
                (*afl).useless_at_start =
                    (*afl).useless_at_start.wrapping_add(1);
                if (*afl).in_bitmap.is_null() && (*afl).shuffle_queue == 0 {
                    printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mNo new instrumentation output, test case may be useless.\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[0m\n\x00" as *const u8 as
                               *const libc::c_char);
                }
            }
            _ => { }
        }
        if (*q).var_behavior != 0 {
            printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mInstrumentation output varies across runs.\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
        q = (*q).next
    }
    if cal_failures != 0 {
        if cal_failures == (*afl).queued_paths {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mAll test cases time out%s, giving up!\x00"
                       as *const u8 as *const libc::c_char,
                   if !skip_crashes.is_null() {
                       b" or crash\x00" as *const u8 as *const libc::c_char
                   } else { b"\x00" as *const u8 as *const libc::c_char });
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 680 as libc::c_int);
            exit(1 as libc::c_int);
        }
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mSkipped %u test cases (%0.02f%%) due to timeouts%s.\x00"
                   as *const u8 as *const libc::c_char, cal_failures,
               cal_failures as libc::c_double *
                   100 as libc::c_int as libc::c_double /
                   (*afl).queued_paths as libc::c_double,
               if !skip_crashes.is_null() {
                   b" or crashes\x00" as *const u8 as *const libc::c_char
               } else { b"\x00" as *const u8 as *const libc::c_char });
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        if cal_failures.wrapping_mul(5 as libc::c_int as libc::c_uint) >
               (*afl).queued_paths {
            printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0m\x1b[1;91mHigh percentage of rejected test cases, check settings!\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
    }
    printf(b"\x1b[1;92m[+] \x1b[0mAll test cases processed.\x00" as *const u8
               as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
}
/* Helper function: link() if possible, copy otherwise. */
unsafe extern "C" fn link_or_copy(mut old_path: *mut u8_0,
                                  mut new_path: *mut u8_0) {
    let mut i: s32 =
        link(old_path as *const libc::c_char,
             new_path as *const libc::c_char);
    let mut sfd: s32 = 0;
    let mut dfd: s32 = 0;
    let mut tmp: *mut u8_0 = 0 as *mut u8_0;
    if i == 0 { return }
    sfd = open(old_path as *const libc::c_char, 0 as libc::c_int);
    if sfd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                   as *const u8 as *const libc::c_char, old_path);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               706 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    dfd =
        open(new_path as *const libc::c_char,
             0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
             0o600 as libc::c_int);
    if dfd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, new_path);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               709 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    tmp =
        DFL_ck_alloc((64 as libc::c_int * 1024 as libc::c_int) as u32_0) as
            *mut u8_0;
    loop  {
        i =
            read(sfd, tmp as *mut libc::c_void,
                 (64 as libc::c_int * 1024 as libc::c_int) as size_t) as s32;
        if !(i > 0 as libc::c_int) { break ; }
        let mut _len: u32_0 = i as u32_0;
        let mut _res: s32 =
            write(dfd, tmp as *const libc::c_void, _len as size_t) as s32;
        if _res as libc::c_uint != _len {
            if _res < 0 as libc::c_int {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char, new_path);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-init.c\x00" as *const u8 as
                           *const libc::c_char, 714 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            } else {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mShort write to %s\x00"
                           as *const u8 as *const libc::c_char, new_path);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-init.c\x00" as *const u8 as
                           *const libc::c_char, 714 as libc::c_int);
                exit(1 as libc::c_int);
            }
        }
    }
    if i < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mread() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               716 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    DFL_ck_free(tmp as *mut libc::c_void);
    close(sfd);
    close(dfd);
}
/* Create hard links for input test cases in the output directory, choosing
   good names and pivoting accordingly. */
#[no_mangle]
pub unsafe extern "C" fn pivot_inputs(mut afl: *mut afl_state_t) {
    let mut q: *mut queue_entry = (*afl).queue;
    let mut id: u32_0 = 0 as libc::c_int as u32_0;
    printf(b"\x1b[1;94m[*] \x1b[0mCreating hard links for all input files...\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    while !q.is_null() {
        let mut nfn: *mut u8_0 = 0 as *mut u8_0;
        let mut rsl: *mut u8_0 =
            strrchr((*q).fname as *const libc::c_char, '/' as i32) as
                *mut u8_0;
        let mut orig_id: u32_0 = 0;
        if rsl.is_null() { rsl = (*q).fname } else { rsl = rsl.offset(1) }
        /* If the original file name conforms to the syntax and the recorded
       ID matches the one we'd assign, just use the original file name.
       This is valuable for resuming fuzzing runs. */
        if strncmp(rsl as *const libc::c_char,
                   b"id:\x00" as *const u8 as *const libc::c_char,
                   3 as libc::c_int as libc::c_ulong) == 0 &&
               sscanf(rsl.offset(3 as libc::c_int as isize) as
                          *const libc::c_char,
                      b"%06u\x00" as *const u8 as *const libc::c_char,
                      &mut orig_id as *mut u32_0) == 1 as libc::c_int &&
               orig_id == id {
            let mut src_str: *mut u8_0 = 0 as *mut u8_0;
            let mut src_id: u32_0 = 0;
            (*afl).resuming_fuzz = 1 as libc::c_int as u8_0;
            nfn =
                ({
                     let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                     let mut _len: s32 =
                         snprintf(0 as *mut libc::c_char,
                                  0 as libc::c_int as libc::c_ulong,
                                  b"%s/queue/%s\x00" as *const u8 as
                                      *const libc::c_char, (*afl).out_dir,
                                  rsl);
                     if _len < 0 as libc::c_int {
                         printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                    as *const u8 as *const libc::c_char);
                         printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                    as *const u8 as *const libc::c_char,
                                b"func_unknown\x00" as *const u8 as
                                    *const libc::c_char,
                                b"src/afl-fuzz-init.c\x00" as *const u8 as
                                    *const libc::c_char, 755 as libc::c_int);
                         exit(1 as libc::c_int);
                     }
                     _tmp =
                         DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                             *mut u8_0;
                     snprintf(_tmp as *mut libc::c_char,
                              (_len + 1 as libc::c_int) as libc::c_ulong,
                              b"%s/queue/%s\x00" as *const u8 as
                                  *const libc::c_char, (*afl).out_dir, rsl);
                     _tmp
                 });
            /* Since we're at it, let's also try to find parent and figure out the
         appropriate depth for this entry. */
            src_str =
                strchr(rsl.offset(3 as libc::c_int as isize) as
                           *const libc::c_char, ':' as i32) as *mut u8_0;
            if !src_str.is_null() &&
                   sscanf(src_str.offset(1 as libc::c_int as isize) as
                              *const libc::c_char,
                          b"%06u\x00" as *const u8 as *const libc::c_char,
                          &mut src_id as *mut u32_0) == 1 as libc::c_int {
                let mut s: *mut queue_entry = (*afl).queue;
                loop  {
                    let fresh4 = src_id;
                    src_id = src_id.wrapping_sub(1);
                    if !(fresh4 != 0 && !s.is_null()) { break ; }
                    s = (*s).next
                }
                if !s.is_null() {
                    (*q).depth =
                        (*s).depth.wrapping_add(1 as libc::c_int as
                                                    libc::c_ulonglong)
                }
                if ((*afl).max_depth as libc::c_ulonglong) < (*q).depth {
                    (*afl).max_depth = (*q).depth as u32_0
                }
            }
        } else {
            /* No dice - invent a new name, capturing the original one as a
         substring. */
            let mut use_name: *mut u8_0 =
                strstr(rsl as *const libc::c_char,
                       b",orig:\x00" as *const u8 as *const libc::c_char) as
                    *mut u8_0;
            if !use_name.is_null() {
                use_name = use_name.offset(6 as libc::c_int as isize)
            } else { use_name = rsl }
            nfn =
                ({
                     let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                     let mut _len: s32 =
                         snprintf(0 as *mut libc::c_char,
                                  0 as libc::c_int as libc::c_ulong,
                                  b"%s/queue/id:%06u,time:0,orig:%s\x00" as
                                      *const u8 as *const libc::c_char,
                                  (*afl).out_dir, id, use_name);
                     if _len < 0 as libc::c_int {
                         printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                    as *const u8 as *const libc::c_char);
                         printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                    as *const u8 as *const libc::c_char,
                                b"func_unknown\x00" as *const u8 as
                                    *const libc::c_char,
                                b"src/afl-fuzz-init.c\x00" as *const u8 as
                                    *const libc::c_char, 787 as libc::c_int);
                         exit(1 as libc::c_int);
                     }
                     _tmp =
                         DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                             *mut u8_0;
                     snprintf(_tmp as *mut libc::c_char,
                              (_len + 1 as libc::c_int) as libc::c_ulong,
                              b"%s/queue/id:%06u,time:0,orig:%s\x00" as
                                  *const u8 as *const libc::c_char,
                              (*afl).out_dir, id, use_name);
                     _tmp
                 })
        }
        /* Pivot to the new queue entry. */
        link_or_copy((*q).fname, nfn);
        DFL_ck_free((*q).fname as *mut libc::c_void);
        (*q).fname = nfn;
        /* Make sure that the passed_det value carries over, too. */
        if (*q).passed_det != 0 { mark_as_det_done(afl, q); }
        q = (*q).next;
        id = id.wrapping_add(1)
    }
    if (*afl).in_place_resume != 0 { nuke_resume_dir(afl); };
}
/* When resuming, try to find the queue position to start from. This makes sense
   only when resuming, and when we can find the original fuzzer_stats. */
#[no_mangle]
pub unsafe extern "C" fn find_start_position(mut afl: *mut afl_state_t)
 -> u32_0 {
    let mut tmp: [u8_0; 4096] =
        [0 as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; /* Ought to be enough for anybody. */
    let mut fn_0: *mut u8_0 = 0 as *mut u8_0;
    let mut off: *mut u8_0 = 0 as *mut u8_0;
    let mut fd: s32 = 0;
    let mut i: s32 = 0;
    let mut ret: u32_0 = 0;
    if (*afl).resuming_fuzz == 0 { return 0 as libc::c_int as u32_0 }
    if (*afl).in_place_resume != 0 {
        fn_0 =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/fuzzer_stats\x00" as *const u8 as
                                  *const libc::c_char, (*afl).out_dir);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-init.c\x00" as *const u8 as
                                *const libc::c_char, 830 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/fuzzer_stats\x00" as *const u8 as
                              *const libc::c_char, (*afl).out_dir);
                 _tmp
             })
    } else {
        fn_0 =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/../fuzzer_stats\x00" as *const u8 as
                                  *const libc::c_char, (*afl).in_dir);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-init.c\x00" as *const u8 as
                                *const libc::c_char, 832 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/../fuzzer_stats\x00" as *const u8 as
                              *const libc::c_char, (*afl).in_dir);
                 _tmp
             })
    }
    fd = open(fn_0 as *const libc::c_char, 0 as libc::c_int);
    DFL_ck_free(fn_0 as *mut libc::c_void);
    if fd < 0 as libc::c_int { return 0 as libc::c_int as u32_0 }
    i =
        read(fd, tmp.as_mut_ptr() as *mut libc::c_void,
             (::std::mem::size_of::<[u8_0; 4096]>() as
                  libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulong)) as s32;
    /* Ignore errors */
    close(fd);
    off =
        strstr(tmp.as_mut_ptr() as *const libc::c_char,
               b"cur_path          : \x00" as *const u8 as
                   *const libc::c_char) as *mut u8_0;
    if off.is_null() { return 0 as libc::c_int as u32_0 }
    ret =
        atoi(off.offset(20 as libc::c_int as isize) as *const libc::c_char) as
            u32_0;
    if ret >= (*afl).queued_paths { ret = 0 as libc::c_int as u32_0 }
    return ret;
}
/* The same, but for timeouts. The idea is that when resuming sessions without
   -t given, we don't want to keep auto-scaling the timeout over and over
   again to prevent it from growing due to random flukes. */
#[no_mangle]
pub unsafe extern "C" fn find_timeout(mut afl: *mut afl_state_t) {
    let mut tmp: [u8_0; 4096] =
        [0 as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; /* Ought to be enough for anybody. */
    let mut fn_0: *mut u8_0 = 0 as *mut u8_0;
    let mut off: *mut u8_0 = 0 as *mut u8_0;
    let mut fd: s32 = 0;
    let mut i: s32 = 0;
    let mut ret: u32_0 = 0;
    if (*afl).resuming_fuzz == 0 { return }
    if (*afl).in_place_resume != 0 {
        fn_0 =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/fuzzer_stats\x00" as *const u8 as
                                  *const libc::c_char, (*afl).out_dir);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-init.c\x00" as *const u8 as
                                *const libc::c_char, 867 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/fuzzer_stats\x00" as *const u8 as
                              *const libc::c_char, (*afl).out_dir);
                 _tmp
             })
    } else {
        fn_0 =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/../fuzzer_stats\x00" as *const u8 as
                                  *const libc::c_char, (*afl).in_dir);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-init.c\x00" as *const u8 as
                                *const libc::c_char, 869 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/../fuzzer_stats\x00" as *const u8 as
                              *const libc::c_char, (*afl).in_dir);
                 _tmp
             })
    }
    fd = open(fn_0 as *const libc::c_char, 0 as libc::c_int);
    DFL_ck_free(fn_0 as *mut libc::c_void);
    if fd < 0 as libc::c_int { return }
    i =
        read(fd, tmp.as_mut_ptr() as *mut libc::c_void,
             (::std::mem::size_of::<[u8_0; 4096]>() as
                  libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulong)) as s32;
    /* Ignore errors */
    close(fd);
    off =
        strstr(tmp.as_mut_ptr() as *const libc::c_char,
               b"exec_timeout      : \x00" as *const u8 as
                   *const libc::c_char) as *mut u8_0;
    if off.is_null() { return }
    ret =
        atoi(off.offset(20 as libc::c_int as isize) as *const libc::c_char) as
            u32_0;
    if ret <= 4 as libc::c_int as libc::c_uint { return }
    (*afl).fsrv.exec_tmout = ret;
    (*afl).timeout_given = 3 as libc::c_int as u8_0;
}
/* A helper function for handle_existing_out_dir(), deleting all prefixed
   files in a directory. */
unsafe extern "C" fn delete_files(mut path: *mut u8_0, mut prefix: *mut u8_0)
 -> u8_0 {
    let mut d: *mut DIR = 0 as *mut DIR;
    let mut d_ent: *mut dirent = 0 as *mut dirent;
    d = opendir(path as *const libc::c_char);
    if d.is_null() { return 0 as libc::c_int as u8_0 }
    loop  {
        d_ent = readdir(d);
        if d_ent.is_null() { break ; }
        if (*d_ent).d_name[0 as libc::c_int as usize] as libc::c_int !=
               '.' as i32 &&
               (prefix.is_null() ||
                    strncmp((*d_ent).d_name.as_mut_ptr(),
                            prefix as *const libc::c_char,
                            strlen(prefix as *const libc::c_char)) == 0) {
            let mut fname: *mut u8_0 =
                ({
                     let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                     let mut _len: s32 =
                         snprintf(0 as *mut libc::c_char,
                                  0 as libc::c_int as libc::c_ulong,
                                  b"%s/%s\x00" as *const u8 as
                                      *const libc::c_char, path,
                                  (*d_ent).d_name.as_mut_ptr());
                     if _len < 0 as libc::c_int {
                         printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                    as *const u8 as *const libc::c_char);
                         printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                    as *const u8 as *const libc::c_char,
                                b"func_unknown\x00" as *const u8 as
                                    *const libc::c_char,
                                b"src/afl-fuzz-init.c\x00" as *const u8 as
                                    *const libc::c_char, 908 as libc::c_int);
                         exit(1 as libc::c_int);
                     }
                     _tmp =
                         DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                             *mut u8_0;
                     snprintf(_tmp as *mut libc::c_char,
                              (_len + 1 as libc::c_int) as libc::c_ulong,
                              b"%s/%s\x00" as *const u8 as
                                  *const libc::c_char, path,
                              (*d_ent).d_name.as_mut_ptr());
                     _tmp
                 });
            if unlink(fname as *const libc::c_char) != 0 {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to delete \'%s\'\x00"
                           as *const u8 as *const libc::c_char, fname);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-fuzz-init.c\x00" as *const u8 as
                           *const libc::c_char, 909 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            }
            DFL_ck_free(fname as *mut libc::c_void);
        }
    }
    closedir(d);
    return (rmdir(path as *const libc::c_char) != 0) as libc::c_int as u8_0;
}
/* Get the number of runnable processes, with some simple smoothing. */
#[no_mangle]
pub unsafe extern "C" fn get_runnable_processes() -> libc::c_double {
    let mut res: libc::c_double = 0 as libc::c_int as libc::c_double;
    /* On Linux, /proc/stat is probably the best way; load averages are
     computed in funny ways and sometimes don't reflect extremely short-lived
     processes well. */
    let mut f: *mut FILE =
        fopen(b"/proc/stat\x00" as *const u8 as *const libc::c_char,
              b"r\x00" as *const u8 as *const libc::c_char);
    let mut tmp: [u8_0; 1024] = [0; 1024];
    let mut val: u32_0 = 0 as libc::c_int as u32_0;
    if f.is_null() { return 0 as libc::c_int as libc::c_double }
    while !fgets(tmp.as_mut_ptr() as *mut libc::c_char,
                 ::std::mem::size_of::<[u8_0; 1024]>() as libc::c_ulong as
                     libc::c_int, f).is_null() {
        if strncmp(tmp.as_mut_ptr() as *const libc::c_char,
                   b"procs_running \x00" as *const u8 as *const libc::c_char,
                   14 as libc::c_int as libc::c_ulong) == 0 ||
               strncmp(tmp.as_mut_ptr() as *const libc::c_char,
                       b"procs_blocked \x00" as *const u8 as
                           *const libc::c_char,
                       14 as libc::c_int as libc::c_ulong) == 0 {
            val =
                (val as
                     libc::c_uint).wrapping_add(atoi(tmp.as_mut_ptr().offset(14
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)
                                                         as
                                                         *const libc::c_char)
                                                    as libc::c_uint) as u32_0
                    as u32_0
        }
    }
    fclose(f);
    if res == 0. {
        res = val as libc::c_double
    } else {
        res =
            res * (1.0f64 - 1.0f64 / 16 as libc::c_int as libc::c_double) +
                val as libc::c_double *
                    (1.0f64 / 16 as libc::c_int as libc::c_double)
    }
    /* ^(__APPLE__ || __FreeBSD__ || __OpenBSD__ || __NetBSD__) */
    return res;
}
/* Delete the temporary directory used for in-place session resume. */
#[no_mangle]
pub unsafe extern "C" fn nuke_resume_dir(mut afl: *mut afl_state_t) {
    let mut fn_0: *mut u8_0 = 0 as *mut u8_0;
    fn_0 =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%s/_resume/.state/deterministic_done\x00" as
                              *const u8 as *const libc::c_char,
                          (*afl).out_dir);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-fuzz-init.c\x00" as *const u8 as
                            *const libc::c_char, 982 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%s/_resume/.state/deterministic_done\x00" as *const u8
                          as *const libc::c_char, (*afl).out_dir);
             _tmp
         });
    if !(delete_files(fn_0,
                      b"id:\x00" as *const u8 as *const libc::c_char as
                          *mut u8_0) != 0) {
        DFL_ck_free(fn_0 as *mut libc::c_void);
        fn_0 =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/_resume/.state/auto_extras\x00" as
                                  *const u8 as *const libc::c_char,
                              (*afl).out_dir);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-init.c\x00" as *const u8 as
                                *const libc::c_char, 986 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/_resume/.state/auto_extras\x00" as *const u8 as
                              *const libc::c_char, (*afl).out_dir);
                 _tmp
             });
        if !(delete_files(fn_0,
                          b"auto_\x00" as *const u8 as *const libc::c_char as
                              *mut u8_0) != 0) {
            DFL_ck_free(fn_0 as *mut libc::c_void);
            fn_0 =
                ({
                     let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                     let mut _len: s32 =
                         snprintf(0 as *mut libc::c_char,
                                  0 as libc::c_int as libc::c_ulong,
                                  b"%s/_resume/.state/redundant_edges\x00" as
                                      *const u8 as *const libc::c_char,
                                  (*afl).out_dir);
                     if _len < 0 as libc::c_int {
                         printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                    as *const u8 as *const libc::c_char);
                         printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                    as *const u8 as *const libc::c_char,
                                b"func_unknown\x00" as *const u8 as
                                    *const libc::c_char,
                                b"src/afl-fuzz-init.c\x00" as *const u8 as
                                    *const libc::c_char, 990 as libc::c_int);
                         exit(1 as libc::c_int);
                     }
                     _tmp =
                         DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                             *mut u8_0;
                     snprintf(_tmp as *mut libc::c_char,
                              (_len + 1 as libc::c_int) as libc::c_ulong,
                              b"%s/_resume/.state/redundant_edges\x00" as
                                  *const u8 as *const libc::c_char,
                              (*afl).out_dir);
                     _tmp
                 });
            if !(delete_files(fn_0,
                              b"id:\x00" as *const u8 as *const libc::c_char
                                  as *mut u8_0) != 0) {
                DFL_ck_free(fn_0 as *mut libc::c_void);
                fn_0 =
                    ({
                         let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                         let mut _len: s32 =
                             snprintf(0 as *mut libc::c_char,
                                      0 as libc::c_int as libc::c_ulong,
                                      b"%s/_resume/.state/variable_behavior\x00"
                                          as *const u8 as *const libc::c_char,
                                      (*afl).out_dir);
                         if _len < 0 as libc::c_int {
                             printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                        as *const u8 as *const libc::c_char);
                             printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    b"func_unknown\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"src/afl-fuzz-init.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    994 as libc::c_int);
                             exit(1 as libc::c_int);
                         }
                         _tmp =
                             DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0)
                                 as *mut u8_0;
                         snprintf(_tmp as *mut libc::c_char,
                                  (_len + 1 as libc::c_int) as libc::c_ulong,
                                  b"%s/_resume/.state/variable_behavior\x00"
                                      as *const u8 as *const libc::c_char,
                                  (*afl).out_dir);
                         _tmp
                     });
                if !(delete_files(fn_0,
                                  b"id:\x00" as *const u8 as
                                      *const libc::c_char as *mut u8_0) != 0)
                   {
                    DFL_ck_free(fn_0 as *mut libc::c_void);
                    fn_0 =
                        ({
                             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                             let mut _len: s32 =
                                 snprintf(0 as *mut libc::c_char,
                                          0 as libc::c_int as libc::c_ulong,
                                          b"%s/_resume/.state\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          (*afl).out_dir);
                             if _len < 0 as libc::c_int {
                                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        b"func_unknown\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"src/afl-fuzz-init.c\x00" as
                                            *const u8 as *const libc::c_char,
                                        998 as libc::c_int);
                                 exit(1 as libc::c_int);
                             }
                             _tmp =
                                 DFL_ck_alloc((_len + 1 as libc::c_int) as
                                                  u32_0) as *mut u8_0;
                             snprintf(_tmp as *mut libc::c_char,
                                      (_len + 1 as libc::c_int) as
                                          libc::c_ulong,
                                      b"%s/_resume/.state\x00" as *const u8 as
                                          *const libc::c_char,
                                      (*afl).out_dir);
                             _tmp
                         });
                    if !(rmdir(fn_0 as *const libc::c_char) != 0 &&
                             *__errno_location() != 2 as libc::c_int) {
                        DFL_ck_free(fn_0 as *mut libc::c_void);
                        fn_0 =
                            ({
                                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                                 let mut _len: s32 =
                                     snprintf(0 as *mut libc::c_char,
                                              0 as libc::c_int as
                                                  libc::c_ulong,
                                              b"%s/_resume\x00" as *const u8
                                                  as *const libc::c_char,
                                              (*afl).out_dir);
                                 if _len < 0 as libc::c_int {
                                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            b"func_unknown\x00" as *const u8
                                                as *const libc::c_char,
                                            b"src/afl-fuzz-init.c\x00" as
                                                *const u8 as
                                                *const libc::c_char,
                                            1002 as libc::c_int);
                                     exit(1 as libc::c_int);
                                 }
                                 _tmp =
                                     DFL_ck_alloc((_len + 1 as libc::c_int) as
                                                      u32_0) as *mut u8_0;
                                 snprintf(_tmp as *mut libc::c_char,
                                          (_len + 1 as libc::c_int) as
                                              libc::c_ulong,
                                          b"%s/_resume\x00" as *const u8 as
                                              *const libc::c_char,
                                          (*afl).out_dir);
                                 _tmp
                             });
                        if !(delete_files(fn_0,
                                          b"id:\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut u8_0) != 0) {
                            DFL_ck_free(fn_0 as *mut libc::c_void);
                            return
                        }
                    }
                }
            }
        }
    }
    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0m_resume directory cleanup failed\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00" as
               *const u8 as *const libc::c_char,
           b"func_unknown\x00" as *const u8 as *const libc::c_char,
           b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
           1010 as libc::c_int);
    exit(1 as libc::c_int);
}
/* Delete fuzzer output directory if we recognize it as ours, if the fuzzer
   is not currently running, and if the last run time isn't too great.
   Resume fuzzing if `-` is set as in_dir or if AFL_AUTORESUME is set */
unsafe extern "C" fn handle_existing_out_dir(mut afl: *mut afl_state_t) {
    let mut current_block: u64;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut fn_0: *mut u8_0 =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%s/fuzzer_stats\x00" as *const u8 as
                              *const libc::c_char, (*afl).out_dir);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-fuzz-init.c\x00" as *const u8 as
                            *const libc::c_char, 1021 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%s/fuzzer_stats\x00" as *const u8 as
                          *const libc::c_char, (*afl).out_dir);
             _tmp
         });
    /* See if the output directory is locked. If yes, bail out. If not,
     create a lock that will persist for the lifetime of the process
     (this requires leaving the descriptor open).*/
    (*afl).fsrv.out_dir_fd =
        open((*afl).out_dir as *const libc::c_char, 0 as libc::c_int);
    if (*afl).fsrv.out_dir_fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                   as *const u8 as *const libc::c_char, (*afl).out_dir);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1028 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    if flock((*afl).fsrv.out_dir_fd, 2 as libc::c_int | 4 as libc::c_int) != 0
           && *__errno_location() == 11 as libc::c_int {
        printf(b"\n\x1b[1;91m[-] \x1b[0mLooks like the job output directory is being actively used by another\n    instance of afl-fuzz. You will need to choose a different %s\n    or stop the other process first.\n\x00"
                   as *const u8 as *const libc::c_char,
               if !(*afl).sync_id.is_null() {
                   b"fuzzer ID\x00" as *const u8 as *const libc::c_char
               } else {
                   b"output location\x00" as *const u8 as *const libc::c_char
               });
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mDirectory \'%s\' is in use\x00"
                   as *const u8 as *const libc::c_char, (*afl).out_dir);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1041 as libc::c_int);
        exit(1 as libc::c_int);
    }
    /* !__sun */
    f =
        fopen(fn_0 as *const libc::c_char,
              b"r\x00" as *const u8 as *const libc::c_char);
    if !f.is_null() {
        let mut start_time2: u64_0 = 0;
        let mut last_update: u64_0 = 0;
        if fscanf(f,
                  b"start_time     : %llu\nlast_update    : %llu\n\x00" as
                      *const u8 as *const libc::c_char,
                  &mut start_time2 as *mut u64_0,
                  &mut last_update as *mut u64_0) != 2 as libc::c_int {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMalformed data in \'%s\'\x00"
                       as *const u8 as *const libc::c_char, fn_0);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1057 as libc::c_int);
            exit(1 as libc::c_int);
        }
        fclose(f);
        /* Autoresume treats a normal run as in_place_resume if a valid out dir
     * already exists */
        if (*afl).in_place_resume == 0 &&
               (*afl).autoresume as libc::c_int != 0 {
            printf(b"\x1b[1;92m[+] \x1b[0mDetected prior run with AFL_AUTORESUME set. Resuming.\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
            (*afl).in_place_resume = 1 as libc::c_int as u8_0
        }
        /* Let's see how much work is at stake. */
        if (*afl).in_place_resume == 0 && last_update > start_time2 &&
               last_update.wrapping_sub(start_time2) >
                   (25 as libc::c_int * 60 as libc::c_int) as
                       libc::c_ulonglong {
            printf(b"\n\x1b[1;91m[-] \x1b[0mThe job output directory already exists and contains the results of more\n    than %d minutes worth of fuzzing. To avoid data loss, afl-fuzz will *NOT*\n    automatically delete this data for you.\n\n    If you wish to start a new session, remove or rename the directory manually,\n    or specify a different output location for this job. To resume the old\n    session, pass \'-\' as input directory in the command line (\'-i -\')\n    or set the \'AFL_AUTORESUME=1\' env variable and try again.\n\x00"
                       as *const u8 as *const libc::c_char,
                   25 as libc::c_int);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mAt-risk data found in \'%s\'\x00"
                       as *const u8 as *const libc::c_char, (*afl).out_dir);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1092 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    DFL_ck_free(fn_0 as *mut libc::c_void);
    /* The idea for in-place resume is pretty simple: we temporarily move the old
     queue/ to a new location that gets deleted once import to the new queue/
     is finished. If _resume/ already exists, the current queue/ may be
     incomplete due to an earlier abort, so we want to use the old _resume/
     dir instead, and we let rename() fail silently. */
    if (*afl).in_place_resume != 0 {
        let mut orig_q: *mut u8_0 =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0; /* Ignore errors */
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/queue\x00" as *const u8 as
                                  *const libc::c_char, (*afl).out_dir);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-init.c\x00" as *const u8 as
                                *const libc::c_char, 1108 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/queue\x00" as *const u8 as *const libc::c_char,
                          (*afl).out_dir);
                 _tmp
             });
        (*afl).in_dir =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/_resume\x00" as *const u8 as
                                  *const libc::c_char, (*afl).out_dir);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-init.c\x00" as *const u8 as
                                *const libc::c_char, 1110 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/_resume\x00" as *const u8 as
                              *const libc::c_char, (*afl).out_dir);
                 _tmp
             });
        rename(orig_q as *const libc::c_char,
               (*afl).in_dir as *const libc::c_char);
        printf(b"\x1b[1;92m[+] \x1b[0mOutput directory exists, will attempt session resume.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        DFL_ck_free(orig_q as *mut libc::c_void);
    } else {
        printf(b"\x1b[1;92m[+] \x1b[0mOutput directory exists but deemed OK to reuse.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    printf(b"\x1b[1;94m[*] \x1b[0mDeleting old session data...\x00" as
               *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    /* Okay, let's get the ball rolling! First, we need to get rid of the entries
     in <afl->out_dir>/.synced/.../id:*, if any are present. */
    if (*afl).in_place_resume == 0 {
        fn_0 =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/.synced\x00" as *const u8 as
                                  *const libc::c_char, (*afl).out_dir);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-init.c\x00" as *const u8 as
                                *const libc::c_char, 1131 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/.synced\x00" as *const u8 as
                              *const libc::c_char, (*afl).out_dir);
                 _tmp
             });
        if delete_files(fn_0, 0 as *mut u8_0) != 0 {
            current_block = 16973340533559238807;
        } else {
            DFL_ck_free(fn_0 as *mut libc::c_void);
            current_block = 9512719473022792396;
        }
    } else { current_block = 9512719473022792396; }
    match current_block {
        9512719473022792396 => {
            /* Next, we need to clean up <afl->out_dir>/queue/.state/ subdirectories: */
            fn_0 =
                ({
                     let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                     let mut _len: s32 =
                         snprintf(0 as *mut libc::c_char,
                                  0 as libc::c_int as libc::c_ulong,
                                  b"%s/queue/.state/deterministic_done\x00" as
                                      *const u8 as *const libc::c_char,
                                  (*afl).out_dir);
                     if _len < 0 as libc::c_int {
                         printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                    as *const u8 as *const libc::c_char);
                         printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                    as *const u8 as *const libc::c_char,
                                b"func_unknown\x00" as *const u8 as
                                    *const libc::c_char,
                                b"src/afl-fuzz-init.c\x00" as *const u8 as
                                    *const libc::c_char, 1139 as libc::c_int);
                         exit(1 as libc::c_int);
                     }
                     _tmp =
                         DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                             *mut u8_0;
                     snprintf(_tmp as *mut libc::c_char,
                              (_len + 1 as libc::c_int) as libc::c_ulong,
                              b"%s/queue/.state/deterministic_done\x00" as
                                  *const u8 as *const libc::c_char,
                              (*afl).out_dir);
                     _tmp
                 });
            if !(delete_files(fn_0,
                              b"id:\x00" as *const u8 as *const libc::c_char
                                  as *mut u8_0) != 0) {
                DFL_ck_free(fn_0 as *mut libc::c_void);
                fn_0 =
                    ({
                         let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                         let mut _len: s32 =
                             snprintf(0 as *mut libc::c_char,
                                      0 as libc::c_int as libc::c_ulong,
                                      b"%s/queue/.state/auto_extras\x00" as
                                          *const u8 as *const libc::c_char,
                                      (*afl).out_dir);
                         if _len < 0 as libc::c_int {
                             printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                        as *const u8 as *const libc::c_char);
                             printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    b"func_unknown\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"src/afl-fuzz-init.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    1143 as libc::c_int);
                             exit(1 as libc::c_int);
                         }
                         _tmp =
                             DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0)
                                 as *mut u8_0;
                         snprintf(_tmp as *mut libc::c_char,
                                  (_len + 1 as libc::c_int) as libc::c_ulong,
                                  b"%s/queue/.state/auto_extras\x00" as
                                      *const u8 as *const libc::c_char,
                                  (*afl).out_dir);
                         _tmp
                     });
                if !(delete_files(fn_0,
                                  b"auto_\x00" as *const u8 as
                                      *const libc::c_char as *mut u8_0) != 0)
                   {
                    DFL_ck_free(fn_0 as *mut libc::c_void);
                    fn_0 =
                        ({
                             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                             let mut _len: s32 =
                                 snprintf(0 as *mut libc::c_char,
                                          0 as libc::c_int as libc::c_ulong,
                                          b"%s/queue/.state/redundant_edges\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          (*afl).out_dir);
                             if _len < 0 as libc::c_int {
                                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        b"func_unknown\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"src/afl-fuzz-init.c\x00" as
                                            *const u8 as *const libc::c_char,
                                        1147 as libc::c_int);
                                 exit(1 as libc::c_int);
                             }
                             _tmp =
                                 DFL_ck_alloc((_len + 1 as libc::c_int) as
                                                  u32_0) as *mut u8_0;
                             snprintf(_tmp as *mut libc::c_char,
                                      (_len + 1 as libc::c_int) as
                                          libc::c_ulong,
                                      b"%s/queue/.state/redundant_edges\x00"
                                          as *const u8 as *const libc::c_char,
                                      (*afl).out_dir);
                             _tmp
                         });
                    if !(delete_files(fn_0,
                                      b"id:\x00" as *const u8 as
                                          *const libc::c_char as *mut u8_0) !=
                             0) {
                        DFL_ck_free(fn_0 as *mut libc::c_void);
                        fn_0 =
                            ({
                                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                                 let mut _len: s32 =
                                     snprintf(0 as *mut libc::c_char,
                                              0 as libc::c_int as
                                                  libc::c_ulong,
                                              b"%s/queue/.state/variable_behavior\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              (*afl).out_dir);
                                 if _len < 0 as libc::c_int {
                                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            b"func_unknown\x00" as *const u8
                                                as *const libc::c_char,
                                            b"src/afl-fuzz-init.c\x00" as
                                                *const u8 as
                                                *const libc::c_char,
                                            1151 as libc::c_int);
                                     exit(1 as libc::c_int);
                                 }
                                 _tmp =
                                     DFL_ck_alloc((_len + 1 as libc::c_int) as
                                                      u32_0) as *mut u8_0;
                                 snprintf(_tmp as *mut libc::c_char,
                                          (_len + 1 as libc::c_int) as
                                              libc::c_ulong,
                                          b"%s/queue/.state/variable_behavior\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          (*afl).out_dir);
                                 _tmp
                             });
                        if !(delete_files(fn_0,
                                          b"id:\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut u8_0) != 0) {
                            DFL_ck_free(fn_0 as *mut libc::c_void);
                            /* Then, get rid of the .state subdirectory itself (should be empty by now)
     and everything matching <afl->out_dir>/queue/id:*. */
                            fn_0 =
                                ({
                                     let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                                     let mut _len: s32 =
                                         snprintf(0 as *mut libc::c_char,
                                                  0 as libc::c_int as
                                                      libc::c_ulong,
                                                  b"%s/queue/.state\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*afl).out_dir);
                                     if _len < 0 as libc::c_int {
                                         printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                         printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                b"func_unknown\x00" as
                                                    *const u8 as
                                                    *const libc::c_char,
                                                b"src/afl-fuzz-init.c\x00" as
                                                    *const u8 as
                                                    *const libc::c_char,
                                                1158 as libc::c_int);
                                         exit(1 as libc::c_int);
                                     }
                                     _tmp =
                                         DFL_ck_alloc((_len +
                                                           1 as libc::c_int)
                                                          as u32_0) as
                                             *mut u8_0;
                                     snprintf(_tmp as *mut libc::c_char,
                                              (_len + 1 as libc::c_int) as
                                                  libc::c_ulong,
                                              b"%s/queue/.state\x00" as
                                                  *const u8 as
                                                  *const libc::c_char,
                                              (*afl).out_dir);
                                     _tmp
                                 });
                            if !(rmdir(fn_0 as *const libc::c_char) != 0 &&
                                     *__errno_location() != 2 as libc::c_int)
                               {
                                DFL_ck_free(fn_0 as *mut libc::c_void);
                                fn_0 =
                                    ({
                                         let mut _tmp: *mut u8_0 =
                                             0 as *mut u8_0;
                                         let mut _len: s32 =
                                             snprintf(0 as *mut libc::c_char,
                                                      0 as libc::c_int as
                                                          libc::c_ulong,
                                                      b"%s/queue\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      (*afl).out_dir);
                                         if _len < 0 as libc::c_int {
                                             printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                             printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    b"func_unknown\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"src/afl-fuzz-init.c\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    1162 as libc::c_int);
                                             exit(1 as libc::c_int);
                                         }
                                         _tmp =
                                             DFL_ck_alloc((_len +
                                                               1 as
                                                                   libc::c_int)
                                                              as u32_0) as
                                                 *mut u8_0;
                                         snprintf(_tmp as *mut libc::c_char,
                                                  (_len + 1 as libc::c_int) as
                                                      libc::c_ulong,
                                                  b"%s/queue\x00" as *const u8
                                                      as *const libc::c_char,
                                                  (*afl).out_dir);
                                         _tmp
                                     });
                                if !(delete_files(fn_0,
                                                  b"id:\x00" as *const u8 as
                                                      *const libc::c_char as
                                                      *mut u8_0) != 0) {
                                    DFL_ck_free(fn_0 as *mut libc::c_void);
                                    /* All right, let's do <afl->out_dir>/crashes/id:* and
   * <afl->out_dir>/hangs/id:*. */
                                    if (*afl).in_place_resume == 0 {
                                        fn_0 =
                                            ({
                                                 let mut _tmp: *mut u8_0 =
                                                     0 as
                                                         *mut u8_0; /* Ignore errors */
                                                 let mut _len: s32 =
                                                     snprintf(0 as
                                                                  *mut libc::c_char,
                                                              0 as libc::c_int
                                                                  as
                                                                  libc::c_ulong,
                                                              b"%s/crashes/README.txt\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              (*afl).out_dir);
                                                 if _len < 0 as libc::c_int {
                                                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            b"func_unknown\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            b"src/afl-fuzz-init.c\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            1171 as
                                                                libc::c_int);
                                                     exit(1 as libc::c_int);
                                                 }
                                                 _tmp =
                                                     DFL_ck_alloc((_len +
                                                                       1 as
                                                                           libc::c_int)
                                                                      as
                                                                      u32_0)
                                                         as *mut u8_0;
                                                 snprintf(_tmp as
                                                              *mut libc::c_char,
                                                          (_len +
                                                               1 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_ulong,
                                                          b"%s/crashes/README.txt\x00"
                                                              as *const u8 as
                                                              *const libc::c_char,
                                                          (*afl).out_dir);
                                                 _tmp
                                             });
                                        unlink(fn_0 as *const libc::c_char);
                                        DFL_ck_free(fn_0 as
                                                        *mut libc::c_void);
                                    }
                                    fn_0 =
                                        ({
                                             let mut _tmp: *mut u8_0 =
                                                 0 as *mut u8_0;
                                             let mut _len: s32 =
                                                 snprintf(0 as
                                                              *mut libc::c_char,
                                                          0 as libc::c_int as
                                                              libc::c_ulong,
                                                          b"%s/crashes\x00" as
                                                              *const u8 as
                                                              *const libc::c_char,
                                                          (*afl).out_dir);
                                             if _len < 0 as libc::c_int {
                                                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        b"func_unknown\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        b"src/afl-fuzz-init.c\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        1177 as libc::c_int);
                                                 exit(1 as libc::c_int);
                                             }
                                             _tmp =
                                                 DFL_ck_alloc((_len +
                                                                   1 as
                                                                       libc::c_int)
                                                                  as u32_0) as
                                                     *mut u8_0;
                                             snprintf(_tmp as
                                                          *mut libc::c_char,
                                                      (_len +
                                                           1 as libc::c_int)
                                                          as libc::c_ulong,
                                                      b"%s/crashes\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      (*afl).out_dir);
                                             _tmp
                                         });
                                    /* Make backup of the crashes directory if it's not empty and if we're
     doing in-place resume. */
                                    if (*afl).in_place_resume as libc::c_int
                                           != 0 &&
                                           rmdir(fn_0 as *const libc::c_char)
                                               != 0 {
                                        let mut cur_t: time_t =
                                            time(0 as *mut time_t);
                                        let mut t: *mut tm =
                                            localtime(&mut cur_t);
                                        let mut nfn: *mut u8_0 =
                                            ({
                                                 let mut _tmp: *mut u8_0 =
                                                     0 as *mut u8_0;
                                                 let mut _len: s32 =
                                                     snprintf(0 as
                                                                  *mut libc::c_char,
                                                              0 as libc::c_int
                                                                  as
                                                                  libc::c_ulong,
                                                              b"%s.%04d-%02d-%02d-%02d:%02d:%02d\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              fn_0,
                                                              (*t).tm_year +
                                                                  1900 as
                                                                      libc::c_int,
                                                              (*t).tm_mon +
                                                                  1 as
                                                                      libc::c_int,
                                                              (*t).tm_mday,
                                                              (*t).tm_hour,
                                                              (*t).tm_min,
                                                              (*t).tm_sec);
                                                 if _len < 0 as libc::c_int {
                                                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            b"func_unknown\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            b"src/afl-fuzz-init.c\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            1191 as
                                                                libc::c_int);
                                                     exit(1 as libc::c_int);
                                                 }
                                                 _tmp =
                                                     DFL_ck_alloc((_len +
                                                                       1 as
                                                                           libc::c_int)
                                                                      as
                                                                      u32_0)
                                                         as *mut u8_0;
                                                 snprintf(_tmp as
                                                              *mut libc::c_char,
                                                          (_len +
                                                               1 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_ulong,
                                                          b"%s.%04d-%02d-%02d-%02d:%02d:%02d\x00"
                                                              as *const u8 as
                                                              *const libc::c_char,
                                                          fn_0,
                                                          (*t).tm_year +
                                                              1900 as
                                                                  libc::c_int,
                                                          (*t).tm_mon +
                                                              1 as
                                                                  libc::c_int,
                                                          (*t).tm_mday,
                                                          (*t).tm_hour,
                                                          (*t).tm_min,
                                                          (*t).tm_sec);
                                                 _tmp
                                             });
                                        /* ^!SIMPLE_FILES */
                                        rename(fn_0 as *const libc::c_char,
                                               nfn as
                                                   *const libc::c_char); /* Ignore errors. */
                                        DFL_ck_free(nfn as *mut libc::c_void);
                                    }
                                    if !(delete_files(fn_0,
                                                      b"id:\x00" as *const u8
                                                          as
                                                          *const libc::c_char
                                                          as *mut u8_0) != 0)
                                       {
                                        DFL_ck_free(fn_0 as
                                                        *mut libc::c_void);
                                        fn_0 =
                                            ({
                                                 let mut _tmp: *mut u8_0 =
                                                     0 as *mut u8_0;
                                                 let mut _len: s32 =
                                                     snprintf(0 as
                                                                  *mut libc::c_char,
                                                              0 as libc::c_int
                                                                  as
                                                                  libc::c_ulong,
                                                              b"%s/hangs\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              (*afl).out_dir);
                                                 if _len < 0 as libc::c_int {
                                                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            b"func_unknown\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            b"src/afl-fuzz-init.c\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            1209 as
                                                                libc::c_int);
                                                     exit(1 as libc::c_int);
                                                 }
                                                 _tmp =
                                                     DFL_ck_alloc((_len +
                                                                       1 as
                                                                           libc::c_int)
                                                                      as
                                                                      u32_0)
                                                         as *mut u8_0;
                                                 snprintf(_tmp as
                                                              *mut libc::c_char,
                                                          (_len +
                                                               1 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_ulong,
                                                          b"%s/hangs\x00" as
                                                              *const u8 as
                                                              *const libc::c_char,
                                                          (*afl).out_dir);
                                                 _tmp
                                             });
                                        /* Backup hangs, too. */
                                        if (*afl).in_place_resume as
                                               libc::c_int != 0 &&
                                               rmdir(fn_0 as
                                                         *const libc::c_char)
                                                   != 0 {
                                            let mut cur_t_0: time_t =
                                                time(0 as *mut time_t);
                                            let mut t_0: *mut tm =
                                                localtime(&mut cur_t_0);
                                            let mut nfn_0: *mut u8_0 =
                                                ({
                                                     let mut _tmp: *mut u8_0 =
                                                         0 as *mut u8_0;
                                                     let mut _len: s32 =
                                                         snprintf(0 as
                                                                      *mut libc::c_char,
                                                                  0 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong,
                                                                  b"%s.%04d-%02d-%02d-%02d:%02d:%02d\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  fn_0,
                                                                  (*t_0).tm_year
                                                                      +
                                                                      1900 as
                                                                          libc::c_int,
                                                                  (*t_0).tm_mon
                                                                      +
                                                                      1 as
                                                                          libc::c_int,
                                                                  (*t_0).tm_mday,
                                                                  (*t_0).tm_hour,
                                                                  (*t_0).tm_min,
                                                                  (*t_0).tm_sec);
                                                     if _len <
                                                            0 as libc::c_int {
                                                         printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
                                                         printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                b"func_unknown\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                b"src/afl-fuzz-init.c\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                1222 as
                                                                    libc::c_int);
                                                         exit(1 as
                                                                  libc::c_int);
                                                     }
                                                     _tmp =
                                                         DFL_ck_alloc((_len +
                                                                           1
                                                                               as
                                                                               libc::c_int)
                                                                          as
                                                                          u32_0)
                                                             as *mut u8_0;
                                                     snprintf(_tmp as
                                                                  *mut libc::c_char,
                                                              (_len +
                                                                   1 as
                                                                       libc::c_int)
                                                                  as
                                                                  libc::c_ulong,
                                                              b"%s.%04d-%02d-%02d-%02d:%02d:%02d\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              fn_0,
                                                              (*t_0).tm_year +
                                                                  1900 as
                                                                      libc::c_int,
                                                              (*t_0).tm_mon +
                                                                  1 as
                                                                      libc::c_int,
                                                              (*t_0).tm_mday,
                                                              (*t_0).tm_hour,
                                                              (*t_0).tm_min,
                                                              (*t_0).tm_sec);
                                                     _tmp
                                                 });
                                            /* ^!SIMPLE_FILES */
                                            rename(fn_0 as
                                                       *const libc::c_char,
                                                   nfn_0 as
                                                       *const libc::c_char); /* Ignore errors. */
                                            DFL_ck_free(nfn_0 as
                                                            *mut libc::c_void);
                                        }
                                        if !(delete_files(fn_0,
                                                          b"id:\x00" as
                                                              *const u8 as
                                                              *const libc::c_char
                                                              as *mut u8_0) !=
                                                 0) {
                                            DFL_ck_free(fn_0 as
                                                            *mut libc::c_void);
                                            /* And now, for some finishing touches. */
                                            if !(*afl).file_extension.is_null()
                                               {
                                                fn_0 =
                                                    ({
                                                         let mut _tmp:
                                                                 *mut u8_0 =
                                                             0 as *mut u8_0;
                                                         let mut _len: s32 =
                                                             snprintf(0 as
                                                                          *mut libc::c_char,
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong,
                                                                      b"%s/.cur_input.%s\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      (*afl).tmp_dir,
                                                                      (*afl).file_extension);
                                                         if _len <
                                                                0 as
                                                                    libc::c_int
                                                            {
                                                             printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                             printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    b"func_unknown\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    b"src/afl-fuzz-init.c\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    1244 as
                                                                        libc::c_int);
                                                             exit(1 as
                                                                      libc::c_int);
                                                         }
                                                         _tmp =
                                                             DFL_ck_alloc((_len
                                                                               +
                                                                               1
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              u32_0)
                                                                 as *mut u8_0;
                                                         snprintf(_tmp as
                                                                      *mut libc::c_char,
                                                                  (_len +
                                                                       1 as
                                                                           libc::c_int)
                                                                      as
                                                                      libc::c_ulong,
                                                                  b"%s/.cur_input.%s\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  (*afl).tmp_dir,
                                                                  (*afl).file_extension);
                                                         _tmp
                                                     })
                                            } else {
                                                fn_0 =
                                                    ({
                                                         let mut _tmp:
                                                                 *mut u8_0 =
                                                             0 as *mut u8_0;
                                                         let mut _len: s32 =
                                                             snprintf(0 as
                                                                          *mut libc::c_char,
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong,
                                                                      b"%s/.cur_input\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      (*afl).tmp_dir);
                                                         if _len <
                                                                0 as
                                                                    libc::c_int
                                                            {
                                                             printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                             printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    b"func_unknown\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    b"src/afl-fuzz-init.c\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    1248 as
                                                                        libc::c_int);
                                                             exit(1 as
                                                                      libc::c_int);
                                                         }
                                                         _tmp =
                                                             DFL_ck_alloc((_len
                                                                               +
                                                                               1
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              u32_0)
                                                                 as *mut u8_0;
                                                         snprintf(_tmp as
                                                                      *mut libc::c_char,
                                                                  (_len +
                                                                       1 as
                                                                           libc::c_int)
                                                                      as
                                                                      libc::c_ulong,
                                                                  b"%s/.cur_input\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  (*afl).tmp_dir);
                                                         _tmp
                                                     })
                                            }
                                            if !(unlink(fn_0 as
                                                            *const libc::c_char)
                                                     != 0 &&
                                                     *__errno_location() !=
                                                         2 as libc::c_int) {
                                                DFL_ck_free(fn_0 as
                                                                *mut libc::c_void);
                                                fn_0 =
                                                    ({
                                                         let mut _tmp:
                                                                 *mut u8_0 =
                                                             0 as *mut u8_0;
                                                         let mut _len: s32 =
                                                             snprintf(0 as
                                                                          *mut libc::c_char,
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong,
                                                                      b"%s/fuzz_bitmap\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      (*afl).out_dir);
                                                         if _len <
                                                                0 as
                                                                    libc::c_int
                                                            {
                                                             printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                             printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    b"func_unknown\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    b"src/afl-fuzz-init.c\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    1255 as
                                                                        libc::c_int);
                                                             exit(1 as
                                                                      libc::c_int);
                                                         }
                                                         _tmp =
                                                             DFL_ck_alloc((_len
                                                                               +
                                                                               1
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              u32_0)
                                                                 as *mut u8_0;
                                                         snprintf(_tmp as
                                                                      *mut libc::c_char,
                                                                  (_len +
                                                                       1 as
                                                                           libc::c_int)
                                                                      as
                                                                      libc::c_ulong,
                                                                  b"%s/fuzz_bitmap\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  (*afl).out_dir);
                                                         _tmp
                                                     });
                                                if !(unlink(fn_0 as
                                                                *const libc::c_char)
                                                         != 0 &&
                                                         *__errno_location()
                                                             !=
                                                             2 as libc::c_int)
                                                   {
                                                    DFL_ck_free(fn_0 as
                                                                    *mut libc::c_void);
                                                    if (*afl).in_place_resume
                                                           == 0 {
                                                        fn_0 =
                                                            ({
                                                                 let mut _tmp:
                                                                         *mut u8_0 =
                                                                     0 as
                                                                         *mut u8_0;
                                                                 let mut _len:
                                                                         s32 =
                                                                     snprintf(0
                                                                                  as
                                                                                  *mut libc::c_char,
                                                                              0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong,
                                                                              b"%s/fuzzer_stats\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char,
                                                                              (*afl).out_dir);
                                                                 if _len <
                                                                        0 as
                                                                            libc::c_int
                                                                    {
                                                                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char);
                                                                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char,
                                                                            b"func_unknown\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char,
                                                                            b"src/afl-fuzz-init.c\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char,
                                                                            1261
                                                                                as
                                                                                libc::c_int);
                                                                     exit(1 as
                                                                              libc::c_int);
                                                                 }
                                                                 _tmp =
                                                                     DFL_ck_alloc((_len
                                                                                       +
                                                                                       1
                                                                                           as
                                                                                           libc::c_int)
                                                                                      as
                                                                                      u32_0)
                                                                         as
                                                                         *mut u8_0;
                                                                 snprintf(_tmp
                                                                              as
                                                                              *mut libc::c_char,
                                                                          (_len
                                                                               +
                                                                               1
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              libc::c_ulong,
                                                                          b"%s/fuzzer_stats\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char,
                                                                          (*afl).out_dir);
                                                                 _tmp
                                                             });
                                                        if unlink(fn_0 as
                                                                      *const libc::c_char)
                                                               != 0 &&
                                                               *__errno_location()
                                                                   !=
                                                                   2 as
                                                                       libc::c_int
                                                           {
                                                            current_block =
                                                                16973340533559238807;
                                                        } else {
                                                            DFL_ck_free(fn_0
                                                                            as
                                                                            *mut libc::c_void);
                                                            current_block =
                                                                6528931666172833996;
                                                        }
                                                    } else {
                                                        current_block =
                                                            6528931666172833996;
                                                    }
                                                    match current_block {
                                                        16973340533559238807
                                                        => {
                                                        }
                                                        _ => {
                                                            fn_0 =
                                                                ({
                                                                     let mut _tmp:
                                                                             *mut u8_0 =
                                                                         0 as
                                                                             *mut u8_0;
                                                                     let mut _len:
                                                                             s32 =
                                                                         snprintf(0
                                                                                      as
                                                                                      *mut libc::c_char,
                                                                                  0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong,
                                                                                  b"%s/plot_data\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char,
                                                                                  (*afl).out_dir);
                                                                     if _len <
                                                                            0
                                                                                as
                                                                                libc::c_int
                                                                        {
                                                                         printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char);
                                                                         printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                b"func_unknown\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                b"src/afl-fuzz-init.c\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                1267
                                                                                    as
                                                                                    libc::c_int);
                                                                         exit(1
                                                                                  as
                                                                                  libc::c_int);
                                                                     }
                                                                     _tmp =
                                                                         DFL_ck_alloc((_len
                                                                                           +
                                                                                           1
                                                                                               as
                                                                                               libc::c_int)
                                                                                          as
                                                                                          u32_0)
                                                                             as
                                                                             *mut u8_0;
                                                                     snprintf(_tmp
                                                                                  as
                                                                                  *mut libc::c_char,
                                                                              (_len
                                                                                   +
                                                                                   1
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  libc::c_ulong,
                                                                              b"%s/plot_data\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char,
                                                                              (*afl).out_dir);
                                                                     _tmp
                                                                 });
                                                            if !(unlink(fn_0
                                                                            as
                                                                            *const libc::c_char)
                                                                     != 0 &&
                                                                     *__errno_location()
                                                                         !=
                                                                         2 as
                                                                             libc::c_int)
                                                               {
                                                                DFL_ck_free(fn_0
                                                                                as
                                                                                *mut libc::c_void);
                                                                fn_0 =
                                                                    ({
                                                                         let mut _tmp:
                                                                                 *mut u8_0 =
                                                                             0
                                                                                 as
                                                                                 *mut u8_0;
                                                                         let mut _len:
                                                                                 s32 =
                                                                             snprintf(0
                                                                                          as
                                                                                          *mut libc::c_char,
                                                                                      0
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong,
                                                                                      b"%s/cmdline\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char,
                                                                                      (*afl).out_dir);
                                                                         if _len
                                                                                <
                                                                                0
                                                                                    as
                                                                                    libc::c_int
                                                                            {
                                                                             printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                             printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    b"func_unknown\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    b"src/afl-fuzz-init.c\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    1271
                                                                                        as
                                                                                        libc::c_int);
                                                                             exit(1
                                                                                      as
                                                                                      libc::c_int);
                                                                         }
                                                                         _tmp
                                                                             =
                                                                             DFL_ck_alloc((_len
                                                                                               +
                                                                                               1
                                                                                                   as
                                                                                                   libc::c_int)
                                                                                              as
                                                                                              u32_0)
                                                                                 as
                                                                                 *mut u8_0;
                                                                         snprintf(_tmp
                                                                                      as
                                                                                      *mut libc::c_char,
                                                                                  (_len
                                                                                       +
                                                                                       1
                                                                                           as
                                                                                           libc::c_int)
                                                                                      as
                                                                                      libc::c_ulong,
                                                                                  b"%s/cmdline\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char,
                                                                                  (*afl).out_dir);
                                                                         _tmp
                                                                     });
                                                                if !(unlink(fn_0
                                                                                as
                                                                                *const libc::c_char)
                                                                         != 0
                                                                         &&
                                                                         *__errno_location()
                                                                             !=
                                                                             2
                                                                                 as
                                                                                 libc::c_int)
                                                                   {
                                                                    DFL_ck_free(fn_0
                                                                                    as
                                                                                    *mut libc::c_void);
                                                                    printf(b"\x1b[1;92m[+] \x1b[0mOutput dir cleanup successful.\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char);
                                                                    printf(b"\x1b[0m\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char);
                                                                    /* Wow... is that all? If yes, celebrate! */
                                                                    return
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    printf(b"\n\x1b[1;91m[-] \x1b[0mWhoops, the fuzzer tried to reuse your output directory, but bumped into\n    some files that shouldn\'t be there or that couldn\'t be removed - so it\n    decided to abort! This happened while processing this path:\n\n    %s\n\n    Please examine and manually delete the files, or specify a different\n    output location for the tool.\n\x00"
               as *const u8 as *const libc::c_char, fn_0);
    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mOutput directory cleanup failed\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00" as
               *const u8 as *const libc::c_char,
           b"func_unknown\x00" as *const u8 as *const libc::c_char,
           b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
           1296 as libc::c_int);
    exit(1 as libc::c_int);
}
/* Prepare output directories and fds. */
#[no_mangle]
pub unsafe extern "C" fn setup_dirs_fds(mut afl: *mut afl_state_t) {
    let mut tmp: *mut u8_0 = 0 as *mut u8_0;
    let mut fd: s32 = 0;
    printf(b"\x1b[1;94m[*] \x1b[0mSetting up output directories...\x00" as
               *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    if !(*afl).sync_id.is_null() &&
           mkdir((*afl).sync_dir as *const libc::c_char,
                 0o700 as libc::c_int as __mode_t) != 0 &&
           *__errno_location() != 17 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, (*afl).sync_dir);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1310 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    if mkdir((*afl).out_dir as *const libc::c_char,
             0o700 as libc::c_int as __mode_t) != 0 {
        if *__errno_location() != 17 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                       as *const u8 as *const libc::c_char, (*afl).out_dir);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1314 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        handle_existing_out_dir(afl);
    } else {
        if (*afl).in_place_resume != 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mResume attempted but old output directory not found\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1321 as libc::c_int);
            exit(1 as libc::c_int);
        }
        (*afl).fsrv.out_dir_fd =
            open((*afl).out_dir as *const libc::c_char, 0 as libc::c_int);
        if (*afl).fsrv.out_dir_fd < 0 as libc::c_int ||
               flock((*afl).fsrv.out_dir_fd,
                     2 as libc::c_int | 4 as libc::c_int) != 0 {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to flock() output directory.\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1329 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        /* !__sun */
    }
    /* Queue directory for any starting & discovered paths. */
    tmp =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%s/queue\x00" as *const u8 as *const libc::c_char,
                          (*afl).out_dir);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-fuzz-init.c\x00" as *const u8 as
                            *const libc::c_char, 1337 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%s/queue\x00" as *const u8 as *const libc::c_char,
                      (*afl).out_dir);
             _tmp
         });
    if mkdir(tmp as *const libc::c_char, 0o700 as libc::c_int as __mode_t) !=
           0 {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, tmp);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1338 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    DFL_ck_free(tmp as *mut libc::c_void);
    /* Top-level directory for queue metadata used for session
     resume and related tasks. */
    tmp =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%s/queue/.state/\x00" as *const u8 as
                              *const libc::c_char, (*afl).out_dir);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-fuzz-init.c\x00" as *const u8 as
                            *const libc::c_char, 1344 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%s/queue/.state/\x00" as *const u8 as
                          *const libc::c_char, (*afl).out_dir);
             _tmp
         });
    if mkdir(tmp as *const libc::c_char, 0o700 as libc::c_int as __mode_t) !=
           0 {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, tmp);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1345 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    DFL_ck_free(tmp as *mut libc::c_void);
    /* Directory for flagging queue entries that went through
     deterministic fuzzing in the past. */
    tmp =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%s/queue/.state/deterministic_done/\x00" as
                              *const u8 as *const libc::c_char,
                          (*afl).out_dir);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-fuzz-init.c\x00" as *const u8 as
                            *const libc::c_char, 1351 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%s/queue/.state/deterministic_done/\x00" as *const u8
                          as *const libc::c_char, (*afl).out_dir);
             _tmp
         });
    if mkdir(tmp as *const libc::c_char, 0o700 as libc::c_int as __mode_t) !=
           0 {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, tmp);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1352 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    DFL_ck_free(tmp as *mut libc::c_void);
    /* Directory with the auto-selected dictionary entries. */
    tmp =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%s/queue/.state/auto_extras/\x00" as *const u8 as
                              *const libc::c_char, (*afl).out_dir);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-fuzz-init.c\x00" as *const u8 as
                            *const libc::c_char, 1357 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%s/queue/.state/auto_extras/\x00" as *const u8 as
                          *const libc::c_char, (*afl).out_dir);
             _tmp
         });
    if mkdir(tmp as *const libc::c_char, 0o700 as libc::c_int as __mode_t) !=
           0 {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, tmp);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1358 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    DFL_ck_free(tmp as *mut libc::c_void);
    /* The set of paths currently deemed redundant. */
    tmp =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%s/queue/.state/redundant_edges/\x00" as *const u8
                              as *const libc::c_char, (*afl).out_dir);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-fuzz-init.c\x00" as *const u8 as
                            *const libc::c_char, 1363 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%s/queue/.state/redundant_edges/\x00" as *const u8 as
                          *const libc::c_char, (*afl).out_dir);
             _tmp
         });
    if mkdir(tmp as *const libc::c_char, 0o700 as libc::c_int as __mode_t) !=
           0 {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, tmp);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1364 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    DFL_ck_free(tmp as *mut libc::c_void);
    /* The set of paths showing variable behavior. */
    tmp =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%s/queue/.state/variable_behavior/\x00" as
                              *const u8 as *const libc::c_char,
                          (*afl).out_dir);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-fuzz-init.c\x00" as *const u8 as
                            *const libc::c_char, 1369 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%s/queue/.state/variable_behavior/\x00" as *const u8
                          as *const libc::c_char, (*afl).out_dir);
             _tmp
         });
    if mkdir(tmp as *const libc::c_char, 0o700 as libc::c_int as __mode_t) !=
           0 {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, tmp);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1370 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    DFL_ck_free(tmp as *mut libc::c_void);
    /* Sync directory for keeping track of cooperating fuzzers. */
    if !(*afl).sync_id.is_null() {
        tmp =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/.synced/\x00" as *const u8 as
                                  *const libc::c_char, (*afl).out_dir);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-init.c\x00" as *const u8 as
                                *const libc::c_char, 1377 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/.synced/\x00" as *const u8 as
                              *const libc::c_char, (*afl).out_dir);
                 _tmp
             });
        if mkdir(tmp as *const libc::c_char, 0o700 as libc::c_int as __mode_t)
               != 0 &&
               ((*afl).in_place_resume == 0 ||
                    *__errno_location() != 17 as libc::c_int) {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                       as *const u8 as *const libc::c_char, tmp);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1380 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        DFL_ck_free(tmp as *mut libc::c_void);
    }
    /* All recorded crashes. */
    tmp =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%s/crashes\x00" as *const u8 as
                              *const libc::c_char, (*afl).out_dir);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-fuzz-init.c\x00" as *const u8 as
                            *const libc::c_char, 1388 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%s/crashes\x00" as *const u8 as *const libc::c_char,
                      (*afl).out_dir);
             _tmp
         });
    if mkdir(tmp as *const libc::c_char, 0o700 as libc::c_int as __mode_t) !=
           0 {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, tmp);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1389 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    DFL_ck_free(tmp as *mut libc::c_void);
    /* All recorded hangs. */
    tmp =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%s/hangs\x00" as *const u8 as *const libc::c_char,
                          (*afl).out_dir);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-fuzz-init.c\x00" as *const u8 as
                            *const libc::c_char, 1394 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%s/hangs\x00" as *const u8 as *const libc::c_char,
                      (*afl).out_dir);
             _tmp
         });
    if mkdir(tmp as *const libc::c_char, 0o700 as libc::c_int as __mode_t) !=
           0 {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, tmp);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1395 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    DFL_ck_free(tmp as *mut libc::c_void);
    /* Generally useful file descriptors. */
    (*afl).fsrv.dev_null_fd =
        open(b"/dev/null\x00" as *const u8 as *const libc::c_char,
             0o2 as libc::c_int);
    if (*afl).fsrv.dev_null_fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open /dev/null\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1401 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    (*afl).fsrv.dev_urandom_fd =
        open(b"/dev/urandom\x00" as *const u8 as *const libc::c_char,
             0 as libc::c_int);
    if (*afl).fsrv.dev_urandom_fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open /dev/urandom\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1405 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    /* Gnuplot output file. */
    tmp =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%s/plot_data\x00" as *const u8 as
                              *const libc::c_char, (*afl).out_dir);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-fuzz-init.c\x00" as *const u8 as
                            *const libc::c_char, 1410 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%s/plot_data\x00" as *const u8 as *const libc::c_char,
                      (*afl).out_dir);
             _tmp
         });
    fd =
        open(tmp as *const libc::c_char,
             0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
             0o600 as libc::c_int);
    if fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, tmp);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1412 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    DFL_ck_free(tmp as *mut libc::c_void);
    (*afl).fsrv.plot_file =
        fdopen(fd, b"w\x00" as *const u8 as *const libc::c_char);
    if (*afl).fsrv.plot_file.is_null() {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mfdopen() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1416 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    fprintf((*afl).fsrv.plot_file,
            b"# unix_time, cycles_done, cur_path, paths_total, pending_total, pending_favs, map_size, unique_crashes, unique_hangs, max_depth, execs_per_sec\n\x00"
                as *const u8 as *const libc::c_char);
    /* ignore errors */
}
#[no_mangle]
pub unsafe extern "C" fn setup_cmdline_file(mut afl: *mut afl_state_t,
                                            mut argv:
                                                *mut *mut libc::c_char) {
    let mut tmp: *mut u8_0 = 0 as *mut u8_0;
    let mut fd: s32 = 0;
    let mut i: u32_0 = 0 as libc::c_int as u32_0;
    let mut cmdline_file: *mut FILE = 0 as *mut FILE;
    /* Store the command line to reproduce our findings */
    tmp =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%s/cmdline\x00" as *const u8 as
                              *const libc::c_char, (*afl).out_dir);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-fuzz-init.c\x00" as *const u8 as
                            *const libc::c_char, 1435 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%s/cmdline\x00" as *const u8 as *const libc::c_char,
                      (*afl).out_dir);
             _tmp
         });
    fd =
        open(tmp as *const libc::c_char,
             0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
             0o600 as libc::c_int);
    if fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, tmp);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1437 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    DFL_ck_free(tmp as *mut libc::c_void);
    cmdline_file = fdopen(fd, b"w\x00" as *const u8 as *const libc::c_char);
    if cmdline_file.is_null() {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mfdopen() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1441 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    while !(*argv.offset(i as isize)).is_null() {
        fprintf(cmdline_file, b"%s\n\x00" as *const u8 as *const libc::c_char,
                *argv.offset(i as isize));
        i = i.wrapping_add(1)
    }
    fclose(cmdline_file);
}
/* Setup the output file for fuzzed data, if not using -f. */
#[no_mangle]
pub unsafe extern "C" fn setup_stdio_file(mut afl: *mut afl_state_t) {
    let mut fn_0: *mut u8_0 = 0 as *mut u8_0; /* Ignore errors */
    if !(*afl).file_extension.is_null() {
        fn_0 =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/.cur_input.%s\x00" as *const u8 as
                                  *const libc::c_char, (*afl).tmp_dir,
                              (*afl).file_extension);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-init.c\x00" as *const u8 as
                                *const libc::c_char, 1461 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/.cur_input.%s\x00" as *const u8 as
                              *const libc::c_char, (*afl).tmp_dir,
                          (*afl).file_extension);
                 _tmp
             })
    } else {
        fn_0 =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/.cur_input\x00" as *const u8 as
                                  *const libc::c_char, (*afl).tmp_dir);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-fuzz-init.c\x00" as *const u8 as
                                *const libc::c_char, 1465 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/.cur_input\x00" as *const u8 as
                              *const libc::c_char, (*afl).tmp_dir);
                 _tmp
             })
    }
    unlink(fn_0 as *const libc::c_char);
    (*afl).fsrv.out_fd =
        open(fn_0 as *const libc::c_char,
             0o2 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
             0o600 as libc::c_int);
    if (*afl).fsrv.out_fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to create \'%s\'\x00"
                   as *const u8 as *const libc::c_char, fn_0);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1473 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    DFL_ck_free(fn_0 as *mut libc::c_void);
}
/* Make sure that core dumps don't go to a program. */
#[no_mangle]
pub unsafe extern "C" fn check_crash_handling() {
    /* This is Linux specific, but I don't think there's anything equivalent on
   *BSD, so we can just let it slide for now. */
    let mut fd: s32 =
        open(b"/proc/sys/kernel/core_pattern\x00" as *const u8 as
                 *const libc::c_char, 0 as libc::c_int);
    let mut fchar: u8_0 = 0;
    if fd < 0 as libc::c_int { return }
    printf(b"\x1b[1;94m[*] \x1b[0mChecking core_pattern...\x00" as *const u8
               as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    if read(fd, &mut fchar as *mut u8_0 as *mut libc::c_void,
            1 as libc::c_int as size_t) == 1 as libc::c_int as libc::c_long &&
           fchar as libc::c_int == '|' as i32 {
        printf(b"\n\x1b[1;91m[-] \x1b[0mHmm, your system is configured to send core dump notifications to an\n    external utility. This will cause issues: there will be an extended delay\n    between stumbling upon a crash and having this information relayed to the\n    fuzzer via the standard waitpid() API.\n    If you\'re just testing, set \'AFL_I_DONT_CARE_ABOUT_MISSING_CRASHES=1\'.\n\n    To avoid having crashes misinterpreted as timeouts, please log in as root\n    and temporarily modify /proc/sys/kernel/core_pattern, like so:\n\n    echo core >/proc/sys/kernel/core_pattern\n\x00"
                   as *const u8 as *const libc::c_char);
        if getenv(b"AFL_I_DONT_CARE_ABOUT_MISSING_CRASHES\x00" as *const u8 as
                      *const libc::c_char).is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mPipe at the beginning of \'core_pattern\'\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1545 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    close(fd);
    /* ^__APPLE__ */
}
/* Check CPU governor. */
#[no_mangle]
pub unsafe extern "C" fn check_cpu_governor(mut afl: *mut afl_state_t) {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut tmp: [u8_0; 128] = [0; 128];
    let mut min: u64_0 = 0 as libc::c_int as u64_0;
    let mut max: u64_0 = 0 as libc::c_int as u64_0;
    if (*afl).afl_env.afl_skip_cpufreq != 0 { return }
    if (*afl).cpu_aff > 0 as libc::c_int {
        snprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                 ::std::mem::size_of::<[u8_0; 128]>() as libc::c_ulong,
                 b"%s%d%s\x00" as *const u8 as *const libc::c_char,
                 b"/sys/devices/system/cpu/cpu\x00" as *const u8 as
                     *const libc::c_char, (*afl).cpu_aff,
                 b"/cpufreq/scaling_governor\x00" as *const u8 as
                     *const libc::c_char);
    } else {
        snprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                 ::std::mem::size_of::<[u8_0; 128]>() as libc::c_ulong,
                 b"%s\x00" as *const u8 as *const libc::c_char,
                 b"/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor\x00"
                     as *const u8 as *const libc::c_char);
    }
    f =
        fopen(b"/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor\x00" as
                  *const u8 as *const libc::c_char,
              b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        if (*afl).cpu_aff > 0 as libc::c_int {
            snprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                     ::std::mem::size_of::<[u8_0; 128]>() as libc::c_ulong,
                     b"%s%d%s\x00" as *const u8 as *const libc::c_char,
                     b"/sys/devices/system/cpu/cpufreq/policy\x00" as
                         *const u8 as *const libc::c_char, (*afl).cpu_aff,
                     b"/scaling_governor\x00" as *const u8 as
                         *const libc::c_char);
        } else {
            snprintf(tmp.as_mut_ptr() as *mut libc::c_char,
                     ::std::mem::size_of::<[u8_0; 128]>() as libc::c_ulong,
                     b"%s\x00" as *const u8 as *const libc::c_char,
                     b"/sys/devices/system/cpu/cpufreq/policy0/scaling_governor\x00"
                         as *const u8 as *const libc::c_char);
        }
        f =
            fopen(tmp.as_mut_ptr() as *const libc::c_char,
                  b"r\x00" as *const u8 as *const libc::c_char)
    }
    if f.is_null() {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mCould not check CPU scaling governor\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        return
    }
    printf(b"\x1b[1;94m[*] \x1b[0mChecking CPU scaling governor...\x00" as
               *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    if fgets(tmp.as_mut_ptr() as *mut libc::c_char, 128 as libc::c_int,
             f).is_null() {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mfgets() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1595 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    fclose(f);
    if strncmp(tmp.as_mut_ptr() as *const libc::c_char,
               b"perf\x00" as *const u8 as *const libc::c_char,
               4 as libc::c_int as libc::c_ulong) == 0 {
        return
    }
    f =
        fopen(b"/sys/devices/system/cpu/cpu0/cpufreq/scaling_min_freq\x00" as
                  *const u8 as *const libc::c_char,
              b"r\x00" as *const u8 as *const libc::c_char);
    if !f.is_null() {
        if fscanf(f, b"%llu\x00" as *const u8 as *const libc::c_char,
                  &mut min as *mut u64_0) != 1 as libc::c_int {
            min = 0 as libc::c_int as u64_0
        }
        fclose(f);
    }
    f =
        fopen(b"/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq\x00" as
                  *const u8 as *const libc::c_char,
              b"r\x00" as *const u8 as *const libc::c_char);
    if !f.is_null() {
        if fscanf(f, b"%llu\x00" as *const u8 as *const libc::c_char,
                  &mut max as *mut u64_0) != 1 as libc::c_int {
            max = 0 as libc::c_int as u64_0
        }
        fclose(f);
    }
    if min == max { return }
    printf(b"\n\x1b[1;91m[-] \x1b[0mWhoops, your system uses on-demand CPU frequency scaling, adjusted\n    between %llu and %llu MHz. Unfortunately, the scaling algorithm in the\n    kernel is imperfect and can miss the short-lived processes spawned by\n    afl-fuzz. To keep things moving, run these commands as root:\n\n    cd /sys/devices/system/cpu\n    echo performance | tee cpu*/cpufreq/scaling_governor\n\n    You can later go back to the original state by replacing \'performance\'\n    with \'ondemand\' or \'powersave\'. If you don\'t want to change the settings,\n    set AFL_SKIP_CPUFREQ to make afl-fuzz skip this check - but expect some\n    performance drop.\n\x00"
               as *const u8 as *const libc::c_char,
           min.wrapping_div(1024 as libc::c_int as libc::c_ulonglong),
           max.wrapping_div(1024 as libc::c_int as libc::c_ulonglong));
    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mSuboptimal CPU scaling governor\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00" as
               *const u8 as *const libc::c_char,
           b"func_unknown\x00" as *const u8 as *const libc::c_char,
           b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
           1640 as libc::c_int);
    exit(1 as libc::c_int);
}
/* Count the number of logical CPU cores. */
#[no_mangle]
pub unsafe extern "C" fn get_core_count(mut afl: *mut afl_state_t) {
    (*afl).cpu_core_count =
        sysconf(_SC_NPROCESSORS_ONLN as libc::c_int) as s32;
    /* ^HAVE_AFFINITY */
    /* ^(__APPLE__ || __FreeBSD__ || __OpenBSD__) */
    if (*afl).cpu_core_count > 0 as libc::c_int {
        let mut cur_runnable: u32_0 = 0 as libc::c_int as u32_0;
        cur_runnable = get_runnable_processes() as u32_0;
        /* __APPLE__ || __FreeBSD__ || __OpenBSD__ */
        printf(b"\x1b[1;92m[+] \x1b[0mYou have %d CPU core%s and %u runnable tasks (utilization: %0.0f%%).\x00"
                   as *const u8 as *const libc::c_char, (*afl).cpu_core_count,
               if (*afl).cpu_core_count > 1 as libc::c_int {
                   b"s\x00" as *const u8 as *const libc::c_char
               } else { b"\x00" as *const u8 as *const libc::c_char },
               cur_runnable,
               cur_runnable as libc::c_double * 100.0f64 /
                   (*afl).cpu_core_count as libc::c_double);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        if (*afl).cpu_core_count > 1 as libc::c_int {
            if cur_runnable as libc::c_double >
                   (*afl).cpu_core_count as libc::c_double * 1.5f64 {
                printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mSystem under apparent load, performance may be spotty.\x00"
                           as *const u8 as *const libc::c_char);
                printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
            } else if cur_runnable.wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) <=
                          (*afl).cpu_core_count as libc::c_uint {
                printf(b"\x1b[1;92m[+] \x1b[0mTry parallel jobs - see %s/parallel_fuzzing.md.\x00"
                           as *const u8 as *const libc::c_char, doc_path);
                printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
            }
        }
    } else {
        (*afl).cpu_core_count = 0 as libc::c_int;
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mUnable to figure out the number of CPU cores.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    };
}
/* Validate and fix up afl->out_dir and sync_dir when using -S. */
#[no_mangle]
pub unsafe extern "C" fn fix_up_sync(mut afl: *mut afl_state_t) {
    let mut x: *mut u8_0 = (*afl).sync_id;
    if (*afl).dumb_mode != 0 {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0m-S / -M and -n are mutually exclusive\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1772 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if (*afl).skip_deterministic != 0 {
        if (*afl).force_deterministic != 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0muse -S instead of -M -d\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1776 as libc::c_int);
            exit(1 as libc::c_int);
        }
        // else
    //  FATAL("-S already implies -d");
    }
    while *x != 0 {
        if *(*__ctype_b_loc()).offset(*x as libc::c_int as isize) as
               libc::c_int &
               _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
               && *x as libc::c_int != '_' as i32 &&
               *x as libc::c_int != '-' as i32 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mNon-alphanumeric fuzzer ID specified via -S or -M\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1785 as libc::c_int);
            exit(1 as libc::c_int);
        }
        x = x.offset(1)
    }
    if strlen((*afl).sync_id as *const libc::c_char) >
           32 as libc::c_int as libc::c_ulong {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mFuzzer ID too long\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1791 as libc::c_int);
        exit(1 as libc::c_int);
    }
    x =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%s/%s\x00" as *const u8 as *const libc::c_char,
                          (*afl).out_dir, (*afl).sync_id);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-fuzz-init.c\x00" as *const u8 as
                            *const libc::c_char, 1793 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%s/%s\x00" as *const u8 as *const libc::c_char,
                      (*afl).out_dir, (*afl).sync_id);
             _tmp
         });
    (*afl).sync_dir = (*afl).out_dir;
    (*afl).out_dir = x;
    if (*afl).force_deterministic == 0 {
        (*afl).skip_deterministic = 1 as libc::c_int as u8_0;
        (*afl).use_splicing = 1 as libc::c_int as u8_0
    };
}
/* Handle screen resize (SIGWINCH). */
unsafe extern "C" fn handle_resize(mut sig: libc::c_int) {
    let mut li: *mut list_t = &mut afl_states;
    let mut head: *mut element_t = get_head(li);
    let mut el_box: *mut element_t = (*head).next;
    if el_box.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mforeach over uninitialized list\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1811 as libc::c_int);
        exit(1 as libc::c_int);
    }
    while el_box != head {
        let mut el: *mut afl_state_t = (*el_box).data as *mut afl_state_t;
        let mut next: *mut element_t = (*el_box).next;
        ::std::ptr::write_volatile(&mut (*el).clear_screen as *mut u8_0,
                                   1 as libc::c_int as u8_0);
        el_box = next
    };
}
/* Check ASAN options. */
#[no_mangle]
pub unsafe extern "C" fn check_asan_opts() {
    let mut x: *mut u8_0 =
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
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1824 as libc::c_int);
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
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1827 as libc::c_int);
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
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1837 as libc::c_int);
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
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1840 as libc::c_int);
            exit(1 as libc::c_int);
        }
    };
}
/* Handle stop signal (Ctrl-C, etc). */
unsafe extern "C" fn handle_stop_sig(mut sig: libc::c_int) {
    let mut li: *mut list_t = &mut afl_states;
    let mut head: *mut element_t = get_head(li);
    let mut el_box: *mut element_t = (*head).next;
    if el_box.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mforeach over uninitialized list\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1857 as libc::c_int);
        exit(1 as libc::c_int);
    }
    while el_box != head {
        let mut el: *mut afl_state_t = (*el_box).data as *mut afl_state_t;
        let mut next: *mut element_t = (*el_box).next;
        ::std::ptr::write_volatile(&mut (*el).stop_soon as *mut u8_0,
                                   1 as libc::c_int as u8_0);
        if (*el).fsrv.child_pid > 0 as libc::c_int {
            kill((*el).fsrv.child_pid, 9 as libc::c_int);
        }
        if (*el).fsrv.fsrv_pid > 0 as libc::c_int {
            kill((*el).fsrv.fsrv_pid, 9 as libc::c_int);
        }
        el_box = next
    };
}
/* Handle skip request (SIGUSR1). */
unsafe extern "C" fn handle_skipreq(mut sig: libc::c_int) {
    let mut li: *mut list_t = &mut afl_states;
    let mut head: *mut element_t = get_head(li);
    let mut el_box: *mut element_t = (*head).next;
    if el_box.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mforeach over uninitialized list\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1865 as libc::c_int);
        exit(1 as libc::c_int);
    }
    while el_box != head {
        let mut el: *mut afl_state_t = (*el_box).data as *mut afl_state_t;
        let mut next: *mut element_t = (*el_box).next;
        (*el).skip_requested = 1 as libc::c_int as u8_0;
        el_box = next
    };
}
/* Do a PATH search and find target binary to see that it exists and
   isn't a shell script - a common and painful mistake. We also check for
   a valid ELF header and for evidence of AFL instrumentation. */
#[no_mangle]
pub unsafe extern "C" fn check_binary(mut afl: *mut afl_state_t,
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
    let mut fd: s32 = 0;
    let mut f_data: *mut u8_0 = 0 as *mut u8_0;
    let mut f_len: u32_0 = 0 as libc::c_int as u32_0;
    printf(b"\x1b[1;94m[*] \x1b[0mValidating target binary...\x00" as
               *const u8 as *const libc::c_char);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    if !strchr(fname as *const libc::c_char, '/' as i32).is_null() ||
           {
               env_path =
                   getenv(b"PATH\x00" as *const u8 as *const libc::c_char) as
                       *mut u8_0;
               env_path.is_null()
           } {
        (*afl).fsrv.target_path = DFL_ck_strdup(fname);
        if stat((*afl).fsrv.target_path as *const libc::c_char, &mut st) != 0
               ||
               !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
                     0o100000 as libc::c_int as libc::c_uint) ||
               st.st_mode & 0o111 as libc::c_int as libc::c_uint == 0 ||
               {
                   f_len = st.st_size as u32_0;
                   (f_len) < 4 as libc::c_int as libc::c_uint
               } {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mProgram \'%s\' not found or not executable\x00"
                       as *const u8 as *const libc::c_char, fname);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1889 as libc::c_int);
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
                (*afl).fsrv.target_path =
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
                                    b"src/afl-fuzz-init.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    1910 as libc::c_int);
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
            } else { (*afl).fsrv.target_path = DFL_ck_strdup(fname) }
            DFL_ck_free(cur_elem as *mut libc::c_void);
            if stat((*afl).fsrv.target_path as *const libc::c_char, &mut st)
                   == 0 &&
                   st.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
                       0o100000 as libc::c_int as libc::c_uint &&
                   st.st_mode & 0o111 as libc::c_int as libc::c_uint != 0 &&
                   {
                       f_len = st.st_size as u32_0;
                       (f_len) >= 4 as libc::c_int as libc::c_uint
                   } {
                break ;
            }
            DFL_ck_free((*afl).fsrv.target_path as *mut libc::c_void);
            (*afl).fsrv.target_path = 0 as *mut u8_0
        }
        if (*afl).fsrv.target_path.is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mProgram \'%s\' not found or not executable\x00"
                       as *const u8 as *const libc::c_char, fname);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 1926 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    if (*afl).afl_env.afl_skip_bin_check as libc::c_int != 0 ||
           (*afl).use_wine as libc::c_int != 0 {
        return
    }
    /* Check for blatant user errors. */
    if strncmp((*afl).fsrv.target_path as *const libc::c_char,
               b"/tmp/\x00" as *const u8 as *const libc::c_char,
               5 as libc::c_int as libc::c_ulong) == 0 &&
           strchr((*afl).fsrv.target_path.offset(5 as libc::c_int as isize) as
                      *const libc::c_char, '/' as i32).is_null() ||
           strncmp((*afl).fsrv.target_path as *const libc::c_char,
                   b"/var/tmp/\x00" as *const u8 as *const libc::c_char,
                   9 as libc::c_int as libc::c_ulong) == 0 &&
               strchr((*afl).fsrv.target_path.offset(9 as libc::c_int as
                                                         isize) as
                          *const libc::c_char, '/' as i32).is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mPlease don\'t keep binaries in /tmp or /var/tmp\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1938 as libc::c_int);
        exit(1 as libc::c_int);
    }
    fd =
        open((*afl).fsrv.target_path as *const libc::c_char,
             0 as libc::c_int);
    if fd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to open \'%s\'\x00"
                   as *const u8 as *const libc::c_char,
               (*afl).fsrv.target_path);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1942 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    f_data =
        mmap(0 as *mut libc::c_void, f_len as size_t, 0x1 as libc::c_int,
             0x2 as libc::c_int, fd, 0 as libc::c_int as __off64_t) as
            *mut u8_0;
    if f_data == -(1 as libc::c_int) as *mut libc::c_void as *mut u8_0 {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to mmap file \'%s\'\x00"
                   as *const u8 as *const libc::c_char,
               (*afl).fsrv.target_path);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1947 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    close(fd);
    if *f_data.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
           &&
           *f_data.offset(1 as libc::c_int as isize) as libc::c_int ==
               '!' as i32 {
        printf(b"\n\x1b[1;91m[-] \x1b[0mOops, the target binary looks like a shell script. Some build systems will\n    sometimes generate shell stubs for dynamically linked programs; try static\n    library mode (./configure --disable-shared) if that\'s the case.\n\n    Another possible cause is that you are actually trying to use a shell\n    wrapper around the fuzzed component. Invoking shell can slow down the\n    fuzzing process by a factor of 20x or more; it\'s best to write the wrapper\n    in a compiled language instead.\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mProgram \'%s\' is a shell script\x00"
                   as *const u8 as *const libc::c_char,
               (*afl).fsrv.target_path);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1969 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if *f_data.offset(0 as libc::c_int as isize) as libc::c_int !=
           0x7f as libc::c_int ||
           memcmp(f_data.offset(1 as libc::c_int as isize) as
                      *const libc::c_void,
                  b"ELF\x00" as *const u8 as *const libc::c_char as
                      *const libc::c_void, 3 as libc::c_int as libc::c_ulong)
               != 0 {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mProgram \'%s\' is not an ELF binary\x00"
                   as *const u8 as *const libc::c_char,
               (*afl).fsrv.target_path);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               1976 as libc::c_int);
        exit(1 as libc::c_int);
    }
    /* ^!__APPLE__ */
    if (*afl).fsrv.qemu_mode == 0 && (*afl).unicorn_mode == 0 &&
           (*afl).dumb_mode == 0 &&
           memmem(f_data as *const libc::c_void, f_len as size_t,
                  b"__AFL_SHM_ID\x00" as *const u8 as *const libc::c_char as
                      *const libc::c_void,
                  strlen(b"__AFL_SHM_ID\x00" as *const u8 as
                             *const libc::c_char).wrapping_add(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong)).is_null()
       {
        printf(b"\n\x1b[1;91m[-] \x1b[0mLooks like the target binary is not instrumented! The fuzzer depends on\n    compile-time instrumentation to isolate interesting test cases while\n    mutating the input data. For more information, and for tips on how to\n    instrument binaries, please see %s/README.md.\n\n    When source code is not available, you may be able to leverage QEMU\n    mode support. Consult the README.md for tips on how to enable this.\n    (It is also possible to use afl-fuzz as a traditional, \"dumb\" fuzzer.\n    For that, you can use the -n option - but expect much worse results.)\n\x00"
                   as *const u8 as *const libc::c_char, doc_path);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mNo instrumentation detected\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               2012 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if (*afl).fsrv.qemu_mode as libc::c_int != 0 &&
           !memmem(f_data as *const libc::c_void, f_len as size_t,
                   b"__AFL_SHM_ID\x00" as *const u8 as *const libc::c_char as
                       *const libc::c_void,
                   strlen(b"__AFL_SHM_ID\x00" as *const u8 as
                              *const libc::c_char).wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong)).is_null()
       {
        printf(b"\n\x1b[1;91m[-] \x1b[0mThis program appears to be instrumented with afl-gcc, but is being run in\n    QEMU mode (-Q). This is probably not what you want -\n    this setup will be slow and offer no practical benefits.\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mInstrumentation found in -Q mode\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               2026 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if !memmem(f_data as *const libc::c_void, f_len as size_t,
               b"__asan_init\x00" as *const u8 as *const libc::c_char as
                   *const libc::c_void, 11 as libc::c_int as size_t).is_null()
           ||
           !memmem(f_data as *const libc::c_void, f_len as size_t,
                   b"__msan_init\x00" as *const u8 as *const libc::c_char as
                       *const libc::c_void,
                   11 as libc::c_int as size_t).is_null() {
        (*afl).fsrv.uses_asan = 1 as libc::c_int as u8_0
    }
    /* Detect persistent & deferred init signatures in the binary. */
    if !memmem(f_data as *const libc::c_void, f_len as size_t,
               b"##SIG_AFL_PERSISTENT##\x00" as *const u8 as
                   *const libc::c_char as *const libc::c_void,
               strlen(b"##SIG_AFL_PERSISTENT##\x00" as *const u8 as
                          *const libc::c_char).wrapping_add(1 as libc::c_int
                                                                as
                                                                libc::c_ulong)).is_null()
       {
        printf(b"\x1b[1;92m[+] \x1b[0m\x1b[1;95mPersistent mode binary detected.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        setenv(b"__AFL_PERSISTENT\x00" as *const u8 as *const libc::c_char,
               b"1\x00" as *const u8 as *const libc::c_char,
               1 as libc::c_int);
        (*afl).persistent_mode = 1 as libc::c_int as u8_0
    } else if !getenv(b"AFL_PERSISTENT\x00" as *const u8 as
                          *const libc::c_char).is_null() {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mAFL_PERSISTENT is no longer supported and may misbehave!\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    if !memmem(f_data as *const libc::c_void, f_len as size_t,
               b"##SIG_AFL_DEFER_FORKSRV##\x00" as *const u8 as
                   *const libc::c_char as *const libc::c_void,
               strlen(b"##SIG_AFL_DEFER_FORKSRV##\x00" as *const u8 as
                          *const libc::c_char).wrapping_add(1 as libc::c_int
                                                                as
                                                                libc::c_ulong)).is_null()
       {
        printf(b"\x1b[1;92m[+] \x1b[0m\x1b[1;95mDeferred forkserver binary detected.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        setenv(b"__AFL_DEFER_FORKSRV\x00" as *const u8 as *const libc::c_char,
               b"1\x00" as *const u8 as *const libc::c_char,
               1 as libc::c_int);
        (*afl).deferred_mode = 1 as libc::c_int as u8_0
    } else if !getenv(b"AFL_DEFER_FORKSRV\x00" as *const u8 as
                          *const libc::c_char).is_null() {
        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mAFL_DEFER_FORKSRV is no longer supported and may misbehave!\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    if munmap(f_data as *mut libc::c_void, f_len as size_t) != 0 {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0munmap() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-fuzz-init.c\x00" as *const u8 as *const libc::c_char,
               2060 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    };
}
/* Trim and possibly create a banner for the run. */
#[no_mangle]
pub unsafe extern "C" fn fix_up_banner(mut afl: *mut afl_state_t,
                                       mut name: *mut u8_0) {
    if (*afl).use_banner.is_null() {
        if !(*afl).sync_id.is_null() {
            (*afl).use_banner = (*afl).sync_id
        } else {
            let mut trim: *mut u8_0 =
                strrchr(name as *const libc::c_char, '/' as i32) as *mut u8_0;
            if trim.is_null() {
                (*afl).use_banner = name
            } else {
                (*afl).use_banner = trim.offset(1 as libc::c_int as isize)
            }
        }
    }
    if strlen((*afl).use_banner as *const libc::c_char) >
           32 as libc::c_int as libc::c_ulong {
        let mut tmp: *mut u8_0 =
            DFL_ck_alloc(36 as libc::c_int as u32_0) as *mut u8_0;
        sprintf(tmp as *mut libc::c_char,
                b"%.32s...\x00" as *const u8 as *const libc::c_char,
                (*afl).use_banner);
        (*afl).use_banner = tmp
    };
}
/* Check if we're on TTY. */
#[no_mangle]
pub unsafe extern "C" fn check_if_tty(mut afl: *mut afl_state_t) {
    let mut ws: winsize =
        winsize{ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0,};
    if (*afl).afl_env.afl_no_ui != 0 {
        printf(b"\x1b[1;92m[+] \x1b[0mDisabling the UI because AFL_NO_UI is set.\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        (*afl).not_on_tty = 1 as libc::c_int as u8_0;
        return
    }
    if ioctl(1 as libc::c_int, 0x5413 as libc::c_int as libc::c_ulong,
             &mut ws as *mut winsize) != 0 {
        if *__errno_location() == 25 as libc::c_int {
            printf(b"\x1b[1;92m[+] \x1b[0mLooks like we\'re not running on a tty, so I\'ll be a bit less verbose.\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
            (*afl).not_on_tty = 1 as libc::c_int as u8_0
        }
        return
    };
}
/* Set up signal handlers. More complicated that needs to be, because libc on
   Solaris doesn't resume interrupted reads(), sets SA_RESETHAND when you call
   siginterrupt(), and does other stupid things. */
#[no_mangle]
pub unsafe extern "C" fn setup_signal_handlers() {
    let mut sa: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_10{sa_handler: None,},
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
    /* Window resize */
    sa.__sigaction_handler.sa_handler =
        Some(handle_resize as unsafe extern "C" fn(_: libc::c_int) -> ());
    sigaction(28 as libc::c_int, &mut sa, 0 as *mut sigaction);
    /* SIGUSR1: skip entry */
    sa.__sigaction_handler.sa_handler =
        Some(handle_skipreq as unsafe extern "C" fn(_: libc::c_int) -> ());
    sigaction(10 as libc::c_int, &mut sa, 0 as *mut sigaction);
    /* Things we don't care about. */
    sa.__sigaction_handler.sa_handler =
        ::std::mem::transmute::<libc::intptr_t,
                                __sighandler_t>(1 as libc::c_int as
                                                    libc::intptr_t);
    sigaction(20 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(13 as libc::c_int, &mut sa, 0 as *mut sigaction);
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
/* Fuzz one */
/* Init */
/* Make a copy of the current command line. */
#[no_mangle]
pub unsafe extern "C" fn save_cmdline(mut afl: *mut afl_state_t,
                                      mut argc: u32_0,
                                      mut argv: *mut *mut libc::c_char) {
    let mut len: u32_0 = 1 as libc::c_int as u32_0;
    let mut i: u32_0 = 0;
    let mut buf: *mut u8_0 = 0 as *mut u8_0;
    i = 0 as libc::c_int as u32_0;
    while i < argc {
        len =
            (len as
                 libc::c_ulong).wrapping_add(strlen(*argv.offset(i as
                                                                     isize)).wrapping_add(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong))
                as u32_0 as u32_0;
        i = i.wrapping_add(1)
    }
    (*afl).orig_cmdline = DFL_ck_alloc(len) as *mut u8_0;
    buf = (*afl).orig_cmdline;
    i = 0 as libc::c_int as u32_0;
    while i < argc {
        let mut l: u32_0 = strlen(*argv.offset(i as isize)) as u32_0;
        if (*argv.offset(i as isize)).is_null() || buf.is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mnull deref detected\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-fuzz-init.c\x00" as *const u8 as
                       *const libc::c_char, 2181 as libc::c_int);
            exit(1 as libc::c_int);
        }
        memcpy(buf as *mut libc::c_void,
               *argv.offset(i as isize) as *const libc::c_void,
               l as libc::c_ulong);
        buf = buf.offset(l as isize);
        if i != argc.wrapping_sub(1 as libc::c_int as libc::c_uint) {
            let fresh5 = buf;
            buf = buf.offset(1);
            *fresh5 = ' ' as i32 as u8_0
        }
        i = i.wrapping_add(1)
    }
    *buf = 0 as libc::c_int as u8_0;
}
