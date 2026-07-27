/*
 * @lc app=leetcode.cn id=51 lang=rust
 *
 * [51] N-Queens
 *
 * https://leetcode.cn/problems/n-queens/description/
 *
 * algorithms
 * Hard (75.27%)
 * Likes:    2502
 * Dislikes: 0
 * Total Accepted:    708K
 * Total Submissions: 940.6K
 * Testcase Example:  '4'
 *
 * The n-queens puzzle is the problem of placing n queens on an n x n
 * chessboard such that no two queens attack each other.
 * 
 * Given an integer n, return all distinct solutions to the n-queens puzzle.
 * You may return the answer in any order.
 * 
 * Each solution contains a distinct board configuration of the n-queens'
 * placement, where 'Q' and '.' both indicate a queen and an empty space,
 * respectively.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: n = 4
 * Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
 * Explanation: There exist two distinct solutions to the 4-queens puzzle as
 * shown above
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: n = 1
 * Output: [["Q"]]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= n <= 9
 * 
 * 
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        todo!()
    }

    fn back_track51(n:u32,result:&mut Vec<Vec<String>>,(x,y):(u32,u32),used:&mut Vec<Vec<bool>>){
        if x==n && y ==n{
            result[x as usize][y as usize] == "Q";
            return;
        }

        

    }
}
// @lc code=end

