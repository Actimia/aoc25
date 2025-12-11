use std::{
  collections::{HashMap, VecDeque},
  str::FromStr,
};

use aoc25::{
  exts::duration::DurationExt,
  graph::Graph,
  time::{time, time_try},
};

const INPUT: &str = include_str!("data/11.txt");

struct Network(HashMap<String, Vec<String>>);

impl FromStr for Network {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut map = HashMap::new();
    for l in s.lines() {
      let (host, outputs) = l.split_once(':').ok_or(anyhow::anyhow!("expected :"))?;

      let outputs: Vec<_> = outputs
        .trim()
        .split_whitespace()
        .map(|o| o.to_owned())
        .collect();

      let host = host.trim().to_owned();
      map.insert(host, outputs);
    }
    Ok(Self(map))
  }
}

impl Network {
  fn to_graph(self) -> (Graph<String, usize>, HashMap<String, usize>) {
    let net = self.0;

    let mut graph = Graph::new();
    graph.add_node("out".to_owned());

    let mut map = HashMap::new();
    map.insert("out".to_owned(), 0);

    for node in net.keys() {
      let i = graph.add_node(node.clone());
      map.insert(node.clone(), i);
    }

    // eprintln!("{map:?}");
    for (node, edges) in &net {
      let src = map.get(node).unwrap();

      for edge in edges {
        if let Some(dst) = map.get(edge) {
          graph.add_edge(*src, *dst, *src);
        }
      }
    }

    (graph, map)
  }
}

fn part_one(Network(net): &Network) -> u64 {
  //
  let mut total = 0;

  let mut queue = VecDeque::new();
  queue.push_back("you");

  while let Some(node) = queue.pop_front() {
    if node == "out" {
      total += 1;
      continue;
    }

    if let Some(outputs) = net.get(node) {
      for output in outputs {
        queue.push_back(output.as_ref());
      }
    }
  }

  total
}

fn part_two(net: Network) -> u64 {
  //
  //let mut total = 0;

  let (graph, map) = net.to_graph();

  let out = map.get("out").unwrap();
  // let svr = map.get("svr").unwrap();

  let mut stack = VecDeque::new();
  stack.push_back(*out);

  let mut result = vec![(0, 0, 0, 0); graph.num_nodes()];

  while let Some(node) = stack.pop_front() {
    let name = graph.get_node(node).unwrap();

    let mut paths = 0;
    if name == "out" {
      paths = 1
    }
    let mut paths_dac = 0;
    let mut paths_fft = 0;
    let mut paths_both = 0;

    for (next, from, (to, dac, fft, both)) in
      graph.neighbors(node).map(|(n, from)| (n, from, result[n]))
    {
      if next != *from {
        stack.push_front(next);
      } else {
        paths += to;
        paths_dac += dac;
        paths_fft += fft;
        paths_both += both;
      }
    }

    if name == "dac" {
      paths_dac = paths;
      paths_both = paths_fft + paths;
      eprintln!("found dac")
    }

    if name == "fft" {
      paths_fft = paths;
      paths_both = paths_dac + paths;
      eprintln!("found fft")
    }

    result[node] = (paths, paths_dac, paths_fft, paths_both);
  }

  // eprintln!("{result:?}");
  let (_, _, _, paths) = result[*map.get("svr").unwrap()];
  paths
}

fn main() -> anyhow::Result<()> {
  let (network, dur) = time_try(|| INPUT.parse())?;
  println!("Parsed points in {}", dur.display());

  let (part_one, dur) = time(|| part_one(&network));
  println!("Part 1: {part_one} (in {})", dur.display());

  let (part_two, dur) = time(|| part_two(network));
  println!("Part 2: {part_two} (in {})", dur.display());

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_INPUT: &str = "aaa: you hhh\nyou: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff\nddd: ggg\neee: out\nfff: out\nggg: out\nhhh: ccc fff iii\niii: out";

  #[test]
  fn test_one() {
    let points = SAMPLE_INPUT.parse().unwrap();
    let total = part_one(&points);
    assert_eq!(total, 5);
  }

  const SAMPLE_INPUT2: &str = "svr: aaa bbb\naaa: fft\nfft: ccc\nbbb: tty\ntty: ccc\nccc: ddd eee\nddd: hub\nhub: fff\neee: dac\ndac: fff\nfff: ggg hhh\nggg: out\nhhh: out";

  #[test]
  fn test_two() {
    let machine = SAMPLE_INPUT2.parse().unwrap();
    let total = part_two(machine);
    assert_eq!(total, 2);
  }
}
