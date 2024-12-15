#![no_std]
#![no_main]

use core::arch::global_asm;

#[panic_handler]
fn _panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

global_asm!(include_str!("main.s"));

#[no_mangle]
pub extern "C" fn kmain(_magic: u32, _mb2_ptr: u32) -> ! {
    loop {}
}
