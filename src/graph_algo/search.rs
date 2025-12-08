use std::collections::VecDeque;

use crate::graph::Graph;

pub enum SearchMode {
  BreadthFirst,
  DepthFirst,
}

impl SearchMode {
  fn next<T>(&self, cands: &mut VecDeque<T>) -> T {
    match self {
      SearchMode::BreadthFirst => cands.pop_front(), // treat as a queue
      SearchMode::DepthFirst => cands.pop_back(),    // treat as a stack
    }
    .expect("is not empty")
  }
}

impl<V, E> Graph<V, E> {
  pub fn search(&self, from: usize, to: usize, mode: SearchMode) -> Option<Vec<usize>> {
    let mut came_from = vec![None; self.num_nodes()];

    let mut candidates: VecDeque<usize> = VecDeque::new();
    candidates.push_back(from);

    while !candidates.is_empty() {
      let cur = mode.next(&mut candidates);

      if cur == to {
        let mut path: Vec<usize> = vec![cur];

        while let Some(n) = came_from[*path.last().unwrap()] {
          path.push(n);
          if n == from {
            break;
          }
        }
        path.reverse();
        return Some(path);
      }

      for (node, _) in self.neighbors(cur) {
        if came_from[node].is_some() {
          continue;
        }
        candidates.push_back(node);
        came_from[node] = Some(cur);
      }
    }

    None
  }

  /// Iterates over all nodes connected to `from`, in the order specified in `mode`.
  /// The iteration order of neighbors is not defined.
  pub fn visit(&self, from: usize, mode: SearchMode) -> impl Iterator<Item = (usize, &V)> {
    let mut visited = vec![false; self.num_nodes()];
    let mut candidates = VecDeque::new();
    visited[from] = true;
    candidates.push_back(from);

    GraphVisitor {
      graph: self,
      mode,
      visited,
      candidates,
    }
  }
}

struct GraphVisitor<'a, N, E> {
  graph: &'a Graph<N, E>,
  mode: SearchMode,
  visited: Vec<bool>,
  candidates: VecDeque<usize>,
}

impl<'a, N, E> Iterator for GraphVisitor<'a, N, E> {
  type Item = (usize, &'a N);

  fn next(&mut self) -> Option<Self::Item> {
    if self.candidates.is_empty() {
      return None;
    }

    let current = self.mode.next(&mut self.candidates);
    for (node, _) in self.graph.neighbors(current) {
      if self.visited[node] {
        continue;
      }
      self.candidates.push_back(node);
      self.visited[node] = true;
    }
    self.graph.get_node(current).map(|node| (current, node))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_visit_bfs() {
    let mut g: Graph<u32, ()> = Graph::new();

    for i in 0..5 {
      g.add_node(i);
    }

    g.add_edge(0, 1, ());
    g.add_edge(1, 2, ());
    g.add_edge(0, 3, ());
    g.add_edge(0, 4, ());

    let nodes: Vec<_> = g
      .visit(0, SearchMode::BreadthFirst)
      .map(|(_, node)| node)
      .copied()
      .collect();
    assert_eq!(nodes, vec![0, 1, 3, 4, 2])
  }

  #[test]
  fn test_visit_dfs() {
    let mut g: Graph<u32, ()> = Graph::new();

    for i in 0..5 {
      g.add_node(i);
    }

    g.add_edge(0, 1, ());
    g.add_edge(1, 2, ());
    g.add_edge(0, 3, ());
    g.add_edge(2, 4, ());

    let nodes: Vec<_> = g
      .visit(0, SearchMode::DepthFirst)
      .map(|(_, node)| node)
      .copied()
      .collect();
    assert_eq!(nodes, vec![0, 3, 1, 2, 4])
  }

  #[test]
  fn test_search_bfs() {
    let mut g: Graph<(), ()> = Graph::new();

    for _ in 0..10 {
      g.add_node(());
    }

    g.add_edge(0, 1, ());
    g.add_edge(1, 2, ());
    g.add_edge(2, 3, ());
    g.add_edge(3, 4, ());

    let path = g
      .search(0, 4, SearchMode::BreadthFirst)
      .expect("should be a path");

    assert_eq!(path, vec![0, 1, 2, 3, 4]);

    g.add_edge(1, 4, ());
    let path = g
      .search(0, 4, SearchMode::BreadthFirst)
      .expect("should be a path");
    assert_eq!(path, vec![0, 1, 4]);

    assert_eq!(g.search(2, 8, SearchMode::BreadthFirst), None);
  }
}
