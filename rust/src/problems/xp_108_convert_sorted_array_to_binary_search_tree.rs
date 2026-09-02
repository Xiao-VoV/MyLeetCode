/*
 * @lc app=leetcode.cn id=108 lang=rust
 *
 * [108] Convert Sorted Array to Binary Search Tree
 *
 * https://leetcode.cn/problems/convert-sorted-array-to-binary-search-tree/description/
 *
 * algorithms
 * Easy (80.62%)
 * Likes:    1821
 * Dislikes: 0
 * Total Accepted:    897.2K
 * Total Submissions: 1.1M
 * Testcase Example:  '[-10,-3,0,5,9]'
 *
 * Given an integer array nums where the elements are sorted in ascending
 * order, convert it to a height-balanced binary search tree.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [-10,-3,0,5,9]
 * Output: [0,-3,9,-10,null,5]
 * Explanation: [0,-10,5,null,-3,null,9] is also accepted:
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [1,3]
 * Output: [3,1]
 * Explanation: [1,null,3] and [3,1] are both height-balanced BSTs.
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= nums.length <= 10^4
 * -10^4 <= nums[i] <= 10^4
 * nums is sorted in a strictly increasing order.
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&nums, 0, nums.len() as i32 -1)
    }

    fn build(nums: &[i32],left:i32,right:i32)->Option<Rc<RefCell<TreeNode>>>{
        if left > right {
            return None;

        }

        let mid = left + (right-left)/2;
        let mut node = TreeNode::new(nums[mid as usize]);

        node.left = Self::build(nums, left, mid-1);
        node.right = Self::build(nums, mid+1, right);

        Some(Rc::new(RefCell::new(node)))
    }
}
// @lc code=end

