use core::arch::global_asm;

global_asm!{
    include_str!("global.asm")
}

extern "C" {
    pub(super) fn asm_get_vendor_string(dest: &mut [u8;12]);
}