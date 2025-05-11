pub struct Solution;

impl Solution {
pub fn solve(board: &mut Vec<Vec<char>>) {
        let rows = board.len();
        if rows == 0 {
            return;
        }
        let cols = board[0].len();

        fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
            if r >= rows || c >= cols || board[r][c] != 'O' {
                return;
            }
            board[r][c] = 'T';
            let directions = [(1,0),(-1,0),(0,1),(0,-1)];
            for (dr, dc) in directions.iter() {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize 
                {
                    dfs(board, nr as usize, nc as usize, rows, cols);
                } 
            }
        }

        for i in 0..rows {
            if board[i][0] == 'O' 
            {
                dfs(board, i, 0, rows, cols);
            }
            if board[i][cols - 1] == 'O'
             {
                dfs(board, i, cols - 1, rows, cols);
            }
        }
        for j in 0..cols{
            if board[0][j] == 'O' 
            {
                dfs(board, 0, j, rows, cols);
            }
            if board[rows - 1][j] == 'O' 
            {
                dfs(board, rows - 1, j, rows, cols);
            }
        }
        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == 'O' 
                {
                    board[i][j] = 'X';
                }
                else if board[i][j] == 'T' 
                {
                    board[i][j] = 'O';
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

pub fn main() {
    
}