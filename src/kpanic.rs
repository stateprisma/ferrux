use core::panic::PanicInfo;
use crate::arch::cpu;


#[panic_handler]
#[allow(unreachable_code)]
fn kpanic(_info: &PanicInfo) -> ! {
    loop {}
    unsafe { cpu::halt() };
}
