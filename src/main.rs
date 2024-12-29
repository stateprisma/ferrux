#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]
#![feature(format_args_nl)]
#![allow(clippy::empty_loop)]

mod arch;
mod kpanic;
mod limine_info;

use arch::cpu::initialize;

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

static mut KERNEL_MODE: KernelMode = KernelMode::EarlyBoot;

#[no_mangle]
#[link_section = ".boot.text"]
unsafe extern "C" fn kentry() -> ! {
    println!("Kernel is in {:?}", KERNEL_MODE);
    println!("Will now attempt hardware discovery and initialization...");
    initialize();
    kmain();
}

#[allow(unreachable_code)]
fn kmain() -> ! {
    println!("Hello from Ferrux!");
    loop {}
    // Catch this and halt
    panic!("kmain reached after the main loop");
}
