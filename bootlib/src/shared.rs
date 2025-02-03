use uefi::mem::memory_map::MemoryMapOwned;

pub struct BootInfo {
    pub memory_map: MemoryMapOwned,
}
