use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::iter;

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
            // Above iterator only returns one element the index
            .flat_map(move |&x| self.adj[x].iter())
            .map(|&x| &self.values[x])
    }

    pub(crate) fn adjacent_vertex_indices(&self, v: &T) -> impl Iterator<Item = &usize> {
        self.get_index(v)
            .into_iter()
            // Above iterator only returns one element the index
            .flat_map(move |&x| self.adj[x].iter())
    }

    pub(crate) fn value(&self, i: usize) -> T {
        self.values[i].clone()
    }

    pub(crate) fn adjacent_vertex_indices_from_index(
        &self,
        i: usize,
    ) -> impl Iterator<Item = &usize> {
        self.adj[i].iter()
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

    pub fn path_with_dfs(&self, v: &T, w: &T) -> impl Iterator<Item = T> {
        // Making sure v and w exists in the graph
        if self.get_index(v).is_none() || self.get_index(w).is_none() {
            return Vec::new().into_iter();
        }

        // Finding a path between.

        let mut edge_to: Vec<usize> = (0..self.count_vertices()).collect();
        let mut marked = vec![false; self.count_vertices()];
        let &starting_vertex = self.get_index(v).unwrap();
        let &destination_vertex = self.get_index(w).unwrap();

        self.dfs(
            starting_vertex,
            destination_vertex,
            &mut marked,
            &mut edge_to,
        );

        // If they are connected then from destination_vertex trace the starting_vertex
        if marked[starting_vertex] == marked[destination_vertex] {
            let mut next = destination_vertex;
            let mut path: Vec<T> = Vec::new();
            while next != starting_vertex {
                path.push(self.value(next));
                next = edge_to[next];
            }
            // pushing the destination_vertex
            path.push(self.values[next].clone());
            path.reverse();
            path.into_iter()
        } else {
            Vec::new().into_iter()
        }
    }

    fn dfs(&self, v: usize, w: usize, marked: &mut [bool], edge_to: &mut [usize]) {
        marked[v] = true;
        for &adj in self.adjacent_vertex_indices_from_index(v) {
            if !marked[adj] {
                self.dfs(adj, w, marked, edge_to);
                edge_to[adj] = v;

                // only keep crawling if destination is not found.
                if adj == w {
                    return;
                }
            }
        }
    }

    pub fn path_with_bfs(&self, v: &T, w: &T) -> impl Iterator<Item = T> {
        // Making sure v and w exists in the graph
        if self.get_index(v).is_none() || self.get_index(w).is_none() {
            return Vec::new().into_iter();
        }

        // Finding a path between.

        let mut edge_to: Vec<usize> = (0..self.count_vertices()).collect();
        let mut marked = vec![false; self.count_vertices()];
        let &starting_vertex = self.get_index(v).unwrap();
        let &destination_vertex = self.get_index(w).unwrap();
        let mut frontier = VecDeque::new();

        frontier.push_front(starting_vertex);
        marked[starting_vertex] = true;

        while !frontier.is_empty() {
            if let Some(current_vertex) = frontier.pop_front() {
                for &adj in self.adjacent_vertex_indices_from_index(current_vertex) {
                    if !marked[adj] {
                        frontier.push_front(adj);
                        marked[adj] = true;
                        edge_to[adj] = current_vertex;

                        if adj == destination_vertex {
                            break;
                        }
                    }
                }
            }
        }
        if marked[starting_vertex] == marked[destination_vertex] {
            let mut next = destination_vertex;
            let mut path: Vec<T> = Vec::new();
            while next != starting_vertex {
                path.push(self.value(next));
                next = edge_to[next];
            }
            // pushing the destination_vertex
            path.push(self.values[next].clone());
            path.reverse();
            path.into_iter()
        } else {
            Vec::new().into_iter()
        }
        // If they are connected then from destination_vertex trace the starting_vertex
    }
}

#[derive(Debug, Default)]
pub struct Digraph<T> {
    adj: Vec<HashSet<usize>>,
    values: Vec<T>,
    index: HashMap<T, usize>,
}

impl<T: Eq + Clone + PartialEq + Hash> Digraph<T> {
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

    pub(crate) fn adjacent_vertex_indices(&self, v: &T) -> impl Iterator<Item = &usize> {
        self.get_index(v)
            .into_iter()
            // Above iterator only returns one element the index
            .flat_map(move |&x| self.adj[x].iter())
    }

    pub(crate) fn adjacent_vertex_indices_from_index(
        &self,
        i: usize,
    ) -> impl Iterator<Item = &usize> {
        self.adj[i].iter()
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

    pub(crate) fn value(&self, i: usize) -> T {
        self.values[i].clone()
    }

    pub fn path_with_dfs(&self, v: &T, w: &T) -> impl Iterator<Item = T> {
        // Making sure v and w exists in the graph
        if self.get_index(v).is_none() || self.get_index(w).is_none() {
            return Vec::new().into_iter();
        }

        // Finding a path between.

        let mut edge_to: Vec<usize> = (0..self.count_vertices()).collect();
        let mut marked = vec![false; self.count_vertices()];
        let &starting_vertex = self.get_index(v).unwrap();
        let &destination_vertex = self.get_index(w).unwrap();

        self.dfs(
            starting_vertex,
            destination_vertex,
            &mut marked,
            &mut edge_to,
        );

        // If they are connected then from destination_vertex trace the starting_vertex
        if marked[starting_vertex] == marked[destination_vertex] {
            let mut next = destination_vertex;
            let mut path: Vec<T> = Vec::new();
            while next != starting_vertex {
                path.push(self.value(next));
                next = edge_to[next];
            }
            // pushing the destination_vertex
            path.push(self.values[next].clone());
            path.reverse();
            path.into_iter()
        } else {
            Vec::new().into_iter()
        }
    }

    fn dfs(&self, v: usize, w: usize, marked: &mut [bool], edge_to: &mut [usize]) {
        marked[v] = true;
        for &adj in self.adjacent_vertex_indices_from_index(v) {
            if !marked[adj] {
                self.dfs(adj, w, marked, edge_to);
                edge_to[adj] = v;
                if adj == w {
                    return;
                }
            }
        }
    }

    pub fn path_with_bfs(&self, v: &T, w: &T) -> impl Iterator<Item = T> {
        // Making sure v and w exists in the graph
        if self.get_index(v).is_none() || self.get_index(w).is_none() {
            return Vec::new().into_iter();
        }

        // Finding a path between.

        let mut edge_to: Vec<usize> = (0..self.count_vertices()).collect();
        let mut marked = vec![false; self.count_vertices()];
        let &starting_vertex = self.get_index(v).unwrap();
        let &destination_vertex = self.get_index(w).unwrap();
        let mut frontier = VecDeque::new();

        frontier.push_front(starting_vertex);
        marked[starting_vertex] = true;

        while !frontier.is_empty() {
            if let Some(current_vertex) = frontier.pop_front() {
                for &adj in self.adjacent_vertex_indices_from_index(current_vertex) {
                    if !marked[adj] {
                        frontier.push_front(adj);
                        marked[adj] = true;
                        edge_to[adj] = current_vertex;

                        if adj == destination_vertex {
                            break;
                        }
                    }
                }
            }
        }
        if marked[starting_vertex] == marked[destination_vertex] {
            let mut next = destination_vertex;
            let mut path: Vec<T> = Vec::new();
            while next != starting_vertex {
                path.push(self.value(next));
                next = edge_to[next];
            }
            // pushing the destination_vertex
            path.push(self.values[next].clone());
            path.reverse();
            path.into_iter()
        } else {
            Vec::new().into_iter()
        }
        // If they are connected then from destination_vertex trace the starting_vertex
    }
}

#[derive(Debug, Default)]
pub struct CC<T> {
    adj: Vec<HashSet<usize>>,
    index: HashMap<T, usize>,
    values: Vec<T>,
    marked: Vec<bool>,
    edge_to: Vec<usize>,
    connected_groups: Vec<usize>,
    count: usize,
    needs_rebuilding: bool,
}

impl<T: Eq + Hash + Clone + PartialEq> CC<T> {
    pub fn new() -> Self {
        Self {
            adj: Vec::new(),
            index: HashMap::new(),
            values: Vec::new(),
            marked: Vec::new(),
            edge_to: Vec::new(),
            connected_groups: Vec::new(),
            count: 0,
            needs_rebuilding: false,
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

        self.needs_rebuilding = true;
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
        self.marked.push(false);
        self.edge_to.push(index);
        self.connected_groups.push(self.count);
        self.count += 1;
    }

    pub fn adjacent_vertices(&self, v: &T) -> impl Iterator<Item = &T> {
        self.get_index(v)
            .into_iter()
            // Above iterator only returns one element the index
            .flat_map(move |&x| self.adj[x].iter())
            .map(|&x| &self.values[x])
    }

    pub(crate) fn adjacent_vertex_indices(&self, v: &T) -> impl Iterator<Item = &usize> {
        self.get_index(v)
            .into_iter()
            // Above iterator only returns one element the index
            .flat_map(move |&x| self.adj[x].iter())
    }

    pub(crate) fn value(&self, i: usize) -> T {
        self.values[i].clone()
    }

    pub(crate) fn adjacent_vertex_indices_from_index(
        &self,
        i: usize,
    ) -> impl Iterator<Item = &usize> {
        self.adj[i].iter()
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

    pub fn vertices(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    pub fn indices(&self) -> impl Iterator<Item = &usize> {
        self.index.values()
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

    pub fn is_connected(&mut self, v: &T, w: &T) -> bool {
        if self.contains(v) || self.contains(w) {
            false
        } else {
            if self.needs_rebuilding {
                self.build();
            }

            let x_index = self.get_index(v);
            let y_index = self.get_index(w);

            if let (Some(&x_index), Some(&y_index)) = (x_index, y_index) {
                self.connected_groups[x_index] == self.connected_groups[y_index]
            } else {
                false
            }
        }
    }
    pub fn build(&mut self) {
        self.count = 0;
        let indices: Vec<usize> = self.indices().copied().collect();
        for w in indices {
            if !self.marked[w] {
                self.dfs_crawler(w);
                self.connected_groups[w] = self.count;
                self.count += 1;
            }
        }
    }

    fn dfs_crawler(&mut self, v: usize) {
        self.marked[v] = true;

        let neighbors: Vec<usize> = self
            .adjacent_vertex_indices_from_index(v)
            .copied()
            .collect();

        for w in neighbors {
            if !self.marked[w] {
                self.dfs_crawler(w);
                self.edge_to[w] = v;
            }
        }
    }
    pub fn average_degree(&self) -> f64 {
        2.0 * self.count_edges() as f64 / self.count_vertices() as f64
    }
}
