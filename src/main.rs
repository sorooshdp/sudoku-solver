use std::fmt;
use std::time::{Duration, Instant};

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

fn is_valid(board: &Board, x: usize, y: usize, guess: u8) -> bool {
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

fn find_free(board: &Board) -> Option<FreeCell> {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if board.cells[i][j].is_none() {
                return Some(FreeCell::new(j, i));
            }
        }
    }

    return None;
}

fn solve(board: &mut Board) -> bool {
    if let Some(free_cell) = find_free(&board) {
        for i in 1..=9 {
            if is_valid(&board, free_cell.x, free_cell.y, i) {
                board.cells[free_cell.y][free_cell.x] = Some(i);

                if solve(board) {
                    return true;
                }

                board.cells[free_cell.y][free_cell.x] = None; // Backtrack by resetting the cell
            }
        }
    } else {
        return true;
    }

    false
}
fn main() {
    let initial_board = Board::from(&[
        [2, 8, 0, 0, 0, 0, 7, 0, 0],
        [0, 9, 0, 0, 5, 8, 0, 6, 0],
        [0, 0, 0, 3, 0, 0, 0, 0, 0],
        [0, 5, 0, 0, 4, 9, 0, 8, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 8, 0, 0, 3, 0, 0, 6],
        [0, 3, 0, 6, 0, 0, 0, 0, 0],
        [9, 0, 0, 0, 3, 4, 0, 0, 2],
        [0, 0, 0, 1, 0, 0, 0, 4, 0],
    ]);

    // Solve and print the board.
    let mut board = initial_board.clone();
    if solve(&mut board) {
        println!("{}", board);
    } else {
        println!("No solution found.");
    }

    // Optionally, run benchmarks from main.
    measure_execution_time(&initial_board);
    run_benchmarks(&initial_board);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let board = Board::from(&[
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ]);

        assert!(is_valid(&board, 2, 0, 1)); // Valid placement
        assert!(!is_valid(&board, 2, 0, 5)); // Invalid (row conflict)
        assert!(!is_valid(&board, 2, 0, 9)); // Invalid (box conflict)
    }

    #[test]
    fn test_solve() {
        let mut board = Board::from(&[
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ]);

        let solved = solve(&mut board);
        assert!(solved);

        let expected_solution = Board::from(&[
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ]);

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                assert_eq!(board.cells[row][col], expected_solution.cells[row][col]);
            }
        }
    }

    #[test]
    fn benchmark_measure_execution_time() {
        // Define an initial board for benchmarking.
        let initial_board = Board::from(&[
            [2, 8, 0, 0, 0, 0, 7, 0, 0],
            [0, 9, 0, 0, 5, 8, 0, 6, 0],
            [0, 0, 0, 3, 0, 0, 0, 0, 0],
            [0, 5, 0, 0, 4, 9, 0, 8, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 8, 0, 0, 3, 0, 0, 6],
            [0, 3, 0, 6, 0, 0, 0, 0, 0],
            [9, 0, 0, 0, 3, 4, 0, 0, 2],
            [0, 0, 0, 1, 0, 0, 0, 4, 0],
        ]);

        // This will print the elapsed time for a single run.
        measure_execution_time(&initial_board);
    }

    #[test]
    fn benchmark_run_benchmarks() {
        // Define an initial board for benchmarking.
        let initial_board = Board::from(&[
            [2, 8, 0, 0, 0, 0, 7, 0, 0],
            [0, 9, 0, 0, 5, 8, 0, 6, 0],
            [0, 0, 0, 3, 0, 0, 0, 0, 0],
            [0, 5, 0, 0, 4, 9, 0, 8, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 8, 0, 0, 3, 0, 0, 6],
            [0, 3, 0, 6, 0, 0, 0, 0, 0],
            [9, 0, 0, 0, 3, 4, 0, 0, 2],
            [0, 0, 0, 1, 0, 0, 0, 4, 0],
        ]);

        // This will print the average elapsed time over 1000 runs.
        run_benchmarks(&initial_board);
    }
}

fn measure_execution_time(initial_board: &Board) {
    // Clone the board so the original remains unsolved.
    let mut board = initial_board.clone();
    let start_time = Instant::now();
    solve(&mut board);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time for single run: {:?}", elapsed_time);
}

fn run_benchmarks(initial_board: &Board) {
const NUM_RUNS: u32 = 100;
    let mut total_elapsed_time = Duration::new(0, 0);

    for _ in 0..NUM_RUNS {
        // Create a fresh board for each run.
        let mut board = initial_board.clone();
        let start_time = Instant::now();
        solve(&mut board);
        total_elapsed_time += start_time.elapsed();
    }

    let average_elapsed_time = total_elapsed_time / NUM_RUNS;
    println!("Average elapsed time over {} runs: {:?}", NUM_RUNS, average_elapsed_time);
}