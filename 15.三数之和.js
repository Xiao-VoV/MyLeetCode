/*
 * @lc app=leetcode.cn id=15 lang=javascript
 *
 * [15] 三数之和
 *
 * https://leetcode.cn/problems/3sum/description/
 *
 * algorithms
 * Medium (38.77%)
 * Likes:    7213
 * Dislikes: 0
 * Total Accepted:    2.1M
 * Total Submissions: 5.4M
 * Testcase Example:  '[-1,0,1,2,-1,-4]'
 *
 * 给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j !=
 * k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请你返回所有和为 0 且不重复的三元组。
 * 
 * 注意：答案中不可以包含重复的三元组。
 * 
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [-1,0,1,2,-1,-4]
 * 输出：[[-1,-1,2],[-1,0,1]]
 * 解释：
 * nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
 * nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
 * nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
 * 不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
 * 注意，输出的顺序和三元组的顺序并不重要。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [0,1,1]
 * 输出：[]
 * 解释：唯一可能的三元组和不为 0 。
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：nums = [0,0,0]
 * 输出：[[0,0,0]]
 * 解释：唯一可能的三元组和为 0 。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 3 <= nums.length <= 3000
 * -10^5 <= nums[i] <= 10^5
 * 
 * 
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
// 三重循环,过于复杂
// var threeSum = function(nums) {
//     let result = [];
//     for(const[i,vi] of nums.entries()){
//         for (const[j,vj] of nums.entries()){
//             for (const[k,vk]of nums.entries()){
//                 if(i != j&& i!=k&& j!=k && (vi+vj+vk)===0){
//                     result.push([vi,vj,vk].sort((a,b)=>a-b));
//                 }
//             }
//         }
//     }
//     let unique = new Set();
//     for (let i = 0; i < result.length; i++) {
//         unique.add(JSON.stringify(result[i]));
//     }
//     return Array.from(unique, JSON.parse);
// };
var threeSum = function(nums) {
    let result = []
    nums = nums.sort((a,b)=>a-b);
    for(let i=0;i<nums.length-2;i++){
        if (i > 0 && nums[i] === nums[i - 1]) continue; // 跳过重复的元素

        for(let j=i+1,k=nums.length-1;j<k;)
        {
            if(nums[i] + nums[j]+ nums[k] == 0)
            {
                result.push([nums[i],nums[j],nums[k]])
                while (j < k && nums[j] === nums[j + 1]) j++; // 跳过重复的元素
                while (j < k && nums[k] === nums[k - 1]) k--; // 跳过重复的元素
                j++;
                k--;
            }else if(nums[i] + nums[j]+ nums[k] < 0){
                j++
                while(nums[j]===nums[j-1])
                {
                    j++
                }
            }else{
                k--
                while(nums[k] === nums[k+1]){
                    k--
                }
            }
        }
    }
    return result;
};

/**

双指针是一种常见的算法技巧，通常用于解决数组或链表相关的问题。其基本思路是使用两个指针，分别指向数组或链表的不同位置，通过移动指针来遍历数据结构，从而找到满足特定条件的元素或子序列。

在你提供的代码中，双指针的思路是通过固定一个元素，然后使用两个指针分别指向该元素之后的数组的两端，通过移动指针来找到和为0的三元组。具体步骤如下：

首先，对数组进行排序，这样可以方便地跳过重复的元素。
然后，遍历数组，对于每个元素，将其作为三元组的第一个元素。
接着，使用两个指针left和right，分别指向当前元素之后的数组的两端。
计算nums[i] + nums[left] + nums[right]的和，如果和为0，则将这三个元素组成的三元组添加到结果数组中。
如果和小于0，则说明nums[left]太小，需要将left指针向右移动。
如果和大于0，则说明nums[right]太大，需要将right指针向左移动。
在移动指针的过程中，需要跳过重复的元素，以避免结果数组中包含重复的三元组。
重复步骤4-7，直到left指针大于等于right指针。
通过这种方式，可以在O(n^2)的时间复杂度内找到所有和为0的三元组。这种双指针的思路在解决类似的问题时非常有效，可以大大降低时间复杂度。

 */
// @lc code=end


// @after-stub-for-debug-begin
module.exports = threeSum;
// @after-stub-for-debug-end