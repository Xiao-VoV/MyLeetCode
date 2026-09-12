/*
 * @lc app=leetcode.cn id=98 lang=rust
 *
 * [98] Validate Binary Search Tree
 *
 * https://leetcode.cn/problems/validate-binary-search-tree/description/
 *
 * algorithms
 * Medium (41.20%)
 * Likes:    2791
 * Dislikes: 0
 * Total Accepted:    1.5M
 * Total Submissions: 3.6M
 * Testcase Example:  '[2,1,3]'
 *
 * Given the root of a binary tree, determine if it is a valid binary search
 * tree (BST).
 * 
 * A valid BST is defined as follows:
 * 
 * 
 * The left subtree of a node contains only nodes with keys strictly less than
 * the node's key.
 * The right subtree of a node contains only nodes with keys strictly greater
 * than the node's key.
 * Both the left and right subtrees must also be binary search trees.
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: root = [2,1,3]
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: root = [5,1,4,null,null,3,6]
 * Output: false
 * Explanation: The root node's value is 5 but its right child's value is
 * 4.
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in the tree is in the range [1, 10^4].
 * -2^31 <= Node.val <= 2^31 - 1
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
use std::clone;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_BST(root, i64::MIN, i64::MAX)
    }

    fn is_BST(root: Option<Rc<RefCell<TreeNode>>>,min_val: i64, max_val: i64)->bool{
        let Some(root) = root else{
            return true;
        };
        let root = root.borrow();

        let val =root.val as i64;
        if val <= min_val || val >= max_val{
            return false;
        }
        Self::is_BST(root.left.as_ref().cloned(), min_val, val) && 
        Self::is_BST(root.right.as_ref().cloned(), val, max_val)

    }
}
// @lc code=end

