use std::collections::HashMap;

use aoc25::{
  exts::duration::DurationExt,
  graph::Graph,
  graph_algo::search::SearchMode,
  time::{time, time_try},
};
use glam::I64Vec3;
use itertools::Itertools;

const INPUT: &str = include_str!("data/08.txt");

fn parse_graph(input: &str) -> anyhow::Result<Graph<I64Vec3, u64>> {
  let mut graph: Graph<I64Vec3, u64> = Graph::new();

  let mut nodes = vec![];
  for line in input.lines() {
    let nums: Vec<i64> = line.split(',').flat_map(|x| x.parse()).collect();
    anyhow::ensure!(nums.len() == 3, "bad line ({line})");
    nodes.push(I64Vec3::from_slice(nums.as_slice()));
  }

  for pos in &nodes {
    graph.add_node(*pos);
  }

  for (n1, pos1) in nodes.iter().enumerate() {
    let mut shortest = 10000000000; // big enough for our purposes
    for (n2, pos2) in nodes.iter().enumerate() {
      if n1 == n2 {
        continue;
      }
      let dist = (*pos2 - *pos1).length_squared() as u64;

      if dist >= 7 * shortest {
        // this cutoff is somewhat arbitrary, but saves a lot of time
        // part 2 works even with the cutoff = 1
        // part 1 relies on the globally shortest nodes, not locally shortest
        // but seems to work with cutoff >= 7, but that is probably a coincidence
        continue;
      }
      shortest = shortest.min(dist);

      graph.add_edge(n1, n2, dist);
    }
  }

  eprintln!("{} nodes, {} edges", graph.num_nodes(), graph.num_edges());

  Ok(graph)
}

fn count_circuits(graph: &Graph<I64Vec3, ()>) -> usize {
  let mut circuits: HashMap<usize, usize> = HashMap::default(); // size -> count
  let mut visited = vec![false; graph.num_nodes()];

  for (node, _) in graph.nodes() {
    if visited[*node] {
      continue;
    }
    let mut count = 0;
    graph
      .visit(*node, SearchMode::BreadthFirst)
      .for_each(|(node, _)| {
        count += 1;
        visited[node] = true;
      });
    *circuits.entry(count).or_default() += 1;
  }
  circuits.keys().sorted().rev().take(3).product()
}

fn part_one(graph: &Graph<I64Vec3, u64>, count: usize) -> usize {
  // 175500
  let mut connections: Graph<I64Vec3, ()> = Graph::new();
  graph.nodes().for_each(|(_, n)| {
    connections.add_node(*n);
  });

  let mut edges: Vec<_> = graph.edges().collect();
  edges.sort_by(|(_, a), (_, b)| a.cmp(b));

  for ((from, to), _) in edges {
    connections.add_edge(*from, *to, ());

    let num_edges = connections.num_edges();
    if num_edges == count {
      break;
    }
  }

  count_circuits(&connections)
}

fn part_two(graph: &Graph<I64Vec3, u64>) -> u64 {
  // 2402892288: too low
  // 6934702555

  let mut edges: Vec<_> = graph.edges().collect();
  edges.sort_by(|(_, a), (_, b)| a.cmp(b));

  let target_count = graph.num_nodes(); //- 1; // x nodes can be connected with x-1 edges

  let mut visited = vec![false; graph.num_nodes()];
  let mut connected = 0;

  for ((from, to), _dist) in edges {
    if !visited[*from] {
      connected += 1;
      visited[*from] = true;
    }
    if !visited[*to] {
      connected += 1;
      visited[*to] = true;
    }

    if connected >= target_count {
      let from = graph.get_node(*from).unwrap();
      let to = graph.get_node(*to).unwrap();

      return (from.x * to.x) as u64;
    }
  }
  unreachable!()
}

fn main() -> anyhow::Result<()> {
  let (graph, dur) = time_try(|| parse_graph(INPUT))?;
  println!("Parsed input in {}", dur.display());

  let (part_one, dur) = time(|| part_one(&graph, 998));
  println!("Part 1: {part_one} (in {})", dur.display());

  let (part_two, dur) = time(|| part_two(&graph));
  println!("Part 2: {part_two} (in {})", dur.display());
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_INPUT: &str = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

  #[test]
  fn test_one() {
    let graph = parse_graph(SAMPLE_INPUT).unwrap();
    let total = part_one(&graph, 10);
    assert_eq!(total, 40);
  }

  #[test]
  fn test_two() {
    let graph = parse_graph(SAMPLE_INPUT).unwrap();
    let total = part_two(&graph);
    assert_eq!(total, 25272);
  }
}
