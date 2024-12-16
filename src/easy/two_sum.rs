/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
///
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
///
/// You can return the answer in any order.
///
/// Example 1:
///
/// - Input: nums = [2,7,11,15], target = 9
/// - Output: [0,1]
///
/// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
///
/// Example 2:
///
/// - Input: nums = [3,2,4], target = 6
/// - Output: [1,2]
///
/// Example 3:
///
/// - Input: nums = [3,3], target = 6
/// - Output: [0,1]
///
///
/// Constraints:
///
/// 2 <= nums.length <= 104
///
/// -109 <= nums[i] <= 109
///
/// -109 <= target <= 109
///
/// Only one valid answer exists.
///
///
/// Follow-up: Can you come up with an algorithm that is less than O(n<sup>2</sup>) time complexity?
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums: Vec<Number> = nums.into_iter().enumerate().map(Number::from).collect(); // O(Nlog(N))
    nums.sort();
    for i in 0..nums.len() {
        // search for target - nums[i] in the sequence nums[i+1..]; // O(log(N))
        // Due to PartiaEq implementation for Number, `old_index` field could be anything i32
        // for `candidate`, the choice of 0 for `old_index` field value is arbitrary.
        let candidate = Number::from((0, target - nums[i].num));
        if let Ok(j) = nums[i + 1..].binary_search(&candidate) {
            return vec![nums[i].old_index, nums[j + i + 1].old_index];
        }
    }
    return vec![];
}

#[derive(Debug, Clone, Copy, Eq)]
struct Number {
    num: i32,
    old_index: i32,
}

impl Number {
    pub fn from(value: (usize, i32)) -> Self {
        Self {
            num: value.1,
            old_index: value.0 as i32,
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.num.eq(&other.num)
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.num.partial_cmp(&other.num)
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.num.cmp(&other.num)
    }
}

pub fn _two_sum_slow(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            let (a, b) = (nums[i], nums[j]);
            if a + b == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return vec![];
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_two_sum() {
        assert_eq!(_two_sum_slow(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);

        assert_eq!(_two_sum_slow(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);

        assert_eq!(_two_sum_slow(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
