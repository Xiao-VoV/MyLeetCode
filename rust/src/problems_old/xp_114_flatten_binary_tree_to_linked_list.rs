/*
 * @lc app=leetcode.cn id=114 lang=rust
 *
 * [114] Flatten Binary Tree to Linked List
 *
 * https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/description/
 *
 * algorithms
 * Medium (76.19%)
 * Likes:    2031
 * Dislikes: 0
 * Total Accepted:    895.1K
 * Total Submissions: 1.2M
 * Testcase Example:  '[1,2,5,3,4,null,6]'
 *
 * Given the root of a binary tree, flatten the tree into a "linked
 * list":
 *
 *
 * The "linked list" should use the same TreeNode class where the right child
 * pointer points to the next node in the list and the left child pointer is
 * always null.
 * The "linked list" should be in the same order as a pre-order traversal of
 * the binary tree.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,2,5,3,4,null,6]
 * Output: [1,null,2,null,3,null,4,null,5,null,6]
 *
 *
 * Example 2:
 *
 *
 * Input: root = []
 * Output: []
 *
 *
 * Example 3:
 *
 *
 * Input: root = [0]
 * Output: [0]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [0, 2000].
 * -100 <= Node.val <= 100
 *
 *
 *
 * Follow up: Can you flatten the tree in-place (with O(1) extra space)?
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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut vec = vec![];
        if let Some(root) = root.clone() {
            Self::P_Order(root, &mut vec);
        }

        // 重构链表
        for i in 0..vec.len().saturating_sub(1) {
            let mut node_mut = vec[i].borrow_mut();
            node_mut.left = None;
            node_mut.right = Some(vec[i + 1].clone());
        }
    }

    fn P_Order<'a>(root: Rc<RefCell<TreeNode>>, vec: &mut Vec<Rc<RefCell<TreeNode>>>) {
        vec.push(root.clone());

        if let Some(left) = root.borrow().left.as_ref().cloned() {
            Self::P_Order(left, vec);
        }

        if let Some(right) = root.borrow().right.as_ref().cloned() {
            Self::P_Order(right, vec);
        }
    }
}
// @lc code=end
