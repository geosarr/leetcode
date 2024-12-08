/// Given an integer x, return true if x is a palindrome and false otherwise.
///
/// **Example 1:**
///
/// - **Input:** x = 121
/// - **Output:** true
///
/// **Explanation:** 121 reads as 121 from left to right and from right to left.
///
/// **Example 2:**
///
/// - **Input:** x = -121
/// - **Output:** false
///
/// **Explanation:** From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
///
/// **Example 3:**
///
/// - **Input:** x = 10
/// - **Output:** false
///
/// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
///  
///
/// **Constraints:**
///
/// -231 <= x <= 231 - 1
///  
/// Follow up: Could you solve it without converting the integer to a string?
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let x = x.to_string().chars().collect::<Vec<char>>();
    let p = x.len();
    for i in 0..p / 2 {
        if x[i] != x[p - 1 - i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_two_sum() {
        assert!(is_palindrome(121));
        assert!(!is_palindrome(-121));
        assert!(!is_palindrome(10));
        assert!(is_palindrome(0));
    }
}
