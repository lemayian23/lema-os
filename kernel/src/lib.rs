// Lema OS kernel — library root.

#![no_std]

use core::panic::PanicInfo;

pub fn init() {
    // Phase 2+ will set up serial, framebuffer, GDT, IDT, etc.
    // For now, our kernel just halts.
}

pub fn hlt_loop() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}

pub fn panic_handler(_info: &PanicInfo) -> ! {
    hlt_loop()
}