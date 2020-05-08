use std::collections::HashSet;
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

struct GameBoard {
  width: u32,
  height: u32,
  walkable: HashSet<Pos>,
  my_pacmen: Vec<PacMan>,
  their_pacmen: Vec<PacMan>,
  pellets: Vec<Pellet>,
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
      pellets: vec![],
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
          type_id,
          speed_turns_left,
          ability_cooldown,
        });
      } else {
        self.their_pacmen.push(PacMan {
          id,
          pos,
          type_id,
          speed_turns_left,
          ability_cooldown,
        });
      }
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let visible_pellet_count = parse_input!(input_line, u32); // all pellets in sight
    self.pellets.clear();
    for i in 0..visible_pellet_count as usize {
      let mut input_line = String::new();
      io::stdin().read_line(&mut input_line).unwrap();
      let inputs = input_line.split(" ").collect::<Vec<_>>();
      let x = parse_input!(inputs[0], u32);
      let y = parse_input!(inputs[1], u32);
      let value = parse_input!(inputs[2], u32); // amount of points this pellet is worth
      self.pellets.push(Pellet { pos: Pos { x, y }, value })
    }
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
  type_id: String,
  speed_turns_left: u32,
  ability_cooldown: u32,
}

impl Hash for PacMan {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
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

    println!("MOVE 0 15 10"); // MOVE <pacId> <x> <y>
  }
}
