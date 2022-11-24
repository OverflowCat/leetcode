impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let w = word.as_bytes();
        fn f(i: usize, j: usize, rem: &[u8], board: &mut Vec<Vec<char>>) -> bool {
            if board[i][j] as u8 == rem[0] {
                if rem.len() == 1 {
                    return true;
                }
                board[i][j] = (board[i][j] as u8 - 64) as char;
                if (i + 1 < board.len() && f(i + 1, j, &rem[1..], board))
                    || (i > 0 && f(i - 1, j, &rem[1..], board))
                    || (j + 1 < board[0].len() && f(i, j + 1, &rem[1..], board))
                    || (j > 0 && f(i, j - 1, &rem[1..], board))
                {
                    return true;
                }
                board[i][j] = (board[i][j] as u8 + 64) as char;
            }
            false
        }

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if f(i, j, w, &mut board) {
                    return true;
                }
            }
        }
        false
    }
}
