#![allow(clippy::undocumented_unsafe_blocks)]

use core::{arch::asm, arch::x86_64::__cpuid_count, mem::transmute};
use spin::Lazy;

pub unsafe fn halt() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

pub struct MSRValue {
    pub eax: u32,
    pub edx: u32,
}

pub unsafe fn asm_read_msr(selector: u32) -> MSRValue {
    let mut eax: u32;
    let mut edx: u32;

    asm!(
        "rdmsr",
        in("ecx") selector,
        out("eax") eax,
        out("edx") edx,
    );

    MSRValue { eax, edx }
}

pub unsafe fn asm_write_msr(selector: u32, value: MSRValue) {
    asm!(
        "wrmsr",
        in("ecx") selector,
        in("eax") value.eax,
        in("edx") value.edx,
    );
}

pub unsafe fn asm_get_ring() -> u8 {
    let mut ring: u32;
    asm!(
        "mov cs, eax
        and eax, 3", out("eax") ring,
    );

    ring as u8
}

pub unsafe fn asm_get_cpu_string(result: &mut [u8; 12]) {
    let (mut a, mut b, mut c);

    asm!(
        "
	    push rbx //preserve rbx
	    mov eax, 0 //eax = leaf
	    mov ecx, 0 //ecx = subleaf
	    cpuid
	    mov eax, ebx // Saves ebx into eax to avoid compiler issues
	    pop rbx //restore rbx
        ",
        out("eax") a,
        out("edx") b,
        out("ecx") c,
    );
    let raw = transmute::<[u32; 3], [u8; 12]>([a, b, c]);
    // Coppy the result into the result buffer
    *result = raw;
}

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

/// reads a word from `port`
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

/// reads a dword from `port`
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
