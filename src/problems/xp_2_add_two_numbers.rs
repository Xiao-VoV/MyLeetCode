/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] Add Two Numbers
 *
 * https://leetcode.cn/problems/add-two-numbers/description/
 *
 * algorithms
 * Medium (46.85%)
 * Likes:    11961
 * Dislikes: 0
 * Total Accepted:    2.8M
 * Total Submissions: 6.1M
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order, and each of their nodes
 * contains a single digit. Add the two numbers and return the sum as a linked
 * list.
 *
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 *
 *
 * Example 1:
 *
 *
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 *
 *
 * Example 2:
 *
 *
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 *
 *
 * Example 3:
 *
 *
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in each linked list is in the range [1, 100].
 * 0 <= Node.val <= 9
 * It is guaranteed that the list represents a number that does not have
 * leading zeros.
 *
 *
 */

// @lc code=start
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use super::Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut dummy = &mut result;
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;
        let tmp: Box<ListNode> = Box::new(ListNode::new(0));
        while l1.is_some() || l2.is_some() || carry != 0 {
            let node =
                (l1.as_ref().unwrap_or(&tmp).val + l2.as_ref().unwrap_or(&tmp).val + carry) % 10;
            carry =
                (l1.as_ref().unwrap_or(&tmp).val + l2.as_ref().unwrap_or(&tmp).val + carry) / 10;
            dummy.next = Some(Box::new(ListNode::new(node)));

            dummy = dummy.next.as_mut().unwrap();

            l1 = l1.take().and_then(|node| node.next);
            l2 = l2.take().and_then(|node| node.next);
        }
        result.next
    }
}

// @lc code=end
