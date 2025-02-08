#![no_std]
#![no_main]

use core::arch::global_asm;

use boot::multiboot2::{self, TagCollector};

mod boot;
mod print;

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

    if let Some(tag) = tags.framebuffer {
        // SAFETY: First and only call to function
        unsafe { print::initialize_fb_from_mb2_tag(tag) };
        print::print_str("Test\nstring\n\nfoofoo\rbar");
    }

    loop {}
}
