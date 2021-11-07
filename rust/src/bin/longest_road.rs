#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::io;

macro_rules! parse_input {
  ($x:expr, $t:ident) => {
    $x.trim().parse::<$t>().unwrap()
  };
}

/// Indicates a position on the game board.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
struct Pos(usize, usize);

impl Pos {
  pub fn adjacent(&self, other: Pos) -> bool {
    let Pos(s1, s2) = *self;
    let Pos(o1, o2) = other;

    s1 == o1 && (s2 + 1 == o2 || s2 - 1 == o2)
      || s2 == o2 && (s1 + 1 == o1 || s1 - 1 == o1)
  }
}

/// Designates a Player and their symbol
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
struct Player(char, char);

/// This is a cell in the grid - help by a Player-path, player-base, or Open
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Slot {
  Path(Player),
  Base(Player),
  Open,
}

impl Slot {
  pub fn new(val: char) -> Self {
    match val {
      'a' => Slot::Path(Player('a', 'A')),
      'A' => Slot::Base(Player('a', 'A')),
      'b' => Slot::Path(Player('b', 'B')),
      'B' => Slot::Base(Player('b', 'B')),
      'c' => Slot::Path(Player('c', 'C')),
      'C' => Slot::Base(Player('c', 'C')),
      'd' => Slot::Path(Player('d', 'D')),
      'D' => Slot::Base(Player('d', 'D')),
      'e' => Slot::Path(Player('e', 'E')),
      'E' => Slot::Base(Player('e', 'E')),
      'f' => Slot::Path(Player('f', 'F')),
      'F' => Slot::Base(Player('f', 'F')),
      'g' => Slot::Path(Player('g', 'G')),
      'G' => Slot::Base(Player('g', 'G')),
      'h' => Slot::Path(Player('h', 'H')),
      'H' => Slot::Base(Player('h', 'H')),
      'i' => Slot::Path(Player('i', 'I')),
      'I' => Slot::Base(Player('i', 'I')),
      'j' => Slot::Path(Player('j', 'J')),
      'J' => Slot::Base(Player('j', 'J')),
      'k' => Slot::Path(Player('k', 'K')),
      'K' => Slot::Base(Player('k', 'K')),
      'l' => Slot::Path(Player('l', 'L')),
      'L' => Slot::Base(Player('l', 'L')),
      'm' => Slot::Path(Player('m', 'M')),
      'M' => Slot::Base(Player('m', 'M')),
      'n' => Slot::Path(Player('n', 'N')),
      'N' => Slot::Base(Player('n', 'N')),
      'o' => Slot::Path(Player('o', 'O')),
      'O' => Slot::Base(Player('o', 'O')),
      'p' => Slot::Path(Player('p', 'P')),
      'P' => Slot::Base(Player('p', 'P')),
      'q' => Slot::Path(Player('q', 'Q')),
      'Q' => Slot::Base(Player('q', 'Q')),
      'r' => Slot::Path(Player('r', 'R')),
      'R' => Slot::Base(Player('r', 'R')),
      's' => Slot::Path(Player('s', 'S')),
      'S' => Slot::Base(Player('s', 'S')),
      't' => Slot::Path(Player('t', 'T')),
      'T' => Slot::Base(Player('t', 'T')),
      'u' => Slot::Path(Player('u', 'U')),
      'U' => Slot::Base(Player('u', 'U')),
      'v' => Slot::Path(Player('v', 'V')),
      'V' => Slot::Base(Player('v', 'V')),
      'w' => Slot::Path(Player('w', 'W')),
      'W' => Slot::Base(Player('w', 'W')),
      'x' => Slot::Path(Player('x', 'X')),
      'X' => Slot::Base(Player('x', 'X')),
      'y' => Slot::Path(Player('y', 'Y')),
      'Y' => Slot::Base(Player('y', 'Y')),
      'z' => Slot::Path(Player('z', 'Z')),
      'Z' => Slot::Base(Player('z', 'Z')),
      _ => Slot::Open,
    }
  }
}

/// This is the game Grid
#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct Grid {
  data: HashMap<Pos, Slot>,
}

impl std::ops::Deref for Grid {
  type Target = HashMap<Pos, Slot>;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl std::ops::DerefMut for Grid {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data
  }
}

impl Grid {
  pub fn new() -> Self {
    Grid::default()
  }

  pub fn new_from(val: &str) -> Self {
    let mut data = HashMap::new();
    for (i, val) in val.chars().enumerate() {
      let slot = Slot::new(val);
      let pos = Pos(i, 0);
      data.insert(pos, slot);
    }

    Grid { data }
  }

  pub fn players(&self) -> Vec<Player> {
    let mut players = vec![];
    for v in self.values() {
      match v {
        Slot::Path(p) => players.push(*p),
        Slot::Base(p) => players.push(*p),
        Slot::Open => {}
      }
    }
    players
  }

  pub fn roads_for_player(&self, player: Player) -> Path {
    let mut v = HashSet::new();
    for (pos, slot) in self.iter() {
      if *slot == Slot::Path(player) {
        v.insert(*pos);
      }
    }
    Path::new_with_data(v)
  }

  pub fn height(&self) -> usize {
    let mut h = 0;
    for key in self.keys() {
      if key.1 + 1 > h {
        h = key.1 + 1
      }
    }
    h
  }

  #[allow(dead_code)]
  pub fn width(&self) -> usize {
    let mut w = 0;
    for key in self.keys() {
      if key.0 + 1 > w {
        w = key.0 + 1
      }
    }
    w
  }

  pub fn push_str(&mut self, val: &str) {
    let height = self.height();
    for (i, val) in val.chars().enumerate() {
      let slot = Slot::new(val);
      let pos = Pos(i, height);
      self.insert(pos, slot);
    }
  }
}

/// This is a collection of points that can be used as a path
#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct Path {
  data: HashSet<Pos>,
}

/// Holds the result of following a path
#[derive(Clone, Debug, Eq, PartialEq)]
enum PathResult {
  /// This is result for a partial path traversal.
  Partial(
    /// The path that is being followed
    Path,
    /// Where we are at on the path.
    Pos,
  ),
  /// This is the complete path with all branches off of it.
  Complete(Vec<Path>),
}

impl Path {
  /// Append the positions from one path to another.
  /// Leaves the other Path empty.
  pub fn append(&mut self, other: &mut Path) {
    for pos in other.drain() {
      self.insert(pos);
    }
  }

  /// Initialize a new Path from a HashSet
  pub fn new_with_data(data: HashSet<Pos>) -> Self {
    Self { data }
  }

  /// A new default Path
  pub fn new() -> Self {
    Self::default()
  }

  /// Get any points adjacent to this one
  pub fn get_adjacent(&self, pos: Pos, exclude: &HashSet<Pos>) -> Self {
    let mut data = HashSet::new();
    for p in &self.data {
      if p.adjacent(pos) && !exclude.contains(p) {
        data.insert(*p);
      }
    }
    Self { data }
  }

  pub fn num_paths_from(&self, pos: Pos, exclude: &HashSet<Pos>) -> usize {
    self.get_adjacent(pos, exclude).len()
  }

  pub fn follow_path(&self, pos: Pos, exclude: &HashSet<Pos>) -> PathResult {
    let mut path = Path::new();
    let mut paths = vec![];
    if !self.contains(&pos)
      || self.len() == 0
      || !self.has_adjacent(pos, exclude)
      || exclude.contains(&pos)
    {
      // End of the line - return path
      return PathResult::Complete(vec![self.clone()]);
    }
    let mut index = 0;
    let mut exclude = exclude.to_owned();
    // Add this point to the exclusion list
    exclude.insert(pos);
    while index < self.len() {
      for i in self.get_adjacent(pos, &exclude).iter() {
        if exclude.contains(i) {
          // skip excluded positions
          continue;
        }
        let mut i = *i;
        loop {
          match self.follow_path(i, &exclude) {
            PathResult::Partial(mut p, pos) => {
              path.append(&mut p);
              i = pos;
            }
            PathResult::Complete(mut p) => {
              // got to the end of a path - add it to the total paths
              paths.append(&mut p);
              break;
            }
          }
        }
        if let PathResult::Partial(mut p, pos) = self.follow_path(i, &exclude) {
          path.append(&mut p);
          exclude.insert(pos);
        }
      }
      paths.push(path.clone());
      index += 1;
    }

    PathResult::Complete(paths)
  }

  pub fn get_adjacent_paths(&self) -> Vec<Path> {
    let exclude = HashSet::new();
    let mut paths = vec![];
    for pos in self.iter() {
      if let PathResult::Complete(mut p) = self.follow_path(*pos, &exclude) {
        paths.append(&mut p)
      }
    }
    paths
  }

  pub fn get_longest_path(&self) -> Option<Path> {
    let mut longest: Option<Path> = None;
    for path in self.get_adjacent_paths() {
      if let Some(p) = &longest {
        if path.len() > p.len() {
          longest = Some(path);
        }
      }
    }

    longest
  }

  pub fn has_adjacent(&self, pos: Pos, exclude: &HashSet<Pos>) -> bool {
    self.get_adjacent(pos, exclude).len() != 0
  }
}

impl std::ops::Deref for Path {
  type Target = HashSet<Pos>;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl std::ops::DerefMut for Path {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data
  }
}

impl std::ops::AddAssign for Path {
  fn add_assign(&mut self, rhs: Self) {
    for pos in rhs.data {
      self.data.insert(pos);
    }
  }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
pub fn main() {
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let n = parse_input!(input_line, i32);
  let mut grid = Grid::new();
  for _ in 0..n as usize {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let line = input_line.trim_matches('\n').to_string();
    grid.push_str(&line);
  }

  let players = grid.players();
  let mut longest_road = 0;
  let mut winner = None;
  for player in players {
    let roads = grid.roads_for_player(player);
    if let Some(longest_path) = roads.get_longest_path() {
      if longest_path.len() > longest_road {
        longest_road = longest_path.len();
        winner = Some(player);
      }
    }
  }

  if let Some(p) = winner {
    println!("{} {}", p.1, longest_road);
  }

  // Write an answer using println!("message...");
  // To debug: eprintln!("Debug message...");
}
