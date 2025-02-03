use core::fmt::Display;

use uefi::Status;

#[derive(Debug)]
pub enum KernelLoadError {
    MemoryError,
    ReadKernelError,
}

impl Display for KernelLoadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                KernelLoadError::MemoryError => "Failed to allocate memory",
                KernelLoadError::ReadKernelError => "Failed to read kernel",
            }
        )
    }
}

impl From<uefi::Error> for KernelLoadError {
    fn from(error: uefi::Error) -> Self {
        match error.status() {
            Status::OUT_OF_RESOURCES | Status::NOT_FOUND => KernelLoadError::ReadKernelError,
            _ => KernelLoadError::ReadKernelError,
        }
    }
}

#[derive(Debug)]
pub enum ElfLoadError {
    MemoryError,
    InvalidElf,
}
impl Display for ElfLoadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ElfLoadError::MemoryError => "Failed to allocate memory",
                ElfLoadError::InvalidElf => "Invalid ELF file",
            }
        )
    }
}

impl From<uefi::Error> for ElfLoadError {
    fn from(error: uefi::Error) -> Self {
        match error.status() {
            Status::OUT_OF_RESOURCES | Status::NOT_FOUND => ElfLoadError::MemoryError,
            _ => ElfLoadError::InvalidElf,
        }
    }
}
