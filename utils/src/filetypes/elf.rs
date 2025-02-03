extern crate alloc;
use core::fmt::Debug;

use super::{
    decode::{Decode, DecodeError},
    reader::ByteReader,
};

#[derive(Debug, Default)]
pub struct ElfHeader {
    pub prefix: u32,
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os: u8,
    pub elf_type: u16,
    pub machine: u16,
    pub version2: u32,
    pub entry: u64,
    pub program_header_offset: u64,
    pub section_header_offset: u64,
    pub flags: u32,
    pub header_size: u16,
    pub program_header_size: u16,
    pub program_header_num: u16,
    pub section_header_size: u16,
    pub section_header_num: u16,
    pub section_header_string_index: u16,
}

#[derive(Debug, Default, Clone)]
pub struct ElfProgramHeader {
    pub segment_type: u32,
    pub flags: u32,
    pub offset: u64,
    pub virtual_addr: u64,
    pub physical_addr: u64,
    pub file_size: u64,
    pub memory_size: u64,
    pub align: u64,
}

pub struct ElfFile<'a> {
    pub little_endian: bool,
    pub bytes: &'a [u8],
    pub header: ElfHeader,
    pub program_headers: alloc::vec::Vec<ElfProgramHeader>,
}

impl<'a> Decode<'a, ElfFile<'a>> for ElfFile<'a> {
    fn decode(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        let little_endian = bytes[5] == 1;
        let elf_reader = ByteReader::new(bytes, little_endian);
        let elf_header = ElfHeader {
            prefix: elf_reader.uint_at_offset::<u32>(0),
            class: elf_reader.uint_at_offset::<u8>(4),
            data: elf_reader.uint_at_offset::<u8>(5),
            version: elf_reader.uint_at_offset::<u8>(6),
            os: elf_reader.uint_at_offset::<u8>(7),
            elf_type: elf_reader.uint_at_offset::<u16>(16),
            machine: elf_reader.uint_at_offset::<u16>(18),
            version2: elf_reader.uint_at_offset::<u32>(20),
            entry: elf_reader.uint_at_offset::<u64>(24),
            program_header_offset: elf_reader.uint_at_offset::<u64>(32),
            section_header_offset: elf_reader.uint_at_offset::<u64>(40),
            flags: elf_reader.uint_at_offset::<u32>(48),
            header_size: elf_reader.uint_at_offset::<u16>(52),
            program_header_size: elf_reader.uint_at_offset::<u16>(54),
            program_header_num: elf_reader.uint_at_offset::<u16>(56),
            section_header_size: elf_reader.uint_at_offset::<u16>(58),
            section_header_num: elf_reader.uint_at_offset::<u16>(60),
            section_header_string_index: elf_reader.uint_at_offset::<u16>(62),
        };
        let mut program_headers =
            alloc::vec![ElfProgramHeader::default(); elf_header.program_header_num as usize];
        for i in 0..elf_header.program_header_num as u64 {
            let offset = (elf_header.program_header_offset
                + i * elf_header.program_header_size as u64) as usize;
            let seg_reader = ByteReader::new(
                &bytes[offset..offset + elf_header.program_header_size as usize],
                little_endian,
            );
            program_headers[i as usize] = ElfProgramHeader {
                segment_type: seg_reader.uint_at_offset::<u32>(0),
                flags: seg_reader.uint_at_offset::<u32>(4),
                offset: seg_reader.uint_at_offset::<u64>(8),
                virtual_addr: seg_reader.uint_at_offset::<u64>(16),
                physical_addr: seg_reader.uint_at_offset::<u64>(24),
                file_size: seg_reader.uint_at_offset::<u64>(32),
                memory_size: seg_reader.uint_at_offset::<u64>(40),
                align: seg_reader.uint_at_offset::<u64>(48),
            }
        }
        Ok(ElfFile {
            little_endian,
            bytes,
            header: elf_header,
            program_headers,
        })
    }
}
