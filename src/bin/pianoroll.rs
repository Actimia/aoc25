use std::{env, fmt::Display, time::Duration};
use yansi::Paint;

use aoc25::{
  grid::Grid,
  seq::{SILENCE, Seq},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum PianoRoll {
  Silence,
  Note(usize),
}

impl Display for PianoRoll {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      PianoRoll::Note(color) => {
        let text = match color {
          0 => "█".red(),
          1 => "█".blue(),
          2 => "█".green(),
          3 => "█".cyan(),
          4 => "█".yellow(),
          5 => "█".magenta(),
          _ => "█".white(),
        };
        write!(f, "{}", text)
      }
      PianoRoll::Silence => write!(f, " "),
    }
  }
}

fn clear() {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
  let args: Vec<String> = env::args().collect();

  let mut patterns: Vec<Seq> = args[1..]
    .iter()
    .map(|p| Seq::try_from(p.as_str()).unwrap())
    .collect();

  let freq = {
    let tempo = 240f64;
    Duration::from_secs_f64(60.0 / tempo)
  };

  let mut grid = Grid::new(24, 40, PianoRoll::Silence);

  loop {
    for x in 0..grid.rows() {
      grid[(x, 0)] = PianoRoll::Silence;
    }
    for (idx, note) in patterns
      .iter_mut()
      .map(|pat| pat.next().unwrap())
      .enumerate()
      .filter(|(_, x)| *x != SILENCE)
    {
      let note = (note.clamp(-12, 12) + 12) as usize;
      if let Some(cell) = grid.get_mut(note, 0) {
        *cell = PianoRoll::Note(idx)
      }
    }

    grid.rotate_cols(-1);

    clear();
    print!("{}", grid);
    std::thread::sleep(freq);
  }
}

// "\033[2J"
