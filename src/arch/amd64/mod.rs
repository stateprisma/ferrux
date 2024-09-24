use core::arch::x86_64::__cpuid_count;

use cpu::{asm_read_msr, asm_write_msr, MSRValue};
use spin::Lazy;

pub mod cpu;

pub static CPU_HAS_MSR: Lazy<bool> = Lazy::new(|| {
    let res = unsafe { __cpuid_count(0, 0) };
    res.edx & 1 << 5 != 0
});

pub fn read_msr(msr: u32) -> MSRValue {
    if !*CPU_HAS_MSR {
        panic!("Processor lacks msr support and read_msr was called!");
    }
    unsafe { asm_read_msr(msr) }
}

pub fn write_msr(msr: u32, value: MSRValue) {
    if !*CPU_HAS_MSR {
        panic!("Processor lacks msr support and write_msr was called!");
    }
    unsafe { asm_write_msr(msr, value) };
}
