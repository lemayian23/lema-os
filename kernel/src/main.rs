#![no_std]
#![no_main]

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;
use limine::BaseRevision;

static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".limine_requests_start")]
static LIMINE_REQUESTS_START: limine::RequestsStartMarker = limine::RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".limine_requests_end")]
static LIMINE_REQUESTS_END: limine::RequestsEndMarker = limine::RequestsEndMarker::new();

#[unsafe(link_section = ".limine_requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Get framebuffer from Limine
    let fb_response = FRAMEBUFFER_REQUEST.response()
        .expect("No framebuffer response");
    
    let framebuffers = fb_response.framebuffers();
    let fb = framebuffers.first().expect("No framebuffer");
    
    let addr = fb.address() as *mut u8;
    let width = fb.width as usize;
    let height = fb.height as usize;
    let pitch = fb.pitch as usize;
    let bpp = fb.bpp as usize;

    // Paint a test pattern: blue gradient
    for y in 0..height {
        for x in 0..width {
            let pixel_offset = y * pitch + x * (bpp / 8);
            unsafe {
                let r = (x * 255 / width) as u8;
                let g = (y * 255 / height) as u8;
                let b = 200u8;
                *addr.add(pixel_offset) = b;
                *addr.add(pixel_offset + 1) = g;
                *addr.add(pixel_offset + 2) = r;
            }
        }
    }

    hlt_loop()
}

fn hlt_loop() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hlt_loop()
}