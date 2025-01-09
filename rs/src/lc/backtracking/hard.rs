#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn solve_sudoku_37(board: &mut Vec<Vec<char>>) {
        fn is_valid(
            row: usize,
            col: usize,
            val: char,
            row_mask: &[i16; 9],
            col_mask: &[i16; 9],
            board: &Vec<Vec<char>>,
        ) -> bool {
            if row_mask[row] & (1 << (val as u8 - b'0')) != 0
                || col_mask[col] & (1 << (val as u8 - b'0')) != 0
            {
                return false;
            }

            let min_row = row / 3 * 3;
            let min_col = col / 3 * 3;

            for i in min_row..min_row + 3 {
                for j in min_col..min_col + 3 {
                    if board[i][j] == val {
                        return false;
                    }
                }
            }

            return true;
        }

        fn solve(
            mut row: usize,
            mut col: usize,
            row_mask: &mut [i16; 9],
            col_mask: &mut [i16; 9],
            board: &mut Vec<Vec<char>>,
        ) -> bool {
            if col == 9 {
                col = 0;
                row += 1;
            }

            if row == 9 {
                return true;
            }

            if board[row][col].is_ascii_digit() {
                return solve(row, col + 1, row_mask, col_mask, board);
            }

            for val in 1..=9 {
                let candidate = (val + b'0') as char;

                if is_valid(row, col, candidate, row_mask, col_mask, board) {
                    board[row][col] = candidate;
                    row_mask[row] |= 1 << val;
                    col_mask[col] |= 1 << val;

                    if solve(row, col + 1, row_mask, col_mask, board) {
                        return true;
                    }

                    row_mask[row] &= !(1 << val);
                    col_mask[col] &= !(1 << val);
                    board[row][col] = '.';
                }
            }

            return false;
        }

        let mut row_mask = [0i16; 9];
        let mut col_mask = [0i16; 9];
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j].is_ascii_digit() {
                    row_mask[i] |= 1 << (board[i][j] as u8 - b'0');
                }

                if board[j][i].is_ascii_digit() {
                    col_mask[i] |= 1 << (board[j][i] as u8 - b'0');
                }
            }
        }

        solve(0, 0, &mut row_mask, &mut col_mask, board);
    }
}
