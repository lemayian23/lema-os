// Lema OS kernel — entry point.
//
// Phase 0: minimal. The kernel doesn't actually load yet — this file exists
// so `cargo check` succeeds against the workspace and Phase 1 has a clean
// place to land `_start`.
//
// Phase 1 will replace this with Limine request structures (framebuffer,
// memory map, RSDP) and a real `_start` symbol that the bootloader jumps to.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Kernel entry point. Called by Limine after it has switched the CPU to
/// long mode, set up initial page tables, and loaded our ELF.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    lema::init();
    lema::hlt_loop()
}

/// Panic handler. Phase 0 just halts. Phase 3 will replace this with a proper
/// framebuffer panic screen showing the panic location and message.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lema::panic_handler(info)
}