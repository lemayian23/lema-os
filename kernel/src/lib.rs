// Lema OS kernel — library root.

#![no_std]

use core::panic::PanicInfo;

#[used]
pub static FRAMEBUFFER_REQUEST: limine::FramebufferRequest =
    limine::FramebufferRequest::new(0);

#[used]
pub static HHDM_REQUEST: limine::HhdmRequest =
    limine::HhdmRequest::new(0);

#[used]
pub static MEMORY_MAP_REQUEST: limine::MemmapRequest =
    limine::MemmapRequest::new(0);

pub fn init() {
    // Phase 1 will use FRAMEBUFFER_REQUEST to draw "Lema v0.1" on screen.
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