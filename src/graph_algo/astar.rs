use std::collections::BTreeSet;

use crate::graph::Graph;

struct HeuristicCost(f64, usize);

impl PartialEq for HeuristicCost {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}
impl Eq for HeuristicCost {}
impl PartialOrd for HeuristicCost {
  #[expect(clippy::non_canonical_partial_ord_impl)]
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.0.total_cmp(&other.0))
  }
}
impl Ord for HeuristicCost {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.0.total_cmp(&other.0)
  }
}

impl<V, E> Graph<V, E> {
  /// Implementation of A* search, ideal for finding a path in planar graphs.
  /// Similar do `dijkstra`, but the search priority is determined by a
  /// heuristic function. The lower the value of the heuristic function, the earlier
  /// the node will be evaluated. The heuristic function is evaluated with the data
  /// for the node and the edge that is being considered,
  pub fn astar(
    &self,
    from: usize,
    to: usize,
    heuristic: impl Fn(&V, &E) -> f64,
  ) -> Option<Vec<usize>> {
    let mut visited: Vec<Option<usize>> = vec![None; self.num_nodes()];

    let mut heap: BTreeSet<HeuristicCost> = BTreeSet::new();
    heap.insert(HeuristicCost(0.0, from));

    while !heap.is_empty() {
      let HeuristicCost(_, node) = heap.pop_first().expect("is not empty");
      // eprintln!("in {}", node);
      if node == to {
        let mut path: Vec<usize> = vec![node];
        // eprintln!("  found");

        while let Some(node) = visited[*path.last().unwrap()] {
          // eprintln!("    backtrack: {}", node);
          path.push(node);
          if node == from {
            break;
          }
        }
        path.reverse();
        return Some(path);
      }

      for (next_node, edge) in self.neighbors(node) {
        let node_data = self.get_node(next_node)?;
        let eval = heuristic(node_data, edge);

        // eprintln!("  next: {}, eval: {}", next_node, eval);

        if visited[next_node].is_some() {
          continue;
        }

        visited[next_node] = Some(node);
        heap.insert(HeuristicCost(eval, next_node));
      }
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use glam::DVec2;

  use super::*;

  #[test]
  fn test_astar() {
    let mut g: Graph<DVec2, ()> = Graph::new();

    g.add_node(DVec2::new(0.0, 0.0));
    g.add_node(DVec2::new(1.0, 1.0));
    g.add_node(DVec2::new(0.0, 1.0));
    g.add_node(DVec2::new(-1.0, 1.0));
    g.add_node(DVec2::new(2.0, 0.0));
    g.add_node(DVec2::new(6.0, 0.0));

    g.add_edge(0, 1, ());
    g.add_edge(1, 2, ());
    g.add_edge(2, 3, ());
    g.add_edge(3, 4, ());
    g.add_edge(1, 4, ());

    let target = g.get_node(4).unwrap();
    let heuristic = move |node: &DVec2, _: &()| (*node - *target).length();
    let result = g.astar(0, 4, heuristic);
    assert_eq!(result, Some(vec![0, 1, 4]));

    let result = g.astar(5, 4, heuristic);
    assert_eq!(result, None)
  }

  #[test]
  fn test_dijkstra_no_path() {
    let mut g: Graph<(), u32> = Graph::new();
    for _ in 0..10 {
      g.add_node(());
    }

    g.add_edge(0, 1, 2);
    g.add_edge(1, 2, 2);

    assert_eq!(g.dijkstra(4, 8), None);
  }
}
