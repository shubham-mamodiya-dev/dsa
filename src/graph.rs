use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct Graph<T> {
    adj: Vec<Vec<usize>>,
    index: HashMap<T, usize>,
    values: Vec<T>,
}

impl<T: Eq + Hash + Clone + PartialEq> Graph<T> {
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
    pub fn count_vertices(&self) -> usize {
        self.values.len()
    }

    pub fn count_edges(&self) -> usize {
        self.adj.iter().map(|x| x.len()).sum::<usize>() / 2
    }

    pub fn vertices(&self) -> Vec<T> {
        self.values.clone()
    }

    pub fn is_edge(&self, x: &T, y: &T) -> bool {
        if !self.contains(x) || !self.contains(y) {
            return false;
        }

        if x == y {
            return true;
        }

        let x_index = self.index.get(x);
        let y_index = self.index.get(y);

        if let (Some(&x_index), Some(y_index)) = (x_index, y_index) {
            self.adj[x_index].contains(y_index)
        } else {
            false
        }
    }

    /// Returns number or vertices connected with v. Returns None if vertex v
    /// doesn't exists in the graph.
    ///
    /// * `v`: Vertex
    pub fn degree(&self, v: &T) -> Option<usize> {
        self.index.get(v).map(|&x| self.adj[x].len())
    }

    pub fn max_degree(&self) -> usize {
        self.values
            .iter()
            // Unwrap is fine here. Because we are computing degree of vertices
            // of degrees that exists in the graph.
            .map(|x| self.degree(x).unwrap())
            .max()
            .unwrap_or(0)
    }

    pub fn average_degree(&self) -> f64 {
        2.0 * self.count_edges() as f64 / self.count_vertices() as f64
    }
}
