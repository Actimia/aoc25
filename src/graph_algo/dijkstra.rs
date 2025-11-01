use std::{collections::BTreeSet, ops::Add};

use num_traits::Zero;

use crate::graph::Graph;

#[derive(PartialEq, Eq)]
struct EdgeCost<E>(E, usize);

impl<E: PartialOrd> PartialOrd for EdgeCost<E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<E: Ord> Ord for EdgeCost<E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<V, E> Graph<V, E>
where
    E: Copy + Add<Output = E> + Ord + Zero, //+ Display + Debug,
{
    /// Finds the path from `from` to `to` that minimizes the total edge weight.
    /// The edge weight could represent a distance, a time, or some other value.
    /// Edge weights must be positive.
    pub fn dijkstra(&self, from: usize, to: usize) -> Option<(E, Vec<usize>)> {
        let mut weights: Vec<Option<(E, usize)>> = vec![None; self.num_nodes()];

        let mut heap: BTreeSet<EdgeCost<E>> = BTreeSet::new();
        heap.insert(EdgeCost(E::zero(), from));
        weights[from] = Some((E::zero(), from));

        while !heap.is_empty() {
            let EdgeCost(cost_here, node) = heap.pop_first().expect("is not empty");
            //eprintln!("in {}, cost: {}", node, cost_here);
            if node == to {
                let mut path: Vec<usize> = vec![node];

                while let Some((_, node)) = weights[*path.last().unwrap()] {
                    path.push(node);
                    if node == from {
                        break;
                    }
                }
                path.reverse();
                return Some((cost_here, path));
            }

            for (next_node, cost_of_edge) in self.neighbors(node) {
                let total_to_next = cost_here + *cost_of_edge;

                // if a path to node with lower cost has already been found, do not consider it further
                let visited = &weights[next_node];
                if let Some((prev_cost, _)) = visited
                    && *prev_cost < total_to_next
                {
                    continue;
                }

                weights[next_node] = Some((total_to_next, node));
                heap.insert(EdgeCost(total_to_next, next_node));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let mut g: Graph<(), u32> = Graph::new();

        for _ in 0..10 {
            g.add_node(());
        }

        g.add_edge(0, 1, 2);
        g.add_edge(1, 2, 2);
        g.add_edge(2, 3, 2);
        g.add_edge(3, 4, 2);
        g.add_edge(0, 4, 10);

        let result = g.dijkstra(0, 4);
        assert_eq!(result, Some((8, vec![0, 1, 2, 3, 4])));

        g.add_edge(2, 3, 6);
        let result = g.dijkstra(0, 4);
        assert_eq!(result, Some((10, vec![0, 4])));
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
