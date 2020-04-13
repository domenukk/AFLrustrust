#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]


#[macro_use]
extern crate c2rust_bitfields;#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;



pub mod src {
pub mod afl_as;
pub mod afl_common;
pub mod afl_forkserver;
pub mod afl_fuzz_bitmap;
pub mod afl_fuzz_cmplog;
pub mod afl_fuzz_extras;
pub mod afl_fuzz_init;
pub mod afl_fuzz_mutators;
pub mod afl_fuzz_one;
pub mod afl_fuzz_python;
pub mod afl_fuzz_queue;
pub mod afl_fuzz_redqueen;
pub mod afl_fuzz_run;
pub mod afl_fuzz_state;
pub mod afl_fuzz_stats;
pub mod afl_gcc;
pub mod afl_gotcpu;
pub mod afl_sharedmem;
} // mod src
pub mod test_instr;

