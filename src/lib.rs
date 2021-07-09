#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(asm)]

pub mod gdt;
pub mod interrupts;
pub mod vga;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
