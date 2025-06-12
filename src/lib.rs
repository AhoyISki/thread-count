#![warn(clippy::pedantic)]
use std::num::NonZeroUsize;

#[cfg_attr(any(target_os = "macos", target_os = "ios"), path = "osx.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_family = "windows", path = "windows.rs")]
mod implementation;

/// Gets the amount of threads for the current process.
/// Returns `None` if there are no threads.
#[must_use]
pub fn thread_count() -> Option<NonZeroUsize> {
    implementation::thread_count()
}

/// Check if the current process is single-threaded.
#[must_use]
pub fn is_single_threaded() -> bool {
    match thread_count() {
        Some(amount) => amount.get() == 1,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    #[test]
    fn thread_count() {
        for i in 0..5 {
            std::thread::spawn(move || {
                assert_eq!(super::thread_count().map(NonZeroUsize::get), Some(i));
            });
        }
    }

    #[test]
    fn is_single_threaded() {
        for i in 0..5 {
            std::thread::spawn(move || {
                if i == 0 {
                    assert!(super::is_single_threaded());
                } else {
                    assert!(!super::is_single_threaded());
                }
            });
        }
    }
}
