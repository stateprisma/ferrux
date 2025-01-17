use crate::arch::cpu::cpu::CPU;
use crate::arch::cpu::cpu_model_fam_stepping;
use core::arch::x86_64::{__rdtscp, _mm_lfence, _mm_pause, _rdtsc};
use core::time::Duration;

use crate::{arch::cpu::asm::asm_get_cpu_string, println};

pub fn initialize() {
    println!("Initialize the bootstrap CPU:");
    let vendor = &mut [0u8; 12];
    unsafe {
        asm_get_cpu_string(vendor);
    };
    let bus_speed = calculate_ticks_per_second();
    let mut cpu_info = CPU.lock();

    cpu_info.bus_speed = bus_speed;
    cpu_info.vendor_id = *vendor;
    cpu_info.cpu_info = unsafe { cpu_model_fam_stepping() };

    cpu_info.print();

    println!("Bootstrap CPU initialized");
}

fn measure_tsc_duration(duration: Duration) -> u64 {
    unsafe {
        let sec = Duration::from_secs(1);

        _mm_lfence(); // Serialize
        let start_tsc = __rdtscp(&mut 0);
        _mm_lfence(); // Serialize

        let start_time = _rdtsc();

        // Busy-wait loop for the specified duration
        let end_time = start_time + duration.as_nanos() as u64;
        while _rdtsc() < end_time {
            _mm_pause();
        }

        _mm_lfence(); // Serialize
        let end_tsc = __rdtscp(&mut 0);
        _mm_lfence(); // Serialize
        (end_tsc - start_tsc) * sec.as_millis() as u64
    }
}

fn calculate_bus_speed(ticks: u64, duration: Duration) -> u64 {
    ticks / duration.as_millis() as u64
}

fn calculate_ticks_per_second() -> u64 {
    let duration = Duration::from_micros(100);
    let ticks = measure_tsc_duration(duration);
    calculate_bus_speed(ticks, duration)
}
