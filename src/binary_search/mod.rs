//leetcode2529
pub fn maximum_count(nums: Vec<i32>) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = nums.len() as i32 - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] >= 0 {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    right = left;
    while right < nums.len() as i32 && nums[right as usize] == 0 {
        right += 1;
    }
    left.max(nums.len() as i32 - right)
}

struct Solution;

impl Solution {
    //闭区间写法
    pub fn bineary_search(vec: &Vec<i32>, target: i64) -> Result<usize, usize> {
        let mut left: i32 = 0;
        let mut right: i32 = vec.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if (vec[mid as usize] as i64) < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        if left <= vec.len() as i32 - 1 {
            Ok(left as usize)
        } else {
            Err(0)
        }
    }

    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
        potions.iter_mut().for_each(|x| {
            *x <<= 1;
        });
        spells
            .into_iter()
            .map(|x| {
                let y = (success - 1) / (x as i64) * 2 + 1;
                let y = if y >> 31 != 0 {
                    i32::MAX
                } else {
                    (y & i32::MAX as i64) as i32
                };
                (potions.len()
                    - (match potions.binary_search(&y) {
                        Ok(_) => unreachable!(),
                        Err(i) => i,
                    })) as i32
            })
            .collect()
    }
}

//leetcode 275
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let length: i32 = citations.len() as i32 - 1;
        let mut left: i32 = 0;
        let mut right: i32 = citations.len() as i32 - 1;
        while left <= right {
            let mid: i32 = left + (right - left) / 2;
            if (length - mid + 1) > citations[mid as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        length - left + 1
    }
}
//leetcode 875
impl Solution {
    fn get_time(piles: &Vec<i32>, speed: i32, limit: i32) -> bool {
        let mut result = piles.len() as i32;
        for i in piles {
            result += (*i - 1) / speed;
            if result > limit {
                return false;
            }
        }
        true
    }
    pub fn min_eating_speed(mut piles: Vec<i32>, h: i32) -> i32 {
        piles.sort_unstable();
        let mut min = 0;
        let mut max = piles[piles.len() - 1];
        while min + 1 < max {
            let mid = min + (max - min) / 2;
            if Solution::get_time(&piles, mid, h) {
                max = mid;
            } else {
                min = mid;
            }
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn leetcode_2529() {
        // assert_eq!(3, maximum_count(vec![-2, -1, -1, 1, 2, 3]));
        // assert_eq!(3, maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]));
        // assert_eq!(4, maximum_count(vec![5, 20, 66, 1314]));
        assert_eq!(
            0,
            maximum_count(vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );
    }

    #[test]
    pub fn test_2300() {
        // let mut spell = vec![5, 1, 3];
        // let mut position = vec![1, 2, 3, 4, 5];
        // let success: i64 = 7;
        // let result = Solution::successful_pairs(spell, position, success);
        // println!("{result:?}");
        let mut spell = vec![3, 1, 2];
        let mut position = vec![8, 5, 8];
        let success: i64 = 16;
        let result = Solution::successful_pairs(spell, position, success);
        println!("{result:?}");
    }
    #[test]
    pub fn test_275() {
        let mut position = vec![0];
        let result = Solution::h_index(position);
        println!("{result:?}");
    }

    #[test]
    pub fn test_875() {
        let mut position = vec![805306368, 805306368, 805306368];
        let result = Solution::min_eating_speed(position, 1000000000);
        println!("{result:?}");
    }
}
