use core::f64;
use std::{collections::HashMap, time::Instant};

use aoc25::{graph::Graph, graph_algo::search::SearchMode, vex::Vex};
use itertools::Itertools;

const INPUT: &str = include_str!("data/08.txt");

fn parse_graph(input: &str) -> anyhow::Result<Graph<Vex<f64, 3>, f64>> {
  let mut graph: Graph<Vex<f64, 3>, f64> = Graph::new();

  let mut nodes = vec![];
  for line in input.lines() {
    let nums: Vec<f64> = line.split(',').flat_map(|x| x.parse()).collect();
    anyhow::ensure!(nums.len() == 3, "bad line ({line})");
    nodes.push(Vex::new([nums[0], nums[1], nums[2]]));
  }

  for pos in &nodes {
    graph.add_node(*pos);
  }

  for (n1, pos1) in nodes.iter().enumerate() {
    for (n2, pos2) in nodes.iter().enumerate() {
      if n1 == n2 {
        continue;
      }
      graph.add_edge(n1, n2, (*pos2 - *pos1).length());
    }
  }

  Ok(graph)
}

fn count_circuits(graph: &Graph<Vex<f64, 3>, ()>) -> usize {
  let mut circuits: HashMap<usize, usize> = HashMap::default(); // size -> count
  let mut visited = vec![false; graph.num_nodes()];

  for (from, _) in graph.nodes() {
    if visited[*from] {
      continue;
    }
    let mut count = 0;
    let mut circuit = vec![];
    graph
      .visit(*from, SearchMode::BreadthFirst)
      .for_each(|(node, _)| {
        count += 1;
        visited[node] = true;
        circuit.push(node)
      });
    *circuits.entry(count).or_default() += 1;
  }

  eprintln!("{circuits:?}");
  circuits.keys().sorted().rev().take(3).product()
}

fn part_one(graph: Graph<Vex<f64, 3>, f64>, count: usize) -> usize {
  // 175500
  let mut connections: Graph<Vex<f64, 3>, ()> = Graph::new();
  graph.nodes().for_each(|(_, n)| {
    connections.add_node(*n);
  });

  let mut edges: Vec<_> = graph.edges().collect();
  edges.sort_by(|a, b| a.1.total_cmp(b.1));

  for ((from, to), _) in edges {
    let num_edges = connections.num_edges();
    if num_edges >= count {
      break;
    }

    if let Some(_) = connections.get_edge(*from, *to) {
      continue;
    }

    connections.add_edge(*from, *to, ());
    /* eprintln!(
      "({num_edges}) {:?} -> {:?} = {dist}",
      graph.get_node(*from).unwrap(),
      graph.get_node(*to).unwrap()
    ); */
  }

  count_circuits(&connections)
}

fn part_two(graph: Graph<Vex<f64, 3>, f64>) -> u64 {
  // 2402892288: too low
  // 6934702555
  let mut connections: Graph<Vex<f64, 3>, ()> = Graph::new();
  graph.nodes().for_each(|(_, n)| {
    connections.add_node(*n);
  });

  let mut edges: Vec<_> = graph.edges().collect();
  edges.sort_by(|a, b| a.1.total_cmp(b.1));

  let target_count = graph.num_nodes() - 1; // x nodes can be connected with x-1 edges

  for ((from, to), _) in edges {
    let target = graph.get_node(*to).unwrap();
    if connections
      .astar(*from, *to, |a, _| (*a - *target).length())
      .is_some()
    {
      continue;
    }

    connections.add_edge(*from, *to, ());
    let num_edges = connections.num_edges();
    /* eprintln!(
      "({num_edges}) {:?} -> {:?} = {dist}",
      graph.get_node(*from).unwrap(),
      graph.get_node(*to).unwrap()
    ); */
    if num_edges == target_count {
      let from = graph.get_node(*from).unwrap();
      let to = graph.get_node(*to).unwrap();

      return (*from.x() as u64) * (*to.x() as u64);
    }
  }
  0
}

fn main() -> anyhow::Result<()> {
  let start = Instant::now();
  let graph = parse_graph(INPUT)?;
  println!("Parsed input in {}ms", start.elapsed().as_millis());

  let start = Instant::now();
  let part_one = part_one(graph.clone(), 998);
  println!("Part 1: {part_one} (in {}ms)", start.elapsed().as_millis());

  let start = Instant::now();
  let part_two = part_two(graph);
  println!("Part 2: {part_two} (in {}ms)", start.elapsed().as_millis());
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_INPUT: &str = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

  #[test]
  fn test_one() {
    let graph = parse_graph(SAMPLE_INPUT).unwrap();
    let total = part_one(graph, 10);
    assert_eq!(total, 40);
  }

  #[test]
  fn test_two() {
    let graph = parse_graph(SAMPLE_INPUT).unwrap();
    let total = part_two(graph);
    assert_eq!(total, 25272);
  }
}
