use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct Graph<T> {
    adj: Vec<HashSet<usize>>,
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

        // add_vertex checks only add vertex if it doesn't exist. But, for new
        // lets keep validation here just for explicitness
        if !(self.contains(&x)) {
            self.add_vertex(x.clone());
        }
        if !(self.contains(&y)) {
            self.add_vertex(y.clone());
        }

        // We can assume x and y exists as vertices.
        if let (Some(&x), Some(&y)) = (self.index.get(&x), self.index.get(&y)) {
            self.adj[x].insert(y);
            self.adj[y].insert(x);
        };
    }

    /// Adds vertex in the graph only if it doesn't exist yet.
    ///
    /// * `v`: The vertex
    pub fn add_vertex(&mut self, v: T) {
        if self.contains(&v) {
            return;
        }

        // only add vertex if it doesn't exist
        let index = self.adj.len();
        self.values.push(v.clone());
        self.adj.push(HashSet::new());
        self.index.insert(v, index);
    }

    pub fn adjacent_vertices(&self, v: &T) -> impl Iterator<Item = &T> {
        self.get_index(v)
            .into_iter()
            .flat_map(move |&x| self.adj[x].iter())
            .map(|&x| &self.values[x])
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

    pub(crate) fn get_index(&self, v: &T) -> Option<&usize> {
        self.index.get(v)
    }

    pub(crate) fn get_value(&self, index: usize) -> Option<T> {
        if index < self.values.len() {
            Some(self.values[index].clone())
        } else {
            None
        }
    }

    pub fn vertices(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    pub fn is_edge(&self, x: &T, y: &T) -> bool {
        if !self.contains(x) || !self.contains(y) {
            return false;
        }

        if x == y {
            return true;
        }

        let x_index = self.get_index(x);
        let y_index = self.get_index(y);

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
        self.get_index(v).map(|&x| self.adj[x].len())
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

impl<T: Eq + Hash + Clone + PartialEq> Finder<T> for Graph<T> {
    fn dfs(&self, v: T, w: T) -> impl Iterator<Item = T> {
        let maked: Vec<bool> = vec![false; self.values.len()];

        let starting_vertex = self.get_index(&v);
        // NOTE: temporarily returning a empty iterator
        Vec::new().into_iter()
    }
    fn bfs(&self, v: T, w: T) -> impl Iterator<Item = T> {
        // NOTE: temporarily returning a empty iterator
        Vec::new().into_iter()
    }
}

#[derive(Debug, Default)]
pub struct Diagraph<T> {
    adj: Vec<HashSet<usize>>,
    values: Vec<T>,
    index: HashMap<T, usize>,
}

impl<T: Eq + Clone + PartialEq + Hash> Diagraph<T> {
    pub fn new() -> Self {
        Self {
            adj: Vec::new(),
            values: Vec::new(),
            index: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, v: T) {
        if self.contains(&v) {
            return;
        }

        let index = self.adj.len();
        self.index.insert(v.clone(), index);
        self.values.push(v);
        self.adj.push(HashSet::new());
    }

    pub fn contains(&self, v: &T) -> bool {
        self.index.contains_key(v)
    }

    /// Adds edge between x and y. If they don't exist then it creates them.
    ///
    /// * `x`: vertex
    /// * `y`: vertex
    pub fn add_edge(&mut self, x: T, y: T) {
        if x == y {
            return;
        }

        // add_vertex checks only add vertex if it doesn't exist. But, for new
        // lets keep validation here just for explicitness
        if !(self.contains(&x)) {
            self.add_vertex(x.clone());
        }
        if !(self.contains(&y)) {
            self.add_vertex(y.clone());
        }

        // We can assume x and y exists as vertices.
        if let (Some(&x), Some(&y)) = (self.get_index(&x), self.get_index(&y)) {
            // only add an edge if it doesn't exist
            self.adj[x].insert(y);
        };
    }
    /// Returns all the vertices that are adjacent to v.
    ///
    /// * `v`: vertex
    pub fn adjacent_vertices(&self, v: &T) -> impl Iterator<Item = &T> {
        self.get_index(v)
            .into_iter()
            .flat_map(move |&x| self.adj[x].iter())
            .map(|&x| &self.values[x])
    }
    /// Returns number of vertices in the graph
    pub fn count_vertices(&self) -> usize {
        self.values.len()
    }

    pub fn count_edges(&self) -> usize {
        self.adj.iter().map(|x| x.len()).sum::<usize>()
    }

    pub fn vertices(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    pub(crate) fn get_index(&self, v: &T) -> Option<&usize> {
        self.index.get(v)
    }

    pub(crate) fn get_value(&self, index: usize) -> Option<T> {
        if index < self.values.len() {
            Some(self.values[index].clone())
        } else {
            None
        }
    }

    /// return true if there is an edge between x and y. It cares about
    /// direction from x to y not y to x
    ///
    /// * `x`:  vertex
    /// * `y`: vertex
    pub fn is_edge(&self, x: &T, y: &T) -> bool {
        if !self.contains(x) || !self.contains(y) {
            return false;
        }

        if x == y {
            return true;
        }

        let x_index = self.get_index(x);
        let y_index = self.get_index(y);

        if let (Some(&x_index), Some(y_index)) = (x_index, y_index) {
            self.adj[x_index].contains(y_index)
        } else {
            false
        }
    }
}

trait Finder<T> {
    fn dfs(&self, v: T, w: T) -> impl Iterator<Item = T>;
    fn bfs(&self, v: T, w: T) -> impl Iterator<Item = T>;
}
