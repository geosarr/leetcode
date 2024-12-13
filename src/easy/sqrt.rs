/// Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.
///
/// You must not use any built-in exponent function or operator.
///
/// For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.
///  
/// Example 1:
///
/// - Input: x = 4
/// - Output: 2
///
/// Explanation: The square root of 4 is 2, so we return 2.
///
/// Example 2:
///
/// - Input: x = 8
/// - Output: 2
///
/// Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.
///
/// Constraints:
///
/// - 0 <= x <= 2<sup>31</sup> - 1
pub fn my_sqrt(x: i32) -> i32 {
    if x <= 1 {
        return x;
    }
    let (mut a, mut b) = (1, x);
    let mut m = a + (b - a) / 2;
    while b - a > 1 {
        if x / m < m {
            b = m;
        } else if x / m > m {
            a = m;
        } else {
            return m;
        }
        m = a + (b - a) / 2;
    }
    return m;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_my_sqrt() {
        // assert_eq!(my_sqrt(1), 1);
        // assert_eq!(my_sqrt(2), 1);
        // assert_eq!(my_sqrt(4), 2);
        // assert_eq!(my_sqrt(6), 2);
        // assert_eq!(my_sqrt(8), 2);
        // assert_eq!(my_sqrt(9), 3);
        // assert_eq!(my_sqrt(8192), 90);
        assert_eq!(my_sqrt(2147483647), 46340);
    }
}
