use std::sync::LazyLock;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct AtomicIdGen {
    current: AtomicUsize,
}

impl AtomicIdGen {
    pub fn new() -> Self {
        Self {
            current: AtomicUsize::new(0),
        }
    }

    pub fn next(&self) -> usize {
        self.current.fetch_add(1, Ordering::Relaxed)
    }
}

pub static GENOME_ID_GENERATOR: LazyLock<AtomicIdGen> = LazyLock::new(|| AtomicIdGen::new());
pub static GENERATION: LazyLock<AtomicIdGen> = LazyLock::new(|| AtomicIdGen::new());
pub static BEST_FITNESS: LazyLock<AtomicIdGen> = LazyLock::new(|| AtomicIdGen::new());
