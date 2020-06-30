//! ckb-std
//!
//! # Modules
//!
//! * `high_level` module: defines high level syscall API
//! * `syscalls` module: defines low level [CKB syscalls](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0009-vm-syscalls/0009-vm-syscalls.md)
//! * `debug!` macro: a `println!` like macro helps debugging
//! * `entry!` macro: defines contract entry point
//! * `default_alloc!` and `libc_alloc!` macro: defines global allocator for no-std rust
#![feature(no_core)]
#![feature(lang_items)]
#![feature(optin_builtin_traits, link_args, start)]
#![no_core]

#[lang = "sized"]
pub trait Sized {}
#[lang = "copy"]
pub trait Copy {}
#[lang = "freeze"]
auto trait Freeze {}

#[link(name = "ckb-syscall")]
extern "C" {
    pub fn syscall(a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64) -> u64;
}

pub const SYS_EXIT: u64 = 93;

/// Exit, this script will be terminated after the exit syscall.
/// exit code `0` represents verification is success, others represent error code.
pub fn exit(code: i8) -> ! {
    unsafe { syscall(code as u64, 0, 0, 0, 0, 0, 0, SYS_EXIT) };
    loop {}
}
