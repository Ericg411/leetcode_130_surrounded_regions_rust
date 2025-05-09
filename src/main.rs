pub struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() {
            return;
        }

        let rows = board.len();
        let cols = board[0].len();

        fn dfs(board: &mut Vec<Vec<char>>, r: isize, c: isize, rows: usize, cols: usize) {
            if r < 0 || r >= rows as isize || c < 0 || c >= cols as isize || board[r as usize][c as usize] != 'O' {
                return;
            }

            board[r as usize][c as usize] = 'T';

            let directions = [(1,0),(-1,0),(0,1),(0,-1)];
            for (dr, dc) in directions.iter() {
                dfs(board, r + dr, c + dc, rows, cols);
            }
        }

        for i in 0..rows {
            if board[i][0] == 'O' {
                dfs(board, i as isize, 0, rows, cols);
            }
            if board[i][cols - 1] == 'O' {
                dfs(board, i as isize, (cols - 1) as isize, rows, cols);
            }
        }
        for j in 0..cols {
            if board[0][j] == 'O' {
                dfs(board, 0, j as isize, rows, cols);
            }
            if board[rows - 1][j] == 'O' {
                dfs(board, (rows - 1) as isize, j as isize, rows, cols);
            }
        }

        for r in 0..rows {
            for c in 0..cols {
                if board[r][c] == 'O' {
                    board[r][c] = 'X';
                } else if board[r][c] == 'T' {
                    board[r][c] = 'O';
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solve() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X']
        ];
        let expected = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X']
        ];

        Solution::solve(&mut board);
        assert_eq!(board, expected);
    }

    #[test]
    fn test_empty_board() {
        let mut board: Vec<Vec<char>> = vec![];
        let expected: Vec<Vec<char>> = vec![];

        Solution::solve(&mut board);
        assert_eq!(board, expected);
    }

    #[test]
    fn test_all_x_board() {
        let mut board = vec![
            vec!['X', 'X'],
            vec!['X', 'X']
        ];
        let expected = board.clone();

        Solution::solve(&mut board);
        assert_eq!(board, expected);
    }

    #[test]
    fn test_no_surrounded() {
        let mut board = vec![
            vec!['O', 'O'],
            vec!['O', 'O']
        ];
        let expected = board.clone();

        Solution::solve(&mut board);
        assert_eq!(board, expected);
    }
}
