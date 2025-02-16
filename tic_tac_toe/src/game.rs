use std::io;
use std::fmt;

// player symbols and board size
const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
const BOARD_SIZE: usize = 3;

// game board type
#[derive(Clone)]
struct Board {
    cells: [[char; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    // create new board
    fn new() -> Self {
        Self {
            cells: [[' '; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    // check if position is valid and empty
    fn is_valid_move(&self, row: usize, col: usize) -> bool {
        row < BOARD_SIZE && col < BOARD_SIZE && self.cells[row][col] == ' '
    }

    // make a move
    fn make_move(&mut self, row: usize, col: usize, player: char) {
        self.cells[row][col] = player;
    }

    // check if player has won
    fn is_winner(&self, player: char) -> bool {
        // check rows
        for row in 0..BOARD_SIZE {
            if self.cells[row].iter().all(|&cell| cell == player) {
                return true;
            }
        }

        // check columns
        for col in 0..BOARD_SIZE {
            if (0..BOARD_SIZE).all(|row| self.cells[row][col] == player) {
                return true;
            }
        }

        // check diagonals
        let diagonal1 = (0..BOARD_SIZE).all(|i| self.cells[i][i] == player);
        let diagonal2 = (0..BOARD_SIZE).all(|i| self.cells[i][BOARD_SIZE - 1 - i] == player);

        diagonal1 || diagonal2
    }

    // check for draw
    fn is_draw(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|&cell| cell != ' '))
    }
}

// display board nicely
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.cells {
            writeln!(f, "-------------")?;
            write!(f, "|")?;
            for &cell in row {
                write!(f, " {} |", cell)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "-------------")
    }
}

// game state
struct Game {
    board: Board,
    current_player: char,
}

impl Game {
    // create new game
    fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: PLAYER_X,
        }
    }

    // get player move
    fn get_player_move(&self) -> (usize, usize) {
        loop {
            println!("Player {} turn (enter row col):", self.current_player);
            let mut input = String::new();
            
            if let Err(e) = io::stdin().read_line(&mut input) {
                eprintln!("Failed to read input: {}", e);
                continue;
            }

            // parse coordinates
            if let Some((row, col)) = self.parse_move(&input) {
                if self.board.is_valid_move(row, col) {
                    return (row, col);
                }
            }
            println!("Invalid move! Please enter row and column (0-2)");
        }
    }

    // parse move input
    fn parse_move(&self, input: &str) -> Option<(usize, usize)> {
        let coords: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        match coords.as_slice() {
            [row, col] => Some((*row, *col)),
            _ => None,
        }
    }

    // switch current player
    fn switch_player(&mut self) {
        self.current_player = if self.current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        };
    }
}

// main game loop
pub fn play_game() {
    let mut game = Game::new();
    
    println!("Welcome to Tic Tac Toe!");
    println!("Enter moves as 'row col' (0-2)");

    loop {
        // display current state
        println!("\n{}", game.board);

        // get and make move
        let (row, col) = game.get_player_move();
        game.board.make_move(row, col, game.current_player);

        // check game end conditions
        if game.board.is_winner(game.current_player) {
            println!("\n{}", game.board);
            println!("Player {} wins!", game.current_player);
            break;
        }

        if game.board.is_draw() {
            println!("\n{}", game.board);
            println!("Game is a draw!");
            break;
        }

        game.switch_player();
    }
}