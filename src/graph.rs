use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct Graph<T> {
    adj: Vec<Vec<usize>>,
    index: HashMap<T, usize>,
}

impl<T: Eq + Hash> Graph<T> {
    pub fn new() -> Self {
        Self {
            adj: Vec::new(),
            index: HashMap::new(),
        }
    }

    pub fn add_edge(&self, x: T, y: T) {}

    pub fn add_vertex(&mut self, v: T) {
        let index = self.adj.len();
        self.adj.push(Vec::new());
        self.index.insert(v, index);
    }
}
