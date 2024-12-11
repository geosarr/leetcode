use std::collections::HashMap;

/// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
///
/// An input string is valid if:
/// - Open brackets must be closed by the same type of brackets.
/// - Open brackets must be closed in the correct order.
/// - Every close bracket has a corresponding open bracket of the same type.
///  
///
/// Example 1:
/// - Input: s = "()"
/// - Output: true
///
/// Example 2:
/// - Input: s = "()[]{}"
/// - Output: true
///
/// Example 3:
/// - Input: s = "(]"
/// - Output: false
///
/// Example 4:
/// - Input: s = "([])"
/// - Output: true
///
/// Constraints:
/// - 1 <= s.length <= 104
/// - s consists of parentheses only '()[]{}'.
pub fn is_valid(s: String) -> bool {
    let strings = s.chars().collect::<Vec<char>>();
    if strings.len() <= 1 {
        return false;
    }
    if is_close(strings[0]) {
        return false;
    }
    let mut stack = Vec::with_capacity(strings.len());
    stack.push(strings[0]);
    for (i, s) in strings[1..].iter().enumerate() {
        // println!("{s}")
        if is_close(*s) {
            let last_open = stack.pop();
            if let Some(c) = last_open {
                if open(*s) != c {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            stack.push(*s);
        }
    }
    if !stack.is_empty() {
        return false;
    }
    true
}

pub fn is_close(c: char) -> bool {
    return "])}".contains(c);
}
pub fn open(c: char) -> char {
    if c == ']' {
        return '[';
    }
    if c == '}' {
        return '{';
    } else {
        return '(';
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_longest_common_prefix() {
        assert!(is_valid("()".to_string()));
        assert!(is_valid("()[]{}".to_string()));
        assert!(!is_valid("(]".to_string()));
        assert!(is_valid("([])".to_string()));
        assert!(!is_valid("([)]".to_string()));
    }
}
