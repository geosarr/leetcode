/// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
///
/// You must implement a solution with a linear runtime complexity and use only constant extra space.
///
/// Example 1:
/// - Input: nums = [2,2,1]
/// - Output: 1
///
/// Example 2:
///
/// - Input: nums = [4,1,2,1,2]
/// - Output: 4
///
/// Example 3:
///
/// - Input: nums = [1]
/// - Output: 1
///  
///
/// Constraints:
/// - 1 <= nums.length <= 3 * 104
/// - -3 * 10<sup>4</sup> <= nums[i] <= 3 * 10<sup>4</sup>
///
/// Each element in the array appears twice except for one element which appears only once.
pub fn single_number(mut arr: Vec<i32>) -> i32 {
    arr.sort();
    let mut u = arr[0];
    let mut i = 1;
    while i < arr.len() {
        if u != arr[i] {
            return u;
        }
        u = arr[i + 1];
        i += 2;
    }
    return u;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(single_number(vec![1]), 1);
        assert_eq!(single_number(vec![2, 2, 1, 3, 1, 3, 5, -1, 8, 8, -1]), 5);
    }
}
