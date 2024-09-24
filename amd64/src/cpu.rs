use core::arch::asm;

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
