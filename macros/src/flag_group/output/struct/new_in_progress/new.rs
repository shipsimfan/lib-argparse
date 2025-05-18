use super::NewInProgress;

impl NewInProgress {
    /// Creates a new [`NewInProgress`]
    pub fn new(count: usize) -> Self {
        NewInProgress { count }
    }
}
