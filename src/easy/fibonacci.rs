use std::collections::HashMap;

/// The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1. That is,
///
/// F(0) = 0, F(1) = 1
///
/// F(n) = F(n - 1) + F(n - 2), for n > 1.
///
/// Example 1:
///
/// - Input: n = 2
/// - Output: 1
///
/// Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
///
/// Example 2:
///
/// - Input: n = 3
/// - Output: 2
///
/// Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
///
/// Example 3:
///
/// - Input: n = 4
/// - Output: 3
///
/// Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.
///
/// Constraints:
///
/// 0 <= n <= 30
pub fn fib(n: i32) -> i32 {
    let hash = HashMap::new();
    _fib(n, hash)[&n]

    // // ###### ALTERNATIVE SOLUTION: Use closed form ###################
    // let sqrt_five = 5f32.sqrt();
    // let golden = (1. + sqrt_five) / 2.;
    // let conj_golden = 1. - golden;
    // let res = (golden.powi(n) - conj_golden.powi(n)) / sqrt_five;
    // let floor_res = res.floor();
    // if res - floor_res > floor_res + 1. - res {
    //     floor_res as i32 + 1
    // } else {
    //     floor_res as i32
    // }
}
pub fn _fib(n: i32, mut hash: HashMap<i32, i32>) -> HashMap<i32, i32> {
    if hash.contains_key(&n) {
        return hash;
    }
    if n <= 1 {
        hash.insert(n, n);
        return hash;
    }
    let mut hash = _fib(n - 1, _fib(n - 2, hash));
    hash.insert(n, hash[&(n - 1)] + hash[&(n - 2)]);
    return hash;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fib() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
    }
}
