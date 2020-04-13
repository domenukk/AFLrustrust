use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn select(__nfds: libc::c_int, __readfds: *mut fd_set,
              __writefds: *mut fd_set, __exceptfds: *mut fd_set,
              __timeout: *mut timeval) -> libc::c_int;
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
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn access(__name: *const libc::c_char, __type: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void)
     -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
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
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
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
/*
   american fuzzy lop++ - common routines
   --------------------------------------

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

   Gather some functions common to multiple executables

   - detect_file_args

 */
/* Detect @@ in args. */
#[no_mangle]
pub static mut be_quiet: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut doc_path: *mut u8_0 =
    b"\x00" as *const u8 as *const libc::c_char as *mut u8_0;
#[no_mangle]
pub static mut last_intr: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut afl_environment_variables: [*mut libc::c_char; 103] =
    [b"AFL_ALIGNED_ALLOC\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_ALLOW_TMP\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_ANALYZE_HEX\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_AS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"AFL_AUTORESUME\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_AS_FORCE_INSTRUMENT\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_BENCH_JUST_ONE\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_BENCH_UNTIL_CRASH\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_CAL_FAST\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_CC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"AFL_CMIN_ALLOW_ANY\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_CMIN_CRASHES_ONLY\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_CODE_END\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_CODE_START\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_COMPCOV_BINNAME\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_COMPCOV_LEVEL\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_CUSTOM_MUTATOR_LIBRARY\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_CUSTOM_MUTATOR_ONLY\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_CXX\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"AFL_DEBUG\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_DEBUG_CHILD_OUTPUT\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_DISABLE_TRIM\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_DONT_OPTIMIZE\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_DUMB_FORKSRV\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_ENTRYPOINT\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_EXIT_WHEN_DONE\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_FAST_CAL\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_FORCE_UI\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_GCC_WHITELIST\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_GCJ\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"AFL_HANG_TMOUT\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_HARDEN\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_I_DONT_CARE_ABOUT_MISSING_CRASHES\x00" as *const u8 as
         *const libc::c_char as *mut libc::c_char,
     b"AFL_IMPORT_FIRST\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_INST_LIBS\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_INST_RATIO\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_KEEP_TRACES\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_KEEP_ASSEMBLY\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LD_HARD_FAIL\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LD_LIMIT_MB\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LD_NO_CALLOC_OVER\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LD_PRELOAD\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LD_VERBOSE\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_CMPLOG\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_INSTRIM\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_CTX\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_INSTRUMENT\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_INSTRIM_LOOPHEAD\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_LTO_AUTODICTIONARY\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_AUTODICTIONARY\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_INSTRIM_SKIPSINGLEBLOCK\x00" as *const u8 as
         *const libc::c_char as *mut libc::c_char,
     b"AFL_LLVM_LAF_SPLIT_COMPARES\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_LAF_SPLIT_COMPARES_BITW\x00" as *const u8 as
         *const libc::c_char as *mut libc::c_char,
     b"AFL_LLVM_LAF_SPLIT_FLOATS\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_LAF_SPLIT_SWITCHES\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_LAF_TRANSFORM_COMPARES\x00" as *const u8 as
         *const libc::c_char as *mut libc::c_char,
     b"AFL_LLVM_NGRAM_SIZE\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_NGRAM_SIZE\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_NOT_ZERO\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_WHITELIST\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_NO_AFFINITY\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_LTO_STARTID\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_LLVM_LTO_DONTWRITEID\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_NO_ARITH\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_NO_BUILTIN\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_NO_CPU_RED\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_NO_FORKSRV\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_NO_UI\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_NO_X86\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_PATH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"AFL_PERFORMANCE_FILE\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_POST_LIBRARY\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_PRELOAD\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_PYTHON_MODULE\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_QEMU_COMPCOV\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_QEMU_COMPCOV_DEBUG\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_QEMU_DEBUG_MAPS\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_QEMU_DISABLE_CACHE\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_QEMU_PERSISTENT_ADDR\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_QEMU_PERSISTENT_CNT\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_QEMU_PERSISTENT_GPR\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_QEMU_PERSISTENT_HOOK\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_QEMU_PERSISTENT_RET\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_QEMU_PERSISTENT_RETADDR_OFFSET\x00" as *const u8 as
         *const libc::c_char as *mut libc::c_char,
     b"AFL_QUIET\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_RANDOM_ALLOC_CANARY\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_REAL_PATH\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_SHUFFLE_QUEUE\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_SKIP_BIN_CHECK\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_SKIP_CPUFREQ\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_SKIP_CRASHES\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_TMIN_EXACT\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_TMPDIR\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_TOKEN_FILE\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_TRACE_PC\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_USE_ASAN\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_USE_MSAN\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_USE_TRACE_PC\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_USE_UBSAN\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_USE_CFISAN\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_WINE_PATH\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"AFL_NO_SNAPSHOT\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char, 0 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn detect_file_args(mut argv: *mut *mut libc::c_char,
                                          mut prog_in: *mut u8_0,
                                          mut use_stdin: *mut u8_0) {
    let mut i: u32_0 = 0 as libc::c_int as u32_0;
    let mut cwd: [u8_0; 4096] = [0; 4096];
    if getcwd(cwd.as_mut_ptr() as *mut libc::c_char,
              ::std::mem::size_of::<[u8_0; 4096]>() as
                  libc::c_ulong).is_null() {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mgetcwd() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-common.c\x00" as *const u8 as *const libc::c_char,
               89 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    /* we are working with libc-heap-allocated argvs. So do not mix them with
   * other allocation APIs like ck_alloc. That would disturb the free() calls.
   */
    while !(*argv.offset(i as isize)).is_null() {
        let mut aa_loc: *mut u8_0 =
            strstr(*argv.offset(i as isize),
                   b"@@\x00" as *const u8 as *const libc::c_char) as
                *mut u8_0;
        if !aa_loc.is_null() {
            if prog_in.is_null() {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0m@@ syntax is not supported by this tool.\x00"
                           as *const u8 as *const libc::c_char);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-common.c\x00" as *const u8 as
                           *const libc::c_char, 100 as libc::c_int);
                exit(1 as libc::c_int);
            }
            *use_stdin = 0 as libc::c_int as u8_0;
            if *prog_in.offset(0 as libc::c_int as isize) as libc::c_int !=
                   0 as libc::c_int {
                // not afl-showmap special case
                let mut n_arg: *mut u8_0 = 0 as *mut u8_0;
                /* Be sure that we're always using fully-qualified paths. */
                *aa_loc = 0 as libc::c_int as u8_0;
                /* Construct a replacement argv value. */
                if *prog_in.offset(0 as libc::c_int as isize) as libc::c_int
                       == '/' as i32 {
                    n_arg =
                        ({
                             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                             let mut _len: s32 =
                                 snprintf(0 as *mut libc::c_char,
                                          0 as libc::c_int as libc::c_ulong,
                                          b"%s%s%s\x00" as *const u8 as
                                              *const libc::c_char,
                                          *argv.offset(i as isize), prog_in,
                                          aa_loc.offset(2 as libc::c_int as
                                                            isize));
                             if _len < 0 as libc::c_int {
                                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        b"func_unknown\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"src/afl-common.c\x00" as *const u8
                                            as *const libc::c_char,
                                        116 as libc::c_int);
                                 exit(1 as libc::c_int);
                             }
                             _tmp =
                                 DFL_ck_alloc((_len + 1 as libc::c_int) as
                                                  u32_0) as *mut u8_0;
                             snprintf(_tmp as *mut libc::c_char,
                                      (_len + 1 as libc::c_int) as
                                          libc::c_ulong,
                                      b"%s%s%s\x00" as *const u8 as
                                          *const libc::c_char,
                                      *argv.offset(i as isize), prog_in,
                                      aa_loc.offset(2 as libc::c_int as
                                                        isize));
                             _tmp
                         })
                } else {
                    n_arg =
                        ({
                             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                             let mut _len: s32 =
                                 snprintf(0 as *mut libc::c_char,
                                          0 as libc::c_int as libc::c_ulong,
                                          b"%s%s/%s%s\x00" as *const u8 as
                                              *const libc::c_char,
                                          *argv.offset(i as isize),
                                          cwd.as_mut_ptr(), prog_in,
                                          aa_loc.offset(2 as libc::c_int as
                                                            isize));
                             if _len < 0 as libc::c_int {
                                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        b"func_unknown\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"src/afl-common.c\x00" as *const u8
                                            as *const libc::c_char,
                                        120 as libc::c_int);
                                 exit(1 as libc::c_int);
                             }
                             _tmp =
                                 DFL_ck_alloc((_len + 1 as libc::c_int) as
                                                  u32_0) as *mut u8_0;
                             snprintf(_tmp as *mut libc::c_char,
                                      (_len + 1 as libc::c_int) as
                                          libc::c_ulong,
                                      b"%s%s/%s%s\x00" as *const u8 as
                                          *const libc::c_char,
                                      *argv.offset(i as isize),
                                      cwd.as_mut_ptr(), prog_in,
                                      aa_loc.offset(2 as libc::c_int as
                                                        isize));
                             _tmp
                         })
                }
                DFL_ck_free(*argv.offset(i as isize) as *mut libc::c_void);
                let ref mut fresh0 = *argv.offset(i as isize);
                *fresh0 = n_arg as *mut libc::c_char
            }
        }
        i = i.wrapping_add(1)
    };
    /* argvs are automatically freed at exit. */
}
/* duplicate the system argv so that
  we can edit (and free!) it later */
#[no_mangle]
pub unsafe extern "C" fn argv_cpy_dup(mut argc: libc::c_int,
                                      mut argv: *mut *mut libc::c_char)
 -> *mut *mut libc::c_char {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut ret: *mut *mut libc::c_char =
        DFL_ck_alloc(((argc + 1 as libc::c_int) as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                          as libc::c_ulong) as
                         u32_0) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < argc {
        let ref mut fresh1 = *ret.offset(i as isize);
        *fresh1 =
            DFL_ck_strdup(*argv.offset(i as isize) as *mut u8_0) as
                *mut libc::c_char;
        i += 1
    }
    let ref mut fresh2 = *ret.offset(i as isize);
    *fresh2 = 0 as *mut libc::c_char;
    return ret;
}
/*
   american fuzzy lop++ - common routines header
   ---------------------------------------------

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

   Gather some functions common to multiple executables

   - detect_file_args

 */
/* STRINGIFY_VAL_SIZE_MAX will fit all stringify_ strings. */
/* frees all args in the given argv,
   previously created by argv_cpy_dup */
#[no_mangle]
pub unsafe extern "C" fn argv_cpy_free(mut argv: *mut *mut libc::c_char) {
    let mut i: u32_0 = 0 as libc::c_int as u32_0;
    while !(*argv.offset(i as isize)).is_null() {
        DFL_ck_free(*argv.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    DFL_ck_free(argv as *mut libc::c_void);
}
/* Rewrite argv for QEMU. */
#[no_mangle]
pub unsafe extern "C" fn get_qemu_argv(mut own_loc: *mut u8_0,
                                       mut target_path_p: *mut *mut u8_0,
                                       mut argc: libc::c_int,
                                       mut argv: *mut *mut libc::c_char)
 -> *mut *mut libc::c_char {
    let mut new_argv: *mut *mut libc::c_char =
        DFL_ck_alloc((::std::mem::size_of::<*mut libc::c_char>() as
                          libc::c_ulong).wrapping_mul((argc +
                                                           4 as libc::c_int)
                                                          as libc::c_ulong) as
                         u32_0) as *mut *mut libc::c_char;
    let mut tmp: *mut u8_0 = 0 as *mut u8_0;
    let mut cp: *mut u8_0 = 0 as *mut u8_0;
    let mut rsl: *mut u8_0 = 0 as *mut u8_0;
    let mut own_copy: *mut u8_0 = 0 as *mut u8_0;
    memcpy(new_argv.offset(3 as libc::c_int as isize) as *mut libc::c_void,
           argv.offset(1 as libc::c_int as isize) as *const libc::c_void,
           (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong as
                libc::c_int * argc) as libc::c_ulong);
    let ref mut fresh3 = *new_argv.offset(2 as libc::c_int as isize);
    *fresh3 = *target_path_p as *mut libc::c_char;
    let ref mut fresh4 = *new_argv.offset(1 as libc::c_int as isize);
    *fresh4 =
        b"--\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    /* Now we need to actually find the QEMU binary to put in argv[0]. */
    tmp =
        getenv(b"AFL_PATH\x00" as *const u8 as *const libc::c_char) as
            *mut u8_0;
    if !tmp.is_null() {
        cp =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/afl-qemu-trace\x00" as *const u8 as
                                  *const libc::c_char, tmp);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-common.c\x00" as *const u8 as
                                *const libc::c_char, 195 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/afl-qemu-trace\x00" as *const u8 as
                              *const libc::c_char, tmp);
                 _tmp
             });
        if access(cp as *const libc::c_char, 1 as libc::c_int) != 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to find \'%s\'\x00"
                       as *const u8 as *const libc::c_char, tmp);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-common.c\x00" as *const u8 as
                       *const libc::c_char, 197 as libc::c_int);
            exit(1 as libc::c_int);
        }
        let ref mut fresh5 = *new_argv.offset(0 as libc::c_int as isize);
        *fresh5 = cp as *mut libc::c_char;
        *target_path_p = *fresh5 as *mut u8_0;
        return new_argv
    }
    own_copy = DFL_ck_strdup(own_loc);
    rsl = strrchr(own_copy as *const libc::c_char, '/' as i32) as *mut u8_0;
    if !rsl.is_null() {
        *rsl = 0 as libc::c_int as u8_0;
        cp =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/afl-qemu-trace\x00" as *const u8 as
                                  *const libc::c_char, own_copy);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-common.c\x00" as *const u8 as
                                *const libc::c_char, 211 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/afl-qemu-trace\x00" as *const u8 as
                              *const libc::c_char, own_copy);
                 _tmp
             });
        DFL_ck_free(own_copy as *mut libc::c_void);
        if access(cp as *const libc::c_char, 1 as libc::c_int) == 0 {
            let ref mut fresh6 = *new_argv.offset(0 as libc::c_int as isize);
            *fresh6 = cp as *mut libc::c_char;
            *target_path_p = *fresh6 as *mut u8_0;
            return new_argv
        }
    } else { DFL_ck_free(own_copy as *mut libc::c_void); }
    if access(b"/usr/local/bin/afl-qemu-trace\x00" as *const u8 as
                  *const libc::c_char, 1 as libc::c_int) == 0 {
        if !cp.is_null() { DFL_ck_free(cp as *mut libc::c_void); }
        let ref mut fresh7 = *new_argv.offset(0 as libc::c_int as isize);
        *fresh7 =
            DFL_ck_strdup(b"/usr/local/bin/afl-qemu-trace\x00" as *const u8 as
                              *const libc::c_char as *mut u8_0) as
                *mut libc::c_char;
        *target_path_p = *fresh7 as *mut u8_0;
        return new_argv
    }
    printf(b"\n\x1b[1;91m[-] \x1b[0mOops, unable to find the \'afl-qemu-trace\' binary. The binary must be built\n    separately by following the instructions in qemu_mode/README.md. If you\n    already have the binary installed, you may need to specify AFL_PATH in the\n    environment.\n\n    Of course, even without QEMU, afl-fuzz can still work with binaries that are\n    instrumented at compile time with afl-gcc. It is also possible to use it as a\n    traditional \"dumb\" fuzzer by specifying \'-n\' in the command line.\n\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mFailed to locate \'afl-qemu-trace\'.\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00" as
               *const u8 as *const libc::c_char,
           b"func_unknown\x00" as *const u8 as *const libc::c_char,
           b"src/afl-common.c\x00" as *const u8 as *const libc::c_char,
           251 as libc::c_int);
    exit(1 as libc::c_int);
}
/* Rewrite argv for Wine+QEMU. */
#[no_mangle]
pub unsafe extern "C" fn get_wine_argv(mut own_loc: *mut u8_0,
                                       mut target_path_p: *mut *mut u8_0,
                                       mut argc: libc::c_int,
                                       mut argv: *mut *mut libc::c_char)
 -> *mut *mut libc::c_char {
    let mut new_argv: *mut *mut libc::c_char =
        DFL_ck_alloc((::std::mem::size_of::<*mut libc::c_char>() as
                          libc::c_ulong).wrapping_mul((argc +
                                                           3 as libc::c_int)
                                                          as libc::c_ulong) as
                         u32_0) as *mut *mut libc::c_char;
    let mut tmp: *mut u8_0 = 0 as *mut u8_0;
    let mut cp: *mut u8_0 = 0 as *mut u8_0;
    let mut rsl: *mut u8_0 = 0 as *mut u8_0;
    let mut own_copy: *mut u8_0 = 0 as *mut u8_0;
    memcpy(new_argv.offset(2 as libc::c_int as isize) as *mut libc::c_void,
           argv.offset(1 as libc::c_int as isize) as *const libc::c_void,
           (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong as
                libc::c_int * argc) as libc::c_ulong);
    let ref mut fresh8 = *new_argv.offset(1 as libc::c_int as isize);
    *fresh8 = *target_path_p as *mut libc::c_char;
    /* Now we need to actually find the QEMU binary to put in argv[0]. */
    tmp =
        getenv(b"AFL_PATH\x00" as *const u8 as *const libc::c_char) as
            *mut u8_0;
    if !tmp.is_null() {
        cp =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/afl-qemu-trace\x00" as *const u8 as
                                  *const libc::c_char, tmp);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-common.c\x00" as *const u8 as
                                *const libc::c_char, 272 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/afl-qemu-trace\x00" as *const u8 as
                              *const libc::c_char, tmp);
                 _tmp
             });
        if access(cp as *const libc::c_char, 1 as libc::c_int) != 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to find \'%s\'\x00"
                       as *const u8 as *const libc::c_char, tmp);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-common.c\x00" as *const u8 as
                       *const libc::c_char, 274 as libc::c_int);
            exit(1 as libc::c_int);
        }
        DFL_ck_free(cp as *mut libc::c_void);
        cp =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/afl-wine-trace\x00" as *const u8 as
                                  *const libc::c_char, tmp);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-common.c\x00" as *const u8 as
                                *const libc::c_char, 278 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/afl-wine-trace\x00" as *const u8 as
                              *const libc::c_char, tmp);
                 _tmp
             });
        if access(cp as *const libc::c_char, 1 as libc::c_int) != 0 {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to find \'%s\'\x00"
                       as *const u8 as *const libc::c_char, tmp);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-common.c\x00" as *const u8 as
                       *const libc::c_char, 280 as libc::c_int);
            exit(1 as libc::c_int);
        }
        let ref mut fresh9 = *new_argv.offset(0 as libc::c_int as isize);
        *fresh9 = cp as *mut libc::c_char;
        *target_path_p = *fresh9 as *mut u8_0;
        return new_argv
    }
    own_copy = DFL_ck_strdup(own_loc);
    rsl = strrchr(own_copy as *const libc::c_char, '/' as i32) as *mut u8_0;
    if !rsl.is_null() {
        *rsl = 0 as libc::c_int as u8_0;
        cp =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/afl-qemu-trace\x00" as *const u8 as
                                  *const libc::c_char, own_copy);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-common.c\x00" as *const u8 as
                                *const libc::c_char, 294 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/afl-qemu-trace\x00" as *const u8 as
                              *const libc::c_char, own_copy);
                 _tmp
             });
        if !cp.is_null() &&
               access(cp as *const libc::c_char, 1 as libc::c_int) == 0 {
            DFL_ck_free(cp as *mut libc::c_void);
            cp =
                ({
                     let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                     let mut _len: s32 =
                         snprintf(0 as *mut libc::c_char,
                                  0 as libc::c_int as libc::c_ulong,
                                  b"%s/afl-wine-trace\x00" as *const u8 as
                                      *const libc::c_char, own_copy);
                     if _len < 0 as libc::c_int {
                         printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                    as *const u8 as *const libc::c_char);
                         printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                    as *const u8 as *const libc::c_char,
                                b"func_unknown\x00" as *const u8 as
                                    *const libc::c_char,
                                b"src/afl-common.c\x00" as *const u8 as
                                    *const libc::c_char, 300 as libc::c_int);
                         exit(1 as libc::c_int);
                     }
                     _tmp =
                         DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                             *mut u8_0;
                     snprintf(_tmp as *mut libc::c_char,
                              (_len + 1 as libc::c_int) as libc::c_ulong,
                              b"%s/afl-wine-trace\x00" as *const u8 as
                                  *const libc::c_char, own_copy);
                     _tmp
                 });
            if access(cp as *const libc::c_char, 1 as libc::c_int) == 0 {
                let ref mut fresh10 =
                    *new_argv.offset(0 as libc::c_int as isize);
                *fresh10 = cp as *mut libc::c_char;
                *target_path_p = *fresh10 as *mut u8_0;
                return new_argv
            }
        }
        DFL_ck_free(own_copy as *mut libc::c_void);
    } else { DFL_ck_free(own_copy as *mut libc::c_void); }
    let mut ncp: *mut u8_0 =
        b"/usr/local/bin/afl-qemu-trace\x00" as *const u8 as
            *const libc::c_char as *mut u8_0;
    if access(ncp as *const libc::c_char, 1 as libc::c_int) == 0 {
        ncp =
            b"/usr/local/bin/afl-wine-trace\x00" as *const u8 as
                *const libc::c_char as *mut u8_0;
        if access(ncp as *const libc::c_char, 1 as libc::c_int) == 0 {
            let ref mut fresh11 = *new_argv.offset(0 as libc::c_int as isize);
            *fresh11 = DFL_ck_strdup(ncp) as *mut libc::c_char;
            *target_path_p = *fresh11 as *mut u8_0;
            return new_argv
        }
    }
    printf(b"\n\x1b[1;91m[-] \x1b[0mOops, unable to find the \'%s\' binary. The binary must be built\n    separately by following the instructions in qemu_mode/README.md. If you\n    already have the binary installed, you may need to specify AFL_PATH in the\n    environment.\n\n    Of course, even without QEMU, afl-fuzz can still work with binaries that are\n    instrumented at compile time with afl-gcc. It is also possible to use it as a\n    traditional \"dumb\" fuzzer by specifying \'-n\' in the command line.\n\x00"
               as *const u8 as *const libc::c_char, ncp);
    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mFailed to locate \'%s\'.\x00"
               as *const u8 as *const libc::c_char, ncp);
    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00" as
               *const u8 as *const libc::c_char,
           b"func_unknown\x00" as *const u8 as *const libc::c_char,
           b"src/afl-common.c\x00" as *const u8 as *const libc::c_char,
           352 as libc::c_int);
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn check_environment_vars(mut envp:
                                                    *mut *mut libc::c_char) {
    if be_quiet != 0 { return }
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    loop  {
        let fresh12 = index;
        index = index + 1;
        env = *envp.offset(fresh12 as isize);
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
                      !afl_environment_variables[i as usize].is_null() {
                if strncmp(env, afl_environment_variables[i as usize],
                           strlen(afl_environment_variables[i as usize])) ==
                       0 as libc::c_int &&
                       *env.offset(strlen(afl_environment_variables[i as
                                                                        usize])
                                       as isize) as libc::c_int == '=' as i32
                   {
                    match_0 = 1 as libc::c_int;
                    val = getenv(afl_environment_variables[i as usize]);
                    if !val.is_null() && *val == 0 {
                        printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mAFL environment variable %s defined but is empty, this can lead to unexpected consequences\x00"
                                   as *const u8 as *const libc::c_char,
                               afl_environment_variables[i as usize]);
                        printf(b"\x1b[0m\n\x00" as *const u8 as
                                   *const libc::c_char);
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
#[no_mangle]
pub unsafe extern "C" fn get_afl_env(mut env: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    val = getenv(env);
    if !val.is_null() {
        if be_quiet == 0 {
            printf(b"\x1b[1;92m[+] \x1b[0mLoaded environment variable %s with value %s\x00"
                       as *const u8 as *const libc::c_char, env, val);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
    }
    return val;
}
/* path to documentation dir        */
/* Get unix time in milliseconds */
#[no_mangle]
pub unsafe extern "C" fn get_cur_time() -> u64_0 {
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut tz: timezone = timezone{tz_minuteswest: 0, tz_dsttime: 0,};
    gettimeofday(&mut tv, &mut tz as *mut timezone as *mut libc::c_void);
    return (tv.tv_sec as
                libc::c_ulonglong).wrapping_mul(1000 as
                                                    libc::c_ulonglong).wrapping_add((tv.tv_usec
                                                                                         /
                                                                                         1000
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_long)
                                                                                        as
                                                                                        libc::c_ulonglong);
}
/* Get unix time in microseconds */
/* Get unix time in microseconds */
#[no_mangle]
pub unsafe extern "C" fn get_cur_time_us() -> u64_0 {
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut tz: timezone = timezone{tz_minuteswest: 0, tz_dsttime: 0,};
    gettimeofday(&mut tv, &mut tz as *mut timezone as *mut libc::c_void);
    return (tv.tv_sec as
                libc::c_ulonglong).wrapping_mul(1000000 as
                                                    libc::c_ulonglong).wrapping_add(tv.tv_usec
                                                                                        as
                                                                                        libc::c_ulonglong);
}
/* Describe integer. The buf should be
   at least 6 bytes to fit all ints we randomly see.
   Will return buf for convenience. */
/* Describe integer. The buf should be
   at least 6 bytes to fit all ints we randomly see.
   Will return buf for convenience. */
#[no_mangle]
pub unsafe extern "C" fn stringify_int(mut buf: *mut u8_0, mut len: size_t,
                                       mut val: u64_0) -> *mut u8_0 {
    /* 0-9999 */
    if val < (1 as libc::c_int * 10000 as libc::c_int) as libc::c_ulonglong {
        snprintf(buf as *mut libc::c_char, len,
                 b"%llu\x00" as *const u8 as *const libc::c_char,
                 val.wrapping_div(1 as libc::c_int as libc::c_ulonglong));
        return buf
    }
    /* 10.0k - 99.9k */
    if (val as libc::c_double) <
           1000 as libc::c_int as libc::c_double * 99.95f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.01fk\x00" as *const u8 as *const libc::c_char,
                 val as libc::c_double /
                     1000 as libc::c_int as libc::c_double);
        return buf
    }
    /* 100k - 999k */
    if val < (1000 as libc::c_int * 1000 as libc::c_int) as libc::c_ulonglong
       {
        snprintf(buf as *mut libc::c_char, len,
                 b"%lluk\x00" as *const u8 as *const libc::c_char,
                 val.wrapping_div(1000 as libc::c_int as libc::c_ulonglong));
        return buf
    }
    /* 1.00M - 9.99M */
    if (val as libc::c_double) <
           (1000 as libc::c_int * 1000 as libc::c_int) as libc::c_double *
               9.995f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.02fM\x00" as *const u8 as *const libc::c_char,
                 val as libc::c_double /
                     (1000 as libc::c_int * 1000 as libc::c_int) as
                         libc::c_double);
        return buf
    }
    /* 10.0M - 99.9M */
    if (val as libc::c_double) <
           (1000 as libc::c_int * 1000 as libc::c_int) as libc::c_double *
               99.95f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.01fM\x00" as *const u8 as *const libc::c_char,
                 val as libc::c_double /
                     (1000 as libc::c_int * 1000 as libc::c_int) as
                         libc::c_double);
        return buf
    }
    /* 100M - 999M */
    if val <
           (1000 as libc::c_int * 1000 as libc::c_int * 1000 as libc::c_int)
               as libc::c_ulonglong {
        snprintf(buf as *mut libc::c_char, len,
                 b"%lluM\x00" as *const u8 as *const libc::c_char,
                 val.wrapping_div((1000 as libc::c_int * 1000 as libc::c_int)
                                      as libc::c_ulonglong));
        return buf
    }
    /* 1.00G - 9.99G */
    if (val as libc::c_double) <
           (1000 as libc::c_longlong * 1000 as libc::c_int as libc::c_longlong
                * 1000 as libc::c_int as libc::c_longlong) as libc::c_double *
               9.995f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.02fG\x00" as *const u8 as *const libc::c_char,
                 val as libc::c_double /
                     (1000 as libc::c_longlong *
                          1000 as libc::c_int as libc::c_longlong *
                          1000 as libc::c_int as libc::c_longlong) as
                         libc::c_double);
        return buf
    }
    /* 10.0G - 99.9G */
    if (val as libc::c_double) <
           (1000 as libc::c_longlong * 1000 as libc::c_int as libc::c_longlong
                * 1000 as libc::c_int as libc::c_longlong) as libc::c_double *
               99.95f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.01fG\x00" as *const u8 as *const libc::c_char,
                 val as libc::c_double /
                     (1000 as libc::c_longlong *
                          1000 as libc::c_int as libc::c_longlong *
                          1000 as libc::c_int as libc::c_longlong) as
                         libc::c_double);
        return buf
    }
    /* 100G - 999G */
    if val <
           (1000 as libc::c_longlong * 1000 as libc::c_int as libc::c_longlong
                * 1000 as libc::c_int as libc::c_longlong *
                1000 as libc::c_int as libc::c_longlong) as libc::c_ulonglong
       {
        snprintf(buf as *mut libc::c_char, len,
                 b"%lluG\x00" as *const u8 as *const libc::c_char,
                 val.wrapping_div((1000 as libc::c_longlong *
                                       1000 as libc::c_int as libc::c_longlong
                                       *
                                       1000 as libc::c_int as
                                           libc::c_longlong) as
                                      libc::c_ulonglong));
        return buf
    }
    /* 1.00T - 9.99G */
    if (val as libc::c_double) <
           (1000 as libc::c_longlong * 1000 as libc::c_int as libc::c_longlong
                * 1000 as libc::c_int as libc::c_longlong *
                1000 as libc::c_int as libc::c_longlong) as libc::c_double *
               9.995f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.02fT\x00" as *const u8 as *const libc::c_char,
                 val as libc::c_double /
                     (1000 as libc::c_longlong *
                          1000 as libc::c_int as libc::c_longlong *
                          1000 as libc::c_int as libc::c_longlong *
                          1000 as libc::c_int as libc::c_longlong) as
                         libc::c_double);
        return buf
    }
    /* 10.0T - 99.9T */
    if (val as libc::c_double) <
           (1000 as libc::c_longlong * 1000 as libc::c_int as libc::c_longlong
                * 1000 as libc::c_int as libc::c_longlong *
                1000 as libc::c_int as libc::c_longlong) as libc::c_double *
               99.95f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.01fT\x00" as *const u8 as *const libc::c_char,
                 val as libc::c_double /
                     (1000 as libc::c_longlong *
                          1000 as libc::c_int as libc::c_longlong *
                          1000 as libc::c_int as libc::c_longlong *
                          1000 as libc::c_int as libc::c_longlong) as
                         libc::c_double);
        return buf
    }
    /* 100T+ */
    strncpy(buf as *mut libc::c_char,
            b"infty\x00" as *const u8 as *const libc::c_char, len);
    *buf.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        = '\u{0}' as i32 as u8_0;
    return buf;
}
/* Describe float. Similar as int. */
/* Describe float. Similar as int. */
#[no_mangle]
pub unsafe extern "C" fn stringify_float(mut buf: *mut u8_0, mut len: size_t,
                                         mut val: libc::c_double)
 -> *mut u8_0 {
    if val < 99.995f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.02f\x00" as *const u8 as *const libc::c_char, val);
    } else if val < 999.95f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.01f\x00" as *const u8 as *const libc::c_char, val);
    } else { stringify_int(buf, len, val as u64_0); }
    return buf;
}
/* Describe integer as memory size. */
/* Describe integer as memory size. */
#[no_mangle]
pub unsafe extern "C" fn stringify_mem_size(mut buf: *mut u8_0,
                                            mut len: size_t, mut val: u64_0)
 -> *mut u8_0 {
    /* 0-9999 */
    if val < (1 as libc::c_int * 10000 as libc::c_int) as libc::c_ulonglong {
        snprintf(buf as *mut libc::c_char, len,
                 b"%llu B\x00" as *const u8 as *const libc::c_char,
                 val.wrapping_div(1 as libc::c_int as libc::c_ulonglong));
        return buf
    }
    /* 10.0k - 99.9k */
    if (val as libc::c_double) <
           1024 as libc::c_int as libc::c_double * 99.95f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.01f kB\x00" as *const u8 as *const libc::c_char,
                 val as libc::c_double /
                     1024 as libc::c_int as libc::c_double);
        return buf
    }
    /* 100k - 999k */
    if val < (1024 as libc::c_int * 1000 as libc::c_int) as libc::c_ulonglong
       {
        snprintf(buf as *mut libc::c_char, len,
                 b"%llu kB\x00" as *const u8 as *const libc::c_char,
                 val.wrapping_div(1024 as libc::c_int as libc::c_ulonglong));
        return buf
    }
    /* 1.00M - 9.99M */
    if (val as libc::c_double) <
           (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_double *
               9.995f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.02f MB\x00" as *const u8 as *const libc::c_char,
                 val as libc::c_double /
                     (1024 as libc::c_int * 1024 as libc::c_int) as
                         libc::c_double);
        return buf
    }
    /* 10.0M - 99.9M */
    if (val as libc::c_double) <
           (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_double *
               99.95f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.01f MB\x00" as *const u8 as *const libc::c_char,
                 val as libc::c_double /
                     (1024 as libc::c_int * 1024 as libc::c_int) as
                         libc::c_double);
        return buf
    }
    /* 100M - 999M */
    if val <
           (1024 as libc::c_int * 1024 as libc::c_int * 1000 as libc::c_int)
               as libc::c_ulonglong {
        snprintf(buf as *mut libc::c_char, len,
                 b"%llu MB\x00" as *const u8 as *const libc::c_char,
                 val.wrapping_div((1024 as libc::c_int * 1024 as libc::c_int)
                                      as libc::c_ulonglong));
        return buf
    }
    /* 1.00G - 9.99G */
    if (val as libc::c_double) <
           (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong) as libc::c_double *
               9.995f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.02f GB\x00" as *const u8 as *const libc::c_char,
                 val as libc::c_double /
                     (1024 as libc::c_longlong *
                          1024 as libc::c_int as libc::c_longlong *
                          1024 as libc::c_int as libc::c_longlong) as
                         libc::c_double);
        return buf
    }
    /* 10.0G - 99.9G */
    if (val as libc::c_double) <
           (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong) as libc::c_double *
               99.95f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.01f GB\x00" as *const u8 as *const libc::c_char,
                 val as libc::c_double /
                     (1024 as libc::c_longlong *
                          1024 as libc::c_int as libc::c_longlong *
                          1024 as libc::c_int as libc::c_longlong) as
                         libc::c_double);
        return buf
    }
    /* 100G - 999G */
    if val <
           (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong *
                1000 as libc::c_int as libc::c_longlong) as libc::c_ulonglong
       {
        snprintf(buf as *mut libc::c_char, len,
                 b"%llu GB\x00" as *const u8 as *const libc::c_char,
                 val.wrapping_div((1024 as libc::c_longlong *
                                       1024 as libc::c_int as libc::c_longlong
                                       *
                                       1024 as libc::c_int as
                                           libc::c_longlong) as
                                      libc::c_ulonglong));
        return buf
    }
    /* 1.00T - 9.99G */
    if (val as libc::c_double) <
           (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong *
                1024 as libc::c_int as libc::c_longlong) as libc::c_double *
               9.995f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.02f TB\x00" as *const u8 as *const libc::c_char,
                 val as libc::c_double /
                     (1024 as libc::c_longlong *
                          1024 as libc::c_int as libc::c_longlong *
                          1024 as libc::c_int as libc::c_longlong *
                          1024 as libc::c_int as libc::c_longlong) as
                         libc::c_double);
        return buf
    }
    /* 10.0T - 99.9T */
    if (val as libc::c_double) <
           (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong *
                1024 as libc::c_int as libc::c_longlong) as libc::c_double *
               99.95f64 {
        snprintf(buf as *mut libc::c_char, len,
                 b"%0.01f TB\x00" as *const u8 as *const libc::c_char,
                 val as libc::c_double /
                     (1024 as libc::c_longlong *
                          1024 as libc::c_int as libc::c_longlong *
                          1024 as libc::c_int as libc::c_longlong *
                          1024 as libc::c_int as libc::c_longlong) as
                         libc::c_double);
        return buf
    }
    /* 100T+ */
    strncpy(buf as *mut libc::c_char,
            b"infty\x00" as *const u8 as *const libc::c_char,
            len.wrapping_sub(1 as libc::c_int as libc::c_ulong));
    *buf.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        = '\u{0}' as i32 as u8_0;
    return buf;
}
/* Describe time delta as string.
   Returns a pointer to buf for convenience. */
/* Describe time delta as string.
   Returns a pointer to buf for convenience. */
#[no_mangle]
pub unsafe extern "C" fn stringify_time_diff(mut buf: *mut u8_0,
                                             mut len: size_t,
                                             mut cur_ms: u64_0,
                                             mut event_ms: u64_0)
 -> *mut u8_0 {
    let mut delta: u64_0 = 0;
    let mut t_d: s32 = 0;
    let mut t_h: s32 = 0;
    let mut t_m: s32 = 0;
    let mut t_s: s32 = 0;
    let mut val_buf: [u8_0; 16] = [0; 16];
    if event_ms == 0 {
        snprintf(buf as *mut libc::c_char, len,
                 b"none seen yet\x00" as *const u8 as *const libc::c_char);
    } else {
        delta = cur_ms.wrapping_sub(event_ms);
        t_d =
            delta.wrapping_div(1000 as libc::c_int as
                                   libc::c_ulonglong).wrapping_div(60 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulonglong).wrapping_div(60
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulonglong).wrapping_div(24
                                                                                                                                               as
                                                                                                                                               libc::c_int
                                                                                                                                               as
                                                                                                                                               libc::c_ulonglong)
                as s32;
        t_h =
            delta.wrapping_div(1000 as libc::c_int as
                                   libc::c_ulonglong).wrapping_div(60 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulonglong).wrapping_div(60
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulonglong).wrapping_rem(24
                                                                                                                                               as
                                                                                                                                               libc::c_int
                                                                                                                                               as
                                                                                                                                               libc::c_ulonglong)
                as s32;
        t_m =
            delta.wrapping_div(1000 as libc::c_int as
                                   libc::c_ulonglong).wrapping_div(60 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulonglong).wrapping_rem(60
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulonglong)
                as s32;
        t_s =
            delta.wrapping_div(1000 as libc::c_int as
                                   libc::c_ulonglong).wrapping_rem(60 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulonglong)
                as s32;
        stringify_int(val_buf.as_mut_ptr(),
                      ::std::mem::size_of::<[u8_0; 16]>() as libc::c_ulong,
                      t_d as u64_0);
        snprintf(buf as *mut libc::c_char, len,
                 b"%s days, %d hrs, %d min, %d sec\x00" as *const u8 as
                     *const libc::c_char, val_buf.as_mut_ptr(), t_h, t_m,
                 t_s);
    }
    return buf;
}
/* Unsafe Describe integer. The buf sizes are not checked.
   This is unsafe but fast.
   Will return buf for convenience. */
/* Unsafe Describe integer. The buf sizes are not checked.
   This is unsafe but fast.
   Will return buf for convenience. */
#[no_mangle]
pub unsafe extern "C" fn u_stringify_int(mut buf: *mut u8_0, mut val: u64_0)
 -> *mut u8_0 {
    /* 0-9999 */
    if val < (1 as libc::c_int * 10000 as libc::c_int) as libc::c_ulonglong {
        sprintf(buf as *mut libc::c_char,
                b"%llu\x00" as *const u8 as *const libc::c_char,
                val.wrapping_div(1 as libc::c_int as libc::c_ulonglong));
        return buf
    }
    /* 10.0k - 99.9k */
    if (val as libc::c_double) <
           1000 as libc::c_int as libc::c_double * 99.95f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.01fk\x00" as *const u8 as *const libc::c_char,
                val as libc::c_double /
                    1000 as libc::c_int as libc::c_double);
        return buf
    }
    /* 100k - 999k */
    if val < (1000 as libc::c_int * 1000 as libc::c_int) as libc::c_ulonglong
       {
        sprintf(buf as *mut libc::c_char,
                b"%lluk\x00" as *const u8 as *const libc::c_char,
                val.wrapping_div(1000 as libc::c_int as libc::c_ulonglong));
        return buf
    }
    /* 1.00M - 9.99M */
    if (val as libc::c_double) <
           (1000 as libc::c_int * 1000 as libc::c_int) as libc::c_double *
               9.995f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.02fM\x00" as *const u8 as *const libc::c_char,
                val as libc::c_double /
                    (1000 as libc::c_int * 1000 as libc::c_int) as
                        libc::c_double);
        return buf
    }
    /* 10.0M - 99.9M */
    if (val as libc::c_double) <
           (1000 as libc::c_int * 1000 as libc::c_int) as libc::c_double *
               99.95f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.01fM\x00" as *const u8 as *const libc::c_char,
                val as libc::c_double /
                    (1000 as libc::c_int * 1000 as libc::c_int) as
                        libc::c_double);
        return buf
    }
    /* 100M - 999M */
    if val <
           (1000 as libc::c_int * 1000 as libc::c_int * 1000 as libc::c_int)
               as libc::c_ulonglong {
        sprintf(buf as *mut libc::c_char,
                b"%lluM\x00" as *const u8 as *const libc::c_char,
                val.wrapping_div((1000 as libc::c_int * 1000 as libc::c_int)
                                     as libc::c_ulonglong));
        return buf
    }
    /* 1.00G - 9.99G */
    if (val as libc::c_double) <
           (1000 as libc::c_longlong * 1000 as libc::c_int as libc::c_longlong
                * 1000 as libc::c_int as libc::c_longlong) as libc::c_double *
               9.995f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.02fG\x00" as *const u8 as *const libc::c_char,
                val as libc::c_double /
                    (1000 as libc::c_longlong *
                         1000 as libc::c_int as libc::c_longlong *
                         1000 as libc::c_int as libc::c_longlong) as
                        libc::c_double);
        return buf
    }
    /* 10.0G - 99.9G */
    if (val as libc::c_double) <
           (1000 as libc::c_longlong * 1000 as libc::c_int as libc::c_longlong
                * 1000 as libc::c_int as libc::c_longlong) as libc::c_double *
               99.95f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.01fG\x00" as *const u8 as *const libc::c_char,
                val as libc::c_double /
                    (1000 as libc::c_longlong *
                         1000 as libc::c_int as libc::c_longlong *
                         1000 as libc::c_int as libc::c_longlong) as
                        libc::c_double);
        return buf
    }
    /* 100G - 999G */
    if val <
           (1000 as libc::c_longlong * 1000 as libc::c_int as libc::c_longlong
                * 1000 as libc::c_int as libc::c_longlong *
                1000 as libc::c_int as libc::c_longlong) as libc::c_ulonglong
       {
        sprintf(buf as *mut libc::c_char,
                b"%lluG\x00" as *const u8 as *const libc::c_char,
                val.wrapping_div((1000 as libc::c_longlong *
                                      1000 as libc::c_int as libc::c_longlong
                                      *
                                      1000 as libc::c_int as libc::c_longlong)
                                     as libc::c_ulonglong));
        return buf
    }
    /* 1.00T - 9.99G */
    if (val as libc::c_double) <
           (1000 as libc::c_longlong * 1000 as libc::c_int as libc::c_longlong
                * 1000 as libc::c_int as libc::c_longlong *
                1000 as libc::c_int as libc::c_longlong) as libc::c_double *
               9.995f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.02fT\x00" as *const u8 as *const libc::c_char,
                val as libc::c_double /
                    (1000 as libc::c_longlong *
                         1000 as libc::c_int as libc::c_longlong *
                         1000 as libc::c_int as libc::c_longlong *
                         1000 as libc::c_int as libc::c_longlong) as
                        libc::c_double);
        return buf
    }
    /* 10.0T - 99.9T */
    if (val as libc::c_double) <
           (1000 as libc::c_longlong * 1000 as libc::c_int as libc::c_longlong
                * 1000 as libc::c_int as libc::c_longlong *
                1000 as libc::c_int as libc::c_longlong) as libc::c_double *
               99.95f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.01fT\x00" as *const u8 as *const libc::c_char,
                val as libc::c_double /
                    (1000 as libc::c_longlong *
                         1000 as libc::c_int as libc::c_longlong *
                         1000 as libc::c_int as libc::c_longlong *
                         1000 as libc::c_int as libc::c_longlong) as
                        libc::c_double);
        return buf
    }
    /* 100T+ */
    strcpy(buf as *mut libc::c_char,
           b"infty\x00" as *const u8 as *const libc::c_char);
    return buf;
}
/* Unsafe describe float. Similar as unsafe int. */
/* Unsafe describe float. Similar as unsafe int. */
#[no_mangle]
pub unsafe extern "C" fn u_stringify_float(mut buf: *mut u8_0,
                                           mut val: libc::c_double)
 -> *mut u8_0 {
    if val < 99.995f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.02f\x00" as *const u8 as *const libc::c_char, val);
    } else if val < 999.95f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.01f\x00" as *const u8 as *const libc::c_char, val);
    } else { return u_stringify_int(buf, val as u64_0) }
    return buf;
}
/* Unsafe describe integer as memory size. */
/* Unsafe describe integer as memory size. */
#[no_mangle]
pub unsafe extern "C" fn u_stringify_mem_size(mut buf: *mut u8_0,
                                              mut val: u64_0) -> *mut u8_0 {
    /* 0-9999 */
    if val < (1 as libc::c_int * 10000 as libc::c_int) as libc::c_ulonglong {
        sprintf(buf as *mut libc::c_char,
                b"%llu B\x00" as *const u8 as *const libc::c_char,
                val.wrapping_div(1 as libc::c_int as libc::c_ulonglong));
        return buf
    }
    /* 10.0k - 99.9k */
    if (val as libc::c_double) <
           1024 as libc::c_int as libc::c_double * 99.95f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.01f kB\x00" as *const u8 as *const libc::c_char,
                val as libc::c_double /
                    1024 as libc::c_int as libc::c_double);
        return buf
    }
    /* 100k - 999k */
    if val < (1024 as libc::c_int * 1000 as libc::c_int) as libc::c_ulonglong
       {
        sprintf(buf as *mut libc::c_char,
                b"%llu kB\x00" as *const u8 as *const libc::c_char,
                val.wrapping_div(1024 as libc::c_int as libc::c_ulonglong));
        return buf
    }
    /* 1.00M - 9.99M */
    if (val as libc::c_double) <
           (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_double *
               9.995f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.02f MB\x00" as *const u8 as *const libc::c_char,
                val as libc::c_double /
                    (1024 as libc::c_int * 1024 as libc::c_int) as
                        libc::c_double);
        return buf
    }
    /* 10.0M - 99.9M */
    if (val as libc::c_double) <
           (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_double *
               99.95f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.01f MB\x00" as *const u8 as *const libc::c_char,
                val as libc::c_double /
                    (1024 as libc::c_int * 1024 as libc::c_int) as
                        libc::c_double);
        return buf
    }
    /* 100M - 999M */
    if val <
           (1024 as libc::c_int * 1024 as libc::c_int * 1000 as libc::c_int)
               as libc::c_ulonglong {
        sprintf(buf as *mut libc::c_char,
                b"%llu MB\x00" as *const u8 as *const libc::c_char,
                val.wrapping_div((1024 as libc::c_int * 1024 as libc::c_int)
                                     as libc::c_ulonglong));
        return buf
    }
    /* 1.00G - 9.99G */
    if (val as libc::c_double) <
           (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong) as libc::c_double *
               9.995f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.02f GB\x00" as *const u8 as *const libc::c_char,
                val as libc::c_double /
                    (1024 as libc::c_longlong *
                         1024 as libc::c_int as libc::c_longlong *
                         1024 as libc::c_int as libc::c_longlong) as
                        libc::c_double);
        return buf
    }
    /* 10.0G - 99.9G */
    if (val as libc::c_double) <
           (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong) as libc::c_double *
               99.95f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.01f GB\x00" as *const u8 as *const libc::c_char,
                val as libc::c_double /
                    (1024 as libc::c_longlong *
                         1024 as libc::c_int as libc::c_longlong *
                         1024 as libc::c_int as libc::c_longlong) as
                        libc::c_double);
        return buf
    }
    /* 100G - 999G */
    if val <
           (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong *
                1000 as libc::c_int as libc::c_longlong) as libc::c_ulonglong
       {
        sprintf(buf as *mut libc::c_char,
                b"%llu GB\x00" as *const u8 as *const libc::c_char,
                val.wrapping_div((1024 as libc::c_longlong *
                                      1024 as libc::c_int as libc::c_longlong
                                      *
                                      1024 as libc::c_int as libc::c_longlong)
                                     as libc::c_ulonglong));
        return buf
    }
    /* 1.00T - 9.99G */
    if (val as libc::c_double) <
           (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong *
                1024 as libc::c_int as libc::c_longlong) as libc::c_double *
               9.995f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.02f TB\x00" as *const u8 as *const libc::c_char,
                val as libc::c_double /
                    (1024 as libc::c_longlong *
                         1024 as libc::c_int as libc::c_longlong *
                         1024 as libc::c_int as libc::c_longlong *
                         1024 as libc::c_int as libc::c_longlong) as
                        libc::c_double);
        return buf
    }
    /* 10.0T - 99.9T */
    if (val as libc::c_double) <
           (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong *
                1024 as libc::c_int as libc::c_longlong) as libc::c_double *
               99.95f64 {
        sprintf(buf as *mut libc::c_char,
                b"%0.01f TB\x00" as *const u8 as *const libc::c_char,
                val as libc::c_double /
                    (1024 as libc::c_longlong *
                         1024 as libc::c_int as libc::c_longlong *
                         1024 as libc::c_int as libc::c_longlong *
                         1024 as libc::c_int as libc::c_longlong) as
                        libc::c_double);
        return buf
    }
    /* 100T+ */
    strcpy(buf as *mut libc::c_char,
           b"infty\x00" as *const u8 as *const libc::c_char);
    return buf;
}
/* Unsafe describe time delta as string.
   Returns a pointer to buf for convenience. */
/* Unsafe describe time delta as string.
   Returns a pointer to buf for convenience. */
#[no_mangle]
pub unsafe extern "C" fn u_stringify_time_diff(mut buf: *mut u8_0,
                                               mut cur_ms: u64_0,
                                               mut event_ms: u64_0)
 -> *mut u8_0 {
    let mut delta: u64_0 = 0;
    let mut t_d: s32 = 0;
    let mut t_h: s32 = 0;
    let mut t_m: s32 = 0;
    let mut t_s: s32 = 0;
    let mut val_buf: [u8_0; 16] = [0; 16];
    if event_ms == 0 {
        sprintf(buf as *mut libc::c_char,
                b"none seen yet\x00" as *const u8 as *const libc::c_char);
    } else {
        delta = cur_ms.wrapping_sub(event_ms);
        t_d =
            delta.wrapping_div(1000 as libc::c_int as
                                   libc::c_ulonglong).wrapping_div(60 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulonglong).wrapping_div(60
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulonglong).wrapping_div(24
                                                                                                                                               as
                                                                                                                                               libc::c_int
                                                                                                                                               as
                                                                                                                                               libc::c_ulonglong)
                as s32;
        t_h =
            delta.wrapping_div(1000 as libc::c_int as
                                   libc::c_ulonglong).wrapping_div(60 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulonglong).wrapping_div(60
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulonglong).wrapping_rem(24
                                                                                                                                               as
                                                                                                                                               libc::c_int
                                                                                                                                               as
                                                                                                                                               libc::c_ulonglong)
                as s32;
        t_m =
            delta.wrapping_div(1000 as libc::c_int as
                                   libc::c_ulonglong).wrapping_div(60 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulonglong).wrapping_rem(60
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulonglong)
                as s32;
        t_s =
            delta.wrapping_div(1000 as libc::c_int as
                                   libc::c_ulonglong).wrapping_rem(60 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulonglong)
                as s32;
        u_stringify_int(val_buf.as_mut_ptr(), t_d as u64_0);
        sprintf(buf as *mut libc::c_char,
                b"%s days, %d hrs, %d min, %d sec\x00" as *const u8 as
                    *const libc::c_char, val_buf.as_mut_ptr(), t_h, t_m, t_s);
    }
    return buf;
}
/* Wrapper for select() and read(), reading exactly len bytes.
  Returns the time passed to read.
  stop_soon should point to a variable indicating ctrl+c was pressed.
  If the wait times out, returns timeout_ms + 1;
  Returns 0 if an error occurred (fd closed, signal, ...); */
/* Wrapper for select() and read(), reading exactly len bytes.
  Returns the time passed to read.
  If the wait times out, returns timeout_ms + 1;
  Returns 0 if an error occurred (fd closed, signal, ...); */
#[no_mangle]
pub unsafe extern "C" fn read_timed(mut fd: s32, mut buf: *mut libc::c_void,
                                    mut len: size_t, mut timeout_ms: u32_0,
                                    mut stop_soon_p: *mut u8_0) -> u32_0 {
    let mut timeout: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut readfds: fd_set = fd_set{__fds_bits: [0; 16],};
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh13 = &mut __d0;
    let fresh14;
    let fresh15 = &mut __d1;
    let fresh16;
    let fresh17 =
        (::std::mem::size_of::<fd_set>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>() as
                                             libc::c_ulong);
    let fresh18 =
        &mut *readfds.__fds_bits.as_mut_ptr().offset(0 as libc::c_int as
                                                         isize) as
            *mut __fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh14), "={di}" (fresh16) : "{ax}"
         (0 as libc::c_int), "0"
         (c2rust_asm_casts::AsmCast::cast_in(fresh13, fresh17)), "1"
         (c2rust_asm_casts::AsmCast::cast_in(fresh15, fresh18)) : "memory" :
         "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh13, fresh17, fresh14);
    c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh18, fresh16);
    readfds.__fds_bits[(fd /
                            (8 as libc::c_int *
                                 ::std::mem::size_of::<__fd_mask>() as
                                     libc::c_ulong as libc::c_int)) as usize]
        |=
        ((1 as libc::c_ulong) <<
             fd %
                 (8 as libc::c_int *
                      ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as
                          libc::c_int)) as __fd_mask;
    timeout.tv_sec =
        timeout_ms.wrapping_div(1000 as libc::c_int as libc::c_uint) as
            __time_t;
    timeout.tv_usec =
        timeout_ms.wrapping_rem(1000 as libc::c_int as
                                    libc::c_uint).wrapping_mul(1000 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
            as __suseconds_t;
    let mut read_total: size_t = 0 as libc::c_int as size_t;
    let mut len_read: size_t = 0 as libc::c_int as size_t;
    while len_read < len {
        /* set exceptfds as well to return when a child exited/closed the pipe. */
        let mut sret: libc::c_int =
            select(fd + 1 as libc::c_int, &mut readfds, 0 as *mut fd_set,
                   0 as *mut fd_set, &mut timeout);
        if sret == 0 {
            // printf("Timeout in sret.");
            return timeout_ms.wrapping_add(1 as libc::c_int as libc::c_uint)
        } else if sret < 0 as libc::c_int {
            /* Retry select for all signals other than than ctrl+c */
            if *__errno_location() == 4 as libc::c_int && *stop_soon_p == 0 {
                continue ;
            }
            return 0 as libc::c_int as u32_0
        } else {
            len_read =
                read(fd,
                     (buf as *mut u8_0).offset(len_read as isize) as
                         *mut libc::c_void, len.wrapping_sub(len_read)) as
                    size_t;
            if len_read == 0 { return 0 as libc::c_int as u32_0 }
            read_total =
                (read_total as libc::c_ulong).wrapping_add(len_read) as size_t
                    as size_t
        }
    }
    let mut exec_ms: s32 =
        ({
             let mut _a: u32_0 = timeout_ms;
             let mut _b: libc::c_ulonglong =
                 (timeout_ms as
                      u64_0).wrapping_sub((timeout.tv_sec *
                                               1000 as libc::c_int as
                                                   libc::c_long +
                                               timeout.tv_usec /
                                                   1000 as libc::c_int as
                                                       libc::c_long) as
                                              libc::c_ulonglong);
             if (_a as libc::c_ulonglong) < _b {
                 _a as libc::c_ulonglong
             } else { _b }
         }) as s32;
    return if exec_ms > 0 as libc::c_int { exec_ms } else { 1 as libc::c_int }
               as u32_0;
    // at least 1 milli must have passed (0 is an error)
}
