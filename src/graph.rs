use std::collections::BTreeMap;

/// Undirected graph where nodes are associated with values of N, and edges are associated with values of E.
pub struct Graph<V, E> {
    nodes: BTreeMap<usize, V>,
    edges: BTreeMap<Edge, E>,
}

#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub struct Edge(usize, usize);

impl Edge {
    fn new(from: usize, to: usize) -> Edge {
        if from <= to {
            Edge(from, to)
        } else {
            Edge(to, from)
        }
    }

    fn touches(&self, node: usize) -> bool {
        self.0 == node || self.1 == node
    }
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Graph {
            nodes: BTreeMap::default(),
            edges: BTreeMap::default(),
        }
    }

    pub fn add_node(&mut self, data: N) -> usize {
        let index = self.nodes.len();
        self.nodes.insert(index, data);
        index
    }

    pub fn remove_node(&mut self, index: usize) -> Option<N> {
        let data = self.nodes.remove(&index);
        self.edges.retain(|edge, _| !edge.touches(index));
        data
    }

    pub fn get_node(&self, index: usize) -> Option<&N> {
        self.nodes.get(&index)
    }

    /// Tuples of neighbor index and edge value
    pub fn neighbors(&self, index: usize) -> impl Iterator<Item = (usize, &E)> {
        self.edges
            .iter()
            .filter(move |(edge, _)| edge.touches(index))
            .map(move |(edge, value)| {
                let Edge(from, to) = edge;
                if *from == index {
                    (to.clone(), value)
                } else {
                    (from.clone(), value)
                }
            })
    }

    pub fn get_edge(&self, from: usize, to: usize) -> Option<&E> {
        self.edges.get(&Edge::new(from, to))
    }

    pub fn are_neighbors(&self, from: usize, to: usize) -> bool {
        self.get_edge(from, to).is_some()
    }

    pub fn nodes(&self) -> impl Iterator<Item = (&usize, &N)> {
        self.nodes.iter()
    }

    pub fn edges(&self) -> impl Iterator<Item = ((&usize, &usize), &E)> {
        self.edges.iter().map(|(edge, v)| ((&edge.0, &edge.1), v))
    }

    /// Takes a path of node indices, returning the edge values between them in order of traversal (if such edges exist)
    pub fn get_edges<'a>(
        &self,
        path: impl Into<&'a [usize]>,
    ) -> impl Iterator<Item = Option<&'_ E>> {
        path.into()
            .windows(2)
            .map(|edge| self.get_edge(edge[0], edge[1]))
    }

    pub fn add_edge(&mut self, from: usize, to: usize, data: E) -> Option<E> {
        let edge = Edge::new(from, to);
        self.edges.insert(edge, data)
    }

    pub fn add_edge_map(
        &mut self,
        from: usize,
        to: usize,
        edge_value: fn(&N, &N) -> E,
    ) -> Option<E> {
        let src = self.get_node(from)?;
        let dst = self.get_node(to)?;

        let edge = Edge::new(from, to);
        self.edges.insert(edge, edge_value(src, dst))
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) -> Option<E> {
        let edge = Edge::new(from, to);
        self.edges.remove(&edge)
    }

    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_retrieval() {
        let mut g: Graph<u32, ()> = Graph::new();
        let i1 = g.add_node(4);

        assert_eq!(g.get_node(i1), Some(&4));
        assert_eq!(g.num_nodes(), 1);
    }

    #[test]
    fn test_edge_retrieval() {
        let mut g: Graph<(), u32> = Graph::new();
        let i1 = g.add_node(());
        let i2 = g.add_node(());

        g.add_edge(i1, i2, 5);

        assert_eq!(g.get_edge(i1, i2), Some(&5));
        assert_eq!(g.get_edge(i2, i1), Some(&5));

        let prev = g.add_edge(i2, i1, 7);
        assert_eq!(prev, Some(5));
        assert_eq!(g.get_edge(i1, i2), Some(&7));

        assert_eq!(g.num_nodes(), 2);
        assert_eq!(g.num_edges(), 1);
    }

    #[test]
    fn test_node_removal() {
        let mut g: Graph<u32, u32> = Graph::new();
        let i1 = g.add_node(4);
        let i2 = g.add_node(5);

        g.add_edge(i1, i2, 1);

        assert_eq!(g.num_nodes(), 2);
        assert_eq!(g.num_edges(), 1);

        let data = g.remove_node(i1);

        assert_eq!(data, Some(4));

        assert_eq!(g.num_nodes(), 1);
        assert_eq!(g.num_edges(), 0);
    }

    #[test]
    fn test_edge_removal() {
        let mut g: Graph<(), u32> = Graph::new();
        let i1 = g.add_node(());
        let i2 = g.add_node(());

        g.add_edge(i1, i2, 5);

        assert_eq!(g.num_edges(), 1);

        let data = g.remove_edge(i1, i2);

        assert_eq!(data, Some(5));

        assert_eq!(g.num_nodes(), 2);
        assert_eq!(g.num_edges(), 0);
    }

    #[test]
    fn test_neighbors() {
        let mut g: Graph<u32, u32> = Graph::new();
        let i1 = g.add_node(1);
        let i2 = g.add_node(2);
        let i3 = g.add_node(3);

        g.add_edge(i2, i1, 11);
        g.add_edge(i2, i3, 12);
        g.add_edge(i1, i3, 13);

        let neighbors: Vec<_> = g.neighbors(i2).collect();

        assert_eq!(neighbors.len(), 2);

        assert_eq!(neighbors[0], (i1, &11));
        assert_eq!(neighbors[1], (i3, &12));
    }
}
