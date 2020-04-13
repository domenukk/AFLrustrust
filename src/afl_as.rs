use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn random() -> libc::c_long;
    #[no_mangle]
    fn srandom(__seed: libc::c_uint);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
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
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
               __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void)
     -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
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
/*
   american fuzzy lop++ - injectable parts
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

   This file houses the assembly-level instrumentation injected into fuzzed
   programs. The instrumentation stores XORed pairs of data: identifiers of the
   currently executing branch and the one that executed immediately before.

   TL;DR: the instrumentation does shm_trace_map[cur_loc ^ prev_loc]++

   The code is designed for 32-bit and 64-bit x86 systems. Both modes should
   work everywhere except for Apple systems. Apple does relocations differently
   from everybody else, so since their OSes have been 64-bit for a longer while,
   I didn't go through the mental effort of porting the 32-bit code.

   In principle, similar code should be easy to inject into any well-behaved
   binary-only code (e.g., using DynamoRIO). Conditional jumps offer natural
   targets for instrumentation, and should offer comparable probe density.

 */
/*
   ------------------
   Performances notes
   ------------------

   Contributions to make this code faster are appreciated! Here are some
   rough notes that may help with the task:

   - Only the trampoline_fmt and the non-setup __afl_maybe_log code paths are
     really worth optimizing; the setup / fork server stuff matters a lot less
     and should be mostly just kept readable.

   - We're aiming for modern CPUs with out-of-order execution and large
     pipelines; the code is mostly follows intuitive, human-readable
     instruction ordering, because "textbook" manual reorderings make no
     substantial difference.

   - Interestingly, instrumented execution isn't a lot faster if we store a
     variable pointer to the setup, log, or return routine and then do a reg
     call from within trampoline_fmt. It does speed up non-instrumented
     execution quite a bit, though, since that path just becomes
     push-call-ret-pop.

   - There is also not a whole lot to be gained by doing SHM attach at a
     fixed address instead of retrieving __afl_area_ptr. Although it allows us
     to have a shorter log routine inserted for conditional jumps and jump
     labels (for a ~10% perf gain), there is a risk of bumping into other
     allocations created by the program or by tools such as ASAN.

   - popf is *awfully* slow, which is why we're doing the lahf / sahf +
     overflow test trick. Unfortunately, this forces us to taint eax / rax, but
     this dependency on a commonly-used register still beats the alternative of
     using pushf / popf.

     One possible optimization is to avoid touching flags by using a circular
     buffer that stores just a sequence of current locations, with the XOR stuff
     happening offline. Alas, this doesn't seem to have a huge impact:

     https://groups.google.com/d/msg/afl-users/MsajVf4fRLo/2u6t88ntUBIJ

   - Preforking one child a bit sooner, and then waiting for the "go" command
     from within the child, doesn't offer major performance gains; fork() seems
     to be relatively inexpensive these days. Preforking multiple children does
     help, but badly breaks the "~1 core per fuzzer" design, making it harder to
     scale up. Maybe there is some middle ground.

   Perhaps of note: in the 64-bit version for all platforms except for Apple,
   the instrumentation is done slightly differently than on 32-bit, with
   __afl_prev_loc and __afl_area_ptr being local to the object file (.lcomm),
   rather than global (.comm). This is to avoid GOTRELPC lookups in the critical
   code path, which AFAICT, are otherwise unavoidable if we want gcc -shared to
   work; simple relocations between .bss and .text won't work on most 64-bit
   platforms in such a case.

   (Fun fact: on Apple systems, .lcomm can segfault the linker.)

   The side effect is that state transitions are measured in a somewhat
   different way, with previous tuple being recorded separately within the scope
   of every .c file. This should have no impact in any practical sense.

   Another side effect of this design is that getenv() will be called once per
   every .o file when running in non-instrumented mode; and since getenv() tends
   to be optimized in funny ways, we need to be very careful to save every
   oddball register it may touch.

 */
static mut trampoline_fmt_32: *const u8_0 =
    b"\n/* --- AFL TRAMPOLINE (32-BIT) --- */\n\n.align 4\n\nleal -16(%%esp), %%esp\nmovl %%edi,  0(%%esp)\nmovl %%edx,  4(%%esp)\nmovl %%ecx,  8(%%esp)\nmovl %%eax, 12(%%esp)\nmovl $0x%08x, %%ecx\ncall __afl_maybe_log\nmovl 12(%%esp), %%eax\nmovl  8(%%esp), %%ecx\nmovl  4(%%esp), %%edx\nmovl  0(%%esp), %%edi\nleal 16(%%esp), %%esp\n\n/* --- END --- */\n\n\x00"
        as *const u8 as *const libc::c_char as *const u8_0;
static mut trampoline_fmt_64: *const u8_0 =
    b"\n/* --- AFL TRAMPOLINE (64-BIT) --- */\n\n.align 4\n\nleaq -(128+24)(%%rsp), %%rsp\nmovq %%rdx,  0(%%rsp)\nmovq %%rcx,  8(%%rsp)\nmovq %%rax, 16(%%rsp)\nmovq $0x%08x, %%rcx\ncall __afl_maybe_log\nmovq 16(%%rsp), %%rax\nmovq  8(%%rsp), %%rcx\nmovq  0(%%rsp), %%rdx\nleaq (128+24)(%%rsp), %%rsp\n\n/* --- END --- */\n\n\x00"
        as *const u8 as *const libc::c_char as *const u8_0;
static mut main_payload_32: *const u8_0 =
    b"\n/* --- AFL MAIN PAYLOAD (32-BIT) --- */\n\n.text\n.att_syntax\n.code32\n.align 8\n\n__afl_maybe_log:\n\n  lahf\n  seto %al\n\n  /* Check if SHM region is already mapped. */\n\n  movl  __afl_area_ptr, %edx\n  testl %edx, %edx\n  je    __afl_setup\n\n__afl_store:\n\n  /* Calculate and store hit for the code location specified in ecx. There\n     is a double-XOR way of doing this without tainting another register,\n     and we use it on 64-bit systems; but it\'s slower for 32-bit ones. */\n\n  movl __afl_prev_loc, %edi\n  xorl %ecx, %edi\n  shrl $1, %ecx\n  movl %ecx, __afl_prev_loc\n\n  incb (%edx, %edi, 1)\n  adcb $0, (%edx, %edi, 1)\n\n__afl_return:\n\n  addb $127, %al\n  sahf\n  ret\n\n.align 8\n\n__afl_setup:\n\n  /* Do not retry setup if we had previous failures. */\n\n  cmpb $0, __afl_setup_failure\n  jne  __afl_return\n\n  /* Map SHM, jumping to __afl_setup_abort if something goes wrong.\n     We do not save FPU/MMX/SSE registers here, but hopefully, nobody\n     will notice this early in the game. */\n\n  pushl %eax\n  pushl %ecx\n\n  pushl $.AFL_SHM_ENV\n  call  getenv\n  addl  $4, %esp\n\n  testl %eax, %eax\n  je    __afl_setup_abort\n\n  pushl %eax\n  call  atoi\n  addl  $4, %esp\n\n  pushl $0          /* shmat flags    */\n  pushl $0          /* requested addr */\n  pushl %eax        /* SHM ID         */\n  call  shmat\n  addl  $12, %esp\n\n  cmpl $-1, %eax\n  je   __afl_setup_abort\n\n  movb $1, (%eax)\n  /* Store the address of the SHM region. */\n\n  movl %eax, __afl_area_ptr\n  movl %eax, %edx\n\n  popl %ecx\n  popl %eax\n\n__afl_forkserver:\n\n  /* Enter the fork server mode to avoid the overhead of execve() calls. */\n\n  pushl %eax\n  pushl %ecx\n  pushl %edx\n\n  /* Phone home and tell the parent that we\'re OK. (Note that signals with\n     no SA_RESTART will mess it up). If this fails, assume that the fd is\n     closed because we were execve()d from an instrumented binary, or because\n     the parent doesn\'t want to use the fork server. */\n\n  pushl $4          /* length    */\n  pushl $__afl_temp /* data      */\n  pushl $(198 + 1)  /* file desc */\n  call  write\n  addl  $12, %esp\n\n  cmpl  $4, %eax\n  jne   __afl_fork_resume\n\n__afl_fork_wait_loop:\n\n  /* Wait for parent by reading from the pipe. Abort if read fails. */\n\n  pushl $4          /* length    */\n  pushl $__afl_temp /* data      */\n  pushl $198        /* file desc */\n  call  read\n  addl  $12, %esp\n\n  cmpl  $4, %eax\n  jne   __afl_die\n\n  /* Once woken up, create a clone of our process. This is an excellent use\n     case for syscall(__NR_clone, 0, CLONE_PARENT), but glibc boneheadedly\n     caches getpid() results and offers no way to update the value, breaking\n     abort(), raise(), and a bunch of other things :-( */\n\n  call fork\n\n  cmpl $0, %eax\n  jl   __afl_die\n  je   __afl_fork_resume\n\n  /* In parent process: write PID to pipe, then wait for child. */\n\n  movl  %eax, __afl_fork_pid\n\n  pushl $4              /* length    */\n  pushl $__afl_fork_pid /* data      */\n  pushl $(198 + 1)      /* file desc */\n  call  write\n  addl  $12, %esp\n\n  pushl $0             /* no flags  */\n  pushl $__afl_temp    /* status    */\n  pushl __afl_fork_pid /* PID       */\n  call  waitpid\n  addl  $12, %esp\n\n  cmpl  $0, %eax\n  jle   __afl_die\n\n  /* Relay wait status to pipe, then loop back. */\n\n  pushl $4          /* length    */\n  pushl $__afl_temp /* data      */\n  pushl $(198 + 1)  /* file desc */\n  call  write\n  addl  $12, %esp\n\n  jmp __afl_fork_wait_loop\n\n__afl_fork_resume:\n\n  /* In child process: close fds, resume execution. */\n\n  pushl $198\n  call  close\n\n  pushl $(198 + 1)\n  call  close\n\n  addl  $8, %esp\n\n  popl %edx\n  popl %ecx\n  popl %eax\n  jmp  __afl_store\n\n__afl_die:\n\n  xorl %eax, %eax\n  call _exit\n\n__afl_setup_abort:\n\n  /* Record setup failure so that we don\'t keep calling\n     shmget() / shmat() over and over again. */\n\n  incb __afl_setup_failure\n  popl %ecx\n  popl %eax\n  jmp __afl_return\n\n.AFL_VARS:\n\n  .comm   __afl_area_ptr, 4, 32\n  .comm   __afl_setup_failure, 1, 32\n  .comm   __afl_prev_loc, 4, 32\n  .comm   __afl_final_loc, 4, 32\n  .comm   __afl_fork_pid, 4, 32\n  .comm   __afl_temp, 4, 32\n\n.AFL_SHM_ENV:\n  .asciz \"__AFL_SHM_ID\"\n\n/* --- END --- */\n\n\x00"
        as *const u8 as *const libc::c_char as *const u8_0;
/* The OpenBSD hack is due to lahf and sahf not being recognized by some
   versions of binutils: http://marc.info/?l=openbsd-cvs&m=141636589924400

   The Apple code is a bit different when calling libc functions because
   they are doing relocations differently from everybody else. We also need
   to work around the crash issue with .lcomm and the fact that they don't
   recognize .string. */
/* ^__APPLE__ */
static mut main_payload_64: *const u8_0 =
    b"\n/* --- AFL MAIN PAYLOAD (64-BIT) --- */\n\n.text\n.att_syntax\n.code64\n.align 8\n\n__afl_maybe_log:\n\n  lahf\n  seto  %al\n\n  /* Check if SHM region is already mapped. */\n\n  movq  __afl_area_ptr(%rip), %rdx\n  testq %rdx, %rdx\n  je    __afl_setup\n\n__afl_store:\n\n  /* Calculate and store hit for the code location specified in rcx. */\n\n  xorq __afl_prev_loc(%rip), %rcx\n  xorq %rcx, __afl_prev_loc(%rip)\n  shrq $1, __afl_prev_loc(%rip)\n\n  incb (%rdx, %rcx, 1)\n  adcb $0, (%rdx, %rcx, 1)\n\n__afl_return:\n\n  addb $127, %al\n  sahf\n  ret\n\n.align 8\n\n__afl_setup:\n\n  /* Do not retry setup if we had previous failures. */\n\n  cmpb $0, __afl_setup_failure(%rip)\n  jne __afl_return\n\n  /* Check out if we have a global pointer on file. */\n\n  movq  __afl_global_area_ptr@GOTPCREL(%rip), %rdx\n  movq  (%rdx), %rdx\n  testq %rdx, %rdx\n  je    __afl_setup_first\n\n  movq %rdx, __afl_area_ptr(%rip)\n  jmp  __afl_store\n\n__afl_setup_first:\n\n  /* Save everything that is not yet saved and that may be touched by\n     getenv() and several other libcalls we\'ll be relying on. */\n\n  leaq -352(%rsp), %rsp\n\n  movq %rax,   0(%rsp)\n  movq %rcx,   8(%rsp)\n  movq %rdi,  16(%rsp)\n  movq %rsi,  32(%rsp)\n  movq %r8,   40(%rsp)\n  movq %r9,   48(%rsp)\n  movq %r10,  56(%rsp)\n  movq %r11,  64(%rsp)\n\n  movq %xmm0,  96(%rsp)\n  movq %xmm1,  112(%rsp)\n  movq %xmm2,  128(%rsp)\n  movq %xmm3,  144(%rsp)\n  movq %xmm4,  160(%rsp)\n  movq %xmm5,  176(%rsp)\n  movq %xmm6,  192(%rsp)\n  movq %xmm7,  208(%rsp)\n  movq %xmm8,  224(%rsp)\n  movq %xmm9,  240(%rsp)\n  movq %xmm10, 256(%rsp)\n  movq %xmm11, 272(%rsp)\n  movq %xmm12, 288(%rsp)\n  movq %xmm13, 304(%rsp)\n  movq %xmm14, 320(%rsp)\n  movq %xmm15, 336(%rsp)\n\n  /* Map SHM, jumping to __afl_setup_abort if something goes wrong. */\n\n  /* The 64-bit ABI requires 16-byte stack alignment. We\'ll keep the\n     original stack ptr in the callee-saved r12. */\n\n  pushq %r12\n  movq  %rsp, %r12\n  subq  $16, %rsp\n  andq  $0xfffffffffffffff0, %rsp\n\n  leaq .AFL_SHM_ENV(%rip), %rdi\ncall getenv@PLT\n\n  testq %rax, %rax\n  je    __afl_setup_abort\n\n  movq  %rax, %rdi\ncall atoi@PLT\n\n  xorq %rdx, %rdx   /* shmat flags    */\n  xorq %rsi, %rsi   /* requested addr */\n  movq %rax, %rdi   /* SHM ID         */\ncall shmat@PLT\n\n  cmpq $-1, %rax\n  je   __afl_setup_abort\n\n  movb $1, (%rax)\n  /* Store the address of the SHM region. */\n\n  movq %rax, %rdx\n  movq %rax, __afl_area_ptr(%rip)\n\n  movq __afl_global_area_ptr@GOTPCREL(%rip), %rdx\n  movq %rax, (%rdx)\n  movq %rax, %rdx\n\n__afl_forkserver:\n\n  /* Enter the fork server mode to avoid the overhead of execve() calls. We\n     push rdx (area ptr) twice to keep stack alignment neat. */\n\n  pushq %rdx\n  pushq %rdx\n\n  /* Phone home and tell the parent that we\'re OK. (Note that signals with\n     no SA_RESTART will mess it up). If this fails, assume that the fd is\n     closed because we were execve()d from an instrumented binary, or because\n     the parent doesn\'t want to use the fork server. */\n\n  movq $4, %rdx               /* length    */\n  leaq __afl_temp(%rip), %rsi /* data      */\n  movq $(198 + 1), %rdi       /* file desc */\ncall write@PLT\n\n  cmpq $4, %rax\n  jne  __afl_fork_resume\n\n__afl_fork_wait_loop:\n\n  /* Wait for parent by reading from the pipe. Abort if read fails. */\n\n  movq $4, %rdx               /* length    */\n  leaq __afl_temp(%rip), %rsi /* data      */\n  movq $198, %rdi             /* file desc */\ncall read@PLT\n  cmpq $4, %rax\n  jne  __afl_die\n\n  /* Once woken up, create a clone of our process. This is an excellent use\n     case for syscall(__NR_clone, 0, CLONE_PARENT), but glibc boneheadedly\n     caches getpid() results and offers no way to update the value, breaking\n     abort(), raise(), and a bunch of other things :-( */\n\ncall fork@PLT\n  cmpq $0, %rax\n  jl   __afl_die\n  je   __afl_fork_resume\n\n  /* In parent process: write PID to pipe, then wait for child. */\n\n  movl %eax, __afl_fork_pid(%rip)\n\n  movq $4, %rdx                   /* length    */\n  leaq __afl_fork_pid(%rip), %rsi /* data      */\n  movq $(198 + 1), %rdi             /* file desc */\ncall write@PLT\n\n  movq $0, %rdx                   /* no flags  */\n  leaq __afl_temp(%rip), %rsi     /* status    */\n  movq __afl_fork_pid(%rip), %rdi /* PID       */\ncall waitpid@PLT\n  cmpq $0, %rax\n  jle  __afl_die\n\n  /* Relay wait status to pipe, then loop back. */\n\n  movq $4, %rdx               /* length    */\n  leaq __afl_temp(%rip), %rsi /* data      */\n  movq $(198 + 1), %rdi         /* file desc */\ncall write@PLT\n\n  jmp  __afl_fork_wait_loop\n\n__afl_fork_resume:\n\n  /* In child process: close fds, resume execution. */\n\n  movq $198, %rdi\ncall close@PLT\n\n  movq $(198 + 1), %rdi\ncall close@PLT\n\n  popq %rdx\n  popq %rdx\n\n  movq %r12, %rsp\n  popq %r12\n\n  movq  0(%rsp), %rax\n  movq  8(%rsp), %rcx\n  movq 16(%rsp), %rdi\n  movq 32(%rsp), %rsi\n  movq 40(%rsp), %r8\n  movq 48(%rsp), %r9\n  movq 56(%rsp), %r10\n  movq 64(%rsp), %r11\n\n  movq  96(%rsp), %xmm0\n  movq 112(%rsp), %xmm1\n  movq 128(%rsp), %xmm2\n  movq 144(%rsp), %xmm3\n  movq 160(%rsp), %xmm4\n  movq 176(%rsp), %xmm5\n  movq 192(%rsp), %xmm6\n  movq 208(%rsp), %xmm7\n  movq 224(%rsp), %xmm8\n  movq 240(%rsp), %xmm9\n  movq 256(%rsp), %xmm10\n  movq 272(%rsp), %xmm11\n  movq 288(%rsp), %xmm12\n  movq 304(%rsp), %xmm13\n  movq 320(%rsp), %xmm14\n  movq 336(%rsp), %xmm15\n\n  leaq 352(%rsp), %rsp\n\n  jmp  __afl_store\n\n__afl_die:\n\n  xorq %rax, %rax\ncall _exit@PLT\n\n__afl_setup_abort:\n\n  /* Record setup failure so that we don\'t keep calling\n     shmget() / shmat() over and over again. */\n\n  incb __afl_setup_failure(%rip)\n\n  movq %r12, %rsp\n  popq %r12\n\n  movq  0(%rsp), %rax\n  movq  8(%rsp), %rcx\n  movq 16(%rsp), %rdi\n  movq 32(%rsp), %rsi\n  movq 40(%rsp), %r8\n  movq 48(%rsp), %r9\n  movq 56(%rsp), %r10\n  movq 64(%rsp), %r11\n\n  movq  96(%rsp), %xmm0\n  movq 112(%rsp), %xmm1\n  movq 128(%rsp), %xmm2\n  movq 144(%rsp), %xmm3\n  movq 160(%rsp), %xmm4\n  movq 176(%rsp), %xmm5\n  movq 192(%rsp), %xmm6\n  movq 208(%rsp), %xmm7\n  movq 224(%rsp), %xmm8\n  movq 240(%rsp), %xmm9\n  movq 256(%rsp), %xmm10\n  movq 272(%rsp), %xmm11\n  movq 288(%rsp), %xmm12\n  movq 304(%rsp), %xmm13\n  movq 320(%rsp), %xmm14\n  movq 336(%rsp), %xmm15\n\n  leaq 352(%rsp), %rsp\n\n  jmp __afl_return\n\n.AFL_VARS:\n\n  .lcomm   __afl_area_ptr, 8\n  .lcomm   __afl_prev_loc, 8\n  .lcomm   __afl_fork_pid, 4\n  .lcomm   __afl_temp, 4\n  .lcomm   __afl_setup_failure, 1\n  .comm    __afl_global_area_ptr, 8, 8\n\n.AFL_SHM_ENV:\n  .asciz \"__AFL_SHM_ID\"\n\n/* --- END --- */\n\n\x00"
        as *const u8 as *const libc::c_char as *const u8_0;
/*
   american fuzzy lop++ - wrapper for GNU as
   -----------------------------------------

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

   The sole purpose of this wrapper is to preprocess assembly files generated
   by GCC / clang and inject the instrumentation bits included from afl-as.h. It
   is automatically invoked by the toolchain when compiling programs using
   afl-gcc / afl-clang.

   Note that it's an explicit non-goal to instrument hand-written assembly,
   be it in separate .s files or in __asm__ blocks. The only aspiration this
   utility has right now is to be able to skip them gracefully and allow the
   compilation process to continue.

   That said, see examples/clang_asm_normalize/ for a solution that may
   allow clang users to make things work even with hand-crafted assembly. Just
   note that there is no equivalent for GCC.

 */
static mut as_params: *mut *mut u8_0 =
    0 as *const *mut u8_0 as *mut *mut u8_0;
/* Parameters passed to the real 'as'   */
static mut input_file: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
/* Originally specified input file      */
static mut modified_file: *mut u8_0 = 0 as *const u8_0 as *mut u8_0;
/* Instrumented file for the real 'as'  */
static mut be_quiet: u8_0 = 0;
/* Quiet mode (no stderr output)        */
static mut clang_mode: u8_0 = 0;
/* Running in clang mode?               */
static mut pass_thru: u8_0 = 0;
/* Just pass data through?              */
static mut just_version: u8_0 = 0;
/* Just show version?                   */
static mut sanitizer: u8_0 = 0;
/* Using ASAN / MSAN                    */
static mut inst_ratio: u32_0 = 100 as libc::c_int as u32_0;
/* Instrumentation probability (%)      */
static mut as_par_cnt: u32_0 = 1 as libc::c_int as u32_0;
/* Number of params to 'as'             */
/* If we don't find --32 or --64 in the command line, default to
   instrumentation for whichever mode we were compiled with. This is not
   perfect, but should do the trick for almost all use cases. */
static mut use_64bit: u8_0 = 1 as libc::c_int as u8_0;
/* ^WORD_SIZE_64 */
/* Examine and modify parameters to pass to 'as'. Note that the file name
   is always the last parameter passed by GCC, so we exploit this property
   to keep the code simple. */
unsafe extern "C" fn edit_params(mut argc: libc::c_int,
                                 mut argv: *mut *mut libc::c_char) {
    let mut current_block: u64;
    let mut tmp_dir: *mut u8_0 =
        getenv(b"TMPDIR\x00" as *const u8 as *const libc::c_char) as
            *mut u8_0;
    let mut afl_as: *mut u8_0 =
        getenv(b"AFL_AS\x00" as *const u8 as *const libc::c_char) as
            *mut u8_0;
    let mut i: u32_0 = 0;
    /* __APPLE__ */
    /* Although this is not documented, GCC also uses TEMP and TMP when TMPDIR
     is not set. We need to check these non-standard variables to properly
     handle the pass_thru logic later on. */
    if tmp_dir.is_null() {
        tmp_dir =
            getenv(b"TEMP\x00" as *const u8 as *const libc::c_char) as
                *mut u8_0
    }
    if tmp_dir.is_null() {
        tmp_dir =
            getenv(b"TMP\x00" as *const u8 as *const libc::c_char) as
                *mut u8_0
    }
    if tmp_dir.is_null() {
        tmp_dir = b"/tmp\x00" as *const u8 as *const libc::c_char as *mut u8_0
    }
    as_params =
        DFL_ck_alloc(((argc + 32 as libc::c_int) as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut u8_0>()
                                                          as libc::c_ulong) as
                         u32_0) as *mut *mut u8_0;
    let ref mut fresh0 = *as_params.offset(0 as libc::c_int as isize);
    *fresh0 =
        if !afl_as.is_null() {
            afl_as
        } else { b"as\x00" as *const u8 as *const libc::c_char as *mut u8_0 };
    let ref mut fresh1 = *as_params.offset(argc as isize);
    *fresh1 = 0 as *mut u8_0;
    i = 1 as libc::c_int as u32_0;
    while i < (argc - 1 as libc::c_int) as libc::c_uint {
        if strcmp(*argv.offset(i as isize),
                  b"--64\x00" as *const u8 as *const libc::c_char) == 0 {
            use_64bit = 1 as libc::c_int as u8_0
        } else if strcmp(*argv.offset(i as isize),
                         b"--32\x00" as *const u8 as *const libc::c_char) == 0
         {
            use_64bit = 0 as libc::c_int as u8_0
        }
        /* __APPLE__ */
        let fresh2 = as_par_cnt;
        as_par_cnt = as_par_cnt.wrapping_add(1);
        let ref mut fresh3 = *as_params.offset(fresh2 as isize);
        *fresh3 = *argv.offset(i as isize) as *mut u8_0;
        i = i.wrapping_add(1)
    }
    /* __APPLE__ */
    input_file =
        *argv.offset((argc - 1 as libc::c_int) as isize) as *mut u8_0;
    if *input_file.offset(0 as libc::c_int as isize) as libc::c_int ==
           '-' as i32 {
        if strcmp(input_file.offset(1 as libc::c_int as isize) as
                      *const libc::c_char,
                  b"-version\x00" as *const u8 as *const libc::c_char) == 0 {
            just_version = 1 as libc::c_int as u8_0;
            modified_file = input_file;
            current_block = 6323139696330425089;
        } else {
            if *input_file.offset(1 as libc::c_int as isize) != 0 {
                printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mIncorrect use (not called through afl-gcc?)\x00"
                           as *const u8 as *const libc::c_char);
                printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       b"func_unknown\x00" as *const u8 as
                           *const libc::c_char,
                       b"src/afl-as.c\x00" as *const u8 as
                           *const libc::c_char, 199 as libc::c_int);
                exit(1 as libc::c_int);
            } else { input_file = 0 as *mut u8_0 }
            current_block = 18377268871191777778;
        }
    } else {
        /* Check if this looks like a standard invocation as a part of an attempt
       to compile a program, rather than using gcc on an ad-hoc .s file in
       a format we may not understand. This works around an issue compiling
       NSS. */
        if strncmp(input_file as *const libc::c_char,
                   tmp_dir as *const libc::c_char,
                   strlen(tmp_dir as *const libc::c_char)) != 0 &&
               strncmp(input_file as *const libc::c_char,
                       b"/var/tmp/\x00" as *const u8 as *const libc::c_char,
                       9 as libc::c_int as libc::c_ulong) != 0 &&
               strncmp(input_file as *const libc::c_char,
                       b"/tmp/\x00" as *const u8 as *const libc::c_char,
                       5 as libc::c_int as libc::c_ulong) != 0 &&
               getenv(b"AFL_AS_FORCE_INSTRUMENT\x00" as *const u8 as
                          *const libc::c_char).is_null() {
            pass_thru = 1 as libc::c_int as u8_0
        } else if !getenv(b"AFL_AS_FORCE_INSTRUMENT\x00" as *const u8 as
                              *const libc::c_char).is_null() {
            unsetenv(b"AFL_AS_FORCE_INSTRUMENT\x00" as *const u8 as
                         *const libc::c_char);
        }
        current_block = 18377268871191777778;
    }
    match current_block {
        18377268871191777778 => {
            modified_file =
                ({
                     let mut _tmp: *mut u8_0 = 0 as *mut u8_0;
                     let mut _len: s32 =
                         snprintf(0 as *mut libc::c_char,
                                  0 as libc::c_int as libc::c_ulong,
                                  b"%s/.afl-%u-%u.s\x00" as *const u8 as
                                      *const libc::c_char, tmp_dir, getpid(),
                                  time(0 as *mut time_t) as u32_0);
                     if _len < 0 as libc::c_int {
                         printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mWhoa, snprintf() fails?!\x00"
                                    as *const u8 as *const libc::c_char);
                         printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                                    as *const u8 as *const libc::c_char,
                                b"func_unknown\x00" as *const u8 as
                                    *const libc::c_char,
                                b"src/afl-as.c\x00" as *const u8 as
                                    *const libc::c_char, 221 as libc::c_int);
                         exit(1 as libc::c_int);
                     }
                     _tmp =
                         DFL_ck_alloc((_len + 1 as libc::c_int) as u32_0) as
                             *mut u8_0;
                     snprintf(_tmp as *mut libc::c_char,
                              (_len + 1 as libc::c_int) as libc::c_ulong,
                              b"%s/.afl-%u-%u.s\x00" as *const u8 as
                                  *const libc::c_char, tmp_dir, getpid(),
                              time(0 as *mut time_t) as u32_0);
                     _tmp
                 })
        }
        _ => { }
    }
    let fresh4 = as_par_cnt;
    as_par_cnt = as_par_cnt.wrapping_add(1);
    let ref mut fresh5 = *as_params.offset(fresh4 as isize);
    *fresh5 = modified_file;
    let ref mut fresh6 = *as_params.offset(as_par_cnt as isize);
    *fresh6 = 0 as *mut u8_0;
}
/* Process input file, generate modified_file. Insert instrumentation in all
   the appropriate places. */
unsafe extern "C" fn add_instrumentation() {
    static mut line: [u8_0; 8192] = [0; 8192];
    let mut inf: *mut FILE = 0 as *mut FILE;
    let mut outf: *mut FILE = 0 as *mut FILE;
    let mut outfd: s32 = 0;
    let mut ins_lines: u32_0 = 0 as libc::c_int as u32_0;
    let mut instr_ok: u8_0 = 0 as libc::c_int as u8_0;
    let mut skip_csect: u8_0 = 0 as libc::c_int as u8_0;
    let mut skip_next_label: u8_0 = 0 as libc::c_int as u8_0;
    let mut skip_intel: u8_0 = 0 as libc::c_int as u8_0;
    let mut skip_app: u8_0 = 0 as libc::c_int as u8_0;
    let mut instrument_next: u8_0 = 0 as libc::c_int as u8_0;
    /* __APPLE__ */
    if !input_file.is_null() {
        inf =
            fopen(input_file as *const libc::c_char,
                  b"r\x00" as *const u8 as *const libc::c_char);
        if inf.is_null() {
            fflush(stdout);
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to read \'%s\'\x00"
                       as *const u8 as *const libc::c_char, input_file);
            printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-as.c\x00" as *const u8 as *const libc::c_char,
                   254 as libc::c_int);
            printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
    } else { inf = stdin }
    outfd =
        open(modified_file as *const libc::c_char,
             0o1 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int,
             0o600 as libc::c_int);
    if outfd < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mUnable to write to \'%s\'\x00"
                   as *const u8 as *const libc::c_char, modified_file);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-as.c\x00" as *const u8 as *const libc::c_char,
               262 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    outf = fdopen(outfd, b"w\x00" as *const u8 as *const libc::c_char);
    if outf.is_null() {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mfdopen() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-as.c\x00" as *const u8 as *const libc::c_char,
               266 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    while !fgets(line.as_mut_ptr() as *mut libc::c_char, 8192 as libc::c_int,
                 inf).is_null() {
        /* In some cases, we want to defer writing the instrumentation trampoline
       until after all the labels, macros, comments, etc. If we're in this
       mode, and if the line starts with a tab followed by a character, dump
       the trampoline now. */
        if pass_thru == 0 && skip_intel == 0 && skip_app == 0 &&
               skip_csect == 0 && instr_ok as libc::c_int != 0 &&
               instrument_next as libc::c_int != 0 &&
               line[0 as libc::c_int as usize] as libc::c_int == '\t' as i32
               &&
               *(*__ctype_b_loc()).offset(line[1 as libc::c_int as usize] as
                                              libc::c_int as isize) as
                   libc::c_int &
                   _ISalpha as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
            fprintf(outf,
                    if use_64bit as libc::c_int != 0 {
                        trampoline_fmt_64
                    } else { trampoline_fmt_32 } as *const libc::c_char,
                    random() %
                        ((1 as libc::c_int) << 16 as libc::c_int) as
                            libc::c_long);
            instrument_next = 0 as libc::c_int as u8_0;
            ins_lines = ins_lines.wrapping_add(1)
        }
        /* Output the actual line, call it a day in pass-thru mode. */
        fputs(line.as_mut_ptr() as *const libc::c_char, outf);
        if pass_thru != 0 { continue ; }
        /* All right, this is where the actual fun begins. For one, we only want to
       instrument the .text section. So, let's keep track of that in processed
       files - and let's set instr_ok accordingly. */
        if line[0 as libc::c_int as usize] as libc::c_int == '\t' as i32 &&
               line[1 as libc::c_int as usize] as libc::c_int == '.' as i32 {
            /* OpenBSD puts jump tables directly inline with the code, which is
         a bit annoying. They use a specific format of p2align directives
         around them, so we use that as a signal. */
            if clang_mode == 0 && instr_ok as libc::c_int != 0 &&
                   strncmp(line.as_mut_ptr().offset(2 as libc::c_int as isize)
                               as *const libc::c_char,
                           b"p2align \x00" as *const u8 as
                               *const libc::c_char,
                           8 as libc::c_int as libc::c_ulong) == 0 &&
                   *(*__ctype_b_loc()).offset(line[10 as libc::c_int as usize]
                                                  as libc::c_int as isize) as
                       libc::c_int &
                       _ISdigit as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 &&
                   line[11 as libc::c_int as usize] as libc::c_int ==
                       '\n' as i32 {
                skip_next_label = 1 as libc::c_int as u8_0
            }
            if strncmp(line.as_mut_ptr().offset(2 as libc::c_int as isize) as
                           *const libc::c_char,
                       b"text\n\x00" as *const u8 as *const libc::c_char,
                       5 as libc::c_int as libc::c_ulong) == 0 ||
                   strncmp(line.as_mut_ptr().offset(2 as libc::c_int as isize)
                               as *const libc::c_char,
                           b"section\t.text\x00" as *const u8 as
                               *const libc::c_char,
                           13 as libc::c_int as libc::c_ulong) == 0 ||
                   strncmp(line.as_mut_ptr().offset(2 as libc::c_int as isize)
                               as *const libc::c_char,
                           b"section\t__TEXT,__text\x00" as *const u8 as
                               *const libc::c_char,
                           21 as libc::c_int as libc::c_ulong) == 0 ||
                   strncmp(line.as_mut_ptr().offset(2 as libc::c_int as isize)
                               as *const libc::c_char,
                           b"section __TEXT,__text\x00" as *const u8 as
                               *const libc::c_char,
                           21 as libc::c_int as libc::c_ulong) == 0 {
                instr_ok = 1 as libc::c_int as u8_0;
                continue ;
            } else if strncmp(line.as_mut_ptr().offset(2 as libc::c_int as
                                                           isize) as
                                  *const libc::c_char,
                              b"section\t\x00" as *const u8 as
                                  *const libc::c_char,
                              8 as libc::c_int as libc::c_ulong) == 0 ||
                          strncmp(line.as_mut_ptr().offset(2 as libc::c_int as
                                                               isize) as
                                      *const libc::c_char,
                                  b"section \x00" as *const u8 as
                                      *const libc::c_char,
                                  8 as libc::c_int as libc::c_ulong) == 0 ||
                          strncmp(line.as_mut_ptr().offset(2 as libc::c_int as
                                                               isize) as
                                      *const libc::c_char,
                                  b"bss\n\x00" as *const u8 as
                                      *const libc::c_char,
                                  4 as libc::c_int as libc::c_ulong) == 0 ||
                          strncmp(line.as_mut_ptr().offset(2 as libc::c_int as
                                                               isize) as
                                      *const libc::c_char,
                                  b"data\n\x00" as *const u8 as
                                      *const libc::c_char,
                                  5 as libc::c_int as libc::c_ulong) == 0 {
                instr_ok = 0 as libc::c_int as u8_0;
                continue ;
            }
        }
        /* Detect off-flavor assembly (rare, happens in gdb). When this is
       encountered, we set skip_csect until the opposite directive is
       seen, and we do not instrument. */
        if !strstr(line.as_mut_ptr() as *const libc::c_char,
                   b".code\x00" as *const u8 as *const libc::c_char).is_null()
           {
            if !strstr(line.as_mut_ptr() as *const libc::c_char,
                       b".code32\x00" as *const u8 as
                           *const libc::c_char).is_null() {
                skip_csect = use_64bit
            }
            if !strstr(line.as_mut_ptr() as *const libc::c_char,
                       b".code64\x00" as *const u8 as
                           *const libc::c_char).is_null() {
                skip_csect = (use_64bit == 0) as libc::c_int as u8_0
            }
        }
        /* Detect syntax changes, as could happen with hand-written assembly.
       Skip Intel blocks, resume instrumentation when back to AT&T. */
        if !strstr(line.as_mut_ptr() as *const libc::c_char,
                   b".intel_syntax\x00" as *const u8 as
                       *const libc::c_char).is_null() {
            skip_intel = 1 as libc::c_int as u8_0
        }
        if !strstr(line.as_mut_ptr() as *const libc::c_char,
                   b".att_syntax\x00" as *const u8 as
                       *const libc::c_char).is_null() {
            skip_intel = 0 as libc::c_int as u8_0
        }
        /* Detect and skip ad-hoc __asm__ blocks, likewise skipping them. */
        if line[0 as libc::c_int as usize] as libc::c_int == '#' as i32 ||
               line[1 as libc::c_int as usize] as libc::c_int == '#' as i32 {
            if !strstr(line.as_mut_ptr() as *const libc::c_char,
                       b"#APP\x00" as *const u8 as
                           *const libc::c_char).is_null() {
                skip_app = 1 as libc::c_int as u8_0
            }
            if !strstr(line.as_mut_ptr() as *const libc::c_char,
                       b"#NO_APP\x00" as *const u8 as
                           *const libc::c_char).is_null() {
                skip_app = 0 as libc::c_int as u8_0
            }
        }
        /* If we're in the right mood for instrumenting, check for function
       names or conditional labels. This is a bit messy, but in essence,
       we want to catch:

         ^main:      - function entry point (always instrumented)
         ^.L0:       - GCC branch label
         ^.LBB0_0:   - clang branch label (but only in clang mode)
         ^\tjnz foo  - conditional branches

       ...but not:

         ^# BB#0:    - clang comments
         ^ # BB#0:   - ditto
         ^.Ltmp0:    - clang non-branch labels
         ^.LC0       - GCC non-branch labels
         ^.LBB0_0:   - ditto (when in GCC mode)
         ^\tjmp foo  - non-conditional jumps

       Additionally, clang and GCC on MacOS X follow a different convention
       with no leading dots on labels, hence the weird maze of #ifdefs
       later on.

     */
        if skip_intel as libc::c_int != 0 || skip_app as libc::c_int != 0 ||
               skip_csect as libc::c_int != 0 || instr_ok == 0 ||
               line[0 as libc::c_int as usize] as libc::c_int == '#' as i32 ||
               line[0 as libc::c_int as usize] as libc::c_int == ' ' as i32 {
            continue ;
        }
        /* Conditional branch instruction (jnz, etc). We append the instrumentation
       right after the branch (to instrument the not-taken path) and at the
       branch destination label (handled later on). */
        if line[0 as libc::c_int as usize] as libc::c_int == '\t' as i32 {
            if line[1 as libc::c_int as usize] as libc::c_int == 'j' as i32 &&
                   line[2 as libc::c_int as usize] as libc::c_int !=
                       'm' as i32 &&
                   (random() % 100 as libc::c_int as libc::c_long) <
                       inst_ratio as libc::c_long {
                fprintf(outf,
                        if use_64bit as libc::c_int != 0 {
                            trampoline_fmt_64
                        } else { trampoline_fmt_32 } as *const libc::c_char,
                        random() %
                            ((1 as libc::c_int) << 16 as libc::c_int) as
                                libc::c_long);
                ins_lines = ins_lines.wrapping_add(1)
            }
        } else if !strstr(line.as_mut_ptr() as *const libc::c_char,
                          b":\x00" as *const u8 as
                              *const libc::c_char).is_null() {
            if line[0 as libc::c_int as usize] as libc::c_int == '.' as i32 {
                /* Label of some sort. This may be a branch destination, but we need to
       read carefully and account for several different formatting
       conventions. */
                /* Everybody else: .L<whatever>: */
                /* __APPLE__ */
                /* .L0: or LBB0_0: style jump destination */
                /* Apple: .L<num> / .LBB<num> */
                if (*(*__ctype_b_loc()).offset(line[2 as libc::c_int as usize]
                                                   as libc::c_int as isize) as
                        libc::c_int &
                        _ISdigit as libc::c_int as libc::c_ushort as
                            libc::c_int != 0 ||
                        clang_mode as libc::c_int != 0 &&
                            strncmp(line.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                                        *const libc::c_char,
                                    b"LBB\x00" as *const u8 as
                                        *const libc::c_char,
                                    3 as libc::c_int as libc::c_ulong) == 0)
                       &&
                       (random() % 100 as libc::c_int as libc::c_long) <
                           inst_ratio as libc::c_long {
                    /* __APPLE__ */
                    /* An optimization is possible here by adding the code only if the
             label is mentioned in the code in contexts other than call / jmp.
             That said, this complicates the code by requiring two-pass
             processing (messy with stdin), and results in a speed gain
             typically under 10%, because compilers are generally pretty good
             about not generating spurious intra-function jumps.

             We use deferred output chiefly to avoid disrupting
             .Lfunc_begin0-style exception handling calculations (a problem on
             MacOS X). */
                    if skip_next_label == 0 {
                        instrument_next = 1 as libc::c_int as u8_0
                    } else { skip_next_label = 0 as libc::c_int as u8_0 }
                }
            } else {
                /* Function label (always instrumented, deferred mode). */
                instrument_next = 1 as libc::c_int as u8_0
            }
        }
    }
    if ins_lines != 0 {
        fputs(if use_64bit as libc::c_int != 0 {
                  main_payload_64
              } else { main_payload_32 } as *const libc::c_char, outf);
    }
    if !input_file.is_null() { fclose(inf); }
    fclose(outf);
    if be_quiet == 0 {
        if ins_lines == 0 {
            printf(b"\x1b[1;93m[!] \x1b[1;97mWARNING: \x1b[0mNo instrumentation targets found%s.\x00"
                       as *const u8 as *const libc::c_char,
                   if pass_thru as libc::c_int != 0 {
                       b" (pass-thru mode)\x00" as *const u8 as
                           *const libc::c_char
                   } else { b"\x00" as *const u8 as *const libc::c_char });
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        } else {
            let mut modeline: [libc::c_char; 100] = [0; 100];
            snprintf(modeline.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 100]>() as
                         libc::c_ulong,
                     b"%s%s%s%s\x00" as *const u8 as *const libc::c_char,
                     if !getenv(b"AFL_HARDEN\x00" as *const u8 as
                                    *const libc::c_char).is_null() {
                         b"hardened\x00" as *const u8 as *const libc::c_char
                     } else {
                         b"non-hardened\x00" as *const u8 as
                             *const libc::c_char
                     },
                     if !getenv(b"AFL_USE_ASAN\x00" as *const u8 as
                                    *const libc::c_char).is_null() {
                         b", ASAN\x00" as *const u8 as *const libc::c_char
                     } else { b"\x00" as *const u8 as *const libc::c_char },
                     if !getenv(b"AFL_USE_MSAN\x00" as *const u8 as
                                    *const libc::c_char).is_null() {
                         b", MSAN\x00" as *const u8 as *const libc::c_char
                     } else { b"\x00" as *const u8 as *const libc::c_char },
                     if !getenv(b"AFL_USE_UBSAN\x00" as *const u8 as
                                    *const libc::c_char).is_null() {
                         b", UBSAN\x00" as *const u8 as *const libc::c_char
                     } else { b"\x00" as *const u8 as *const libc::c_char });
            printf(b"\x1b[1;92m[+] \x1b[0mInstrumented %u locations (%s-bit, %s mode, ratio %u%%).\x00"
                       as *const u8 as *const libc::c_char, ins_lines,
                   if use_64bit as libc::c_int != 0 {
                       b"64\x00" as *const u8 as *const libc::c_char
                   } else { b"32\x00" as *const u8 as *const libc::c_char },
                   modeline.as_mut_ptr(), inst_ratio);
            printf(b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char);
        }
    };
}
/* Main entry point */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut pid: s32 = 0;
    let mut rand_seed: u32_0 = 0;
    let mut status: libc::c_int = 0;
    let mut inst_ratio_str: *mut u8_0 =
        getenv(b"AFL_INST_RATIO\x00" as *const u8 as *const libc::c_char) as
            *mut u8_0;
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut tz: timezone = timezone{tz_minuteswest: 0, tz_dsttime: 0,};
    clang_mode =
        !getenv(b"__AFL_CLANG_MODE\x00" as *const u8 as
                    *const libc::c_char).is_null() as libc::c_int as u8_0;
    if isatty(2 as libc::c_int) != 0 &&
           getenv(b"AFL_QUIET\x00" as *const u8 as
                      *const libc::c_char).is_null() ||
           !getenv(b"AFL_DEBUG\x00" as *const u8 as
                       *const libc::c_char).is_null() {
        printf(b"\x1b[0;36mafl-as++2.63d\x1b[0m by Michal Zalewski\n\x00" as
                   *const u8 as *const libc::c_char);
    } else { be_quiet = 1 as libc::c_int as u8_0 }
    if argc < 2 as libc::c_int ||
           argc == 2 as libc::c_int &&
               strcmp(*argv.offset(1 as libc::c_int as isize),
                      b"-h\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
        fprintf(stdout,
                b"afl-as++2.63d by Michal Zalewski\n\n%s [-h]\n\nThis is a helper application for afl-fuzz. It is a wrapper around GNU \'as\',\nexecuted by the toolchain whenever using afl-gcc or afl-clang. You probably\ndon\'t want to run this program directly.\n\nRarely, when dealing with extremely complex projects, it may be advisable\nto set AFL_INST_RATIO to a value less than 100 in order to reduce the\nodds of instrumenting every discovered branch.\n\nEnvironment variables used:\nAFL_AS: path to assembler to use for instrumented files\nAFL_CC: fall back path to assembler\nAFL_CXX: fall back path to assembler\nTMPDIR: directory to use for temporary files\nTEMP: fall back path to directory for temporary files\nTMP: fall back path to directory for temporary files\nAFL_INST_RATIO: user specified instrumentation ratio\nAFL_QUIET: suppress verbose output\nAFL_KEEP_ASSEMBLY: leave instrumented assembly files\nAFL_AS_FORCE_INSTRUMENT: force instrumentation for asm sources\nAFL_HARDEN, AFL_USE_ASAN, AFL_USE_MSAN, AFL_USE_UBSAN:\n  used in the instrumentation summary message\n\x00"
                    as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize));
        exit(1 as libc::c_int);
    }
    gettimeofday(&mut tv, &mut tz as *mut timezone as *mut libc::c_void);
    rand_seed = (tv.tv_sec ^ tv.tv_usec ^ getpid() as libc::c_long) as u32_0;
    srandom(rand_seed);
    edit_params(argc, argv);
    if !inst_ratio_str.is_null() {
        if sscanf(inst_ratio_str as *const libc::c_char,
                  b"%u\x00" as *const u8 as *const libc::c_char,
                  &mut inst_ratio as *mut u32_0) != 1 as libc::c_int ||
               inst_ratio > 100 as libc::c_int as libc::c_uint {
            printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mBad value of AFL_INST_RATIO (must be between 0 and 100)\x00"
                       as *const u8 as *const libc::c_char);
            printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                       as *const u8 as *const libc::c_char,
                   b"func_unknown\x00" as *const u8 as *const libc::c_char,
                   b"src/afl-as.c\x00" as *const u8 as *const libc::c_char,
                   569 as libc::c_int);
            exit(1 as libc::c_int);
        }
    }
    if !getenv(b"__AFL_AS_LOOPCHECK\x00" as *const u8 as
                   *const libc::c_char).is_null() {
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mEndless loop when calling \'as\' (remove \'.\' from your PATH)\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-as.c\x00" as *const u8 as *const libc::c_char,
               574 as libc::c_int);
        exit(1 as libc::c_int);
    }
    setenv(b"__AFL_AS_LOOPCHECK\x00" as *const u8 as *const libc::c_char,
           b"1\x00" as *const u8 as *const libc::c_char, 1 as libc::c_int);
    /* When compiling with ASAN, we don't have a particularly elegant way to skip
     ASAN-specific branches. But we can probabilistically compensate for
     that... */
    if !getenv(b"AFL_USE_ASAN\x00" as *const u8 as
                   *const libc::c_char).is_null() ||
           !getenv(b"AFL_USE_MSAN\x00" as *const u8 as
                       *const libc::c_char).is_null() {
        sanitizer = 1 as libc::c_int as u8_0;
        if getenv(b"AFL_INST_RATIO\x00" as *const u8 as
                      *const libc::c_char).is_null() {
            inst_ratio =
                (inst_ratio as
                     libc::c_uint).wrapping_div(3 as libc::c_int as
                                                    libc::c_uint) as u32_0 as
                    u32_0
        }
    }
    if just_version == 0 { add_instrumentation(); }
    pid = fork();
    if pid == 0 {
        execvp(*as_params.offset(0 as libc::c_int as isize) as
                   *const libc::c_char,
               as_params as *mut *mut libc::c_char as
                   *const *mut libc::c_char);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-] PROGRAM ABORT : \x1b[0mOops, failed to execute \'%s\' - check your PATH\x00"
                   as *const u8 as *const libc::c_char,
               *as_params.offset(0 as libc::c_int as isize));
        printf(b"\x1b[1;91m\n         Location : \x1b[0m%s(), %s:%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-as.c\x00" as *const u8 as *const libc::c_char,
               594 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if pid < 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mfork() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-as.c\x00" as *const u8 as *const libc::c_char,
               598 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    if waitpid(pid, &mut status, 0 as libc::c_int) <= 0 as libc::c_int {
        fflush(stdout);
        printf(b"\x0f\x1b)B\x1b[?25h\x1b[0m\x1b[1;91m\n[-]  SYSTEM ERROR : \x1b[0mwaitpid() failed\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"\x1b[1;91m\n    Stop location : \x1b[0m%s(), %s:%u\n\x00" as
                   *const u8 as *const libc::c_char,
               b"func_unknown\x00" as *const u8 as *const libc::c_char,
               b"src/afl-as.c\x00" as *const u8 as *const libc::c_char,
               600 as libc::c_int);
        printf(b"\x1b[1;91m       OS message : \x1b[0m%s\n\x00" as *const u8
                   as *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    if getenv(b"AFL_KEEP_ASSEMBLY\x00" as *const u8 as
                  *const libc::c_char).is_null() {
        unlink(modified_file as *const libc::c_char);
    }
    exit((status & 0xff00 as libc::c_int) >> 8 as libc::c_int);
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
