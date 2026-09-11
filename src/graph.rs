use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct Graph<T> {
    adj: Vec<Vec<usize>>,
    index: HashMap<T, usize>,
}

impl<T: Eq + Hash + Clone> Graph<T> {
    pub fn new() -> Self {
        Self {
            adj: Vec::new(),
            index: HashMap::new(),
        }
    }

    /// Adds edge between x and y. If they don't exist then it creates them.
    ///
    /// * `x`: vertex
    /// * `y`: vertex
    pub fn add_edge(&mut self, x: T, y: T) {
        if x == y {
            return;
        }
        if !(self.index.contains_key(&x)) {
            self.add_vertex(x.clone());
        }
        if !(self.index.contains_key(&y)) {
            self.add_vertex(y.clone());
        }

        if let (Some(index_x), Some(index_y)) = (self.index.get(&x), self.index.get(&y)) {
            self.adj[*index_x].push(*index_y);
            self.adj[*index_y].push(*index_x);
        };
    }

    /// Adds vertes in the graph only if it don't exist yet.
    ///
    /// * `v`: The vertex
    pub fn add_vertex(&mut self, v: T) {
        let index = self.adj.len();
        if !self.index.contains_key(&v) {
            self.adj.push(Vec::new());
        }
        self.index.entry(v).or_insert(index);
    }
}
