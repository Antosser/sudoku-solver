# Sudoku Solver

A Rust implementation of a backtracking algorithm to solve Sudoku puzzles.

## Features

- Reads Sudoku puzzles from standard input
- Implements a backtracking algorithm with random number ordering for each cell
- Visual progress bar showing solving progress
- Colorful terminal output of the Sudoku board
- Comprehensive test suite for validation logic

## Usage

Provide the Sudoku puzzle as a single line of 81 characters via standard input

- Use digits 1-9 for known cells
- Use any other character (e.g., '.', or space) for empty cells

Example:

```bash
echo "5.1..4..26.95.738...7...4....63.2.1.21......67..9..8243..6.12.896.7..1.5..52..67." | cargo run
```

## Implementation Details

- The solver uses a backtracking approach with randomized digit ordering for each empty cell
- Board validation checks rows, columns, and 3x3 subgrids
- Progress is displayed using `indicatif` progress bar
- Board is printed with colored borders for better visibility

## Dependencies

- `colored` - For colored terminal output
- `indicatif` - For progress bar display
- `rand` - For random number generation
- `tap` - For convenient collection manipulation

## Testing

Run tests with:

```bash
cargo test
```

## Performance

The solver uses an optimized backtracking approach:

- Processes empty cells in a specific order
- Maintains state for efficient backtracking
- Tracks maximum recursion depth reached

## Example Output

The program will display:

1. The initial board
2. A progress bar showing solving progress
3. The solved board (if found)
4. Statistics including iteration count

For unsolvable puzzles, it will report "No solution" with iteration statistics.
