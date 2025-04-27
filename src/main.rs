use colored::Colorize;
use indicatif::ProgressBar;
use rand::seq::SliceRandom;
use tap::Tap;

struct Board {
    cells: [[Option<u8>; 9]; 9],
}

impl Board {
    fn print(&self) {
        for y in 0..19 {
            for x in 0..37 {
                let is_col_div = x % 4 == 0;
                let is_main_col_div = x % 12 == 0;
                let is_row_div = y % 2 == 0;
                let is_main_row_div = y % 6 == 0;
                let is_cell_number = x >= 2 && y >= 1 && (x - 2) % 4 == 0 && (y - 1) % 2 == 0;

                if is_col_div && is_row_div {
                    if is_main_col_div || is_main_row_div {
                        print!("+")
                    } else {
                        print!("{}", "+".bright_black());
                    }
                } else if is_col_div {
                    if is_main_col_div {
                        print!("|")
                    } else {
                        print!("{}", "|".bright_black());
                    }
                } else if is_row_div {
                    if is_main_row_div {
                        print!("-")
                    } else {
                        print!("{}", "-".bright_black());
                    }
                } else if is_cell_number {
                    if let Some(digit) = self.cells[(x - 2) / 4][(y - 1) / 2] {
                        print!("{}", digit);
                    } else {
                        print!(" ");
                    }
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn new() -> Board {
        let mut buf = String::new();
        let mut cells = [[None; 9]; 9];

        std::io::stdin().read_line(&mut buf).unwrap();
        assert_eq!(buf.len(), 82);

        for (i, c) in buf.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                cells[i % 9][i / 9] = Some(digit as u8);
            }
        }

        Board { cells }
    }

    #[allow(dead_code)]
    fn empty() -> Board {
        Board {
            cells: [[None; 9]; 9],
        }
    }

    fn get_empty_cells(&self) -> Vec<(usize, usize)> {
        let mut empty_cells = Vec::new();

        for y in (0..9).step_by(3) {
            for x in (0..9).step_by(3) {
                for dy in 0..3 {
                    for dx in 0..3 {
                        let x = x + dx;
                        let y = y + dy;

                        if self.cells[x][y].is_none() {
                            empty_cells.push((x, y));
                        }
                    }
                }
            }
        }

        empty_cells
    }

    fn is_valid(&self) -> bool {
        // Check rows
        for y in 0..9 {
            let mut seen = [false; 9];
            for x in 0..9 {
                if let Some(digit) = self.cells[x][y] {
                    if seen[digit as usize - 1] {
                        return false;
                    }
                    seen[digit as usize - 1] = true;
                }
            }
        }

        // Check columns
        for x in 0..9 {
            let mut seen = [false; 9];
            for y in 0..9 {
                if let Some(digit) = self.cells[x][y] {
                    if seen[digit as usize - 1] {
                        return false;
                    }
                    seen[digit as usize - 1] = true;
                }
            }
        }

        // Check squares
        for y in (0..9).step_by(3) {
            for x in (0..9).step_by(3) {
                let mut seen = [false; 9];
                for dy in 0..3 {
                    for dx in 0..3 {
                        if let Some(digit) = self.cells[x + dx][y + dy] {
                            if seen[digit as usize - 1] {
                                return false;
                            }
                            seen[digit as usize - 1] = true;
                        }
                    }
                }
            }
        }

        true
    }
}

fn main() {
    let mut rng = rand::rng();

    let mut board = Board::new();

    board.print();

    let empty_cells = board.get_empty_cells();
    let cell_number_order = (0..empty_cells.len())
        .map(|_| [1, 2, 3, 4, 5, 6, 7, 8, 9].tap_mut(|v| v.shuffle(&mut rng)))
        .collect::<Vec<_>>();

    let pb = ProgressBar::new(empty_cells.len() as u64);

    let mut state: Vec<i32> = vec![-1];
    let mut max_depth = 0;

    for iteration in 0.. {
        let cell_index = state.len() - 1;
        let number_index = state.last().unwrap();

        if *number_index < 8 {
            state[cell_index] += 1;
            let actual_number = cell_number_order[cell_index][state[cell_index] as usize];
            board.cells[empty_cells[cell_index].0][empty_cells[cell_index].1] = Some(actual_number);

            if board.is_valid() {
                if state.len() == empty_cells.len() {
                    pb.set_position(empty_cells.len() as u64);
                    pb.finish();
                    board.print();
                    println!("{iteration} iterations");
                    return;
                } else {
                    if state.len() > max_depth {
                        max_depth = state.len();
                        pb.set_position(state.len() as u64);
                    }
                    state.push(-1);
                }
            }
        } else {
            board.cells[empty_cells[cell_index].0][empty_cells[cell_index].1] = None;

            if state.len() > 1 {
                state.pop();
            } else {
                println!("No solution");
                println!("{iteration} iterations");
                println!("Max depth: {max_depth}");
                return;
            }
        }

        // board.print();
        // std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_empty_board() {
        let board = Board::empty(); // All cells are None
        assert!(board.is_valid());
    }

    #[test]
    fn test_valid_row_but_invalid_column() {
        let mut board = Board::empty();
        // Fill first row with 1-9 (valid)
        for y in 0..9 {
            board.cells[0][y] = Some(y as u8 + 1);
        }
        // Add a duplicate '1' in the first column (invalid)
        board.cells[1][0] = Some(1);
        assert!(!board.is_valid()); // Should fail column check
    }

    #[test]
    fn test_valid_row_and_column_but_invalid_subgrid() {
        let mut board = Board::empty();
        // Fill top-left subgrid with 1-9 (valid if spread out)
        for x in 0..3 {
            for y in 0..3 {
                board.cells[x][y] = Some((x * 3 + y + 1) as u8);
            }
        }
        // Add a duplicate '1' in the same subgrid (invalid)
        board.cells[0][1] = Some(1);
        assert!(!board.is_valid()); // Should fail subgrid check
    }

    #[test]
    fn test_partially_filled_valid_board() {
        let mut board = Board::empty();
        // Fill diagonal with unique numbers (valid)
        for i in 0..9 {
            board.cells[i][i] = Some(i as u8 + 1);
        }
        assert!(board.is_valid());
    }

    #[test]
    fn test_duplicate_in_row() {
        let mut board = Board::empty();
        board.cells[0][0] = Some(1);
        board.cells[0][1] = Some(1); // Duplicate in row 0
        assert!(!board.is_valid());
    }

    #[test]
    fn test_duplicate_in_column() {
        let mut board = Board::empty();
        board.cells[0][0] = Some(1);
        board.cells[1][0] = Some(1); // Duplicate in column 0
        assert!(!board.is_valid());
    }

    #[test]
    fn test_duplicate_in_subgrid() {
        let mut board = Board::empty();
        // Both in top-left subgrid
        board.cells[0][0] = Some(1);
        board.cells[1][1] = Some(1); // Duplicate in subgrid (0,0)-(2,2)
        assert!(!board.is_valid());
    }
}
