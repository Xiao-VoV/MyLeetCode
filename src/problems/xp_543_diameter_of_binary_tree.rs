/*
 * @lc app=leetcode.cn id=543 lang=rust
 *
 * [543] Diameter of Binary Tree
 *
 * https://leetcode.cn/problems/diameter-of-binary-tree/description/
 *
 * algorithms
 * Easy (64.24%)
 * Likes:    1957
 * Dislikes: 0
 * Total Accepted:    865.9K
 * Total Submissions: 1.3M
 * Testcase Example:  '[1,2,3,4,5]'
 *
 * Given the root of a binary tree, return the length of the diameter of the
 * tree.
 *
 * The diameter of a binary tree is the length of the longest path between any
 * two nodes in a tree. This path may or may not pass through the root.
 *
 * The length of a path between two nodes is represented by the number of edges
 * between them.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,2,3,4,5]
 * Output: 3
 * Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].
 *
 *
 * Example 2:
 *
 *
 * Input: root = [1,2]
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [1, 10^4].
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
            right: None,
        }
    }
}
use super::Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        Self::dfs(root.as_ref(), &mut max_depth);
        max_depth
    }

    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, max_depth: &mut i32) -> i32 {
        let Some(node) = node else {
            return 0;
        };

        let node = node.borrow();

        let left_depth = Self::dfs(node.left.as_ref(), max_depth);
        let right_depth = Self::dfs(node.right.as_ref(), max_depth);

        *max_depth = (*max_depth).max(left_depth + right_depth);

        left_depth.max(right_depth) + 1
    }
}
// @lc code=end
