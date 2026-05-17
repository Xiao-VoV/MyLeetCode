/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] Swap Nodes in Pairs
 *
 * https://leetcode.cn/problems/swap-nodes-in-pairs/description/
 *
 * algorithms
 * Medium (75.02%)
 * Likes:    2619
 * Dislikes: 0
 * Total Accepted:    1.4M
 * Total Submissions: 1.9M
 * Testcase Example:  '[1,2,3,4]'
 *
 * Given a linked list, swap every two adjacent nodes and return its head. You
 * must solve the problem without modifying the values in the list's nodes
 * (i.e., only nodes themselves may be changed.)
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: head = [1,2,3,4]
 * 
 * Output: [2,1,4,3]
 * 
 * Explanation:
 * 
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: head = []
 * 
 * Output: []
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: head = [1]
 * 
 * Output: [1]
 * 
 * 
 * Example 4:
 * 
 * 
 * Input: head = [1,2,3]
 * 
 * Output: [2,1,3]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in the list is in the range [0, 100].
 * 0 <= Node.val <= 100
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        // let (mut first,mut second) = (None,None);

        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut prev = &mut dummy;

        while let Some(mut node1) = prev.next.take(){
            match node1.next.take() {
                Some(mut node2)=>{
                    node1.next = node2.next.take();
                    node2.next = Some(node1);
                    prev.next = Some(node2);
                    prev = prev.next.as_mut().unwrap().next.as_mut().unwrap();
                }
                None=>{
                    prev.next = Some(node1);
                    break;
                }
            }

        }
        dummy.next
    }
}
// @lc code=end

