#[must_use]
pub fn timeit(label: impl Into<String>) -> impl Drop {
    use std::time::Instant;

    struct Guard {
        label: String,
        start: Instant,
    }

    impl Drop for Guard {
        fn drop(&mut self) {
            eprintln!("{}: {:.2?}", self.label, self.start.elapsed())
        }
    }

    Guard { label: label.into(), start: Instant::now() }
}
