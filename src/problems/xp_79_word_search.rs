use std::{io::SeekFrom::Current, result};

/*
 * @lc app=leetcode.cn id=79 lang=rust
 *
 * [79] Word Search
 *
 * https://leetcode.cn/problems/word-search/description/
 *
 * algorithms
 * Medium (51.28%)
 * Likes:    2214
 * Dislikes: 0
 * Total Accepted:    933.6K
 * Total Submissions: 1.8M
 * Testcase Example:  '[["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]]\n"ABCCED"'
 *
 * Given an m x n grid of characters board and a string word, return true if
 * word exists in the grid.
 *
 * The word can be constructed from letters of sequentially adjacent cells,
 * where adjacent cells are horizontally or vertically neighboring. The same
 * letter cell may not be used more than once.
 *
 *
 * Example 1:
 *
 *
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word
 * = "ABCCED"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word
 * = "SEE"
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word
 * = "ABCB"
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * m == board.length
 * n = board[i].length
 * 1 <= m, n <= 6
 * 1 <= word.length <= 15
 * board and word consists of only lowercase and uppercase English letters.
 *
 *
 *
 * Follow up: Could you use search pruning to make your solution faster with a
 * larger board?
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word_chars: Vec<char> = word.chars().collect();
        let m = board.len();
        let n = board[0].len();
        let mut used = vec![vec![false; board[0].len()]; board.len()];
        for i in 0..m {
            for j in 0..n {
                if Self::back_track79(i as isize, j as isize, &mut used, 0, &word_chars, &board) {
                    return true;
                }
            }
        }
        false
    }

    fn back_track79(
        row: isize,
        col: isize,
        used: &mut Vec<Vec<bool>>,
        index: usize,
        word: &Vec<char>,
        board: &Vec<Vec<char>>,
    ) -> bool {
        if index == word.len() {
            return true;
        }
        let m = board.len();
        let n = board[0].len();

        if row < 0
            || col < 0
            || row as i32 >= m as i32
            || col as i32 >= n as i32
            || used[row as usize][col as usize]
            || board[row as usize][col as usize] != word[index]
        {
            return false;
        }

        used[row as usize][col as usize] = true;
        let fond = Self::back_track79(row + 1, col, used, index + 1, word, board)
            || Self::back_track79(row - 1, col, used, index + 1, word, board)
            || Self::back_track79(row, col + 1, used, index + 1, word, board)
            || Self::back_track79(row, col - 1, used, index + 1, word, board);

        used[row as usize][col as usize] = false;
        fond
    }
}
// @lc code=end
