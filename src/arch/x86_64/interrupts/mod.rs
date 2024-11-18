//! This module helps detect and initialize the hardware timers present in the LP
//!

use core::arch::asm;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::idt::InterruptDescriptorTable;

mod apic;
mod apic_consts;
mod isa_handler;
mod vectors;

lazy_static!{
    pub(crate) static ref IDT_STATE: Mutex<InterruptDescriptorTable> = {
        let idt = InterruptDescriptorTable::new();
        Mutex::new(idt)
    };
}

pub fn irq_disable() {
    unsafe {
        asm!("cli");
    };
}

pub fn irq_restore() {
    unsafe {
        asm!("sti");
    };
}

pub fn initialize_interrupts() {
    let idt = IDT_STATE.lock();
    idt.load();
}
