// Lema OS kernel — library root.
//
// Phase 0: a tiny `no_std` surface that compiles against the freestanding
// `x86_64-unknown-none` target. Phase 1+ will add subsystems here.

#![no_std]

use core::panic::PanicInfo;

/// Called once by `_start` after Limine has handed us control.
pub fn init() {
    // Phase 0: nothing to initialize. We're a skeleton that halts.
}

/// Halt the CPU until the next interrupt, forever.
pub fn hlt_loop() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}

/// Panic handler. Phase 3 will replace this with a framebuffer panic screen.
pub fn panic_handler(_info: &PanicInfo) -> ! {
    // Phase 0: just halt. We can't print anywhere yet.
    hlt_loop()
}