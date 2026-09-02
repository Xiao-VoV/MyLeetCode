/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 *
 * https://leetcode.cn/problems/merge-two-sorted-lists/description/
 *
 * algorithms
 * Easy (68.18%)
 * Likes:    4053
 * Dislikes: 0
 * Total Accepted:    2.5M
 * Total Submissions: 3.6M
 * Testcase Example:  '[1,2,4]\n[1,3,4]'
 *
 * You are given the heads of two sorted linked lists list1 and list2.
 * 
 * Merge the two lists into one sorted list. The list should be made by
 * splicing together the nodes of the first two lists.
 * 
 * Return the head of the merged linked list.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: list1 = [1,2,4], list2 = [1,3,4]
 * Output: [1,1,2,3,4,4]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: list1 = [], list2 = []
 * Output: []
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: list1 = [], list2 = [0]
 * Output: [0]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in both lists is in the range [0, 50].
 * -100 <= Node.val <= 100
 * Both list1 and list2 are sorted in non-decreasing order.
 * 
 * 
 */

// @lc code=start
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

use super::Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;
        let mut head = ListNode::new(0);
        let mut result = &mut head;
        while list1.is_some() && list2.is_some(){
            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val{
                let next = list1.as_mut().unwrap().next.take();
                result.next = list1;
                list1 = next;
            }else{
                let next = list2.as_mut().unwrap().next.take();
                result.next = list2;
                list2 = next;
            }

            result = result.next.as_mut().unwrap();
        }

        if list1.is_some(){
            result.next = list1;
        }

        if list2.is_some(){
            result.next = list2;
        }
        head.next   
    }
}
// @lc code=end

