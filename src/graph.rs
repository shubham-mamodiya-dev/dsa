use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Graph<T> {
    adj: Vec<Vec<usize>>,
    values: Vec<T>,
    index: HashMap<T, usize>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            adj: Vec::new(),
            values: Vec::new(),
            index: HashMap::new(),
        }
    }

    pub fn add_edge(&self, x: T, y: T) {}
}
