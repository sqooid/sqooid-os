extern crate alloc;
use core::ptr::slice_from_raw_parts_mut;

use uefi::boot::{allocate_pages, MemoryType};
use utils::filetypes::elf::ElfFile;

use crate::error::ElfLoadError;

pub fn load_elf(elf: &ElfFile) -> Result<*const u8, ElfLoadError> {
    let mut max_addr = 0u64;
    elf.program_headers
        .iter()
        .filter(|x| x.segment_type == 1)
        .for_each(|x| {
            if x.virtual_addr + x.memory_size > max_addr {
                max_addr = x.virtual_addr + x.memory_size;
            }
        });
    let kernel_mem = unsafe {
        slice_from_raw_parts_mut(
            allocate_pages(
                uefi::boot::AllocateType::AnyPages,
                MemoryType::LOADER_DATA,
                max_addr as usize,
            )?
            .as_ptr(),
            max_addr as usize,
        )
        .as_mut()
        .ok_or(ElfLoadError::MemoryError)?
    };
    for seg in elf.program_headers.iter() {
        // Only load LOAD type segments
        if seg.segment_type != 1 {
            continue;
        }
        for i in 0..seg.memory_size {
            kernel_mem[i as usize] = match i < seg.file_size {
                true => elf.bytes[(seg.offset + i) as usize],
                false => 0,
            }
        }
    }

    Ok(kernel_mem.as_ptr().cast::<u8>())
}
