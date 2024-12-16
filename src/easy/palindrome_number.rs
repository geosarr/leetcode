/// Given an integer x, return true if x is a palindrome and false otherwise.
///
/// Example 1:
///
/// - Input: x = 121
/// - Output: true
///
/// Explanation: 121 reads as 121 from left to right and from right to left.
///
/// Example 2:
///
/// - Input: x = -121
/// - Output: false
///
/// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
///
/// Example 3:
///
/// - Input: x = 10
/// - Output: false
///
/// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
///
/// Constraints:
///
/// -2<sup>31</sup> <= x <= 2<sup>31</sup> - 1
///  
/// Follow up: Could you solve it without converting the integer to a string?
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    if x < 10 {
        return true;
    }
    let mut y = x;
    let nb_dgts = nb_digits(x);
    let mut dgts = Vec::with_capacity(nb_dgts);
    for _ in 0..nb_dgts {
        let low = y % 10;
        dgts.push(low);
        y = (y - low) / 10;
    }
    for i in 0..nb_dgts / 2 {
        if dgts[i] != dgts[nb_dgts - 1 - i] {
            return false;
        }
    }
    true
}

fn nb_digits(n: i32) -> usize {
    ((n as f32).log10().floor() + 1.) as usize
}
pub fn _is_palindrome_with_string(x: i32) -> bool {
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
    fn test_palindrome() {
        assert!(is_palindrome(121));
        assert!(!is_palindrome(-121));
        assert!(is_palindrome(1001));
        assert!(!is_palindrome(2022));
        assert!(!is_palindrome(10));
        assert!(is_palindrome(0));
    }
}
