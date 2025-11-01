use crate::graph::Graph;

fn pythagorean_distance((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> f64 {
    let x_diff = x1.abs_diff(*x2);
    let y_diff = y1.abs_diff(*y2);
    let sum = x_diff * x_diff + y_diff * y_diff;
    (sum as f64).sqrt()
}

impl<V, E> Graph<V, E> {
    pub fn astar(&self, from: usize, to: usize) -> Option<Vec<usize>> {
        None
    }
}
