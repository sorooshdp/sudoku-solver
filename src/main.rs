use std::fmt;
const BOARD_SIZE: usize = 9;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Board {
    cells: [[Option<u8>; BOARD_SIZE]; BOARD_SIZE],
}

#[derive(PartialEq)]
struct FreeCell {
    x: usize,
    y: usize,
}

impl FreeCell {
    fn new(x: usize, y: usize) -> FreeCell {
        return FreeCell { x, y };
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            cells: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

/// Construct a `Board` from a 2D array.
impl From<&[[u8; BOARD_SIZE]; BOARD_SIZE]> for Board {
    fn from(array_2d: &[[u8; BOARD_SIZE]; BOARD_SIZE]) -> Self {
        let mut board = Board::default();
        for (y, row) in array_2d.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                board.cells[y][x] = if *item == 0 { None } else { Some(*item) };
            }
        }
        board
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        // Top border
        output.push_str("┏━━━━━━━━━━━━━━━━━━━━━━━┓\n");

        for row in 0..BOARD_SIZE {
            if row % 3 == 0 {
                // Row separator
                output.push_str("┣━━━━━━━━━━━━━━━━━━━━━━━┫\n");
            }

            for col in 0..BOARD_SIZE {
                if col % 3 == 0 {
                    output.push_str("┃ ");
                }

                match self.cells[row][col] {
                    Some(value) => output.push_str(&format!("{} ", value)),
                    None => output.push_str(". "),
                }
            }

            output.push_str("┃\n");
        }

        // Bottom border
        output.push_str("┗━━━━━━━━━━━━━━━━━━━━━━━┛\n");

        write!(f, "{}", output)
    }
}

fn is_valid(board: Board, x: usize, y: usize, guess: u8) -> bool {
    for i in 0..BOARD_SIZE {
        if board.cells[y][i] == Some(guess) || board.cells[i][x] == Some(guess) {
            return false;
        }
    }

    let box_x = x / 3 * 3;
    let box_y = y / 3 * 3;

    for i in box_y..box_y + 3 {
        for j in box_x..box_x + 3 {
            if board.cells[i][j] == Some(guess) {
                return false;
            }
        }
    }

    true
}

fn find_free(board: Board) -> FreeCell {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if board.cells[i][j].is_none() {
                return FreeCell::new(j, i);
            }
        }
    }

    return FreeCell::new(10, 10);
}

fn solve(board: &mut Board) -> bool {
    let free_cell = find_free(*board);

    if free_cell.x == 10 {
        return true;
    }

    for i in 1..=9 {
        if is_valid(*board, free_cell.x, free_cell.y, i.try_into().unwrap()) {
            board.cells[free_cell.y][free_cell.x] = Some(i);

            if solve(board) {
                return true;
            }

            board.cells[free_cell.y][free_cell.x] = None; // Backtrack by resetting the cell
        }
    }

    false
}
fn main() {
    let mut board = Board::from(&[
        [0, 2, 0, 0, 0, 0, 0, 0, 0], // row 1
        [0, 0, 0, 6, 0, 0, 0, 0, 3], // row 2
        [0, 7, 4, 0, 8, 0, 0, 0, 0], // row 3
        [0, 0, 0, 0, 0, 3, 0, 0, 2], // row 4
        [0, 8, 0, 0, 4, 0, 0, 1, 0], // row 5
        [6, 0, 0, 5, 0, 0, 0, 0, 0], // row 6
        [0, 0, 0, 0, 1, 0, 7, 8, 0], // row 7
        [5, 0, 0, 0, 0, 9, 0, 0, 0], // row 8
        [0, 0, 0, 0, 0, 0, 0, 4, 0], // row 9
    ]);

    solve(&mut board);

    println!("{}", board);
}
