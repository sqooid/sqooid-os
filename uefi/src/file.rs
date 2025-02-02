extern crate alloc;

use log::info;
use uefi::{
    boot::{self},
    proto::media::file::{File, FileAttribute, FileInfo},
    CStr16,
};

use crate::error::KernelLoadError;

pub fn open_kernel_elf(elf_name: &CStr16) -> Result<alloc::vec::Vec<u8>, KernelLoadError> {
    let mut dir = open_root_dir()?;
    let mut handle = dir.open(
        elf_name,
        uefi::proto::media::file::FileMode::Read,
        FileAttribute::empty(),
    )?;
    let file_info = handle.get_boxed_info::<FileInfo>()?;
    let kernel_size = file_info.file_size() as usize;

    info!("Kernel size: {} bytes", kernel_size);

    let mut kernel_bytes = alloc::vec![0u8; kernel_size];
    info!(
        "Allocated {} bytes for reading kernel",
        file_info.file_size()
    );
    let mut file = handle
        .into_regular_file()
        .ok_or_else(|| KernelLoadError::ReadKernelError)?;

    let bytes = file.read(&mut kernel_bytes)?;
    info!("Read {} bytes", bytes);

    info!("Successfully read kernel from file");

    Ok(kernel_bytes)
}

fn open_root_dir() -> Result<uefi::proto::media::file::Directory, KernelLoadError> {
    let mut fs_proto = boot::get_image_file_system(boot::image_handle())?;
    let fs = fs_proto.get_mut().unwrap();
    let dir = fs.open_volume()?;
    Ok(dir)
}
