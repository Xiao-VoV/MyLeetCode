/*
 * @lc app=leetcode.cn id=61 lang=javascript
 *
 * [61] 旋转链表
 *
 * https://leetcode.cn/problems/rotate-list/description/
 *
 * algorithms
 * Medium (41.44%)
 * Likes:    1112
 * Dislikes: 0
 * Total Accepted:    429.1K
 * Total Submissions: 1M
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * 给你一个链表的头节点 head ，旋转链表，将链表每个节点向右移动 k 个位置。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：head = [1,2,3,4,5], k = 2
 * 输出：[4,5,1,2,3]
 *
 *
 * 示例 2：
 *
 *
 * 输入：head = [0,1,2], k = 4
 * 输出：[2,0,1]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 链表中节点的数目在范围 [0, 500] 内
 * -100 <= Node.val <= 100
 * 0 <= k <= 2 * 10^9
 *
 *
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @param {number} k
 * @return {ListNode}
 */
var rotateRight = function (head, k) {
    /**
     * 1. 计算链表长度：首先需要计算链表的长度，以便确定旋转后的新头节点位置。
     * 2. 计算实际旋转步数：由于旋转步数 k 可能大于链表长度，因此需要对 k 取模，得到实际需要旋转的步数。
     * 3. 找到新的头节点和尾节点：使用快慢指针的方法，快指针先走 k 步，然后快慢指针一起走，直到快指针走到链表末尾。此时，慢指针指向的节点就是新链表的尾节点，慢指针的下一个节点就是新链表的头节点。
     * 4. 旋转链表：将新链表的尾节点的 next 指针指向 null，将原链表的尾节点的 next 指针指向原链表的头节点，从而完成链表的旋转。
     */
    let length = 0;
    let tmp = head;
    if (!head) {
        return head;
    }
    while (tmp) {
        tmp = tmp.next;
        length++;
    }
    let s = k % length;
    if (s == 0) {
        return head;
    }
    let slow = head,
        fast = head;
    while (s > 0) {
        fast = fast.next;
        s--;
    }
    while (fast.next) {
        fast = fast.next;
        slow = slow.next;
    }
    let newHead = slow.next;
    slow.next = null;
    fast.next = head;

    return newHead;
};
// @lc code=end

// @after-stub-for-debug-begin
module.exports = rotateRight;
// @after-stub-for-debug-end
