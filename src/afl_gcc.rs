use ::libc;
extern "C" {
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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn access(__name: *const libc::c_char, __type: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type u8_0 = uint8_t;
pub type u32_0 = uint32_t;
pub type s32 = int32_t;
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
   american fuzzy lop++ - wrapper for GCC and clang
   ------------------------------------------------

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

   This program is a drop-in replacement for GCC or clang. The most common way
   of using it is to pass the path to afl-gcc or afl-clang via CC when invoking
   ./configure.

   (Of course, use CXX and point it to afl-g++ / afl-clang++ for C++ code.)

   The wrapper needs to know the path to afl-as (renamed to 'as'). The default
   is /usr/local/lib/afl/. A convenient way to specify alternative directories
   would be to set AFL_PATH.

   If AFL_HARDEN is set, the wrapper will compile the target app with various
   hardening options that may help detect memory management issues more
   reliably. You can also specify AFL_USE_ASAN to enable ASAN.

   If you want to call a non-default compiler as a next step of the chain,
   specify its location via AFL_CC or AFL_CXX.

 */
static mut as_path: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
/* Path to the AFL 'as' wrapper      */
static mut cc_params: *mut *mut u8_0 =
    0 as *const *mut u8_0 as *mut *mut u8_0;
/* Parameters passed to the real CC  */
static mut cc_par_cnt: u32_0 = 1 as libc::c_int as u32_0;
/* Param count, including argv0      */
static mut be_quiet: u8_0 = 0;
/* Quiet mode                        */
static mut clang_mode: u8_0 = 0;
/* Invoked as afl-clang*?            */
/* Try to find our "fake" GNU assembler in AFL_PATH or at the location derived
   from argv[0]. If that fails, abort. */
unsafe extern "C" fn find_as(mut argv0: *mut u8_0) {
    let mut afl_path: *mut u8_0 =
        getenv(b"AFL_PATH\x00" as *const u8 as *const libc::c_char) as
            *mut u8_0;
    let mut slash: *mut u8_0 = 0 as *mut u8_0;
    let mut tmp: *mut u8_0 = 0 as *mut u8_0;
    if !afl_path.is_null() {
        tmp =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/as\x00" as *const u8 as
                                  *const libc::c_char, afl_path);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-gcc.c\x00" as *const u8 as
                                *const libc::c_char, 67 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/as\x00" as *const u8 as *const libc::c_char,
                          afl_path);
                 _tmp
             });
        if access(tmp as *const libc::c_char, 1 as libc::c_int) == 0 {
            as_path = afl_path;
            DFL_ck_free(tmp as *mut libc::c_void);
            return
        }
        DFL_ck_free(tmp as *mut libc::c_void);
    }
    slash = strrchr(argv0 as *const libc::c_char, '/' as i32) as *mut u8_0;
    if !slash.is_null() {
        let mut dir: *mut u8_0 = 0 as *mut u8_0;
        *slash = 0 as libc::c_int as u8_0;
        dir = DFL_ck_strdup(argv0);
        *slash = '/' as i32 as u8_0;
        tmp =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%s/afl-as\x00" as *const u8 as
                                  *const libc::c_char, dir);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-gcc.c\x00" as *const u8 as
                                *const libc::c_char, 91 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%s/afl-as\x00" as *const u8 as
                              *const libc::c_char, dir);
                 _tmp
             });
        if access(tmp as *const libc::c_char, 1 as libc::c_int) == 0 {
            as_path = dir;
            DFL_ck_free(tmp as *mut libc::c_void);
            return
        }
        DFL_ck_free(tmp as *mut libc::c_void);
        DFL_ck_free(dir as *mut libc::c_void);
    }
    if access(b"/usr/local/lib/afl/as\x00" as *const u8 as
                  *const libc::c_char, 1 as libc::c_int) == 0 {
        as_path =
            b"/usr/local/lib/afl\x00" as *const u8 as *const libc::c_char as
                *mut u8_0;
        return
    }
    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to find AFL wrapper binary for \'as\'. Please set AFL_PATH\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00" as
               *const u8 as *const libc::c_char,
           b"func_unknown\x00" as *const u8 as *const libc::c_char,
           b"src/afl-gcc.c\x00" as *const u8 as *const libc::c_char,
           113 as libc::c_int);
    exit(1 as libc::c_int);
}
/* Copy argv to cc_params, making the necessary edits. */
unsafe extern "C" fn edit_params(mut argc: u32_0,
                                 mut argv: *mut *mut libc::c_char) {
    let mut fortify_set: u8_0 = 0 as libc::c_int as u8_0;
    let mut asan_set: u8_0 = 0 as libc::c_int as u8_0;
    let mut name: *mut u8_0 = 0 as *mut u8_0;
    cc_params =
        DFL_ck_alloc((argc.wrapping_add(128 as libc::c_int as libc::c_uint) as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut u8_0>()
                                                          as libc::c_ulong) as
                         u32_0) as *mut *mut u8_0;
    name =
        strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32) as
            *mut u8_0;
    if name.is_null() {
        name = *argv.offset(0 as libc::c_int as isize) as *mut u8_0
    } else { name = name.offset(1) }
    if strncmp(name as *const libc::c_char,
               b"afl-clang\x00" as *const u8 as *const libc::c_char,
               9 as libc::c_int as libc::c_ulong) == 0 {
        clang_mode = 1 as libc::c_int as u8_0;
        setenv(b"__AFL_CLANG_MODE\x00" as *const u8 as *const libc::c_char,
               b"1\x00" as *const u8 as *const libc::c_char,
               1 as libc::c_int);
        if strcmp(name as *const libc::c_char,
                  b"afl-clang++\x00" as *const u8 as *const libc::c_char) == 0
           {
            let mut alt_cxx: *mut u8_0 =
                getenv(b"AFL_CXX\x00" as *const u8 as *const libc::c_char) as
                    *mut u8_0;
            let ref mut fresh0 = *cc_params.offset(0 as libc::c_int as isize);
            *fresh0 =
                if !alt_cxx.is_null() && *alt_cxx as libc::c_int != 0 {
                    alt_cxx
                } else {
                    b"clang++\x00" as *const u8 as *const libc::c_char as
                        *mut u8_0
                }
        } else {
            let mut alt_cc: *mut u8_0 =
                getenv(b"AFL_CC\x00" as *const u8 as *const libc::c_char) as
                    *mut u8_0;
            let ref mut fresh1 = *cc_params.offset(0 as libc::c_int as isize);
            *fresh1 =
                if !alt_cc.is_null() && *alt_cc as libc::c_int != 0 {
                    alt_cc
                } else {
                    b"clang\x00" as *const u8 as *const libc::c_char as
                        *mut u8_0
                }
        }
    } else if strcmp(name as *const libc::c_char,
                     b"afl-g++\x00" as *const u8 as *const libc::c_char) == 0
     {
        let mut alt_cxx_0: *mut u8_0 =
            getenv(b"AFL_CXX\x00" as *const u8 as *const libc::c_char) as
                *mut u8_0;
        let ref mut fresh2 = *cc_params.offset(0 as libc::c_int as isize);
        *fresh2 =
            if !alt_cxx_0.is_null() && *alt_cxx_0 as libc::c_int != 0 {
                alt_cxx_0
            } else {
                b"g++\x00" as *const u8 as *const libc::c_char as *mut u8_0
            }
    } else if strcmp(name as *const libc::c_char,
                     b"afl-gcj\x00" as *const u8 as *const libc::c_char) == 0
     {
        let mut alt_cc_0: *mut u8_0 =
            getenv(b"AFL_GCJ\x00" as *const u8 as *const libc::c_char) as
                *mut u8_0;
        let ref mut fresh3 = *cc_params.offset(0 as libc::c_int as isize);
        *fresh3 =
            if !alt_cc_0.is_null() && *alt_cc_0 as libc::c_int != 0 {
                alt_cc_0
            } else {
                b"gcj\x00" as *const u8 as *const libc::c_char as *mut u8_0
            }
    } else {
        let mut alt_cc_1: *mut u8_0 =
            getenv(b"AFL_CC\x00" as *const u8 as *const libc::c_char) as
                *mut u8_0;
        let ref mut fresh4 = *cc_params.offset(0 as libc::c_int as isize);
        *fresh4 =
            if !alt_cc_1.is_null() && *alt_cc_1 as libc::c_int != 0 {
                alt_cc_1
            } else {
                b"gcc\x00" as *const u8 as *const libc::c_char as *mut u8_0
            }
    }
    loop  {
        argc = argc.wrapping_sub(1);
        if !(argc != 0) { break ; }
        argv = argv.offset(1);
        let mut cur: *mut u8_0 = *argv as *mut u8_0;
        if strncmp(cur as *const libc::c_char,
                   b"-B\x00" as *const u8 as *const libc::c_char,
                   2 as libc::c_int as libc::c_ulong) == 0 {
            if be_quiet == 0 {
                printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0m-B is already set, overriding\x00"
                           as *const u8 as *const libc::c_char);
                printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
            }
            if *cur.offset(2 as libc::c_int as isize) == 0 &&
                   argc > 1 as libc::c_int as libc::c_uint {
                argc = argc.wrapping_sub(1);
                argv = argv.offset(1)
            }
        } else {
            if strcmp(cur as *const libc::c_char,
                      b"-integrated-as\x00" as *const u8 as
                          *const libc::c_char) == 0 {
                continue ;
            }
            if strcmp(cur as *const libc::c_char,
                      b"-pipe\x00" as *const u8 as *const libc::c_char) == 0 {
                continue ;
            }
            if strcmp(cur as *const libc::c_char,
                      b"-fsanitize=address\x00" as *const u8 as
                          *const libc::c_char) == 0 ||
                   strcmp(cur as *const libc::c_char,
                          b"-fsanitize=memory\x00" as *const u8 as
                              *const libc::c_char) == 0 {
                asan_set = 1 as libc::c_int as u8_0
            }
            if !strstr(cur as *const libc::c_char,
                       b"FORTIFY_SOURCE\x00" as *const u8 as
                           *const libc::c_char).is_null() {
                fortify_set = 1 as libc::c_int as u8_0
            }
            let fresh5 = cc_par_cnt;
            cc_par_cnt = cc_par_cnt.wrapping_add(1);
            let ref mut fresh6 = *cc_params.offset(fresh5 as isize);
            *fresh6 = cur
        }
    }
    let fresh7 = cc_par_cnt;
    cc_par_cnt = cc_par_cnt.wrapping_add(1);
    let ref mut fresh8 = *cc_params.offset(fresh7 as isize);
    *fresh8 = b"-B\x00" as *const u8 as *const libc::c_char as *mut u8_0;
    let fresh9 = cc_par_cnt;
    cc_par_cnt = cc_par_cnt.wrapping_add(1);
    let ref mut fresh10 = *cc_params.offset(fresh9 as isize);
    *fresh10 = as_path;
    if clang_mode != 0 {
        let fresh11 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh12 = *cc_params.offset(fresh11 as isize);
        *fresh12 =
            b"-no-integrated-as\x00" as *const u8 as *const libc::c_char as
                *mut u8_0
    }
    if !getenv(b"AFL_HARDEN\x00" as *const u8 as
                   *const libc::c_char).is_null() {
        let fresh13 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh14 = *cc_params.offset(fresh13 as isize);
        *fresh14 =
            b"-fstack-protector-all\x00" as *const u8 as *const libc::c_char
                as *mut u8_0;
        if fortify_set == 0 {
            let fresh15 = cc_par_cnt;
            cc_par_cnt = cc_par_cnt.wrapping_add(1);
            let ref mut fresh16 = *cc_params.offset(fresh15 as isize);
            *fresh16 =
                b"-D_FORTIFY_SOURCE=2\x00" as *const u8 as *const libc::c_char
                    as *mut u8_0
        }
    }
    if asan_set != 0 {
        /* With GCJ and Eclipse installed, you can actually compile Java! The
       instrumentation will work (amazingly). Alas, unhandled exceptions do
       not call abort(), so afl-fuzz would need to be modified to equate
       non-zero exit codes with crash conditions when working with Java
       binaries. Meh. */
        /* __APPLE__ */
        /* Pass this on to afl-as to adjust map density. */
        setenv(b"AFL_USE_ASAN\x00" as *const u8 as *const libc::c_char,
               b"1\x00" as *const u8 as *const libc::c_char,
               1 as libc::c_int);
    } else if !getenv(b"AFL_USE_ASAN\x00" as *const u8 as
                          *const libc::c_char).is_null() {
        if !getenv(b"AFL_USE_MSAN\x00" as *const u8 as
                       *const libc::c_char).is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mASAN and MSAN are mutually exclusive\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-gcc.c\x00" as *const u8 as *const libc::c_char,
                   265 as libc::c_int);
            exit(1 as libc::c_int);
        }
        if !getenv(b"AFL_HARDEN\x00" as *const u8 as
                       *const libc::c_char).is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mASAN and AFL_HARDEN are mutually exclusive\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-gcc.c\x00" as *const u8 as *const libc::c_char,
                   268 as libc::c_int);
            exit(1 as libc::c_int);
        }
        let fresh17 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh18 = *cc_params.offset(fresh17 as isize);
        *fresh18 =
            b"-U_FORTIFY_SOURCE\x00" as *const u8 as *const libc::c_char as
                *mut u8_0;
        let fresh19 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh20 = *cc_params.offset(fresh19 as isize);
        *fresh20 =
            b"-fsanitize=address\x00" as *const u8 as *const libc::c_char as
                *mut u8_0
    } else if !getenv(b"AFL_USE_MSAN\x00" as *const u8 as
                          *const libc::c_char).is_null() {
        if !getenv(b"AFL_USE_ASAN\x00" as *const u8 as
                       *const libc::c_char).is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mASAN and MSAN are mutually exclusive\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-gcc.c\x00" as *const u8 as *const libc::c_char,
                   275 as libc::c_int);
            exit(1 as libc::c_int);
        }
        if !getenv(b"AFL_HARDEN\x00" as *const u8 as
                       *const libc::c_char).is_null() {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mMSAN and AFL_HARDEN are mutually exclusive\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-gcc.c\x00" as *const u8 as *const libc::c_char,
                   278 as libc::c_int);
            exit(1 as libc::c_int);
        }
        let fresh21 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh22 = *cc_params.offset(fresh21 as isize);
        *fresh22 =
            b"-U_FORTIFY_SOURCE\x00" as *const u8 as *const libc::c_char as
                *mut u8_0;
        let fresh23 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh24 = *cc_params.offset(fresh23 as isize);
        *fresh24 =
            b"-fsanitize=memory\x00" as *const u8 as *const libc::c_char as
                *mut u8_0
    }
    if !getenv(b"AFL_USE_UBSAN\x00" as *const u8 as
                   *const libc::c_char).is_null() {
        let fresh25 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh26 = *cc_params.offset(fresh25 as isize);
        *fresh26 =
            b"-fsanitize=undefined\x00" as *const u8 as *const libc::c_char as
                *mut u8_0;
        let fresh27 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh28 = *cc_params.offset(fresh27 as isize);
        *fresh28 =
            b"-fsanitize-undefined-trap-on-error\x00" as *const u8 as
                *const libc::c_char as *mut u8_0;
        let fresh29 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh30 = *cc_params.offset(fresh29 as isize);
        *fresh30 =
            b"-fno-sanitize-recover=all\x00" as *const u8 as
                *const libc::c_char as *mut u8_0
    }
    if getenv(b"AFL_DONT_OPTIMIZE\x00" as *const u8 as
                  *const libc::c_char).is_null() {
        let fresh31 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh32 = *cc_params.offset(fresh31 as isize);
        *fresh32 = b"-g\x00" as *const u8 as *const libc::c_char as *mut u8_0;
        let fresh33 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh34 = *cc_params.offset(fresh33 as isize);
        *fresh34 =
            b"-O3\x00" as *const u8 as *const libc::c_char as *mut u8_0;
        let fresh35 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh36 = *cc_params.offset(fresh35 as isize);
        *fresh36 =
            b"-funroll-loops\x00" as *const u8 as *const libc::c_char as
                *mut u8_0;
        /* Two indicators that you're building for fuzzing; one of them is
       AFL-specific, the other is shared with libfuzzer. */
        let fresh37 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh38 = *cc_params.offset(fresh37 as isize);
        *fresh38 =
            b"-D__AFL_COMPILER=1\x00" as *const u8 as *const libc::c_char as
                *mut u8_0;
        let fresh39 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh40 = *cc_params.offset(fresh39 as isize);
        *fresh40 =
            b"-DFUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION=1\x00" as *const u8
                as *const libc::c_char as *mut u8_0
    }
    if !getenv(b"AFL_NO_BUILTIN\x00" as *const u8 as
                   *const libc::c_char).is_null() {
        let fresh41 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh42 = *cc_params.offset(fresh41 as isize);
        *fresh42 =
            b"-fno-builtin-strcmp\x00" as *const u8 as *const libc::c_char as
                *mut u8_0;
        let fresh43 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh44 = *cc_params.offset(fresh43 as isize);
        *fresh44 =
            b"-fno-builtin-strncmp\x00" as *const u8 as *const libc::c_char as
                *mut u8_0;
        let fresh45 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh46 = *cc_params.offset(fresh45 as isize);
        *fresh46 =
            b"-fno-builtin-strcasecmp\x00" as *const u8 as *const libc::c_char
                as *mut u8_0;
        let fresh47 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh48 = *cc_params.offset(fresh47 as isize);
        *fresh48 =
            b"-fno-builtin-strncasecmp\x00" as *const u8 as
                *const libc::c_char as *mut u8_0;
        let fresh49 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh50 = *cc_params.offset(fresh49 as isize);
        *fresh50 =
            b"-fno-builtin-memcmp\x00" as *const u8 as *const libc::c_char as
                *mut u8_0;
        let fresh51 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh52 = *cc_params.offset(fresh51 as isize);
        *fresh52 =
            b"-fno-builtin-bcmp\x00" as *const u8 as *const libc::c_char as
                *mut u8_0;
        let fresh53 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh54 = *cc_params.offset(fresh53 as isize);
        *fresh54 =
            b"-fno-builtin-strstr\x00" as *const u8 as *const libc::c_char as
                *mut u8_0;
        let fresh55 = cc_par_cnt;
        cc_par_cnt = cc_par_cnt.wrapping_add(1);
        let ref mut fresh56 = *cc_params.offset(fresh55 as isize);
        *fresh56 =
            b"-fno-builtin-strcasestr\x00" as *const u8 as *const libc::c_char
                as *mut u8_0
    }
    let ref mut fresh57 = *cc_params.offset(cc_par_cnt as isize);
    *fresh57 = 0 as *mut u8_0;
}
/* Main entry point */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut env_info: *mut libc::c_char =
        b"Environment variables used by afl-gcc:\nAFL_CC: path to the C compiler to use\nAFL_CXX: path to the C++ compiler to use\nAFL_GCJ: path to the java compiler to use\nAFL_PATH: path to the instrumenting assembler\nAFL_DONT_OPTIMIZE: disable optimization instead of -O3\nAFL_NO_BUILTIN: compile for use with libtokencap.so\nAFL_QUIET: suppress verbose output\nAFL_CAL_FAST: speed up the initial calibration\nAFL_HARDEN: adds code hardening to catch memory bugs\nAFL_USE_ASAN: activate address sanitizer\nAFL_USE_MSAN: activate memory sanitizer\nAFL_USE_UBSAN: activate undefined behaviour sanitizer\n\nEnvironment variables used by afl-as (called by afl-gcc):\nAFL_AS: path to the assembler to use\nTMPDIR: set the directory for temporary files of afl-as\nTEMP: fall back path to directory for temporary files\nTMP: fall back path to directory for temporary files\nAFL_INST_RATIO: percentage of branches to instrument\nAFL_QUIET: suppress verbose output\nAFL_KEEP_ASSEMBLY: leave instrumented assembly files\nAFL_AS_FORCE_INSTRUMENT: force instrumentation for asm sources\n\x00"
            as *const u8 as *const libc::c_char as *mut libc::c_char;
    if argc == 2 as libc::c_int &&
           strcmp(*argv.offset(1 as libc::c_int as isize),
                  b"-h\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        printf(b"afl-cc++2.63d by Michal Zalewski\n\n\x00" as *const u8 as
                   *const libc::c_char);
        printf(b"%s \n\n\x00" as *const u8 as *const libc::c_char,
               *argv.offset(0 as libc::c_int as isize));
        printf(b"afl-gcc has no command line options\n\n%s\n\x00" as *const u8
                   as *const libc::c_char, env_info);
        printf(b"NOTE: afl-gcc is deprecated, llvm_mode is much faster and has more options\n\x00"
                   as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if isatty(2 as libc::c_int) != 0 &&
           getenv(b"AFL_QUIET\x00" as *const u8 as
                      *const libc::c_char).is_null() ||
           !getenv(b"AFL_DEBUG\x00" as *const u8 as
                       *const libc::c_char).is_null() {
        printf(b"\x1b[0;36mafl-cc++2.63d\x1b[0m by Michal Zalewski\n\x00" as
                   *const u8 as *const libc::c_char);
        printf(b"\x1b[1;93m[!] \x1b[1;97mNOTE: \x1b[0mafl-gcc is deprecated, llvm_mode is much faster and has more options\n\x00"
                   as *const u8 as *const libc::c_char);
    } else { be_quiet = 1 as libc::c_int as u8_0 }
    if argc < 2 as libc::c_int {
        printf(b"\nThis is a helper application for afl-fuzz. It serves as a drop-in replacement\nfor gcc or clang, letting you recompile third-party code with the required\nruntime instrumentation. A common use pattern would be one of the following:\n\n  CC=%s/afl-gcc ./configure\n  CXX=%s/afl-g++ ./configure\n\n%s\x00"
                   as *const u8 as *const libc::c_char,
               b"/usr/local/bin\x00" as *const u8 as *const libc::c_char,
               b"/usr/local/bin\x00" as *const u8 as *const libc::c_char,
               env_info);
        exit(1 as libc::c_int);
    }
    find_as(*argv.offset(0 as libc::c_int as isize) as *mut u8_0);
    edit_params(argc as u32_0, argv);
    execvp(*cc_params.offset(0 as libc::c_int as isize) as
               *const libc::c_char,
           cc_params as *mut *mut libc::c_char as *const *mut libc::c_char);
    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mOops, failed to execute \'%s\' - check your PATH\x00"
               as *const u8 as *const libc::c_char,
           *cc_params.offset(0 as libc::c_int as isize));
    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00" as
               *const u8 as *const libc::c_char,
           b"func_unknown\x00" as *const u8 as *const libc::c_char,
           b"src/afl-gcc.c\x00" as *const u8 as *const libc::c_char,
           420 as libc::c_int);
    exit(1 as libc::c_int);
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
