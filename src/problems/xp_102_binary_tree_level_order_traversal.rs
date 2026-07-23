/*
 * @lc app=leetcode.cn id=102 lang=rust
 *
 * [102] Binary Tree Level Order Traversal
 *
 * https://leetcode.cn/problems/binary-tree-level-order-traversal/description/
 *
 * algorithms
 * Medium (70.75%)
 * Likes:    2320
 * Dislikes: 0
 * Total Accepted:    1.7M
 * Total Submissions: 2.4M
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * Given the root of a binary tree, return the level order traversal of its
 * nodes' values. (i.e., from left to right, level by level).
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: root = [3,9,20,null,null,15,7]
 * Output: [[3],[9,20],[15,7]]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: root = [1]
 * Output: [[1]]
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: root = []
 * Output: []
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in the tree is in the range [0, 2000].
 * -1000 <= Node.val <= 1000
 * 
 * 
 */

// @lc code=start
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use super::Solution;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let Some(root) = root else{
            return result;
        };
        
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back(root);        

        while !queue.is_empty(){
            let level_size = queue.len();
            let mut level_val = vec![];

            for _ in 0..level_size{
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                level_val.push(node.val);

                if let Some(left) = node.left.clone(){
                    queue.push_back(left);
                }
                if let Some(right) = node.right.clone(){
                    queue.push_back(right);
                }
            }

            result.push(level_val);
        }
        result
    }
}
// @lc code=end

