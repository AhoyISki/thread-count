use std::num::NonZeroUsize;
use std::{fs, process};

pub(crate) fn thread_count() -> Option<NonZeroUsize> {
    let mut amount: Option<NonZeroUsize> = None;
    let status = fs::read_to_string(format!("/proc/{}/status", process::id()))
        .expect("Failed reading status");

    for line in status.split('\n') {
        let line = line.to_lowercase();
        if line.starts_with("threads:") {
            let new_line = line.replace("threads:", "");

            amount = NonZeroUsize::new(
                new_line.trim().parse::<usize>().expect("Failed parsing thread count"),
            );
        }
    }

    amount
}
