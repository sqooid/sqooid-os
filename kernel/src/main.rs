#![no_std]
#![no_main]

pub mod memory;

use core::panic::PanicInfo;

use bootlib::shared::BootInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "sysv64" fn main(boot_info: &BootInfo) -> ! {
    loop {}
}
