use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn execv(__path: *const libc::c_char, __argv: *const *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn setsid() -> __pid_t;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    static mut be_quiet: u8_0;
    #[no_mangle]
    static mut doc_path: *mut u8_0;
    /* Describe integer. The buf should be
   at least 6 bytes to fit all ints we randomly see.
   Will return buf for convenience. */
    #[no_mangle]
    fn stringify_int(buf: *mut u8_0, len: size_t, val: u64_0) -> *mut u8_0;
    /* Describe integer as memory size. */
    #[no_mangle]
    fn stringify_mem_size(buf: *mut u8_0, len: size_t, val: u64_0)
     -> *mut u8_0;
    /* Wrapper for select() and read(), reading exactly len bytes.
  Returns the time passed to read.
  stop_soon should point to a variable indicating ctrl+c was pressed.
  If the wait times out, returns timeout_ms + 1;
  Returns 0 if an error occurred (fd closed, signal, ...); */
    #[no_mangle]
    fn read_timed(fd: s32, buf: *mut libc::c_void, len: size_t,
                  timeout_ms: u32_0, stop_soon_p: *mut u8_0) -> u32_0;
    #[no_mangle]
    fn signal(__sig: libc::c_int, __handler: __sighandler_t)
     -> __sighandler_t;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
               __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit)
     -> libc::c_int;
    #[no_mangle]
    fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit)
     -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __rlim_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
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
pub type element_t = list_element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_element {
    pub pre_status: pre_status_t,
    pub prev: *mut list_element,
    pub next: *mut list_element,
    pub data: *mut libc::c_void,
}
pub type pre_status_t = prealloc_status;
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
pub struct list {
    pub element_prealloc_buf: [element_t; 64],
    pub element_prealloc_count: s32,
}
pub type list_t = list;
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
pub type afl_forkserver_t = afl_forkserver;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type rlim_t = __rlim_t;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub type __rlimit_resource_t = libc::c_int;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
/* a program that includes afl-forkserver needs to define these */
/* Target uses ASAN?                */
/* SHM with instrumentation bitmap  */
/* use stdin for sending data       */
/* PID of the fork server           */
/* PID of the fuzzed program        */
/* FD of the lock file              */
/* Persistent fd for fsrv->out_file */
/* Persistent fd for /dev/urandom   */
/* Persistent fd for /dev/null      */
/* Fork server control pipe (write) */
/* Fork server status pipe (read)   */
/* Configurable exec timeout (ms)   */
/* map size used by the target      */
/* is snapshot feature used         */
/* Memory cap for child (MB)        */
/* File to fuzz, if any             */
/* Path of the target */
/* Gnuplot output file              */
/* Traced process timed out?        */
/* Fauxsrv for non-forking targets? */
/* if prev forkserver run timed out */
/* if running in qemu mode or not   */
/* the name of the cmplog binary    */
/* Function to kick off the forkserver child */
/* for autodictionary: afl ptr      */
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
#[inline]
unsafe extern "C" fn DFL_ck_free(mut mem: *mut libc::c_void) {
    if mem.is_null() { return }
    free(mem);
}
/*
   american fuzzy lop++ - forkserver code
   --------------------------------------

   Originally written by Michal Zalewski

   Forkserver design by Jann Horn <jannhorn@googlemail.com>

   Now maintained by Marc Heuse <mh@mh-sec.de>,
                        Heiko Eißfeldt <heiko.eissfeldt@hexco.de> and
                        Andrea Fioraldi <andreafioraldi@gmail.com>

   Copyright 2016, 2017 Google Inc. All rights reserved.
   Copyright 2019-2020 AFLplusplus Project. All rights reserved.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at:

     http://www.apache.org/licenses/LICENSE-2.0

   Shared code that implements a forkserver. This is used by the fuzzer
   as well the other components like afl-tmin.

 */
/* *
 * The correct fds for reading and writing pipes
 */
/* Describe integer as memory size. */
#[no_mangle]
pub static mut fsrv_list: list_t =
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
unsafe extern "C" fn fsrv_exec_child(mut fsrv: *mut afl_forkserver_t,
                                     mut argv: *mut *mut libc::c_char) {
    execv((*fsrv).target_path as *const libc::c_char,
          argv as *const *mut libc::c_char);
}
/* Initializes the struct */
#[no_mangle]
pub unsafe extern "C" fn afl_fsrv_init(mut fsrv: *mut afl_forkserver_t) {
    // this structure needs default so we initialize it if this was not done
  // already
    (*fsrv).use_stdin = 1 as libc::c_int as u8_0;
    (*fsrv).out_fd = -(1 as libc::c_int);
    (*fsrv).out_dir_fd = -(1 as libc::c_int);
    (*fsrv).dev_null_fd = -(1 as libc::c_int);
    (*fsrv).dev_urandom_fd = -(1 as libc::c_int);
    (*fsrv).exec_tmout = 1000 as libc::c_int as u32_0;
    (*fsrv).mem_limit = 50 as libc::c_int as u64_0;
    (*fsrv).child_pid = -(1 as libc::c_int);
    (*fsrv).map_size = ((1 as libc::c_int) << 16 as libc::c_int) as u32_0;
    (*fsrv).use_fauxsrv = 0 as libc::c_int as u8_0;
    (*fsrv).prev_timed_out = 0 as libc::c_int as u32_0;
    (*fsrv).init_child_func =
        Some(fsrv_exec_child as
                 unsafe extern "C" fn(_: *mut afl_forkserver_t,
                                      _: *mut *mut libc::c_char) -> ());
    list_append(&mut fsrv_list, fsrv as *mut libc::c_void);
}
/* Initialize a new forkserver instance, duplicating "global" settings */
#[no_mangle]
pub unsafe extern "C" fn afl_fsrv_init_dup(mut fsrv_to: *mut afl_forkserver_t,
                                           mut from: *mut afl_forkserver_t) {
    (*fsrv_to).use_stdin = (*from).use_stdin;
    (*fsrv_to).dev_null_fd = (*from).dev_null_fd;
    (*fsrv_to).exec_tmout = (*from).exec_tmout;
    (*fsrv_to).mem_limit = (*from).mem_limit;
    (*fsrv_to).map_size = (*from).map_size;
    (*fsrv_to).dev_urandom_fd = (*from).dev_urandom_fd;
    // These are forkserver specific.
    (*fsrv_to).out_fd = -(1 as libc::c_int);
    (*fsrv_to).out_dir_fd = -(1 as libc::c_int);
    (*fsrv_to).child_pid = -(1 as libc::c_int);
    (*fsrv_to).use_fauxsrv = 0 as libc::c_int as u8_0;
    (*fsrv_to).prev_timed_out = 0 as libc::c_int as u32_0;
    (*fsrv_to).init_child_func =
        Some(fsrv_exec_child as
                 unsafe extern "C" fn(_: *mut afl_forkserver_t,
                                      _: *mut *mut libc::c_char) -> ());
    list_append(&mut fsrv_list, fsrv_to as *mut libc::c_void);
}
/* Internal forkserver for dumb_mode=1 and non-forkserver mode runs.
  It execvs for each fork, forwarding exit codes and child pids to afl. */
unsafe extern "C" fn afl_fauxsrv_execv(mut fsrv: *mut afl_forkserver_t,
                                       mut argv: *mut *mut libc::c_char) {
    let mut tmp: [libc::c_uchar; 4] =
        [0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
         0 as libc::c_int as libc::c_uchar,
         0 as libc::c_int as libc::c_uchar];
    let mut child_pid: pid_t = -(1 as libc::c_int);
    if be_quiet == 0 {
        printf(b"\x1b[1;94m[*] \x1b[0mUsing Fauxserver:\x00" as *const u8 as
                   *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    /* Phone home and tell the parent that we're OK. If parent isn't there,
     assume we're not running in forkserver mode and just execute program. */
    if write(198 as libc::c_int + 1 as libc::c_int,
             tmp.as_mut_ptr() as *const libc::c_void,
             4 as libc::c_int as size_t) != 4 as libc::c_int as libc::c_long {
        abort(); // TODO: Abort?
    }
    let mut old_sigchld_handler:
            Option<unsafe extern "C" fn(_: libc::c_int) -> ()> =
        signal(17 as libc::c_int, None);
    loop  {
        let mut was_killed: uint32_t = 0;
        let mut status: libc::c_int = 0;
        /* Wait for parent by reading from the pipe. Exit if read fails. */
        if read(198 as libc::c_int,
                &mut was_killed as *mut uint32_t as *mut libc::c_void,
                4 as libc::c_int as size_t) !=
               4 as libc::c_int as libc::c_long {
            exit(0 as libc::c_int);
        }
        /* Create a clone of our process. */
        child_pid = fork();
        if child_pid < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mFork failed\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-forkserver.c\x00" as *const u8 as
                       *const libc::c_char, 143 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        /* In child process: close fds, resume execution. */
        if child_pid == 0 {
            // New child
            signal(17 as libc::c_int, old_sigchld_handler);
            // FORKSRV_FD is for communication with AFL, we don't need it in the
      // child.
            close(198 as libc::c_int);
            close(198 as libc::c_int + 1 as libc::c_int);
            // TODO: exec...
            execv((*fsrv).target_path as *const libc::c_char,
                  argv as *const *mut libc::c_char);
            /* Use a distinctive bitmap signature to tell the parent about execv()
        falling through. */
            *((*fsrv).trace_bits as *mut u32_0) = 0xfee1dead as libc::c_uint;
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mExecv failed in fauxserver.\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-forkserver.c\x00" as *const u8 as
                       *const libc::c_char, 164 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        /* In parent process: write PID to AFL. */
        if write(198 as libc::c_int + 1 as libc::c_int,
                 &mut child_pid as *mut pid_t as *const libc::c_void,
                 4 as libc::c_int as size_t) !=
               4 as libc::c_int as libc::c_long {
            exit(0 as libc::c_int);
        }
        /* after child exited, get and relay exit status to parent through waitpid.
     */
        if waitpid(child_pid, &mut status, 0 as libc::c_int) <
               0 as libc::c_int {
            // Zombie Child could not be collected. Scary!
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mFauxserver could not determin child\'s exit code. \x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-forkserver.c\x00" as *const u8 as
                       *const libc::c_char, 178 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        /* Relay wait status to AFL pipe, then loop back. */
        if write(198 as libc::c_int + 1 as libc::c_int,
                 &mut status as *mut libc::c_int as *const libc::c_void,
                 4 as libc::c_int as size_t) !=
               4 as libc::c_int as libc::c_long {
            exit(0 as libc::c_int);
        }
    };
}
/* Spins up fork server (instrumented mode only). The idea is explained here:

   http://lcamtuf.blogspot.com/2014/10/fuzzing-binaries-without-execve.html

   In essence, the instrumentation allows us to skip execve(), and just keep
   cloning a stopped child. So, we just execute once, and then send commands
   through a pipe. The other part of this logic is in afl-as.h / llvm_mode */
#[no_mangle]
pub unsafe extern "C" fn afl_fsrv_start(mut fsrv: *mut afl_forkserver_t,
                                        mut argv: *mut *mut libc::c_char,
                                        mut stop_soon_p: *mut u8_0,
                                        mut debug_child_output: u8_0) {
    let mut st_pipe: [libc::c_int; 2] = [0; 2];
    let mut ctl_pipe: [libc::c_int; 2] = [0; 2];
    let mut status: libc::c_int = 0;
    let mut rlen: s32 = 0;
    if be_quiet == 0 {
        printf(b"\x1b[1;94m[*] \x1b[0mSpinning up the fork server...\x00" as
                   *const u8 as *const libc::c_char);
        printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
    }
    if (*fsrv).use_fauxsrv != 0 {
        /* TODO: Come up with sone nice way to initalize this all */
        if (*fsrv).init_child_func !=
               Some(fsrv_exec_child as
                        unsafe extern "C" fn(_: *mut afl_forkserver_t,
                                             _: *mut *mut libc::c_char) -> ())
           {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mDifferent forkserver not compatible with fauxserver\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-forkserver.c\x00" as *const u8 as
                       *const libc::c_char, 212 as libc::c_int);
            exit(1 as libc::c_int);
        }
        (*fsrv).init_child_func =
            Some(afl_fauxsrv_execv as
                     unsafe extern "C" fn(_: *mut afl_forkserver_t,
                                          _: *mut *mut libc::c_char) -> ())
    }
    if pipe(st_pipe.as_mut_ptr()) != 0 || pipe(ctl_pipe.as_mut_ptr()) != 0 {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mpipe() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-forkserver.c\x00" as *const u8 as
                   *const libc::c_char, 218 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    (*fsrv).child_timed_out = 0 as libc::c_int as u8_0;
    (*fsrv).fsrv_pid = fork();
    if (*fsrv).fsrv_pid < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mfork() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-forkserver.c\x00" as *const u8 as
                   *const libc::c_char, 223 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    if (*fsrv).fsrv_pid == 0 {
        /* CHILD PROCESS */
        let mut r: rlimit = rlimit{rlim_cur: 0, rlim_max: 0,};
        /* Umpf. On OpenBSD, the default fd limit for root users is set to
       soft 128. Let's try to fix that... */
        if getrlimit(RLIMIT_NOFILE as libc::c_int, &mut r) == 0 &&
               r.rlim_cur <
                   (198 as libc::c_int + 2 as libc::c_int) as libc::c_ulong {
            r.rlim_cur = (198 as libc::c_int + 2 as libc::c_int) as rlim_t;
            setrlimit(RLIMIT_NOFILE as libc::c_int, &mut r);
            /* Ignore errors */
        }
        if (*fsrv).mem_limit != 0 {
            r.rlim_cur = ((*fsrv).mem_limit as rlim_t) << 20 as libc::c_int;
            r.rlim_max = r.rlim_cur;
            setrlimit(RLIMIT_AS as libc::c_int, &mut r);
            /* Ignore errors */
            /* ^RLIMIT_AS */
        }
        /* Dumping cores is slow and can lead to anomalies if SIGKILL is delivered
       before the dump is complete. */
        //    r.rlim_max = r.rlim_cur = 0;
    //    setrlimit(RLIMIT_CORE, &r);                      /* Ignore errors */
        /* Isolate the process and configure standard descriptors. If out_file is
       specified, stdin is /dev/null; otherwise, out_fd is cloned instead. */
        setsid();
        if debug_child_output == 0 {
            dup2((*fsrv).dev_null_fd, 1 as libc::c_int);
            dup2((*fsrv).dev_null_fd, 2 as libc::c_int);
        }
        if (*fsrv).use_stdin == 0 {
            dup2((*fsrv).dev_null_fd, 0 as libc::c_int);
        } else {
            dup2((*fsrv).out_fd, 0 as libc::c_int);
            close((*fsrv).out_fd);
        }
        /* Set up control and status pipes, close the unneeded original fds. */
        if dup2(ctl_pipe[0 as libc::c_int as usize], 198 as libc::c_int) <
               0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mdup2() failed\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-forkserver.c\x00" as *const u8 as
                       *const libc::c_char, 288 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        if dup2(st_pipe[1 as libc::c_int as usize],
                198 as libc::c_int + 1 as libc::c_int) < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mdup2() failed\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-forkserver.c\x00" as *const u8 as
                       *const libc::c_char, 289 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        close(ctl_pipe[0 as libc::c_int as usize]);
        close(ctl_pipe[1 as libc::c_int as usize]);
        close(st_pipe[0 as libc::c_int as usize]);
        close(st_pipe[1 as libc::c_int as usize]);
        close((*fsrv).out_dir_fd);
        close((*fsrv).dev_null_fd);
        close((*fsrv).dev_urandom_fd);
        if !(*fsrv).plot_file.is_null() { fclose((*fsrv).plot_file); }
        /* This should improve performance a bit, since it stops the linker from
       doing extra work post-fork(). */
        if getenv(b"LD_BIND_LAZY\x00" as *const u8 as
                      *const libc::c_char).is_null() {
            setenv(b"LD_BIND_NOW\x00" as *const u8 as *const libc::c_char,
                   b"1\x00" as *const u8 as *const libc::c_char,
                   0 as libc::c_int);
        }
        /* Set sane defaults for ASAN if nothing else specified. */
        setenv(b"ASAN_OPTIONS\x00" as *const u8 as *const libc::c_char,
               b"abort_on_error=1:detect_leaks=0:malloc_context_size=0:symbolize=0:allocator_may_return_null=1\x00"
                   as *const u8 as *const libc::c_char, 0 as libc::c_int);
        /* MSAN is tricky, because it doesn't support abort_on_error=1 at this
       point. So, we do this in a very hacky way. */
        setenv(b"MSAN_OPTIONS\x00" as *const u8 as *const libc::c_char,
               b"exit_code=86:symbolize=0:abort_on_error=1:malloc_context_size=0:allocator_may_return_null=1:msan_track_origins=0\x00"
                   as *const u8 as *const libc::c_char, 0 as libc::c_int);
        (*fsrv).init_child_func.expect("non-null function pointer")(fsrv,
                                                                    argv);
        /* Use a distinctive bitmap signature to tell the parent about execv()
       falling through. */
        *((*fsrv).trace_bits as *mut u32_0) = 0xfee1dead as libc::c_uint;
        exit(0 as libc::c_int);
    }
    /* PARENT PROCESS */
    /* Close the unneeded endpoints. */
    close(ctl_pipe[0 as libc::c_int as usize]);
    close(st_pipe[1 as libc::c_int as usize]);
    (*fsrv).fsrv_ctl_fd = ctl_pipe[1 as libc::c_int as usize];
    (*fsrv).fsrv_st_fd = st_pipe[0 as libc::c_int as usize];
    /* Wait for the fork server to come up, but don't wait too long. */
    rlen = 0 as libc::c_int;
    if (*fsrv).exec_tmout != 0 {
        let mut time: u32_0 =
            read_timed((*fsrv).fsrv_st_fd,
                       &mut status as *mut libc::c_int as *mut libc::c_void,
                       4 as libc::c_int as size_t,
                       (*fsrv).exec_tmout.wrapping_mul(10 as libc::c_int as
                                                           libc::c_uint),
                       stop_soon_p);
        if time == 0 {
            kill((*fsrv).fsrv_pid, 9 as libc::c_int);
        } else if time >
                      (*fsrv).exec_tmout.wrapping_mul(10 as libc::c_int as
                                                          libc::c_uint) {
            (*fsrv).child_timed_out = 1 as libc::c_int as u8_0;
            kill((*fsrv).fsrv_pid, 9 as libc::c_int);
        } else { rlen = 4 as libc::c_int }
    } else {
        rlen =
            read((*fsrv).fsrv_st_fd,
                 &mut status as *mut libc::c_int as *mut libc::c_void,
                 4 as libc::c_int as size_t) as s32
    }
    /* If we have a four-byte "hello" message from the server, we're all set.
     Otherwise, try to figure out what went wrong. */
    if rlen == 4 as libc::c_int {
        if be_quiet == 0 {
            printf(b"\x1b[1;92m[+] \x1b[0mAll right - fork server is up.\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
        if status as libc::c_uint & 0x8f000001 as libc::c_uint ==
               0x8f000001 as libc::c_uint {
            if be_quiet == 0 {
                printf(b"\x1b[1;94m[*] \x1b[0mExtended forkserver functions received (%08x).\x00"
                           as *const u8 as *const libc::c_char, status);
                printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
            }
            if status & 0x20000000 as libc::c_int == 0x20000000 as libc::c_int
               {
                (*fsrv).snapshot = 1 as libc::c_int as u32_0;
                if be_quiet == 0 {
                    printf(b"\x1b[1;94m[*] \x1b[0mUsing SNAPSHOT feature.\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[0m\n\x00" as *const u8 as
                               *const libc::c_char);
                }
            }
            if status & 0x40000000 as libc::c_int == 0x40000000 as libc::c_int
               {
                (*fsrv).map_size =
                    (((status & 0xfffffe as libc::c_int) >> 1 as libc::c_int)
                         + 1 as libc::c_int) as u32_0;
                if (*fsrv).map_size.wrapping_rem(8 as libc::c_int as
                                                     libc::c_uint) != 0 {
                    // should not happen
                    (*fsrv).map_size =
                        ((*fsrv).map_size.wrapping_add(8 as libc::c_int as
                                                           libc::c_uint) >>
                             3 as libc::c_int) << 3 as libc::c_int
                }
                if be_quiet == 0 {
                    printf(b"\x1b[1;94m[*] \x1b[0mTarget map size: %u\x00" as
                               *const u8 as *const libc::c_char,
                           (*fsrv).map_size);
                    printf(b"\x1b[0m\n\x00" as *const u8 as
                               *const libc::c_char);
                }
                if (*fsrv).map_size >
                       ((1 as libc::c_int) << 16 as libc::c_int) as
                           libc::c_uint {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mTarget\'s coverage map size of %u is larger than the one this afl++ is compiled with (%u)\n\x00"
                               as *const u8 as *const libc::c_char,
                           (*fsrv).map_size,
                           (1 as libc::c_int) << 16 as libc::c_int);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-forkserver.c\x00" as *const u8 as
                               *const libc::c_char, 408 as libc::c_int);
                    exit(1 as libc::c_int);
                }
            }
            if status & 0x10000000 as libc::c_int == 0x10000000 as libc::c_int
               {
                if (*fsrv).function_ptr.is_none() ||
                       (*fsrv).function_opt.is_null() {
                    // this is not afl-fuzz - we deny and return
                    status =
                        (0xffffffff as libc::c_uint ^
                             (0x8f000001 as libc::c_uint |
                                  0x10000000 as libc::c_int as libc::c_uint))
                            as libc::c_int;
                    if write((*fsrv).fsrv_ctl_fd,
                             &mut status as *mut libc::c_int as
                                 *const libc::c_void,
                             4 as libc::c_int as size_t) !=
                           4 as libc::c_int as libc::c_long {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWriting to forkserver failed.\x00"
                                   as *const u8 as *const libc::c_char);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-forkserver.c\x00" as *const u8 as
                                   *const libc::c_char, 419 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                    return
                }
                if be_quiet == 0 {
                    printf(b"\x1b[1;94m[*] \x1b[0mUsing AUTODICT feature.\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[0m\n\x00" as *const u8 as
                               *const libc::c_char);
                }
                status =
                    (0x8f000001 as libc::c_uint |
                         0x10000000 as libc::c_int as libc::c_uint) as
                        libc::c_int;
                if write((*fsrv).fsrv_ctl_fd,
                         &mut status as *mut libc::c_int as
                             *const libc::c_void, 4 as libc::c_int as size_t)
                       != 4 as libc::c_int as libc::c_long {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWriting to forkserver failed.\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-forkserver.c\x00" as *const u8 as
                               *const libc::c_char, 427 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if read((*fsrv).fsrv_st_fd,
                        &mut status as *mut libc::c_int as *mut libc::c_void,
                        4 as libc::c_int as size_t) !=
                       4 as libc::c_int as libc::c_long {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mReading from forkserver failed.\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-forkserver.c\x00" as *const u8 as
                               *const libc::c_char, 429 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                if status < 2 as libc::c_int ||
                       status as u32_0 >
                           0xffffff as libc::c_int as libc::c_uint {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mDictionary has an illegal size: %d\x00"
                               as *const u8 as *const libc::c_char, status);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-forkserver.c\x00" as *const u8 as
                               *const libc::c_char, 432 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                let mut len: u32_0 = status as u32_0;
                let mut offset: u32_0 = 0 as libc::c_int as u32_0;
                let mut count: u32_0 = 0 as libc::c_int as u32_0;
                let mut dict: *mut u8_0 = DFL_ck_alloc(len) as *mut u8_0;
                if dict.is_null() {
                    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mCould not allocate %u bytes of autodictionary memmory\x00"
                               as *const u8 as *const libc::c_char, len);
                    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           b"func_unknown\x00" as *const u8 as
                               *const libc::c_char,
                           b"src/afl-forkserver.c\x00" as *const u8 as
                               *const libc::c_char, 437 as libc::c_int);
                    exit(1 as libc::c_int);
                }
                while len != 0 as libc::c_int as libc::c_uint {
                    rlen =
                        read((*fsrv).fsrv_st_fd,
                             dict.offset(offset as isize) as
                                 *mut libc::c_void, len as size_t) as s32;
                    if rlen > 0 as libc::c_int {
                        len =
                            (len as
                                 libc::c_uint).wrapping_sub(rlen as
                                                                libc::c_uint)
                                as u32_0 as u32_0;
                        offset =
                            (offset as
                                 libc::c_uint).wrapping_add(rlen as
                                                                libc::c_uint)
                                as u32_0 as u32_0
                    } else {
                        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mReading autodictionary fail at position %u with %u bytes left.\x00"
                                   as *const u8 as *const libc::c_char,
                               offset, len);
                        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               b"func_unknown\x00" as *const u8 as
                                   *const libc::c_char,
                               b"src/afl-forkserver.c\x00" as *const u8 as
                                   *const libc::c_char, 452 as libc::c_int);
                        exit(1 as libc::c_int);
                    }
                }
                offset = 0 as libc::c_int as u32_0;
                while offset < status as libc::c_uint &&
                          (*dict.offset(offset as isize) as
                               libc::c_uint).wrapping_add(offset) <
                              status as libc::c_uint {
                    (*fsrv).function_ptr.expect("non-null function pointer")((*fsrv).function_opt
                                                                                 as
                                                                                 *mut libc::c_void,
                                                                             dict.offset(offset
                                                                                             as
                                                                                             isize).offset(1
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               isize),
                                                                             *dict.offset(offset
                                                                                              as
                                                                                              isize)
                                                                                 as
                                                                                 u32_0);
                    offset =
                        (offset as
                             libc::c_uint).wrapping_add((1 as libc::c_int +
                                                             *dict.offset(offset
                                                                              as
                                                                              isize)
                                                                 as
                                                                 libc::c_int)
                                                            as libc::c_uint)
                            as u32_0 as u32_0;
                    count = count.wrapping_add(1)
                }
                if be_quiet == 0 {
                    printf(b"\x1b[1;94m[*] \x1b[0mLoaded %u autodictionary entries\x00"
                               as *const u8 as *const libc::c_char, count);
                    printf(b"\x1b[0m\n\x00" as *const u8 as
                               *const libc::c_char);
                }
                DFL_ck_free(dict as *mut libc::c_void);
            }
        }
        return
    }
    if (*fsrv).child_timed_out != 0 {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mTimeout while initializing fork server (adjusting -t may help)\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-forkserver.c\x00" as *const u8 as
                   *const libc::c_char, 480 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if waitpid((*fsrv).fsrv_pid, &mut status, 0 as libc::c_int) <=
           0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mwaitpid() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-forkserver.c\x00" as *const u8 as
                   *const libc::c_char, 482 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar as
           libc::c_int >> 1 as libc::c_int > 0 as libc::c_int {
        if (*fsrv).mem_limit != 0 &&
               (*fsrv).mem_limit < 500 as libc::c_int as libc::c_ulonglong &&
               (*fsrv).uses_asan as libc::c_int != 0 {
            printf(b"\n\x1b[1;91m[-] \x1b[0mWhoops, the target binary crashed suddenly, before receiving any input\n    from the fuzzer! Since it seems to be built with ASAN and you have a\n    restrictive memory limit configured, this is expected; please read\n    %s/notes_for_asan.md for help.\n\x00"
                       as *const u8 as *const libc::c_char, doc_path);
        } else if (*fsrv).mem_limit == 0 {
            printf(b"\n\x1b[1;91m[-] \x1b[0mWhoops, the target binary crashed suddenly, before receiving any input\n    from the fuzzer! There are several probable explanations:\n\n    - The binary is just buggy and explodes entirely on its own. If so, you\n      need to fix the underlying problem or find a better replacement.\n\n    - Less likely, there is a horrible bug in the fuzzer. If other options\n      fail, poke <afl-users@googlegroups.com> for troubleshooting tips.\n\x00"
                       as *const u8 as *const libc::c_char);
        } else {
            let mut val_buf: [u8_0; 16] = [0; 16];
            printf(b"\n\x1b[1;91m[-] \x1b[0mWhoops, the target binary crashed suddenly, before receiving any input\n    from the fuzzer! There are several probable explanations:\n\n    - The current memory limit (%s) is too restrictive, causing the\n      target to hit an OOM condition in the dynamic linker. Try bumping up\n      the limit with the -m setting in the command line. A simple way confirm\n      this diagnosis would be:\n\n      ( ulimit -Sd $[%llu << 10]; /path/to/fuzzed_app )\n\n      Tip: you can use http://jwilk.net/software/recidivm to quickly\n      estimate the required amount of virtual memory for the binary.\n\n    - The binary is just buggy and explodes entirely on its own. If so, you\n      need to fix the underlying problem or find a better replacement.\n\n    - Less likely, there is a horrible bug in the fuzzer. If other options\n      fail, poke <afl-users@googlegroups.com> for troubleshooting tips.\n\x00"
                       as *const u8 as *const libc::c_char,
                   stringify_mem_size(val_buf.as_mut_ptr(),
                                      ::std::mem::size_of::<[u8_0; 16]>() as
                                          libc::c_ulong,
                                      (*fsrv).mem_limit << 20 as libc::c_int),
                   (*fsrv).mem_limit.wrapping_sub(1 as libc::c_int as
                                                      libc::c_ulonglong));
        }
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mFork server crashed with signal %d\x00"
                   as *const u8 as *const libc::c_char,
               status & 0x7f as libc::c_int);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-forkserver.c\x00" as *const u8 as
                   *const libc::c_char, 558 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if *((*fsrv).trace_bits as *mut u32_0) == 0xfee1dead as libc::c_uint {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mUnable to execute target application (\'%s\')\x00"
                   as *const u8 as *const libc::c_char,
               *argv.offset(0 as libc::c_int as isize));
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-forkserver.c\x00" as *const u8 as
                   *const libc::c_char, 563 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if (*fsrv).mem_limit != 0 &&
           (*fsrv).mem_limit < 500 as libc::c_int as libc::c_ulonglong &&
           (*fsrv).uses_asan as libc::c_int != 0 {
        printf(b"\n\x1b[1;91m[-] \x1b[0mHmm, looks like the target binary terminated before we could complete a\n    handshake with the injected code. Since it seems to be built with ASAN and\n    you have a restrictive memory limit configured, this is expected; please\n    read %s/notes_for_asan.md for help.\n\x00"
                   as *const u8 as *const libc::c_char, doc_path);
    } else if (*fsrv).mem_limit == 0 {
        printf(b"\n\x1b[1;91m[-] \x1b[0mHmm, looks like the target binary terminated before we could complete a\n    handshake with the injected code. Perhaps there is a horrible bug in the\n    fuzzer. Poke <afl-users@googlegroups.com> for troubleshooting tips.\n\x00"
                   as *const u8 as *const libc::c_char);
    } else {
        let mut val_buf_0: [u8_0; 16] = [0; 16];
        printf(b"\n\x1b[1;91m[-] \x1b[0mHmm, looks like the target binary terminated before we could complete a\n    handshake with the injected code. There are %s probable explanations:\n\n%s    - The current memory limit (%s) is too restrictive, causing an OOM\n      fault in the dynamic linker. This can be fixed with the -m option. A\n      simple way to confirm the diagnosis may be:\n\n      ( ulimit -Sd $[%llu << 10]; /path/to/fuzzed_app )\n\n      Tip: you can use http://jwilk.net/software/recidivm to quickly\n      estimate the required amount of virtual memory for the binary.\n\n    - Less likely, there is a horrible bug in the fuzzer. If other options\n      fail, poke <afl-users@googlegroups.com> for troubleshooting tips.\n\x00"
                   as *const u8 as *const libc::c_char,
               if !getenv(b"__AFL_DEFER_FORKSRV\x00" as *const u8 as
                              *const libc::c_char).is_null() {
                   b"three\x00" as *const u8 as *const libc::c_char
               } else { b"two\x00" as *const u8 as *const libc::c_char },
               if !getenv(b"__AFL_DEFER_FORKSRV\x00" as *const u8 as
                              *const libc::c_char).is_null() {
                   b"    - You are using deferred forkserver, but __AFL_INIT() is never\n      reached before the program terminates.\n\n\x00"
                       as *const u8 as *const libc::c_char
               } else { b"\x00" as *const u8 as *const libc::c_char },
               stringify_int(val_buf_0.as_mut_ptr(),
                             ::std::mem::size_of::<[u8_0; 16]>() as
                                 libc::c_ulong,
                             (*fsrv).mem_limit << 20 as libc::c_int),
               (*fsrv).mem_limit.wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulonglong));
    }
    printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mFork server handshake failed\x00"
               as *const u8 as *const libc::c_char);
    printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00" as
               *const u8 as *const libc::c_char,
           b"func_unknown\x00" as *const u8 as *const libc::c_char,
           b"src/afl-forkserver.c\x00" as *const u8 as *const libc::c_char,
           627 as libc::c_int);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn afl_fsrv_kill(mut fsrv: *mut afl_forkserver_t) {
    if (*fsrv).child_pid > 0 as libc::c_int {
        kill((*fsrv).child_pid, 9 as libc::c_int);
    }
    if (*fsrv).fsrv_pid > 0 as libc::c_int {
        kill((*fsrv).fsrv_pid, 9 as libc::c_int);
        if waitpid((*fsrv).fsrv_pid, 0 as *mut libc::c_int, 0 as libc::c_int)
               <= 0 as libc::c_int {
            printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0merror waitpid\n\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn afl_fsrv_killall() {
    let mut li: *mut list_t = &mut fsrv_list;
    let mut head: *mut element_t = get_head(li);
    let mut el_box: *mut element_t = (*head).next;
    if el_box.is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mforeach over uninitialized list\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-forkserver.c\x00" as *const u8 as
                   *const libc::c_char, 649 as libc::c_int);
        exit(1 as libc::c_int);
    }
    while el_box != head {
        let mut el: *mut afl_forkserver_t =
            (*el_box).data as *mut afl_forkserver_t;
        let mut next: *mut element_t = (*el_box).next;
        afl_fsrv_kill(el);
        el_box = next
    };
}
#[no_mangle]
pub unsafe extern "C" fn afl_fsrv_deinit(mut fsrv: *mut afl_forkserver_t) {
    afl_fsrv_kill(fsrv);
    list_remove(&mut fsrv_list, fsrv as *mut libc::c_void);
}
