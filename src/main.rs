#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]
#![feature(format_args_nl)]

mod kpanic;
mod arch;
mod limine_info;

#[macro_use]
mod log;

#[no_mangle]
#[link_section = ".boot.text"]
unsafe extern "C" fn kentry() -> ! {
    arch::initialize();
}

#[allow(unreachable_code)]
fn kmain() -> ! {
    println!("Hello from prisma");
    loop {}
    // Catch this and halt
    panic!("kmain reached after the main loop");
}
