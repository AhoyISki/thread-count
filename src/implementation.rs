//! Fallback if no OS is matched.
use std::num::NonZeroUsize;

#[inline]
pub(crate) fn thread_count() -> Option<NonZeroUsize> {
    None
}
