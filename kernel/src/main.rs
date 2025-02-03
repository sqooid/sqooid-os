#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "sysv64" fn main() -> ! {
    loop {}
}
