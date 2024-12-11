/// Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
///
/// Example 1:
///
/// - Input: haystack = "sadbutsad", needle = "sad"
/// - Output: 0
///
/// Explanation: "sad" occurs at index 0 and 6.
///
/// The first occurrence is at index 0, so we return 0.
///
/// Example 2:
///
/// - Input: haystack = "leetcode", needle = "leeto"
/// - Output: -1
///
/// Explanation: "leeto" did not occur in "leetcode", so we return -1.
///  
/// Constraints:
///
/// 1 <= haystack.length, needle.length <= 104
/// haystack and needle consist of only lowercase English characters.
pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack = haystack.chars().collect::<Vec<_>>();
    let needle = needle.chars().collect::<Vec<_>>();
    let (len_h, len_n) = (haystack.len(), needle.len());
    if (len_h < len_n) | needle.is_empty() {
        return -1;
    }
    for (pos, h) in haystack.iter().enumerate() {
        if *h == needle[0] {
            let sub_h = &haystack[pos..std::cmp::min(len_h, pos + len_n)];
            if sub_h.len() == len_n {
                let mut are_equal = true;
                for k in 1..len_n {
                    if needle[k] != sub_h[k] {
                        are_equal = false;
                        break;
                    }
                }
                if are_equal {
                    return pos as i32;
                }
            }
        }
    }
    return -1;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_str_str() {
        assert_eq!(str_str("sadbutsad".to_owned(), "sad".to_owned()), 0);
        assert_eq!(str_str("leetcode".to_owned(), "leeto".to_owned()), -1);
    }
}
