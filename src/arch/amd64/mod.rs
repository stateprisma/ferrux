use core::arch::asm;
use core::arch::x86_64::__cpuid_count;

use crate::arch::global_asm::asm_get_vendor_string;
use cpu::{asm_read_msr, asm_write_msr, MSRValue};
use spin::Lazy;

pub mod cpu;
pub mod global_asm;
pub mod interrupts;

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

pub fn set_msr_bit(msr: u32, bit: u8) {
    let mut val = read_msr(msr);
    val.edx |= 1 << bit;
    write_msr(msr, val);
}

pub fn clear_msr_bit(msr: u32, bit: u8) {
    let mut val = read_msr(msr);
    val.edx &= !(1 << bit);
    write_msr(msr, val);
}

/// Test the flags of the processor to determine if the interrupts are enabled
pub fn asm_are_interrupts_enabled() -> bool {
    let mut flags: u64;
    unsafe { asm!("pushf\n\tpop {}", out(reg) flags) };
    (flags & 1 << 9) != 0
}

pub fn asm_outb(port: u16, val: u8) {
    unsafe {
        asm!(
        "
            out dx, al
        ",
        in("dx") port,
        in("al") val,
        );
    }
}

pub fn asm_inb(port: u16) -> u8 {
    let val: u8;
    unsafe {
        asm!(
        "
            in al, dx
        ",
        out("al") val,
        in("dx") port,
        );
    }
    val
}

/// outputs `word` to `port`
pub fn asm_outw(port: u16, word: u16) {
    unsafe {
        asm!(
        "
            out dx, ax
        ",
        in("dx") port,
        in("ax") word,
        );
    }
}

/// outputs `dword` to `port`
pub fn asm_outdw(port: u16, dword: u32) {
    unsafe {
        asm!(
        "
            out dx, eax
        ",
        in("dx") port,
        in("eax") dword,
        );
    }
}

pub fn asm_inw(port: u16) -> u16 {
    let word: u16;
    unsafe {
        asm!(
        "
            in ax, dx
        ",
        out("ax") word,
        in("dx") port,
        );
    }
    word
}

pub fn asm_indw(port: u16) -> u32 {
    let dword: u32;
    unsafe {
        asm!(
        "
            in eax, dx
        ",
        out("eax") dword,
        in("dx") port,
        );
    }
    dword
}

pub fn get_vendor_str(dest: &mut [u8; 12]) {
    unsafe {
        asm_get_vendor_string(dest);
    }
}
