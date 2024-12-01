use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct SlabAlloc {
    slabs: [AtomicUsize; 1024],
}

impl SlabAlloc {
    pub fn new() -> Self {
        Self {
            slabs: [const { AtomicUsize::new(0) }; 1024],
        }
    }

    fn find_free_slab(&self) -> Option<usize> {
        for (i, slab) in self.slabs.iter().enumerate() {
            if slab.load(Ordering::Relaxed) == 0 {
                return Some(i);
            }
        }
        None
    }
}
