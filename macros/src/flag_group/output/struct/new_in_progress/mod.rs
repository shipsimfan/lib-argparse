mod new;
mod to_tokens;

/// Fills a tuple with [`None`]s to initialize the in-progress tuple
pub struct NewInProgress {
    /// The number of elements to initialize
    count: usize,
}
