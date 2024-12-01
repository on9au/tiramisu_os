use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
    sync::atomic::{AtomicPtr, Ordering},
};

/// Size of a slab in bytes
const SLAB_SIZE: usize = 4096;

/// Tiramisu OS Slab Allocator
pub struct SlabAlloc {
    free_list: AtomicPtr<Slab>,
    memory_start: usize,
}

/// A slab of memory
pub struct Slab {
    next: Option<NonNull<Slab>>,
}

unsafe impl GlobalAlloc for SlabAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        let mut head = self.free_list.load(Ordering::Acquire);
        while !head.is_null() {
            let slab = unsafe { &*(head as *mut Slab) };
            let next = slab.next.map_or(core::ptr::null_mut(), |n| n.as_ptr());
            if self
                .free_list
                .compare_exchange(head, next, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return head as *mut u8;
            } else {
                head = self.free_list.load(Ordering::Acquire);
            }
        }
        // No free slabs available
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let slab = ptr as *mut Slab;
        let mut head = self.free_list.load(Ordering::Acquire);
        loop {
            unsafe {
                (*slab).next = NonNull::new(head as *mut Slab);
            }
            if self
                .free_list
                .compare_exchange(head, slab, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                break;
            } else {
                head = self.free_list.load(Ordering::Acquire);
            }
        }
    }
}

impl SlabAlloc {
    pub const fn new(memory_start: usize) -> Self {
        Self {
            free_list: AtomicPtr::new(core::ptr::null_mut()),
            memory_start,
        }
    }

    pub unsafe fn add_slab(&self, slab_ptr: *mut u8) {
        unsafe {
            self.dealloc(
                slab_ptr,
                Layout::from_size_align_unchecked(SLAB_SIZE, SLAB_SIZE),
            )
        };
    }
}
