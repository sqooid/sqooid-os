#![no_main]
#![no_std]

pub mod elf;
pub mod error;
pub mod file;

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
    let kernel_loaded_addr = elf.load().expect("Failed to load ELF file");

    info!("Exiting bootloader");
    info!("Entry address: 0x{:x}", elf.entry_addr);
    let _ = unsafe { uefi::boot::exit_boot_services(MemoryType::LOADER_DATA) };

    let kernel_main: extern "sysv64" fn() -> ! =
        unsafe { core::mem::transmute(elf.entry_addr + kernel_loaded_addr as usize) };
    kernel_main();

    Status::SUCCESS
}
