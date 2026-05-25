/*
 * @lc app=leetcode.cn id=226 lang=rust
 *
 * [226] Invert Binary Tree
 *
 * https://leetcode.cn/problems/invert-binary-tree/description/
 *
 * algorithms
 * Easy (82.46%)
 * Likes:    2102
 * Dislikes: 0
 * Total Accepted:    1.5M
 * Total Submissions: 1.8M
 * Testcase Example:  '[4,2,7,1,3,6,9]'
 *
 * Given the root of a binary tree, invert the tree, and return its root.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: root = [4,2,7,1,3,6,9]
 * Output: [4,7,2,9,6,3,1]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: root = [2,1,3]
 * Output: [2,3,1]
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
 * The number of nodes in the tree is in the range [0, 100].
 * -100 <= Node.val <= 100
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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root{
            Some(n)=>{
                let left = n.borrow().left.clone();
                let right = n.borrow().right.clone();

                n.borrow_mut().left = right;
                n.borrow_mut().right = left;

                Self::invert_tree(n.borrow().left.clone());
                Self::invert_tree(n.borrow().right.clone());
                Some(n.clone())
            }
            None=>None
        }
    }
}
// @lc code=end

