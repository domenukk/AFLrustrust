use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
/*
   american fuzzy lop++ - a trivial program to test the build
   --------------------------------------------------------
   Originally written by Michal Zalewski
   Copyright 2014 Google Inc. All rights reserved.
   Copyright 2019-2020 AFLplusplus Project. All rights reserved.
   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at:
     http://www.apache.org/licenses/LICENSE-2.0
 */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut fd: libc::c_int = 0 as libc::c_int;
    let mut buff: [libc::c_char; 8] = [0; 8];
    let mut buf: *mut libc::c_char = buff.as_mut_ptr();
    // we support command line parameter and stdin
    if argc == 2 as libc::c_int {
        buf = *argv.offset(1 as libc::c_int as isize);
        printf(b"Input %s - \x00" as *const u8 as *const libc::c_char, buf);
    } else {
        if argc >= 3 as libc::c_int &&
               strcmp(*argv.offset(1 as libc::c_int as isize),
                      b"-f\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
            fd =
                open(*argv.offset(2 as libc::c_int as isize),
                     0 as libc::c_int);
            if fd < 0 as libc::c_int {
                fprintf(stderr,
                        b"Error: unable to open %s\n\x00" as *const u8 as
                            *const libc::c_char,
                        *argv.offset(2 as libc::c_int as isize));
                exit(-(1 as libc::c_int));
            }
        }
        if read(fd, buf as *mut libc::c_void,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) <
               1 as libc::c_int as libc::c_long {
            printf(b"Hum?\n\x00" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int
        }
    }
    // we support three input cases (plus a 4th if stdin is used but there is no
  // input)
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
        printf(b"Looks like a zero to me!\n\x00" as *const u8 as
                   *const libc::c_char);
    } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                  '1' as i32 {
        printf(b"Pretty sure that is a one!\n\x00" as *const u8 as
                   *const libc::c_char);
    } else {
        printf(b"Neither one or zero? How quaint!\n\x00" as *const u8 as
                   *const libc::c_char);
    }
    return 0 as libc::c_int;
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
