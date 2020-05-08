use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Result};
use std::hash::{Hash, Hasher};
use std::io;

macro_rules! parse_input {
  ($x:expr, $t:ident) => {
    $x.trim().parse::<$t>().unwrap()
  };
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Pos {
  x: u32,
  y: u32,
}

impl Default for Pos {
  fn default() -> Self {
    Pos { x: 0, y: 0 }
  }
}

impl Display for Pos {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{} {}", self.x, self.y)
  }
}

impl Pos {
  fn new(x: u32, y: u32) -> Self {
    Pos { x, y }
  }

  fn up(&self, dist: u32) -> Pos {
    let x = self.x;
    let y = if self.y >= dist { self.y - dist } else { 0 };
    Pos { x, y }
  }

  fn left(&self, dist: u32) -> Pos {
    let x = if self.x >= dist { self.x - dist } else { 0 };
    let y = self.y;
    Pos { x, y }
  }

  fn down(&self, dist: u32) -> Pos {
    let x = self.x;
    let y = self.y + dist;
    Pos { x, y }
  }

  fn right(&self, dist: u32) -> Pos {
    let x = self.x + dist;
    let y = self.y;
    Pos { x, y }
  }

  fn distance_to(&self, other: Pos) -> u32 {
    let x = self.x as i32;
    let y = self.y as i32;
    let ox = other.x as i32;
    let oy = other.y as i32;

    let dx = (x - ox).abs() as f64;
    let dy = (y - oy).abs() as f64;

    let rsq = dx * dx + dy * dy;

    rsq.sqrt() as u32
  }
}

struct GameBoard {
  width: u32,
  height: u32,
  walkable: HashSet<Pos>,
  my_pacmen: Vec<PacMan>,
  their_pacmen: Vec<PacMan>,
  pellets: HashMap<Pos, Pellet>,
  my_score: u32,
  opponent_score: u32,
}

impl Default for GameBoard {
  fn default() -> Self {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], u32); // size of the grid
    let height = parse_input!(inputs[1], u32); // top left corner is (x=0, y=0)
    let mut walkable = HashSet::new();
    for y in 0..height as u32 {
      let mut input_line = String::new();
      io::stdin().read_line(&mut input_line).unwrap();
      let row = input_line.trim_end().to_string(); // one line of the grid: space " " is floor, pound "#" is wall
      let chars = row.chars().collect::<Vec<char>>();
      let width = chars.len();
      for x in 0..width as u32 {
        let value = chars[x as usize];
        if value == ' ' {
          walkable.insert(Pos { x, y });
        }
      }
    }
    GameBoard {
      width,
      height,
      walkable,
      my_pacmen: vec![],
      their_pacmen: vec![],
      pellets: HashMap::new(),
      my_score: 0,
      opponent_score: 0,
    }
  }
}

impl GameBoard {
  fn update(&mut self) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    self.my_score = parse_input!(inputs[0], u32);
    self.opponent_score = parse_input!(inputs[1], u32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let visible_pac_count = parse_input!(input_line, u32); // all your pacs and enemy pacs in sight
    self.my_pacmen.clear();
    self.their_pacmen.clear();
    for i in 0..visible_pac_count as usize {
      let mut input_line = String::new();
      io::stdin().read_line(&mut input_line).unwrap();
      let inputs = input_line.split(" ").collect::<Vec<_>>();
      let id = parse_input!(inputs[0], u32); // pac number (unique within a team)
      let mine = parse_input!(inputs[1], u32); // true if this pac is yours
      let x = parse_input!(inputs[2], u32); // position in the grid
      let y = parse_input!(inputs[3], u32); // position in the grid
      let pos = Pos { x, y };
      let type_id = inputs[4].trim().to_string(); // unused in wood leagues
      let speed_turns_left = parse_input!(inputs[5], u32); // unused in wood leagues
      let ability_cooldown = parse_input!(inputs[6], u32); // unused in wood leagues
      if mine == 1 {
        self.my_pacmen.push(PacMan {
          id,
          pos,
          target: pos,
          type_id,
          speed_turns_left,
          ability_cooldown,
        });
      } else {
        self.their_pacmen.push(PacMan {
          id,
          pos,
          target: pos,
          type_id,
          speed_turns_left,
          ability_cooldown,
        });
      }
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let visible_pellet_count = parse_input!(input_line, u32); // all pellets in sight
    eprintln!("Visible Pellets: {}", visible_pellet_count);
    self.pellets.clear();
    for _ in 0..visible_pellet_count as usize {
      let mut input_line = String::new();
      io::stdin().read_line(&mut input_line).unwrap();
      let inputs = input_line.split(" ").collect::<Vec<_>>();
      let x = parse_input!(inputs[0], u32);
      let y = parse_input!(inputs[1], u32);
      let value = parse_input!(inputs[2], u32); // amount of points this pellet is worth
      self.pellets.insert(Pos { x, y }, Pellet { pos: Pos { x, y }, value });
    }
  }

  fn find_pellet(&mut self) -> String {
    let mut result = vec![];
    let mut targets = vec![];
    // todo iterate through pacmen to get targets
    for pac_man in &self.my_pacmen {
      targets.push(pac_man.target);
      targets.push(pac_man.pos);
    }

    for pac_man in &mut self.my_pacmen {
      eprintln!("pacman: {}, target: {}", pac_man.pos, pac_man.target);

      if pac_man.pos != pac_man.target {
        result.push(pac_man.move_to_target());
      }

      let mut pellets = self
        .pellets
        .iter()
        .filter_map(|(pos, pel)| {
          if pel.value > 1 && !targets.contains(pos) {
            Some(pos)
          } else {
            None
          }
        })
        .collect::<Vec<_>>();

      let mut distance = std::u32::MAX;
      let mut nearest = Pos::default();

      if pellets.is_empty() {
        for pellet in self.pellets.keys() {
          if targets.contains(pellet) {
            continue;
          }
          let this_dist = pac_man.pos.distance_to(*pellet);
          if this_dist < distance {
            distance = this_dist;
            nearest = *pellet;
          }
        }

        pac_man.target = nearest;
      } else {
        for p in pellets {
          let this_dist = pac_man.pos.distance_to(*p);
          if this_dist < distance {
            distance = this_dist;
            nearest = *p;
          }
        }

        pac_man.target = nearest;
      }

      result.push(pac_man.move_to_target());
    }
    // Default
    result.join(" | ")
  }
}

enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Eq, PartialEq, Clone)]
struct PacMan {
  id: u32,
  pos: Pos,
  target: Pos,
  type_id: String,
  speed_turns_left: u32,
  ability_cooldown: u32,
}

impl Hash for PacMan {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
  }
}

impl Display for PacMan {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.id)
  }
}

impl PacMan {
  fn move_to(&self, pos: Pos) -> String {
    // TODO find the closest pellet for each PacMan
    format!("MOVE {} {}", self, pos)
    // String::from("MOVE 0 15 10") // MOVE <pacId> <x> <y>
  }

  fn move_to_target(&self) -> String {
    format!("MOVE {} {}", self, self.target)
  }
}
struct Pellet {
  // position of the pellet
  pos: Pos,
  // the score value of the pellet
  value: u32,
}

/**
 * Grab the pellets as fast as you can!
 **/
fn main() {
  let mut game = GameBoard::default();

  // game loop
  loop {
    game.update();

    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", game.find_pellet()); // MOVE <pacId> <x> <y>
  }
}
