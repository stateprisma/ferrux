#![no_std]
#![no_main]

mod kpanic;
mod arch;
mod limine_info;

#[no_mangle]
unsafe extern "C" fn kentry() -> ! {
    kmain();
}

#[allow(unreachable_code)]
fn kmain() -> ! {
    loop {}
    // Catch this and halt
    panic!("kmain reached after the main loop");
}
