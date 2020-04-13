use ::libc;
use ::c2rust_bitfields;
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
    fn setenv(__name: *const libc::c_char, __value: *const libc::c_char,
              __replace: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn shmctl(__shmid: libc::c_int, __cmd: libc::c_int, __buf: *mut shmid_ds)
     -> libc::c_int;
    #[no_mangle]
    fn shmget(__key: key_t, __size: size_t, __shmflg: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn shmat(__shmid: libc::c_int, __shmaddr: *const libc::c_void,
             __shmflg: libc::c_int) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __key_t = libc::c_int;
pub type __syscall_ulong_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type key_t = __key_t;
pub type u8_0 = uint8_t;
pub type u32_0 = uint32_t;
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
pub type element_t = list_element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_element {
    pub pre_status: pre_status_t,
    pub prev: *mut list_element,
    pub next: *mut list_element,
    pub data: *mut libc::c_void,
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
pub type pre_status_t = prealloc_status;
pub type prealloc_status = libc::c_uint;
pub const PRE_STATUS_MALLOC: prealloc_status = 2;
pub const PRE_STATUS_USED: prealloc_status = 1;
pub const PRE_STATUS_UNUSED: prealloc_status = 0;
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
/* free in buf */
/* used in buf */
/* system malloc */
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
pub type sharedmem_t = sharedmem;
pub type list_t = list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list {
    pub element_prealloc_buf: [element_t; 64],
    pub element_prealloc_count: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shmid_ds {
    pub shm_perm: ipc_perm,
    pub shm_segsz: size_t,
    pub shm_atime: __time_t,
    pub shm_dtime: __time_t,
    pub shm_ctime: __time_t,
    pub shm_cpid: __pid_t,
    pub shm_lpid: __pid_t,
    pub shm_nattch: shmatt_t,
    pub __glibc_reserved5: __syscall_ulong_t,
    pub __glibc_reserved6: __syscall_ulong_t,
}
pub type shmatt_t = __syscall_ulong_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_perm {
    pub __key: __key_t,
    pub uid: __uid_t,
    pub gid: __gid_t,
    pub cuid: __uid_t,
    pub cgid: __gid_t,
    pub mode: __mode_t,
    pub __seq: libc::c_ushort,
    pub __pad2: libc::c_ushort,
    pub __glibc_reserved1: __syscall_ulong_t,
    pub __glibc_reserved2: __syscall_ulong_t,
}
// extern unsigned char *trace_bits;
/* ID of the SHM region              */
/* shared memory region */
/* actual allocated size */
/* in use by shmem app */
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
unsafe extern "C" fn get_head(mut list: *mut list_t) -> *mut element_t {
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
   american fuzzy lop++ - shared memory related code
   -------------------------------------------------

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

   Shared code to handle the shared memory. This is used by the fuzzer
   as well the other components like afl-tmin, afl-showmap, etc...

 */
#[no_mangle]
pub static mut shm_list: list_t =
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
/* Get rid of shared memory. */
#[no_mangle]
pub unsafe extern "C" fn afl_shm_deinit(mut shm: *mut sharedmem_t) {
    // TODO: clang reports a potential UAF in this function/makro(?)
    list_remove(&mut shm_list, shm as *mut libc::c_void);
    shmctl((*shm).shm_id, 0 as libc::c_int, 0 as *mut shmid_ds);
    if (*shm).cmplog_mode != 0 {
        shmctl((*shm).cmplog_shm_id, 0 as libc::c_int, 0 as *mut shmid_ds);
    }
    (*shm).map = 0 as *mut u8_0;
}
/* Configure shared memory.
   Returns a pointer to shm->map for ease of use.
*/
#[no_mangle]
pub unsafe extern "C" fn afl_shm_init(mut shm: *mut sharedmem_t,
                                      mut map_size: size_t,
                                      mut dumb_mode: libc::c_uchar)
 -> *mut u8_0 {
    (*shm).size_used = map_size;
    (*shm).size_alloc = (*shm).size_used;
    (*shm).map = 0 as *mut u8_0;
    let mut shm_str: *mut u8_0 = 0 as *mut u8_0;
    (*shm).shm_id =
        shmget(0 as libc::c_int, map_size,
               0o1000 as libc::c_int | 0o2000 as libc::c_int |
                   0o600 as libc::c_int);
    if (*shm).shm_id < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mshmget() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-sharedmem.c\x00" as *const u8 as *const libc::c_char,
               156 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    if (*shm).cmplog_mode != 0 {
        (*shm).cmplog_shm_id =
            shmget(0 as libc::c_int,
                   ::std::mem::size_of::<cmp_map>() as libc::c_ulong,
                   0o1000 as libc::c_int | 0o2000 as libc::c_int |
                       0o600 as libc::c_int);
        if (*shm).cmplog_shm_id < 0 as libc::c_int {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mshmget() failed\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-sharedmem.c\x00" as *const u8 as
                       *const libc::c_char, 163 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
    }
    shm_str =
        ({
             let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
             let mut _len: s32 =
                 snprintf(0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_ulong,
                          b"%d\x00" as *const u8 as *const libc::c_char,
                          (*shm).shm_id);
             if _len < 0 as libc::c_int {
                 printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                            as *const u8 as *const libc::c_char);
                 printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        b"func_unknown\x00" as *const u8 as
                            *const libc::c_char,
                        b"src/afl-sharedmem.c\x00" as *const u8 as
                            *const libc::c_char, 167 as libc::c_int);
                 exit(1 as libc::c_int);
             }
             _tmp =
                 DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                     *mut u8_0;
             snprintf(_tmp as *mut libc::c_char,
                      (_len + 1 as libc::c_int) as libc::c_ulong,
                      b"%d\x00" as *const u8 as *const libc::c_char,
                      (*shm).shm_id);
             _tmp
         });
    /* If somebody is asking us to fuzz instrumented binaries in dumb mode,
     we don't want them to detect instrumentation, since we won't be sending
     fork server commands. This should be replaced with better auto-detection
     later on, perhaps? */
    if dumb_mode == 0 {
        setenv(b"__AFL_SHM_ID\x00" as *const u8 as *const libc::c_char,
               shm_str as *const libc::c_char, 1 as libc::c_int);
    }
    DFL_ck_free(shm_str as *mut libc::c_void);
    if (*shm).cmplog_mode != 0 {
        shm_str =
            ({
                 let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                 let mut _len: s32 =
                     snprintf(0 as *mut libc::c_char,
                              0 as libc::c_int as libc::c_ulong,
                              b"%d\x00" as *const u8 as *const libc::c_char,
                              (*shm).cmplog_shm_id);
                 if _len < 0 as libc::c_int {
                     printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                as *const u8 as *const libc::c_char);
                     printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"func_unknown\x00" as *const u8 as
                                *const libc::c_char,
                            b"src/afl-sharedmem.c\x00" as *const u8 as
                                *const libc::c_char, 180 as libc::c_int);
                     exit(1 as libc::c_int);
                 }
                 _tmp =
                     DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                         *mut u8_0;
                 snprintf(_tmp as *mut libc::c_char,
                          (_len + 1 as libc::c_int) as libc::c_ulong,
                          b"%d\x00" as *const u8 as *const libc::c_char,
                          (*shm).cmplog_shm_id);
                 _tmp
             });
        if dumb_mode == 0 {
            setenv(b"__AFL_CMPLOG_SHM_ID\x00" as *const u8 as
                       *const libc::c_char, shm_str as *const libc::c_char,
                   1 as libc::c_int);
        }
        DFL_ck_free(shm_str as *mut libc::c_void);
    }
    (*shm).map =
        shmat((*shm).shm_id, 0 as *const libc::c_void, 0 as libc::c_int) as
            *mut u8_0;
    if (*shm).map == -(1 as libc::c_int) as *mut libc::c_void as *mut u8_0 ||
           (*shm).map.is_null() {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mshmat() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-sharedmem.c\x00" as *const u8 as *const libc::c_char,
               190 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    if (*shm).cmplog_mode != 0 {
        (*shm).cmp_map =
            shmat((*shm).cmplog_shm_id, 0 as *const libc::c_void,
                  0 as libc::c_int) as *mut cmp_map;
        if (*shm).cmp_map ==
               -(1 as libc::c_int) as *mut libc::c_void as *mut cmp_map ||
               (*shm).cmp_map.is_null() {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mshmat() failed\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-sharedmem.c\x00" as *const u8 as
                       *const libc::c_char, 196 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
    }
    list_append(&mut shm_list, shm as *mut libc::c_void);
    return (*shm).map;
}
