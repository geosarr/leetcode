/// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
///
/// You must write an algorithm with O(log n) runtime complexity.
///
/// Example 1:
///
/// - Input: nums = [1,3,5,6], target = 5
/// - Output: 2
///
/// Example 2:
///
/// - Input: nums = [1,3,5,6], target = 2
/// - Output: 1
///
/// Example 3:
///
/// - Input: nums = [1,3,5,6], target = 7
/// - Output: 4
///
/// Constraints:
/// - 1 <= nums.length <= 10<sup>4</sup>
/// - -10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>
/// - nums contains distinct values sorted in ascending order.
/// - -10<sup>4</sup> <= target <= 10<sup>4</sup>
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(index) => index as i32,
        Err(index) => index as i32,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_search_insert() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
