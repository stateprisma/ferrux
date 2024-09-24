use core::panic::PanicInfo;

#[cfg(target_arch = "aarch64")]
use aarch64::cpu;
#[cfg(target_arch = "x86_64")]
use amd64::cpu;

#[panic_handler]
#[allow(unreachable_code)]
fn kpanic(info: &PanicInfo) -> ! {
    loop {}
    unsafe { cpu::halt() };
}
