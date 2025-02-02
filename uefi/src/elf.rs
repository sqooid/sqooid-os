extern crate alloc;
use core::ptr::slice_from_raw_parts_mut;

use uefi::boot::{allocate_pages, MemoryType};

use crate::error::ElfLoadError;

#[derive(Debug)]
pub struct Elf<'a> {
    elf_bytes: &'a [u8],
    pub entry_addr: usize,
    program_headers: alloc::vec::Vec<ElfProgramHeader>,
}

#[derive(Debug, Clone, Default)]
pub struct ElfProgramHeader {
    m_type: u32,
    flags: u32,
    offset: u64,
    virtual_addr: u64,
    physical_addr: u64,
    file_size: u64,
    memory_size: u64,
    align: u64,
}

fn u64_from_bytes(bytes: &[u8], little_endian: bool) -> u64 {
    uint_from_bytes(bytes, 8, little_endian)
}

fn u16_from_bytes(bytes: &[u8], little_endian: bool) -> u16 {
    uint_from_bytes(bytes, 2, little_endian) as u16
}

fn u32_from_bytes(bytes: &[u8], little_endian: bool) -> u32 {
    uint_from_bytes(bytes, 4, little_endian) as u32
}

fn uint_from_bytes(bytes: &[u8], count: usize, little_endian: bool) -> u64 {
    let mut result = 0;
    for i in 0..count {
        match little_endian {
            true => {
                result |= (bytes[i] as u64) << (i * 8);
            }
            false => {
                result |= (bytes[i] as u64) << ((count - 1 - i) * 8);
            }
        }
    }
    result
}

impl<'a> Elf<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, ElfLoadError> {
        let little_endian = bytes[5] == 1;
        let entry_addr = u64_from_bytes(&bytes[24..32], little_endian) as usize;
        let program_headers_offset = u64_from_bytes(&bytes[32..40], little_endian) as usize;
        let program_headers_size = u16_from_bytes(&bytes[54..56], little_endian) as usize;
        let program_headers_num = u16_from_bytes(&bytes[56..58], little_endian) as usize;
        let mut program_headers = alloc::vec![
        ElfProgramHeader::default();
            program_headers_num
        ];

        for i in 0..program_headers_num {
            let offset = program_headers_offset + i * program_headers_size;
            let header = &bytes[offset..offset + program_headers_size];
            program_headers[i as usize] = ElfProgramHeader {
                m_type: u32_from_bytes(&header[0..4], little_endian),
                flags: u32_from_bytes(&header[4..8], little_endian),
                offset: u64_from_bytes(&header[8..16], little_endian),
                virtual_addr: u64_from_bytes(&header[16..24], little_endian),
                physical_addr: u64_from_bytes(&header[24..32], little_endian),
                file_size: u64_from_bytes(&header[32..40], little_endian),
                memory_size: u64_from_bytes(&header[40..48], little_endian),
                align: u64_from_bytes(&header[48..56], little_endian),
            };
        }

        Ok(Elf {
            elf_bytes: bytes,
            entry_addr,
            program_headers,
        })
    }

    pub fn load(&self) -> Result<(), ElfLoadError> {
        let fixed_size = self
            .program_headers
            .iter()
            .filter(|x| x.m_type == 1)
            .map(|x| x.memory_size)
            .sum::<u64>() as usize;
        let kernel_mem = unsafe {
            slice_from_raw_parts_mut(
                allocate_pages(
                    uefi::boot::AllocateType::AnyPages,
                    MemoryType::LOADER_DATA,
                    fixed_size,
                )?
                .as_ptr(),
                fixed_size,
            )
            .as_mut()
            .ok_or(ElfLoadError::MemoryError)?
        };
        for seg in &self.program_headers {
            // Only load LOAD type segments
            if seg.m_type != 1 {
                continue;
            }
            for i in 0..seg.memory_size {
                kernel_mem[i as usize] = match i < seg.file_size {
                    true => self.elf_bytes[(seg.offset + i) as usize],
                    false => 0,
                }
            }
        }

        Ok(())
    }
}
