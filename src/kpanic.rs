use core::panic::PanicInfo;
use crate::arch::cpu;


#[panic_handler]
#[allow(unreachable_code)]
fn kpanic(info: &PanicInfo) -> ! {
    loop {}
    unsafe { cpu::halt() };
}
