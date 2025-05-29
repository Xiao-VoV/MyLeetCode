/*
 * @lc app=leetcode.cn id=4 lang=javascript
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number}
 */
var findMedianSortedArrays = function(nums1, nums2) {
    let nums3 = [...nums1,...nums2].sort((a, b) => a - b);
    if(nums3.length%2 === 0){
        return (nums3[nums3.length/2]+nums3[nums3.length/2-1])/2
    }
    else{
        return (nums3[(nums3.length-1)/2])
    }
};
/**
 * 在 JavaScript 中，sort() 方法默认会将数组元素转换为字符串并按照 Unicode 码点顺序进行排序。这意味着，如果数组中包含负数或非整数，sort() 方法可能会得到不正确的排序结果。
例如，考虑以下数组：
let arr = [10, 5, 20, 1, 3];
如果直接调用 arr.sort()，得到的结果可能是 [1, 10, 20, 3, 5]，这显然不是按照数字大小排序的。
 */

// @lc code=end

