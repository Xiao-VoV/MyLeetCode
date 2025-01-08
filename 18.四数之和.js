/*
 * @lc app=leetcode.cn id=18 lang=javascript
 *
 * [18] 四数之和
 *
 * https://leetcode.cn/problems/4sum/description/
 *
 * algorithms
 * Medium (36.67%)
 * Likes:    2009
 * Dislikes: 0
 * Total Accepted:    655.2K
 * Total Submissions: 1.8M
 * Testcase Example:  '[1,0,-1,0,-2,2]\n0'
 *
 * 给你一个由 n 个整数组成的数组 nums ，和一个目标值 target 。请你找出并返回满足下述全部条件且不重复的四元组 [nums[a],
 * nums[b], nums[c], nums[d]] （若两个四元组元素一一对应，则认为两个四元组重复）：
 * 
 * 
 * 0 <= a, b, c, d < n
 * a、b、c 和 d 互不相同
 * nums[a] + nums[b] + nums[c] + nums[d] == target
 * 
 * 
 * 你可以按 任意顺序 返回答案 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,0,-1,0,-2,2], target = 0
 * 输出：[[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [2,2,2,2,2], target = 8
 * 输出：[[2,2,2,2]]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= nums.length <= 200
 * -10^9 <= nums[i] <= 10^9
 * -10^9 <= target <= 10^9
 * 
 * 
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[][]}
 */
var fourSum = function(nums, target) {
    // 错误思路,不能双指针嵌套
    // nums = nums.sort((a,b)=>a-b)
    // let result = []
    // for(let outLeft = 0,outRight = nums.length-1;outLeft<outRight;outLeft++,outRight--)
    // {
    //     if (outLeft > 0 && nums[outLeft] === nums[outLeft - 1]) continue;
    //     if (outRight < nums.length-1 && nums[outRight] === nums[outRight + 1]) continue;
    //     for(innerLeft = outLeft+1,innerRight = outRight-1;innerLeft <innerRight;innerLeft++,innerRight--)
    //     {
    //         let sum = nums[outLeft]+nums[outRight]+nums[innerLeft]+nums[innerRight]
    //         if(sum === target)
    //         {
    //             result.push([nums[outLeft],nums[outRight],nums[innerLeft],nums[innerRight]].sort((a,b)=>a-b))
    //             while (innerLeft < innerRight && nums[innerLeft] === nums[innerLeft + 1]) innerLeft++; // 跳过重复的元素
    //             while (innerLeft < innerRight && nums[innerRight] === nums[innerRight - 1]) innerRight--; // 跳过重复的元素
    //             innerLeft++;
    //             innerRight--;
    //         }else if(sum < 0){
    //             innerLeft ++;
    //         }else{
    //             innerRight--;
    //         }
    //     }
    // }
    // return result;
    nums = nums.sort((a,b)=>a-b)
    let result = []
    for(let left = 0; left < nums.length - 3; left++) {
        if (left > 0 && nums[left] === nums[left - 1]) continue;
        for(let right = nums.length - 1; right > left + 2; right--) {
            if (right < nums.length - 1 && nums[right] === nums[right + 1]) continue;
            let i = left + 1, j = right - 1;
            while (i < j) {
                let sum = nums[left] + nums[i] + nums[j] + nums[right];
                if (sum === target) {
                    result.push([nums[left], nums[i], nums[j], nums[right]]);
                    while (i < j && nums[i] === nums[i + 1]) i++;
                    while (i < j && nums[j] === nums[j - 1]) j--;
                    i++;
                    j--;
                } else if (sum < target) {
                    i++;
                } else {
                    j--;
                }
            }
        }
    }
    return result;
};
// @lc code=end

