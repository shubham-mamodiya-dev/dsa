#[derive(Debug, Default)]
pub struct Graph<T> {
    adj: Vec<Vec<usize>>,
    values: Vec<T>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            adj: Vec::new(),
            values: Vec::new(),
        }
    }
}
