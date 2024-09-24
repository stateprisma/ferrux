#![no_std]
#![no_main]

mod kpanic;

mod limine_info;

#[cfg(target_arch = "aarch64")]
use aarch64::cpu;
#[cfg(target_arch = "x86_64")]
use amd64::cpu;

#[no_mangle]
unsafe extern "C" fn kentry() -> ! {
    // cpu::setup();
    kmain();
}

#[allow(unreachable_code)]
fn kmain() -> ! {
    loop {}
    // Catch this and halt
    panic!("kmain reached after the main loop");
}
