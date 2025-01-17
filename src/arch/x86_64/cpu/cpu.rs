use spin::Mutex;

use crate::println;

pub static CPU: Mutex<Cpu> = Mutex::new(Cpu::new());

#[derive(Debug)]
pub struct CpuInfo {
    pub ext_family: u8,
    pub ext_model: u8,
    pub family: u8,
    pub model: u8,
    pub stepping: u8,
}

#[derive(Debug)]
pub struct Cpu {
    pub id: u8,
    pub apic_id: u8,
    pub bus_speed: u64,
    pub vendor_id: [u8; 12],
    pub cpu_info: CpuInfo,
}

impl Cpu {
    pub const fn new() -> Cpu {
        Cpu {
            id: 0,
            apic_id: 0,
            bus_speed: 0,
            vendor_id: [0; 12],
            cpu_info: CpuInfo {
                ext_family: 0,
                ext_model: 0,
                family: 0,
                model: 0,
                stepping: 0,
            },
        }
    }

    pub fn print(&self) {
        println!("CPU ID: {}", self.id);
        println!("APIC ID: {}", self.apic_id);
        println!(
            "Bus Speed: {}MHz ({}Hz)",
            self.bus_speed / 1_000_000,
            self.bus_speed
        );
        println!(
            "Vendor ID: {}",
            core::str::from_utf8(&self.vendor_id).unwrap()
        );
        println!("CPU Info: {:?}", self.cpu_info);
    }
}
