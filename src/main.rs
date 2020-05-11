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
        entry.update(pos, type_id, speed_turns_left, ability_cooldown);
      } else {
        let entry = self.their_pacmen.entry(id).or_insert(updated);
        entry.update(pos, type_id, speed_turns_left, ability_cooldown);
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
        return LookResult::TheirPac(*man, 0);
      }
    }

    for (_, man) in &self.my_pacmen {
      if man.pos == pos {
        return LookResult::MyPac(*man, 0);
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

  fn look_ahead(&self, id: u32) -> Option<LookResult> {
    let man =
      if let Some(man) = self.my_pacmen.get(&id) { man } else { return None };
    let mut score = 0;
    for i in 1..35 {
      let pos = man.look_forward(i, self.width, self.height);
      let result = self.look_at(pos);
      match result {
        LookResult::Pellet(p) => score += p,
        LookResult::Floor => {}
        LookResult::Wall => {
          if i == 1 {
            return Some(result);
          } else {
            break;
          }
        }
        LookResult::MyPac(man, p) => {
          return Some(LookResult::MyPac(man, p + score))
        }
        LookResult::TheirPac(man, p) => {
          return Some(LookResult::TheirPac(man, p + score))
        }
      }
    }

    if score > 0 {
      Some(LookResult::Pellet(score))
    } else {
      let pos = man.look_forward(1, self.width, self.height);
      Some(self.look_at(pos))
    }
  }

  fn look_left(&self, id: u32) -> Option<LookResult> {
    let man =
      if let Some(man) = self.my_pacmen.get(&id) { man } else { return None };
    let mut score = 0;
    for i in 1..35 {
      let pos = man.look_left(i, self.width, self.height);
      let result = self.look_at(pos);
      match result {
        LookResult::Pellet(p) => score += p,
        LookResult::Floor => {}
        LookResult::Wall => {
          if i == 1 {
            return Some(result);
          } else {
            break;
          }
        }
        LookResult::MyPac(man, p) => {
          return Some(LookResult::MyPac(man, p + score))
        }
        LookResult::TheirPac(man, p) => {
          return Some(LookResult::TheirPac(man, p + score))
        }
      }
    }

    if score > 0 {
      Some(LookResult::Pellet(score))
    } else {
      let pos = man.look_left(1, self.width, self.height);
      Some(self.look_at(pos))
    }
  }

  fn look_right(&self, id: u32) -> Option<LookResult> {
    let man =
      if let Some(man) = self.my_pacmen.get(&id) { man } else { return None };
    let mut score = 0;
    for i in 1..35 {
      let pos = man.look_right(i, self.width, self.height);
      let result = self.look_at(pos);
      match result {
        LookResult::Pellet(p) => score += p,
        LookResult::Floor => {}
        LookResult::Wall => {
          if i == 1 {
            return Some(result);
          } else {
            break;
          }
        }
        LookResult::MyPac(man, p) => {
          return Some(LookResult::MyPac(man, p + score))
        }
        LookResult::TheirPac(man, p) => {
          return Some(LookResult::TheirPac(man, p + score))
        }
      }
    }

    if score > 0 {
      Some(LookResult::Pellet(score))
    } else {
      let pos = man.look_right(1, self.width, self.height);
      Some(self.look_at(pos))
    }
  }

  fn look_behind(&self, id: u32) -> Option<LookResult> {
    let man =
      if let Some(man) = self.my_pacmen.get(&id) { man } else { return None };
    let mut score = 0;
    for i in 1..35 {
      let pos = man.look_behind(i, self.width, self.height);
      let result = self.look_at(pos);
      match result {
        LookResult::Pellet(p) => score += p,
        LookResult::Floor => {}
        LookResult::Wall => {
          if i == 1 {
            return Some(result);
          } else {
            break;
          }
        }
        LookResult::MyPac(man, p) => {
          return Some(LookResult::MyPac(man, p + score))
        }
        LookResult::TheirPac(man, p) => {
          return Some(LookResult::TheirPac(man, p + score))
        }
      }
    }

    if score > 0 {
      Some(LookResult::Pellet(score))
    } else {
      let pos = man.look_behind(1, self.width, self.height);
      Some(self.look_at(pos))
    }
  }

  fn get_commands(&mut self) -> String {
    let mut result = vec![];
    let keys = self.my_pacmen.keys().into_iter().cloned().collect::<Vec<_>>();
    for i in keys {
      // Single pac-man iteration
      let mut ahead_score = 0;
      let mut ahead_friendly = false;
      let mut right_score = 0;
      let mut right_friendly = false;
      let mut left_score = 0;
      let mut left_friendly = false;
      let mut behind_score = 0;
      let mut behind_friendly = false;
      if let Some(r) = self.look_ahead(i) {
        let mut man = self.my_pacmen.get_mut(&i).unwrap();
        match r {
          LookResult::Pellet(p) => {
            eprintln!("Pellet found for pac @ {}", man.pos);
            ahead_score = p;
          }
          LookResult::Floor => {
            eprintln!("Floor found for pac @ {}", man.pos);
          }
          LookResult::Wall => {
            eprintln!("Wall found for pac @ {}", man.pos);
          }
          LookResult::MyPac(other, p) => {
            eprintln!("Friendly found for pac @ {}", man.pos);
            ahead_score = p;
            if other.direction.reversed() == man.direction {
              ahead_friendly = true;
            }
          }
          LookResult::TheirPac(other, p) => {
            eprintln!("Enemy found for pac @ {}", man.pos);
            // Can I beat it?
            ahead_score = p;
            if man.type_id.beats(other.type_id) {
              if man.ability_cooldown == 0 {
                result.push(man.boost());
                continue;
              } else {
                result.push(man.forward());
                continue;
              }
            } else if man.ability_cooldown == 0 {
              result.push(man.switch(other.type_id.beat_it()));
              continue;
            }
          }
        }
      }

      if let Some(r) = self.look_right(i) {
        let mut man = self.my_pacmen.get_mut(&i).unwrap();
        match r {
          LookResult::Pellet(p) => {
            eprintln!("Pellet found for pac @ {}", man.pos);
            right_score = p;
          }
          LookResult::Floor => {
            eprintln!("Floor found for pac @ {}", man.pos);
          }
          LookResult::Wall => {
            eprintln!("Wall found for pac @ {}", man.pos);
          }
          LookResult::MyPac(other, p) => {
            eprintln!("Friendly found for pac @ {}", man.pos);
            right_score = p;
            if other.direction.reversed() == man.direction {
              right_friendly = true;
            }
          }
          LookResult::TheirPac(other, p) => {
            eprintln!("Enemy found for pac @ {}", man.pos);
            // Can I beat it?
            right_score = p;
            if man.type_id.beats(other.type_id) {
              if man.ability_cooldown == 0 {
                result.push(man.boost());
                continue;
              } else {
                result.push(man.forward());
                continue;
              }
            } else if man.ability_cooldown == 0 {
              result.push(man.switch(other.type_id.beat_it()));
              continue;
            }
          }
        }
      }

      if let Some(r) = self.look_left(i) {
        let mut man = self.my_pacmen.get_mut(&i).unwrap();
        match r {
          LookResult::Pellet(p) => {
            eprintln!("Pellet found for pac @ {}", man.pos);
            left_score = p;
          }
          LookResult::Floor => {
            eprintln!("Floor found for pac @ {}", man.pos);
          }
          LookResult::Wall => {
            eprintln!("Wall found for pac @ {}", man.pos);
          }
          LookResult::MyPac(other, p) => {
            eprintln!("Friendly found for pac @ {}", man.pos);
            left_score = p;
            if other.direction.reversed() == man.direction {
              left_friendly = true;
            }
          }
          LookResult::TheirPac(other, p) => {
            eprintln!("Enemy found for pac @ {}", man.pos);
            // Can I beat it?
            left_score = p;
            if man.type_id.beats(other.type_id) {
              if man.ability_cooldown == 0 {
                result.push(man.boost());
                continue;
              } else {
                result.push(man.forward());
                continue;
              }
            } else if man.ability_cooldown == 0 {
              result.push(man.switch(other.type_id.beat_it()));
              continue;
            }
          }
        }
      }

      if let Some(r) = self.look_behind(i) {
        let mut man = self.my_pacmen.get_mut(&i).unwrap();
        match r {
          LookResult::Pellet(p) => {
            eprintln!("Pellet found for pac @ {}", man.pos);
            behind_score = p;
          }
          LookResult::Floor => {
            eprintln!("Floor found for pac @ {}", man.pos);
          }
          LookResult::Wall => {
            eprintln!("Wall found for pac @ {}", man.pos);
          }
          LookResult::MyPac(other, p) => {
            eprintln!("Friendly found for pac @ {}", man.pos);
            behind_score = p;
            if other.direction.reversed() == man.direction {
              behind_friendly = true;
            }
          }
          LookResult::TheirPac(other, p) => {
            eprintln!("Enemy found for pac @ {}", man.pos);
            // Can I beat it?
            behind_score = p;
            if man.type_id.beats(other.type_id) {
              if man.ability_cooldown == 0 {
                result.push(man.boost());
              } else {
                result.push(man.forward());
              }
            } else if man.ability_cooldown == 0 {
              result.push(man.switch(other.type_id.beat_it()));
            }
          }
        }
      }

      // find maximum score
      let mut man = self.my_pacmen.get_mut(&i).unwrap();
      if ahead_score > right_score
        && ahead_score > left_score
        && ahead_score > behind_score
        && !ahead_friendly
      {
        result.push(man.forward());
      } else if right_score > ahead_score
        && right_score > left_score
        && right_score > behind_score
        && !right_friendly
      {
        man.turn_right();
        result.push(man.forward());
      } else if left_score > ahead_score
        && left_score > right_score
        && left_score > behind_score
        && !left_friendly
      {
        man.turn_left();
        result.push(man.forward());
      } else if behind_score > ahead_score
        && behind_score > right_score
        && behind_score > left_score
        && !behind_friendly
      {
        man.turn_around();
        result.push(man.forward());
      } else {
        // find the nearest big pellet and get that
        // get big pellets
        let mut big_pellets = self
          .pellets
          .iter()
          .filter_map(|(p, s)| if *s > 1 { Some(*p) } else { None })
          .collect::<Vec<_>>();
        // check for no pellets
        if big_pellets.is_empty() {
          continue;
        }
        // find the nearest
        let mut nearest = big_pellets.pop().unwrap();
        while !big_pellets.is_empty() {
          let next = big_pellets.pop().unwrap();
          if next.distance_to(man.pos) < nearest.distance_to(man.pos) {
            nearest = next;
          }
        }
        result.push(man.move_to(nearest));
      }
    }

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

impl Display for Direction {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Direction::Up => write!(f, "Up"),
      Direction::Down => write!(f, "Down"),
      Direction::Left => write!(f, "Left"),
      Direction::Right => write!(f, "Right"),
    }
  }
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

  fn switch(&self, type_id: PacType) -> Command {
    Command::Switch(*self, type_id)
  }

  fn boost(&self) -> Command {
    Command::Speed(*self)
  }

  fn turn_left(&mut self) {
    eprintln!("PacMan @ {} is turning", self.pos);
    eprintln!("was {}", self.direction);
    self.direction = match self.direction {
      Direction::Up => Direction::Left,
      Direction::Down => Direction::Right,
      Direction::Left => Direction::Down,
      Direction::Right => Direction::Up,
    };
    eprintln!("now {}", self.direction);
  }

  fn turn_right(&mut self) {
    eprintln!("PacMan @ {} is turning", self.pos);
    eprintln!("was {}", self.direction);
    self.direction = match self.direction {
      Direction::Up => Direction::Right,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
      Direction::Right => Direction::Down,
    };
    eprintln!("now {}", self.direction);
  }

  fn turn_around(&mut self) {
    eprintln!("PacMan @ {} is turning", self.pos);
    eprintln!("was {}", self.direction);
    self.direction = match self.direction {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    };
    eprintln!("now {}", self.direction);
  }

  fn update(
    &mut self,
    pos: Pos,
    pac_type: PacType,
    speed_turns_left: u32,
    cooldown: u32,
  ) {
    self.pos = pos;
    self.type_id = pac_type;
    self.speed_turns_left = speed_turns_left;
    self.ability_cooldown = cooldown;
  }

  fn look_forward(&self, dist: u32, width: u32, height: u32) -> Pos {
    let x = self.pos.x;
    let y = self.pos.y;
    match self.direction {
      Direction::Up => {
        Pos { x, y: if y >= dist { y - dist } else { height - (dist - y) } }
      }
      Direction::Down => Pos {
        x,
        y: if y + dist > height { y + dist - height } else { y + dist },
      },
      Direction::Left => {
        Pos { x: if x >= dist { x - dist } else { width - (dist - x) }, y }
      }
      Direction::Right => {
        Pos { x: if x + dist > width { x + dist - width } else { x + dist }, y }
      }
    }
  }

  fn look_right(&self, dist: u32, width: u32, height: u32) -> Pos {
    let x = self.pos.x;
    let y = self.pos.y;
    match self.direction {
      Direction::Left => {
        Pos { x, y: if y >= dist { y - dist } else { height - (dist - y) } }
      }
      Direction::Right => Pos {
        x,
        y: if y + dist > height { y + dist - height } else { y + dist },
      },
      Direction::Down => {
        Pos { x: if x >= dist { x - dist } else { width - (dist - x) }, y }
      }
      Direction::Up => {
        Pos { x: if x + dist > width { x + dist - width } else { x + dist }, y }
      }
    }
  }

  fn look_left(&self, dist: u32, width: u32, height: u32) -> Pos {
    let x = self.pos.x;
    let y = self.pos.y;
    match self.direction {
      Direction::Right => {
        Pos { x, y: if y >= dist { y - dist } else { height - (dist - y) } }
      }
      Direction::Left => Pos {
        x,
        y: if y + dist > height { y + dist - height } else { y + dist },
      },
      Direction::Up => {
        Pos { x: if x >= dist { x - dist } else { width - (dist - x) }, y }
      }
      Direction::Down => {
        Pos { x: if x + dist > width { x + dist - width } else { x + dist }, y }
      }
    }
  }

  fn look_behind(&self, dist: u32, width: u32, height: u32) -> Pos {
    let x = self.pos.x;
    let y = self.pos.y;
    match self.direction {
      Direction::Down => {
        Pos { x, y: if y >= dist { y - dist } else { height - (dist - y) } }
      }
      Direction::Up => Pos {
        x,
        y: if y + dist > height { y + dist - height } else { y + dist },
      },
      Direction::Right => {
        Pos { x: if x >= dist { x - dist } else { width - (dist - x) }, y }
      }
      Direction::Left => {
        Pos { x: if x + dist > width { x + dist - width } else { x + dist }, y }
      }
    }
  }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum LookResult {
  Pellet(u32),
  Floor,
  Wall,
  MyPac(PacMan, u32),
  TheirPac(PacMan, u32),
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

    println!("{}", game.get_commands()); // MOVE <pacId> <x> <y>
  }
}
