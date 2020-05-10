use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Result};
use std::hash::{Hash, Hasher};
use std::io;
use std::str::FromStr;

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

#[derive(Eq, PartialEq)]
struct GameBoard {
  width: u32,
  height: u32,
  walkable: HashSet<Pos>,
  my_pacmen: HashMap<u32, PacMan>,
  their_pacmen: HashMap<u32, PacMan>,
  pellets: HashMap<Pos, u32>,
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
      my_pacmen: HashMap::new(),
      their_pacmen: HashMap::new(),
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
      let type_id = PacType::from_str(&type_id).unwrap();
      let speed_turns_left = parse_input!(inputs[5], u32); // unused in wood leagues
      let ability_cooldown = parse_input!(inputs[6], u32); // unused in wood leagues
      let updated = PacMan::new(id, pos, type_id);
      if mine == 1 {
        let entry = self.my_pacmen.entry(id).or_insert(updated);
        let new =
          entry.update(pos, type_id, speed_turns_left, ability_cooldown);
        self.my_pacmen.insert(id, new);
      } else {
        let entry = self.their_pacmen.entry(id).or_insert(updated);
        let new =
          entry.update(pos, type_id, speed_turns_left, ability_cooldown);
        self.their_pacmen.insert(id, new);
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
      self.pellets.insert(Pos { x, y }, value);
    }
  }

  fn look_at(&self, pos: Pos) -> LookResult {
    for (_, man) in &self.their_pacmen {
      if man.pos == pos {
        return LookResult::PacMan(*man);
      }
    }

    for (_, man) in &self.my_pacmen {
      if man.pos == pos {
        return LookResult::PacMan(*man);
      }
    }

    if self.pellets.contains_key(&pos) {
      return LookResult::Pellet(*self.pellets.get(&pos).unwrap());
    }

    if self.walkable.contains(&pos) {
      LookResult::Floor
    } else {
      LookResult::Wall
    }
  }

  fn find_pellet(&mut self) -> String {
    let mut result = vec![];
    let mut updated = self.my_pacmen.clone();
    for (_, man) in &self.my_pacmen {
      let mut i = 1;
      let man = loop {
        let man = if let Some(m) = updated.get(&man.id) { *m } else { *man };
        let pos = man.look_forward(i);
        let lr = self.look_at(pos);
        match lr {
          LookResult::Pellet(_) => {
            eprintln!("Found a pellet at {}", pos);
          }
          LookResult::Floor => {
            eprintln!("Found a floor at {}", pos);
          }
          LookResult::Wall => {
            eprintln!("Found a wall at {}", pos);
            if i == 1 {
              let man = man.turn_left();
              updated.insert(man.id, man);
              break man;
            }
            break man;
          }
          LookResult::PacMan(p) => {
            // is this my pacman?
            if self.my_pacmen.contains_key(&p.id) {
              // is it heading this direction?
              if p.direction.reversed() == man.direction {
                // turn right and try again
                let man = man.turn_left();
                updated.insert(man.id, man);
                i = 1;
                continue;
              }
            } else {
              // NOT MY PACMAN - Can I Eat him?
              if man.type_id.beats(p.type_id) {
                result.push(Command::Speed(man));
                break man;
              } else if man.ability_cooldown == 0 {
                result.push(Command::Switch(man, p.type_id.beat_it()));
                break man;
              } else {
                // turn and look somewhere else
                let man = man.turn_left();
                updated.insert(man.id, man);
                i = 1;
                continue;
              }
            }
          }
        }

        // if for some reason we don't hit a wall or anything we need to break the loop
        if i >= 35 {
          eprintln!("Got to max distance without hitting a wall!");
          break man;
        }
        i += 1;
      };

      result.push(man.forward());
    }

    self.my_pacmen = updated;
    let result = result
      .into_iter()
      .map(|c| c.to_string())
      .collect::<Vec<String>>()
      .join(" | ");

    eprintln!("result: {}", result);
    result
  }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn reversed(&self) -> Direction {
    match self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum PacType {
  Rock,
  Paper,
  Scissors,
}

impl PacType {
  fn beats(self, other: PacType) -> bool {
    use PacType::*;
    match (self, other) {
      (Rock, Rock) => false,
      (Rock, Paper) => false,
      (Rock, Scissors) => true,
      (Paper, Rock) => true,
      (Paper, Paper) => false,
      (Paper, Scissors) => false,
      (Scissors, Rock) => false,
      (Scissors, Paper) => true,
      (Scissors, Scissors) => false,
    }
  }

  fn ties(self, other: PacType) -> bool {
    self == other
  }

  fn beat_it(self) -> PacType {
    match self {
      PacType::Rock => PacType::Paper,
      PacType::Paper => PacType::Scissors,
      PacType::Scissors => PacType::Rock,
    }
  }
}

impl Display for PacType {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      PacType::Rock => write!(f, "ROCK"),
      PacType::Paper => write!(f, "PAPER"),
      PacType::Scissors => write!(f, "SCISSORS"),
    }
  }
}

impl FromStr for PacType {
  type Err = ();

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    match s {
      "ROCK" => Ok(PacType::Rock),
      "PAPER" => Ok(PacType::Paper),
      "SCISSORS" => Ok(PacType::Scissors),
      _ => Err(()),
    }
  }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Command {
  Move(PacMan, Pos),
  Switch(PacMan, PacType),
  Speed(PacMan),
}

impl Display for Command {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Command::Move(man, pos) => write!(f, "MOVE {} {}", man, pos),
      Command::Switch(man, pac_type) => {
        write!(f, "SWITCH {} {}", man, pac_type)
      }
      Command::Speed(man) => write!(f, "SPEED {}", man),
    }
  }
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct PacMan {
  id: u32,
  pos: Pos,
  direction: Direction,
  type_id: PacType,
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
  fn new(id: u32, pos: Pos, type_id: PacType) -> Self {
    PacMan {
      id,
      pos,
      direction: Direction::Up,
      type_id,
      speed_turns_left: 0,
      ability_cooldown: 0,
    }
  }

  fn move_to(&self, pos: Pos) -> Command {
    Command::Move(*self, pos)
  }

  fn forward(&self) -> Command {
    match self.direction {
      Direction::Up => Command::Move(*self, self.pos.up(1)),
      Direction::Down => Command::Move(*self, self.pos.down(1)),
      Direction::Left => Command::Move(*self, self.pos.left(1)),
      Direction::Right => Command::Move(*self, self.pos.right(1)),
    }
  }

  fn turn_right(&self) -> PacMan {
    let direction = match self.direction {
      Direction::Up => Direction::Right,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
      Direction::Right => Direction::Down,
    };

    PacMan {
      id: self.id,
      pos: self.pos,
      direction,
      type_id: self.type_id,
      speed_turns_left: self.speed_turns_left,
      ability_cooldown: self.ability_cooldown,
    }
  }

  fn turn_left(&self) -> PacMan {
    let direction = match self.direction {
      Direction::Up => Direction::Left,
      Direction::Down => Direction::Right,
      Direction::Left => Direction::Down,
      Direction::Right => Direction::Up,
    };

    PacMan {
      id: self.id,
      pos: self.pos,
      direction,
      type_id: self.type_id,
      speed_turns_left: self.speed_turns_left,
      ability_cooldown: self.ability_cooldown,
    }
  }

  fn update(
    &self,
    pos: Pos,
    pac_type: PacType,
    speed_turns_left: u32,
    cooldown: u32,
  ) -> Self {
    let type_id = pac_type;
    let ability_cooldown = cooldown;
    PacMan {
      id: self.id,
      pos,
      direction: self.direction,
      type_id,
      speed_turns_left,
      ability_cooldown,
    }
  }

  fn look_forward(&self, dist: u32) -> Pos {
    match self.direction {
      Direction::Up => Pos {
        x: self.pos.x,
        y: if self.pos.y >= dist { self.pos.y - dist } else { 0 },
      },
      Direction::Down => Pos { x: self.pos.x, y: self.pos.y + dist },
      Direction::Left => Pos {
        x: if self.pos.x >= dist { self.pos.x - dist } else { 0 },
        y: self.pos.y,
      },
      Direction::Right => Pos { x: self.pos.x + dist, y: self.pos.y },
    }
  }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum LookResult {
  Pellet(u32),
  Floor,
  Wall,
  PacMan(PacMan),
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
