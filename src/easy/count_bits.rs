/// Given an integer n, return an array ans of length n + 1 such that for each i (0 <= i <= n),
/// ans[i] is the number of 1's in the binary representation of i.
///
/// Example 1:
///
/// - Input: n = 2
/// - Output: [0,1,1]
///
/// Explanation:
///
/// - 0 --> 0
/// - 1 --> 1
/// - 2 --> 10
///
/// Example 2:
///
/// - Input: n = 5
/// - Output: [0,1,1,2,1,2]
///
/// Explanation:
/// - 0 --> 0
/// - 1 --> 1
/// - 2 --> 10
/// - 3 --> 11
/// - 4 --> 100
/// - 5 --> 101
/// Constraints:
///
/// 0 <= n <= 10<sup>5</sup>
pub fn count_bits(n: i32) -> Vec<i32> {
    (0..n + 1)
        .map(|mut k| {
            let mut count = 0;
            while k > 0 {
                count += k % 2;
                k /= 2;
            }
            count
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_count_bits() {
        assert_eq!(count_bits(1), vec![0, 1]);
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
