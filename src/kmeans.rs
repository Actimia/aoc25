use crate::{exts::numbers::F64Ext, vex::Vex};

fn centroid<const N: usize>(points: &Vec<Vex<f64, N>>) -> Vex<f64, N> {
  let mut sum = Vex::zero();
  for p in points {
    sum += *p;
  }
  sum / points.len() as f64
}

pub fn kmeans<const N: usize>(items: impl AsRef<[Vex<f64, N>]>, k: usize) -> Vec<Vec<Vex<f64, N>>> {
  let items = items.as_ref();
  let mut centroids: Vec<Vex<f64, N>> = items[..k].to_vec();

  loop {
    let mut clusters: Vec<Vec<Vex<f64, N>>> = vec![vec![]; k];

    for p in items {
      if let Some((idx, _)) = centroids
        .iter()
        .enumerate()
        .min_by_key(|(_, centroid)| (**centroid - *p).length().comparable())
      {
        clusters[idx].push(*p)
      }
    }

    let new_centroids: Vec<Vex<f64, N>> = clusters.iter().map(centroid).collect();
    if new_centroids == centroids {
      return clusters;
    } else {
      centroids = new_centroids;
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_kmeans() {
    let items = vec![
      Vex::new([4., 3.]),
      Vex::new([4., 1.]),
      Vex::new([-8., 2.]),
      Vex::new([-7., 3.]),
    ];
    let buckets = kmeans(items, 2);
    assert_eq!(buckets.len(), 2);
    assert_eq!(buckets[0], vec![Vex::new([-8., 2.]), Vex::new([-7., 3.])]);
    assert_eq!(buckets[1], vec![Vex::new([4., 3.]), Vex::new([4., 1.])]);
  }
}
