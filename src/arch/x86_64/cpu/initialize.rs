use core::str;

use crate::{arch::cpu::asm::asm_get_cpu_string, println};

pub fn initialize() {
    println!("Initialize the CPU");
    let vendor = &mut [0u8; 12];
    let vendor_str: &str;
    unsafe {
        asm_get_cpu_string(vendor);
        vendor_str = core::str::from_utf8_unchecked(vendor);
    };
    println!("CPU Vendor: {}", vendor_str);
}
