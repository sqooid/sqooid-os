#![no_main]
#![no_std]

pub mod elf;
pub mod error;
pub mod file;

use core::ffi::c_void;

use elf::Elf;
use file::open_kernel_elf;
use log::info;
use uefi::{boot::MemoryType, prelude::*};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    info!("Reading kernel");
    let kernel_bytes = open_kernel_elf(cstr16!("kernel.elf")).expect("Failed to open kernel.elf");

    info!("Loading kernel");
    let elf = Elf::from_bytes(&kernel_bytes).expect("Failed to parse ELF file");
    elf.load().expect("Failed to load ELF file");

    info!("Exiting bootloader");
    unsafe {
        let _ = uefi::boot::exit_boot_services(MemoryType::LOADER_DATA);
    }
    let handle = unsafe { Handle::from_ptr(elf.entry_addr as *mut c_void).unwrap() };
    boot::start_image(handle).unwrap();

    Status::SUCCESS
}
