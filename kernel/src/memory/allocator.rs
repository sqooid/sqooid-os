use core::alloc::{GlobalAlloc, Layout};

pub struct Allocator {}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator {};

unsafe impl Sync for Allocator {}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}
