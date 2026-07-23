/*
 * @lc app=leetcode.cn id=199 lang=rust
 *
 * [199] Binary Tree Right Side View
 *
 * https://leetcode.cn/problems/binary-tree-right-side-view/description/
 *
 * algorithms
 * Medium (73.67%)
 * Likes:    1352
 * Dislikes: 0
 * Total Accepted:    873.2K
 * Total Submissions: 1.2M
 * Testcase Example:  '[1,2,3,null,5,null,4]'
 *
 * Given the root of a binary tree, imagine yourself standing on the right side
 * of it, return the values of the nodes you can see ordered from top to
 * bottom.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: root = [1,2,3,null,5,null,4]
 * 
 * Output: [1,3,4]
 * 
 * Explanation:
 * 
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: root = [1,2,3,4,null,null,null,5]
 * 
 * Output: [1,3,4,5]
 * 
 * Explanation:
 * 
 * 
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: root = [1,null,3]
 * 
 * Output: [1,3]
 * 
 * 
 * Example 4:
 * 
 * 
 * Input: root = []
 * 
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
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
// use super::Solution;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {

        use std::collections::VecDeque;
        let mut result = Vec::<i32>::new();
        let mut queue = VecDeque::new();

        queue.push_back(root);

        while  !queue.is_empty() {
            let level_len = queue.len();

            for i in 0..level_len {
                if let Some(Some(node)) = queue.pop_front() {
                    let node_ref = node.borrow();
                    if i == level_len - 1 {
                        result.push(node_ref.val);
                    }
                    if let Some(left) = node_ref.left.as_ref().cloned(){
                        queue.push_back(Some(left));
                    }
                    if let Some(right) = node_ref.right.as_ref().cloned(){
                        queue.push_back(Some(right));
                    }
                }
            }
        }

        result
    }
}
// @lc code=end

