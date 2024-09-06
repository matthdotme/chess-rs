#[cfg(test)]
mod tests;

use colored::Colorize;
use std::{io,fmt};
use std::collections::HashMap;

static BOARDSIZE: usize = 8;
static BAR: &str        = "    ----------------------------------------";
static EDGELABELS: &str = "      a    b    c    d    e    f    g    h  ";

#[derive(Debug, Clone, Copy, PartialEq)]
enum Team {
    White,
    Black,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Team::White => { write!(f, "{}", "White".bright_white()) },
            Team::Black => { write!(f, "{}", "Black".on_white().black()) },
        }
    }
}

// impl Team {
//     fn test(&self) -> fmt::Result {
//         if self == &Team::White {
//             write!(f, "White")
//         } else {
//             write!(f, "Black")
//         }
//      }
//  }

#[derive(Debug, Clone, Copy, PartialEq)]
enum Role {
    Pawn,
    Castle,
    Knight,
    Bishop,
    Queen,
    King,
}

// impl fmt::Display for Role {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match &self {
//             Role::Pawn => { write!(f, "{}", "W") }, Team::Black => { write!{f, "{}", "B") },
//         }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq)]
struct Piece {
    role: Role,
    team: Team,
    has_moved: bool,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // if &self.team == Team::White {
        //     let
        match &self.team {
            Team::White => {
                match &self.role {
                    Role::Pawn   => {write!(f, "{}", "wi".bright_white() ) },
                    Role::Castle => {write!(f, "{}", "wC".bright_white() ) },
                    Role::Knight => {write!(f, "{}", "wH".bright_white() ) },
                    Role::Bishop => {write!(f, "{}", "wB".bright_white() ) },
                    Role::Queen  => {write!(f, "{}", "wQ".bright_white() ) },
                    Role::King   => {write!(f, "{}", "wK".bright_white() ) },
                }
            },
            Team::Black => {
                match &self.role {
                    Role::Pawn   => {write!(f, "{}", "bi".on_white().black() ) },
                    Role::Castle => {write!(f, "{}", "bC".on_white().black() ) },
                    Role::Knight => {write!(f, "{}", "bH".on_white().black() ) },
                    Role::Bishop => {write!(f, "{}", "bB".on_white().black() ) },
                    Role::Queen  => {write!(f, "{}", "bQ".on_white().black() ) },
                    Role::King   => {write!(f, "{}", "bK".on_white().black() ) },
                }
            },
        }
    }
}

fn generate_valid_positions_hashmap() -> HashMap<String, (usize, usize)> {
    // TODO refactor to use flat_map to map functional approach instead of hard coding
    // 'a'..'h'.flat_map(|rank| => 0..=7.map(|file| => write!("{}{}", rank, file)))
    HashMap::from([
        ("a1".to_string(), (7 as usize, 0 as usize)), ("a2".to_string(), (6 as usize, 0 as usize)), ("a3".to_string(), (5 as usize, 0 as usize)), ("a4".to_string(), (4 as usize, 0 as usize)), ("a5".to_string(), (3 as usize, 0 as usize)), ("a6".to_string(), (2 as usize, 0 as usize)), ("a7".to_string(), (1 as usize, 0 as usize)), ("a8".to_string(), (0 as usize, 0 as usize)),
        ("b1".to_string(), (7 as usize, 1 as usize)), ("b2".to_string(), (6 as usize, 1 as usize)), ("b3".to_string(), (5 as usize, 1 as usize)), ("b4".to_string(), (4 as usize, 1 as usize)), ("b5".to_string(), (3 as usize, 1 as usize)), ("b6".to_string(), (2 as usize, 1 as usize)), ("b7".to_string(), (1 as usize, 1 as usize)), ("b8".to_string(), (0 as usize, 1 as usize)),
        ("c1".to_string(), (7 as usize, 1 as usize)), ("c2".to_string(), (6 as usize, 1 as usize)), ("c3".to_string(), (5 as usize, 1 as usize)), ("c4".to_string(), (4 as usize, 1 as usize)), ("c5".to_string(), (3 as usize, 1 as usize)), ("c6".to_string(), (2 as usize, 1 as usize)), ("c7".to_string(), (1 as usize, 1 as usize)), ("c8".to_string(), (0 as usize, 1 as usize)),
        ("d1".to_string(), (7 as usize, 1 as usize)), ("d2".to_string(), (6 as usize, 1 as usize)), ("d3".to_string(), (5 as usize, 1 as usize)), ("d4".to_string(), (4 as usize, 1 as usize)), ("d5".to_string(), (3 as usize, 1 as usize)), ("d6".to_string(), (2 as usize, 1 as usize)), ("d7".to_string(), (1 as usize, 1 as usize)), ("d8".to_string(), (0 as usize, 1 as usize)),
        ("e1".to_string(), (7 as usize, 1 as usize)), ("e2".to_string(), (6 as usize, 1 as usize)), ("e3".to_string(), (5 as usize, 1 as usize)), ("e4".to_string(), (4 as usize, 1 as usize)), ("e5".to_string(), (3 as usize, 1 as usize)), ("e6".to_string(), (2 as usize, 1 as usize)), ("e7".to_string(), (1 as usize, 1 as usize)), ("e8".to_string(), (0 as usize, 1 as usize)),
        ("f1".to_string(), (7 as usize, 1 as usize)), ("f2".to_string(), (6 as usize, 1 as usize)), ("f3".to_string(), (5 as usize, 1 as usize)), ("f4".to_string(), (4 as usize, 1 as usize)), ("f5".to_string(), (3 as usize, 1 as usize)), ("f6".to_string(), (2 as usize, 1 as usize)), ("f7".to_string(), (1 as usize, 1 as usize)), ("f8".to_string(), (0 as usize, 1 as usize)),
        ("g1".to_string(), (7 as usize, 1 as usize)), ("g2".to_string(), (6 as usize, 1 as usize)), ("g3".to_string(), (5 as usize, 1 as usize)), ("g4".to_string(), (4 as usize, 1 as usize)), ("g5".to_string(), (3 as usize, 1 as usize)), ("g6".to_string(), (2 as usize, 1 as usize)), ("g7".to_string(), (1 as usize, 1 as usize)), ("g8".to_string(), (0 as usize, 1 as usize)),
        ("h1".to_string(), (7 as usize, 1 as usize)), ("h2".to_string(), (6 as usize, 1 as usize)), ("h3".to_string(), (5 as usize, 1 as usize)), ("h4".to_string(), (4 as usize, 1 as usize)), ("h5".to_string(), (3 as usize, 1 as usize)), ("h6".to_string(), (2 as usize, 1 as usize)), ("h7".to_string(), (1 as usize, 1 as usize)), ("h8".to_string(), (0 as usize, 1 as usize)),
    ])
}

struct Game {
    board: Board,
    current_team: Team,
    turn_count: u8,
    // valid_positions: HashSet<&str>,
    valid_positions: HashMap<String, (usize, usize)>,
}

impl Game {
    fn new() -> Self {
        Game {
            board: Board::new(),
            current_team: Team::White,
            turn_count: 0 as u8,
            // valid_positions: HashSet::with_capacity(64);
            valid_positions: generate_valid_positions_hashmap(),
            // 0.0 = a8, 7,7 = h8 // what did I mean by this??
        }
    }

    fn start(&mut self) {
        let mut exit_status: bool = false;
        'gameloop: loop {
            if exit_status {
                println!("{}", "You typed 'exit', now exiting the game.".red());
                break 'gameloop;
            }
            self.display_grid();
            exit_status = self.take_turn();
        }
    }

    // fn next_move(&mut self) {
    //     println!("{ moves, so it is {:?} team's turn.", self.turn_count, self.turn);
    //     println!("{:?} made a move.", self.turn);
    //     self.turn_count = self.turn_count + 1;
    //     if self.turn == Team::White {
    //         self.turn == Team::Black;
    //     } else {
    //         self.turn = Team::White;
    //     }
    // }

    fn take_turn(&mut self) -> bool {
        let mut from_position: String = String::new();
        let mut to_position: String = String::new();

        println!("{} moves. It is {}'s turn.", self.turn_count, self.current_team);

        // get which piece to move from user
        let mut row_from: usize;
        let mut col_from: usize;

        'from: loop {
            io::stdin().read_line(&mut from_position).unwrap();
            from_position.pop();
            if from_position == "exit" {
                return true;
            }
            if !self.valid_positions.contains_key(from_position.as_str()) {
                println!("Not a valid input, please try again.");
                continue;
            }
            (row_from, col_from) = self.valid_positions.get(&from_position.to_string()).unwrap().clone();

            match self.board.grid[row_from][col_from] {
                None => {
                    println!("No piece exists at {}, please try again.", from_position);
                    from_position.clear();
                    continue;
                },
                Some(piece) => {
                    // match piece.team {
                    //     self.current_team {
                    //     break 'from;
                    //     },
                    //     _ => {
                    //         continue;
                    //     },
                    // }

                    if piece.team == self.current_team {
                        break 'from;
                    } else {
                        println!("You don't own that piece, please try again.");
                        from_position.clear();
                        continue;
                    }

                    // match piece.team {
                    //     self.current_team {
                    //         break 'from;
                    //     },
                    //     _ => {},
                    // }
                },
            }
            // (row_from, col_from) = match.valid_positions.get(&from_positions.to_string()) {
            //     Some((row_from, col_from)) => row_from.clone(), col_from.clone()),
            //     _ => {
            //         println!("No piece exists at {}, please try again.", from_position);
            //         from_position.clear();
            //         continue;
            //     },
            // };
        }

        // validate that piece is correct team
        // println!("{}", self.valid_positions.

        // get destination piece from user
        let mut row_to: usize;
        let mut col_to: usize;

        'to: loop {
            println!("Where do you want to move {}?", from_position); // TODO name the piece at this position
            io::stdin().read_line(&mut to_position).unwrap();
            to_position.pop();
            if to_position == "exit" {
                return true;
            }
            if !self.valid_positions.contains_key(to_position.as_str()) {
                println!("Not a valid input, please try again.");
                to_position.clear();
                continue;
            }
            (row_to, col_to) = self.valid_positions.get(&to_position.to_string()).unwrap().clone();

            if self.board.grid[row_to][col_to] != None { // TODO join these two lines of logic into
                                                         // short circuit condition? eg if not None
                                                         // and team == current_team
                if self.board.grid[row_to][col_to].unwrap().team == self.current_team {
                    println!(
                        "Your {} already occupies this space, make a different move.",
                        self.board.grid[row_to][col_to].unwrap()
                    );
                    to_position.clear();
                    continue;
                }
            }
            break 'to;
        }

        // let row_from: usize;
        // let col_from: usize;
        // let row_to: usize;
        // let col_to: usize;

        // TODO for readability, adapt logic to use "horizontal_delta" and "vertical_delta" instead
        // of recomputing the x, y distances
        match self.board.grid[row_from][col_from].unwrap().role {
            Role::Knight => {
                if ((row_to as i8 - row_from as i8).abs() == 2 && (col_to as i8 - col_from as i8).abs() == 1)
                    || ((row_to as i8 - row_from as i8).abs() == 1 && (col_to as i8 - col_from as i8).abs() == 2) {
                    match self.board.grid[row_to][col_to] {
                        None => {
                            self.move_piece(row_from, col_from, row_to, col_to);
                        },
                        Some(piece) => {
                            if piece.team != self.board.grid[row_from][col_from].unwrap().team {
                                self.move_piece(row_from, col_from, row_to, col_to);
                            }
                        },
                    }
                } else {
                    println!("Not a valid move for {:?}", Role::Knight);
                }
                //
                //
            },
            Role::Pawn => {
                //
                println!("Your valid moves are:\n{:?}", self.generate_valid_pawn_moves(row_from, col_from));
                if col_from == col_to {
                    if !self.board.grid[row_from][col_from].unwrap().has_moved {
                        if row_from - row_to > 0 && row_from - row_to <= 2 {
                            self.move_piece(row_from, col_from, row_to, col_to);
                        } else {
                            println!("Invalid move 1: first move can only move forward, and only by 1 or 2 spaces.");
                        }
                    } else if row_from - row_to == 1 {
                        self.move_piece(row_from, col_from, row_to, col_to);
                    } else {
                        println!("Invalid move 2: must move forward.");
                    }
                } else {
                    println!("Invalid move 3: cannot move sideways (or diagonal unless En Passant or attacking).");
                }
            },
            role => { println!("Movement not enabled for {:?} yet.", role); },
        }

        match self.current_team {
            Team::White => {
                // self.current_team = Team::Black; // TODO change this back once turns are squeaky
                // clean and buttery smooth
                self.current_team = Team::White;
            },
            Team::Black => {
                self.current_team = Team::White;

            },
        }
        self.turn_count += 1;

        false
    }

    fn move_piece(&mut self, row_from: usize, col_from: usize, row_to: usize, col_to: usize) {
        //
        // why did I .take() the first one but not the 2nd?
        let source: Option<Piece> = self.board.grid[row_from][col_from].take();
        let destination: Option<Piece> = self.board.grid[row_to][col_to];

        match (source, destination) {
            (Some(from), Some(to)) => {
                println!("{:?} {:?} takes {:?} {:?}!", from.team, from.role, to.team, to.role);
            },
            (Some(from), None) => {
                println!("Moving {:?} {:?} to {:?} {:?}.", from.team, from.role, row_to, col_to);
            },
            _ => {
                println!("Somehow attempting invalid move: {:?},{:?} -> {:?},{:?}", row_from, col_from, row_to, col_to);
            },
        }
        self.board.grid[row_to][col_to] = source;
        self.board.grid[row_to][col_to].unwrap().has_moved = true;
    }

    fn display_grid(&self) {
        println!("\n{}", EDGELABELS);
        println!("{}", BAR);
        let mut row_label: u8 = 8;
        for row in &self.board.grid {
            print!(" {} |", row_label);
            for position in row {
                match position {
                    None => {
                        print!("    ");
                    },
                    Some(piece) => {
                        print!(" {} ", piece);
                    },
                }
                print!("|");
            }
            println!(" {}", row_label);
            row_label -= 1;
            println!("{}", BAR);
        }
        println!("{}\n", EDGELABELS);
    }

    //

    fn generate_valid_pawn_moves(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        // validate (row, col) is a pawn
        match self.board.grid[row][col] {
            None => {
                println!("error ~364");
                vec![]
            },
            Some(piece) => {
                match piece.role {
                    Role::Pawn => {
                        let mut valid_pawn_destinations: Vec<(usize, usize)> = vec![];
                        if self.board.grid[row - 1][col] != None {
                            valid_pawn_destinations.push((row + 1, col));
                            if self.board.grid[row - 2][col] != None && !piece.has_moved {
                                valid_pawn_destinations.push((row + 2, col));
                            }
                            // TODO shouldn't it be col > 1?
                            if col > 0 && col <= 8
                                && self.board.grid[row - 1][col - 1] != None
                                && self.board.grid[row - 1][col - 1].unwrap().team != piece.team {
                                // check left attack move
                                valid_pawn_destinations.push((row - 1, col - 1));
                            }
                            if col <= 7
                                && self.board.grid[row - 1][col + 1] != None
                                && self.board.grid[row - 1][col + 1].unwrap().team != piece.team {
                                // check right attack move
                                valid_pawn_destinations.push((row - 1, col + 1));
                            }
                        }
                        valid_pawn_destinations
                    }
                    _ => {
                        println!("error ~392");
                        vec![]
                    },
                }
            },
        }
    }
}

#[derive(Debug)]
struct Board {
    // grid: [[Square; BOARDSIZE]; BOARDSIZE],
    grid: Vec<Vec<Option<Piece>>>,
}

impl Board {
    fn new() -> Self {
        let mut board = Board {
            grid: vec![vec![None; BOARDSIZE]; BOARDSIZE],
        };

        for i in 0..BOARDSIZE {
            board.grid[1][i] = Some(Piece { role: Role::Pawn, team: Team::Black, has_moved: false });
            board.grid[6][i] = Some(Piece { role: Role::Pawn, team: Team::White, has_moved: false });
        }

        board.grid[0][0] = Some(Piece { role: Role::Castle, team: Team::Black, has_moved: false });
        board.grid[0][7] = Some(Piece { role: Role::Castle, team: Team::Black, has_moved: false });
        board.grid[7][0] = Some(Piece { role: Role::Castle, team: Team::White, has_moved: false });
        board.grid[7][7] = Some(Piece { role: Role::Castle, team: Team::White, has_moved: false });

        board.grid[0][1] = Some(Piece { role: Role::Knight, team: Team::Black, has_moved: false });
        board.grid[0][6] = Some(Piece { role: Role::Knight, team: Team::Black, has_moved: false });
        board.grid[7][1] = Some(Piece { role: Role::Knight, team: Team::White, has_moved: false });
        board.grid[7][6] = Some(Piece { role: Role::Knight, team: Team::White, has_moved: false });

        board.grid[0][2] = Some(Piece { role: Role::Bishop, team: Team::Black, has_moved: false });
        board.grid[0][5] = Some(Piece { role: Role::Bishop, team: Team::Black, has_moved: false });
        board.grid[7][2] = Some(Piece { role: Role::Bishop, team: Team::White, has_moved: false });
        board.grid[7][5] = Some(Piece { role: Role::Bishop, team: Team::White, has_moved: false });

        board.grid[0][3] = Some(Piece { role: Role::Queen,  team: Team::Black, has_moved: false });
        board.grid[0][4] = Some(Piece { role: Role::King,   team: Team::Black, has_moved: false });

        board.grid[7][3] = Some(Piece { role: Role::Queen,  team: Team::White, has_moved: false });
        board.grid[7][4] = Some(Piece { role: Role::King,   team: Team::White, has_moved: false });
        board
    }
}

// impl fmt::Display for Board { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     for v in &self.grid {
//         return write!(f, "{:f}\n", v);
//      }
//      Ok(())
// }

fn main() {
    let mut game = Game::new();
    game.start();
}
