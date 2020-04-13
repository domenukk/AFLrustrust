use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn sched_yield() -> libc::c_int;
    #[no_mangle]
    fn sched_setaffinity(__pid: __pid_t, __cpusetsize: size_t,
                         __cpuset: *const cpu_set_t) -> libc::c_int;
    #[no_mangle]
    fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> libc::c_int;
    #[no_mangle]
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
               __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    /* Get unix time in microseconds */
    #[no_mangle]
    fn get_cur_time_us() -> u64_0;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
pub type __rusage_who = libc::c_int;
pub const RUSAGE_THREAD: __rusage_who = 1;
pub const RUSAGE_CHILDREN: __rusage_who = -1;
pub const RUSAGE_SELF: __rusage_who = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub c2rust_unnamed_0: C2RustUnnamed_12,
    pub c2rust_unnamed_1: C2RustUnnamed_11,
    pub c2rust_unnamed_2: C2RustUnnamed_10,
    pub c2rust_unnamed_3: C2RustUnnamed_9,
    pub c2rust_unnamed_4: C2RustUnnamed_8,
    pub c2rust_unnamed_5: C2RustUnnamed_7,
    pub c2rust_unnamed_6: C2RustUnnamed_6,
    pub c2rust_unnamed_7: C2RustUnnamed_5,
    pub c2rust_unnamed_8: C2RustUnnamed_4,
    pub c2rust_unnamed_9: C2RustUnnamed_3,
    pub c2rust_unnamed_10: C2RustUnnamed_2,
    pub c2rust_unnamed_11: C2RustUnnamed_1,
    pub c2rust_unnamed_12: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ru_nivcsw: libc::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ru_nvcsw: libc::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ru_nsignals: libc::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ru_msgrcv: libc::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ru_msgsnd: libc::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ru_oublock: libc::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ru_inblock: libc::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub ru_nswap: libc::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ru_majflt: libc::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub ru_minflt: libc::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub ru_isrss: libc::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ru_idrss: libc::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub ru_ixrss: libc::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub ru_maxrss: libc::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
pub type __rusage_who_t = __rusage_who;
pub type uint32_t = __uint32_t;
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
/*
   american fuzzy lop++ - free CPU gizmo
   -----------------------------------

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

   This tool provides a fairly accurate measurement of CPU preemption rate.
   It is meant to complement the quick-and-dirty load average widget shown
   in the afl-fuzz UI. See docs/parallel_fuzzing.md for more info.

   For some work loads, the tool may actually suggest running more instances
   than you have CPU cores. This can happen if the tested program is spending
   a portion of its run time waiting for I/O, rather than being 100%
   CPU-bound.

   The idea for the getrusage()-based approach comes from Jakub Wilk.

 */
/* __linux__ || __FreeBSD__ || __NetBSD__ || __APPLE__ */
/* Get CPU usage in microseconds. */
unsafe extern "C" fn get_cpu_usage_us() -> u64_0 {
    let mut u: rusage =
        rusage{ru_utime: timeval{tv_sec: 0, tv_usec: 0,},
               ru_stime: timeval{tv_sec: 0, tv_usec: 0,},
               c2rust_unnamed: C2RustUnnamed_13{ru_maxrss: 0,},
               c2rust_unnamed_0: C2RustUnnamed_12{ru_ixrss: 0,},
               c2rust_unnamed_1: C2RustUnnamed_11{ru_idrss: 0,},
               c2rust_unnamed_2: C2RustUnnamed_10{ru_isrss: 0,},
               c2rust_unnamed_3: C2RustUnnamed_9{ru_minflt: 0,},
               c2rust_unnamed_4: C2RustUnnamed_8{ru_majflt: 0,},
               c2rust_unnamed_5: C2RustUnnamed_7{ru_nswap: 0,},
               c2rust_unnamed_6: C2RustUnnamed_6{ru_inblock: 0,},
               c2rust_unnamed_7: C2RustUnnamed_5{ru_oublock: 0,},
               c2rust_unnamed_8: C2RustUnnamed_4{ru_msgsnd: 0,},
               c2rust_unnamed_9: C2RustUnnamed_3{ru_msgrcv: 0,},
               c2rust_unnamed_10: C2RustUnnamed_2{ru_nsignals: 0,},
               c2rust_unnamed_11: C2RustUnnamed_1{ru_nvcsw: 0,},
               c2rust_unnamed_12: C2RustUnnamed_0{ru_nivcsw: 0,},};
    getrusage(RUSAGE_SELF, &mut u);
    return (u.ru_utime.tv_sec as
                libc::c_ulonglong).wrapping_mul(1000000 as
                                                    libc::c_ulonglong).wrapping_add(u.ru_utime.tv_usec
                                                                                        as
                                                                                        libc::c_ulonglong).wrapping_add((u.ru_stime.tv_sec
                                                                                                                             as
                                                                                                                             libc::c_ulonglong).wrapping_mul(1000000
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_ulonglong)).wrapping_add(u.ru_stime.tv_usec
                                                                                                                                                                                                      as
                                                                                                                                                                                                      libc::c_ulonglong);
}
/* Measure preemption rate. */
unsafe extern "C" fn measure_preemption(mut target_ms: u32_0) -> u32_0 {
    let mut v1: u32_0 = 0;
    let mut v2: u32_0 = 0 as libc::c_int as u32_0;
    let mut st_t: u64_0 = 0;
    let mut en_t: u64_0 = 0;
    let mut st_c: u64_0 = 0;
    let mut en_c: u64_0 = 0;
    let mut real_delta: u64_0 = 0;
    let mut slice_delta: u64_0 = 0;
    let mut loop_repeats: s32 = 0 as libc::c_int;
    st_t = get_cur_time_us();
    st_c = get_cpu_usage_us();
    loop  {
        ::std::ptr::write_volatile(&mut v1 as *mut u32_0,
                                   (10 as libc::c_int * 1000 as libc::c_int *
                                        1000 as libc::c_int) as u32_0);
        loop  {
            let fresh0 =
                ::std::ptr::read_volatile::<u32_0>(&v1 as *const u32_0);
            ::std::ptr::write_volatile(&mut v1 as *mut u32_0,
                                       ::std::ptr::read_volatile::<u32_0>(&v1
                                                                              as
                                                                              *const u32_0).wrapping_sub(1));
            if !(fresh0 != 0) { break ; }
            ::std::ptr::write_volatile(&mut v2 as *mut u32_0,
                                       ::std::ptr::read_volatile::<u32_0>(&v2
                                                                              as
                                                                              *const u32_0).wrapping_add(1))
        }
        sched_yield();
        en_t = get_cur_time_us();
        if !(en_t.wrapping_sub(st_t) <
                 target_ms.wrapping_mul(1000 as libc::c_int as libc::c_uint)
                     as libc::c_ulonglong) {
            break ;
        }
        loop_repeats += 1
    }
    /* Let's see what percentage of this time we actually had a chance to
     run, and how much time was spent in the penalty box. */
    en_c = get_cpu_usage_us();
    real_delta =
        en_t.wrapping_sub(st_t).wrapping_div(1000 as libc::c_int as
                                                 libc::c_ulonglong);
    slice_delta =
        en_c.wrapping_sub(st_c).wrapping_div(1000 as libc::c_int as
                                                 libc::c_ulonglong);
    return real_delta.wrapping_mul(100 as libc::c_int as
                                       libc::c_ulonglong).wrapping_div(slice_delta)
               as u32_0;
}
/* Do the benchmark thing. */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    if argc > 1 as libc::c_int {
        printf(b"afl-gotcpu++2.63d by Michal Zalewski\n\x00" as *const u8 as
                   *const libc::c_char);
        printf(b"\n%s \n\n\x00" as *const u8 as *const libc::c_char,
               *argv.offset(0 as libc::c_int as isize));
        printf(b"afl-gotcpu does not have command line options\n\x00" as
                   *const u8 as *const libc::c_char);
        printf(b"afl-gotcpu prints out which CPUs are available\n\x00" as
                   *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    let mut cpu_cnt: u32_0 =
        sysconf(_SC_NPROCESSORS_ONLN as libc::c_int) as u32_0;
    let mut idle_cpus: u32_0 = 0 as libc::c_int as u32_0;
    let mut maybe_cpus: u32_0 = 0 as libc::c_int as u32_0;
    let mut i: u32_0 = 0;
    printf(b"\x1b[0;36mafl-gotcpu++2.63d\x1b[0m by Michal Zalewski\n\x00" as
               *const u8 as *const libc::c_char);
    printf(b"\x1b[1;94m[*] \x1b[0mMeasuring per-core preemption rate (this will take %0.02f sec)...\x00"
               as *const u8 as *const libc::c_char,
           1000 as libc::c_int as libc::c_double /
               1000 as libc::c_int as libc::c_double);
    printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as u32_0;
    while i < cpu_cnt {
        let mut fr: s32 = fork();
        if fr < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mfork failed\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-gotcpu.c\x00" as *const u8 as
                       *const libc::c_char, 157 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        if fr == 0 {
            let mut util_perc: u32_0 = 0;
            let mut c: cpu_set_t = cpu_set_t{__bits: [0; 16],};
            libc::memset(&mut c as *mut cpu_set_t as *mut libc::c_void,
                         '\u{0}' as i32,
                         ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong
                             as libc::size_t);
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
            if sched_setaffinity(0 as libc::c_int,
                                 ::std::mem::size_of::<cpu_set_t>() as
                                     libc::c_ulong, &mut c) != 0 {
                fflush(stdout);
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0msched_setaffinity failed for cpu %d\x00"
                           as *const u8 as *const libc::c_char, i);
                printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-gotcpu.c\x00" as *const u8 as
                           *const libc::c_char, 196 as libc::c_int);
                printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                           *const u8 as *const libc::c_char,
                       strerror(*__errno_location()));
                exit(1 as libc::c_int);
            }
            util_perc = measure_preemption(1000 as libc::c_int as u32_0);
            if util_perc < 110 as libc::c_int as libc::c_uint {
                printf(b"    Core #%u: \x1b[1;92mAVAILABLE\x1b[0m(%u%%)\n\x00"
                           as *const u8 as *const libc::c_char, i, util_perc);
                exit(0 as libc::c_int);
            } else {
                if util_perc < 250 as libc::c_int as libc::c_uint {
                    printf(b"    Core #%u: \x1b[1;93mCAUTION \x1b[0m(%u%%)\n\x00"
                               as *const u8 as *const libc::c_char, i,
                           util_perc);
                    exit(1 as libc::c_int);
                }
            }
            printf(b"    Core #%u: \x1b[1;91mOVERBOOKED \x1b[0m(%u%%)\n\x1b[0m\x00"
                       as *const u8 as *const libc::c_char, i, util_perc);
            exit(2 as libc::c_int);
        }
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as u32_0;
    while i < cpu_cnt {
        let mut ret: libc::c_int = 0;
        if waitpid(-(1 as libc::c_int), &mut ret, 0 as libc::c_int) <
               0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mwaitpid failed\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-gotcpu.c\x00" as *const u8 as
                       *const libc::c_char, 224 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        if (ret & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
               0 as libc::c_int {
            idle_cpus = idle_cpus.wrapping_add(1)
        }
        if (ret & 0xff00 as libc::c_int) >> 8 as libc::c_int <=
               1 as libc::c_int {
            maybe_cpus = maybe_cpus.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    printf(b"\x1b[1;90m\n>>> \x00" as *const u8 as *const libc::c_char);
    if idle_cpus != 0 {
        if maybe_cpus == idle_cpus {
            printf(b"\x1b[1;92mPASS: \x1b[0mYou can run more processes on %u core%s.\x00"
                       as *const u8 as *const libc::c_char, idle_cpus,
                   if idle_cpus > 1 as libc::c_int as libc::c_uint {
                       b"s\x00" as *const u8 as *const libc::c_char
                   } else { b"\x00" as *const u8 as *const libc::c_char });
        } else {
            printf(b"\x1b[1;92mPASS: \x1b[0mYou can run more processes on %u to %u core%s.\x00"
                       as *const u8 as *const libc::c_char, idle_cpus,
                   maybe_cpus,
                   if maybe_cpus > 1 as libc::c_int as libc::c_uint {
                       b"s\x00" as *const u8 as *const libc::c_char
                   } else { b"\x00" as *const u8 as *const libc::c_char });
        }
        printf(b"\x1b[1;90m <<<\x1b[0m\n\n\x00" as *const u8 as
                   *const libc::c_char);
        return 0 as libc::c_int
    }
    if maybe_cpus != 0 {
        printf(b"\x1b[1;93mCAUTION: \x1b[0mYou may still have %u core%s available.\x00"
                   as *const u8 as *const libc::c_char, maybe_cpus,
               if maybe_cpus > 1 as libc::c_int as libc::c_uint {
                   b"s\x00" as *const u8 as *const libc::c_char
               } else { b"\x00" as *const u8 as *const libc::c_char });
        printf(b"\x1b[1;90m <<<\x1b[0m\n\n\x00" as *const u8 as
                   *const libc::c_char);
        return 1 as libc::c_int
    }
    printf(b"\x1b[1;91mFAIL: \x1b[0mAll cores are overbooked.\x00" as
               *const u8 as *const libc::c_char);
    printf(b"\x1b[1;90m <<<\x1b[0m\n\n\x00" as *const u8 as
               *const libc::c_char);
    return 2 as libc::c_int;
    /* ^HAVE_AFFINITY */
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
