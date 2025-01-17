#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]
#![feature(format_args_nl)]
#![allow(clippy::empty_loop)]

mod arch;
mod kpanic;
mod limine_info;

use core::{time::Duration, u128};

use arch::cpu::initialize;
use spin::Mutex;

#[macro_use]
mod log;

#[derive(Debug)]
enum KernelMode {
    /// Kernel is in early boot stage, assume no hardware is discovered and setup yet.
    EarlyBoot,
    /// Kernel has found and initialized hardware, but is still in single-user mode.
    SingleUser,
    /// Kernel is fully setup and initialized, this is the normal mode under most circumstances.
    MultiUser,
    /// A non fatal non recoverable error has occurred, the system sould inform the user and disable the offending module(s).
    Wounded,
    /// BSOD has occurred, the system should panic and halt.
    Panicked,
}

static KERNEL_MODE: Mutex<KernelMode> = Mutex::new(KernelMode::EarlyBoot);
static MONOTONIC_CLOCK: Mutex<u128> = Mutex::new(0);

#[no_mangle]
#[link_section = ".boot.text"]
unsafe extern "C" fn kentry() -> ! {
    println!("Kernel is in {:?}", KERNEL_MODE);
    println!("Will now attempt hardware discovery and initialization...");
    initialize();
    *KERNEL_MODE.lock() = KernelMode::SingleUser;
    kmain();
}

#[allow(unreachable_code)]
fn kmain() -> ! {
    println!("Hello from Ferrux!");
    loop {}
    // Catch this and halt
    panic!("kmain reached after the main loop");
}
