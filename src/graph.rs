use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct Graph<T> {
    adj: Vec<Vec<usize>>,
    index: HashMap<T, usize>,
    values: Vec<T>,
}

impl<T: Eq + Hash + Clone> Graph<T> {
    pub fn new() -> Self {
        Self {
            adj: Vec::new(),
            index: HashMap::new(),
            values: Vec::new(),
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
        if !(self.contains(&x)) {
            self.add_vertex(x.clone());
        }
        if !(self.contains(&y)) {
            self.add_vertex(y.clone());
        }

        // We can assume x and y exists as vertices.
        if let (Some(&x), Some(&y)) = (self.index.get(&x), self.index.get(&y)) {
            if !self.adj[x].contains(&y) {
                self.adj[x].push(y);
            }
            if !self.adj[y].contains(&x) {
                self.adj[y].push(x);
            }
        };
    }

    /// Adds vertex in the graph only if it doesn't exist yet.
    ///
    /// * `v`: The vertex
    pub fn add_vertex(&mut self, v: T) {
        if self.contains(&v) {
            return;
        }

        // only add vertex if it d
        let index = self.adj.len();
        self.values.push(v.clone());
        self.adj.push(Vec::new());
        self.index.insert(v, index);
    }

    pub fn adjcent_vertices(&self, v: &T) -> Vec<T> {
        let adjcent_vertices: Vec<T> = {
            match self.index.get(v) {
                Some(&x) => self.adj[x].clone(),
                None => Vec::new(),
            }
        }
        .iter()
        .map(|&x| self.values[x].clone())
        .collect();

        adjcent_vertices
    }

    /// Checks if Vertex exists in the graph
    ///
    /// * `v`: Vertex
    pub fn contains(&self, v: &T) -> bool {
        self.index.contains_key(v)
    }

    /// Returns number of vertices in the graph
    pub fn count(&self) -> usize {
        self.values.len()
    }

    pub fn count_edges(&self) -> usize {
        self.adj.iter().map(|x| x.len()).sum::<usize>() / 2
    }
}
