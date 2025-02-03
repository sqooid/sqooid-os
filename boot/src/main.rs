#![no_main]
#![no_std]

pub mod elf;
pub mod error;
pub mod file;

use bootlib::shared::BootInfo;
use elf::load_elf;
use file::open_kernel_elf;
use log::info;
use uefi::{boot::MemoryType, prelude::*};
use utils::filetypes::{decode::Decode, elf::ElfFile};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    info!("Reading kernel");
    let kernel_bytes = open_kernel_elf(cstr16!("kernel.elf")).expect("Failed to open kernel.elf");

    info!("Loading kernel");
    let elf = ElfFile::decode(&kernel_bytes).expect("Failed to parse ELF file");
    let kernel_loaded_addr = load_elf(&elf).expect("Failed to load ELF file");

    info!("Exiting bootloader");
    info!("Entry address: 0x{:x}", elf.header.entry);
    let memory_map = unsafe { uefi::boot::exit_boot_services(MemoryType::LOADER_DATA) };

    let kernel_main: extern "sysv64" fn(&BootInfo) -> ! =
        unsafe { core::mem::transmute(elf.header.entry as usize + kernel_loaded_addr as usize) };
    kernel_main(&BootInfo { memory_map });

    Status::SUCCESS
}
