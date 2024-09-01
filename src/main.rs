#![no_std]
#![no_main]

#[panic_handler]
fn _panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> usize {
    0
}
