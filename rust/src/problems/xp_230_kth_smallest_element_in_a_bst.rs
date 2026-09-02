/*
 * @lc app=leetcode.cn id=230 lang=rust
 *
 * [230] Kth Smallest Element in a BST
 *
 * https://leetcode.cn/problems/kth-smallest-element-in-a-bst/description/
 *
 * algorithms
 * Medium (79.81%)
 * Likes:    1116
 * Dislikes: 0
 * Total Accepted:    775.5K
 * Total Submissions: 971.7K
 * Testcase Example:  '[3,1,4,null,2]\n1'
 *
 * Given the root of a binary search tree, and an integer k, return the k^th
 * smallest value (1-indexed) of all the values of the nodes in the tree.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: root = [3,1,4,null,2], k = 1
 * Output: 1
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: root = [5,3,6,2,4,null,null,1], k = 3
 * Output: 3
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in the tree is n.
 * 1 <= k <= n <= 10^4
 * 0 <= Node.val <= 10^4
 * 
 * 
 * 
 * Follow up: If the BST is modified often (i.e., we can do insert and delete
 * operations) and you need to find the kth smallest frequently, how would you
 * optimize?
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
use std::cell::{Ref, RefCell};
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {

        let mut vec = Rc::new(RefCell::new(Vec::<i32>::new()));
        Self::bfs(root,vec,k);
        // vec.borrow()[(k-1) as usize]
        todo!()
    }

    fn bfs(node:Option<Rc<RefCell<TreeNode>>>, vec:Rc<RefCell<Vec<i32>>>,k:i32){
        let Some(node) = node else{
            return;
        };

        Self::bfs(node.borrow().left.as_ref().cloned(),vec.clone(),k);
        vec.borrow_mut().push(node.borrow().val);
        if vec.borrow().len() as i32 == k{
            return;
        }
        Self::bfs(node.borrow().right.as_ref().cloned(),vec.clone(),k);
    }
}
// @lc code=end

