/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
 *
 * https://leetcode.cn/problems/remove-nth-node-from-end-of-list/description/
 *
 * algorithms
 * Medium (53.26%)
 * Likes:    3311
 * Dislikes: 0
 * Total Accepted:    2.1M
 * Total Submissions: 4M
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * Given the head of a linked list, remove the n^th node from the end of the
 * list and return its head.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: head = [1,2,3,4,5], n = 2
 * Output: [1,2,3,5]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: head = [1], n = 1
 * Output: []
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: head = [1,2], n = 1
 * Output: [1]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in the list is sz.
 * 1 <= sz <= 30
 * 0 <= Node.val <= 100
 * 1 <= n <= sz
 * 
 * 
 * 
 * Follow up: Could you do this in one pass?
 * 
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

// use super::Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = head;
        let mut first = &head;

        let mut length = 0;
        while let Some(node) = first{
            length +=1;

            first = &node.next;
        }

        if length ==n{
            return head.unwrap().next;
        }

        let mut second = head.as_mut().unwrap();

        for _ in 0..(length-n-1){
            second = second.next.as_mut().unwrap();
        }

        let mut next = second.next.take().unwrap();
        second.next = next.next.take();

        head
    }
}
// @lc code=end

