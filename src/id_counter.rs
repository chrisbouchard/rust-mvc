use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

static COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;

pub fn next_id() -> usize {
    COUNTER.fetch_add(1, Ordering::AcqRel)
}

