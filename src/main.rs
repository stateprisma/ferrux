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
    EarlyBoot,
    SingleUser,
    MultiUser,
    Wounded,
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
