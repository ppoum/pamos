#![no_std]
#![no_main]

use core::arch::global_asm;

use boot::multiboot2::{self, FramebufferTag, TagCollector};

mod boot;

#[panic_handler]
fn _panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

global_asm!(include_str!("main.s"));

#[no_mangle]
pub extern "C" fn kmain(magic: u32, mb2_ptr: usize) -> ! {
    if magic != boot::multiboot2::EAX_MB2_MAGIC {
        panic!("No MB2 information")
    }

    // SAFETY: Magic in first arg, 2nd arg should be a valid pointer
    let boot_info = unsafe { multiboot2::BootInfo::try_from_addr(mb2_ptr).unwrap() };
    let tags = TagCollector::try_new(&boot_info).unwrap();

    if let Some(FramebufferTag {
        horizontal_res,
        vertical_res,
        scanline_size,
        address,
        ..
    }) = tags.framebuffer
    {
        for i in 0..*vertical_res as usize {
            for j in 0..*horizontal_res as usize {
                let pixel_offset = j + (*scanline_size as usize * i);
                let addr = *address as usize + 4 * pixel_offset;

                let r = (255 * j / *horizontal_res as usize) as u32;
                let b = (255 * i / *vertical_res as usize) as u32;
                // Assume RGB for now
                let color = (r << 16) + b;
                // let color = (255 << 16) + 255;
                unsafe { *(addr as *mut u32) = color };
            }
        }
    }

    loop {}
}
