#![allow(dead_code)]

use std::{collections::BTreeMap, fmt::Display};

#[derive(PartialEq)]
enum Color {
    Black,
    White,
}

enum Cell {
    Empty,
    Stone(Color),
}
struct Board {
    size: u8,
    board: BTreeMap<Point, Cell>,
}

impl Board {
    fn new(size: u8) -> Board {
        let valid_size = match size {
            3 | 9 | 19 => size,
            _ => panic!("'{}' is an invalid board size", size),
        };

        let mut board = BTreeMap::new();
        for i in 0..size {
            for j in 0..size {
                board.insert(Point(i, j), Cell::Empty);
            }
        }
        Board {
            size: valid_size,
            board: board,
        }
    }
    fn change_cell(&mut self, point: &Point, cell: Cell) {
        let board = &mut self.board;
        if let Some(c) = board.get_mut(point) {
            match c {
                Cell::Empty => {
                    *c = cell;
                }
                Cell::Stone(_) => panic!("There's already a stone here"),
            }
        } else {
            panic!("You cannot play here");
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header: String = (0..self.size)
            .map(|i| i)
            .map(|i| format!("|{:02}|\t", i))
            .collect();
        write!(f, "|##|\t{}", header)?;
        for i in 0..self.size {
            writeln!(f, "")?;
            write!(f, "|{:02}|\t", i)?;
            for j in 0..self.size {
                match self.board.get(&Point(i, j)).unwrap() {
                    Cell::Empty => write!(f, "|{} | \t", "*")?,
                    Cell::Stone(color) if Color::Black.eq(color) => write!(f, "|{} | \t", "B")?,
                    Cell::Stone(color) if Color::White.eq(color) => write!(f, "|{} |\t", "W")?,
                    _ => panic!("not expected"),
                }
            }
        }
        write!(f, "")
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq)]
struct Point(u8, u8);

struct Player {
    name: &'static str,
    color: Color,
}
pub struct Game {
    player_1: Player,
    player_2: Player,
    current_player: Color,
    board: Board,
}

impl Game {
    pub fn new(player_1: &'static str, player_2: &'static str, size: u8) -> Game {
        let player_1 = Player {
            name: player_1,
            color: Color::Black,
        };
        let player_2 = Player {
            name: player_2,
            color: Color::White,
        };
        Game {
            player_1,
            player_2,
            board: Board::new(size),
            current_player: Color::Black,
        }
    }
    pub fn get_player_1(&self) -> &str {
        self.player_1.name
    }
    pub fn get_player_2(&self) -> &str {
        self.player_2.name
    }
    fn play(&mut self, point: &Point) -> () {
        if Color::Black == self.current_player {
            self.board.change_cell(point, Cell::Stone(Color::Black));
            self.current_player = Color::White;
        } else {
            self.board.change_cell(point, Cell::Stone(Color::White));
            self.current_player = Color::Black;
        }
    }
}
#[cfg(test)]
mod test {
    use crate::board::goban::Point;

    use super::Game;
    #[test]
    fn create_game() {
        let mut g: Game = Game::new("nordine", "mohamed", 19);
        assert_eq!("nordine", g.get_player_1());
        assert_eq!("mohamed", g.get_player_2());
        g.play(&Point(4, 4));
        g.play(&Point(6, 5));
        g.play(&Point(6, 4));
        println!("{}", g.board);
    }
    #[test]
    #[should_panic(expected = "'11' is an invalid board size")]
    fn invalid_board_size() {
        Game::new("nordine", "mohamed", 11);
    }
}
