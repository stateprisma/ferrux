use crate::arch::asm_are_interrupts_enabled;
use crate::arch::interrupts::apic::Apic;
use crate::arch::interrupts::vectors::IV_HANDLERS;
use crate::arch::interrupts::{irq_disable, irq_restore};
use core::arch::global_asm;
use core::cell::RefCell;
use spin::{Lazy, Mutex};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;

fn default_handler(vector: u64) {
    // TODO: Add logger here
    Apic::signal_eoi();
}

static mut IV_HANDLER_FNS: [fn(vector: u64); 224] = [default_handler; 224];

extern "C" {
    pub(crate) fn isr_dummy();
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum IntIdx {
    Timer = 0x20,
}

pub fn load_handlers() {
    let mut idt = IDT_STATE.lock();
    idt.double_fault.set_handler_fn(double_fault_handler);
    idt.general_protection_fault.set_handler_fn(gpf_handler);
    let mut idt_slice = idt.slice_mut(32..255);

    if asm_are_interrupts_enabled() {
        irq_disable();
    }
    for (gate, h) in IV_HANDLERS.iter().enumerate() {
        let i = gate + 32;
        // idt_slice[i].set_handler_fn(h);
    }

    if !asm_are_interrupts_enabled() {
        irq_restore();
    }
}

#[no_mangle]
pub extern "C" fn isr_handler(vector: u64) {
    let called_by = vector - 32;
    unsafe {
        let h = IV_HANDLER_FNS[called_by as usize];
        h(vector);
    }
}

pub fn register_iv_handler(h: fn(vector: u64), vector: u8) {
    if asm_are_interrupts_enabled() {
        irq_disable();
    }
    if vector < 32 {
        panic!("Cannot set vector handler lower than 32");
    }
    unsafe {
        let idx = (vector - 32) as usize;
        IV_HANDLER_FNS[idx] = h;
        // TODO: logln!("setup handler for {}", idx);
    }
    if !asm_are_interrupts_enabled() {
        irq_restore();
    }
}

global_asm! {
    include_str!("isa_handler.asm"),
}

extern "x86-interrupt" fn double_fault_handler(_: InterruptStackFrame, code: u64) -> ! {
    panic!("Double fault was raised with code: {}", code)
}

extern "x86-interrupt" fn gpf_handler(_: InterruptStackFrame, code: u64) {
    // TODO: Log the gpf and some context here
}


