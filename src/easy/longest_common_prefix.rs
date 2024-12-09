/// Write a function to find the longest common prefix string amongst an array of strings.
///
/// If there is no common prefix, return an empty string "".
///
/// Example 1:
///
/// - Input: strs = ["flower","flow","flight"]
/// - Output: "fl"
///
/// Example 2:
///
/// - Input: strs = ["dog","racecar","car"]
/// - Output: ""
///
/// Explanation: There is no common prefix among the input strings.
///  
/// Constraints:
///
/// - 1 <= strs.length <= 200
/// - 0 <= strs[i].length <= 200
/// - strs[i] consists of only lowercase English letters.
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let m = strs
        .iter()
        .map(|string| string.chars().collect::<Vec<_>>().len())
        .min()
        .unwrap();
    let mut res = String::with_capacity(m);
    for k in 0..m {
        // Safe to unwrap since m is the min length of all strings
        let first = strs[0].clone();
        let kth = first.chars().nth(k).unwrap();
        if strs.iter().any(|s| s.chars().nth(k).unwrap() != kth) {
            break;
        } else {
            res.push(kth);
        }
    }
    return res;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_owned(),
                "flow".to_owned(),
                "flight".to_owned(),
            ]),
            "fl".to_owned()
        );
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_owned(),
                "racecar".to_owned(),
                "car".to_owned()
            ]),
            "".to_owned()
        );
    }
}
