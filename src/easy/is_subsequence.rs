pub fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() {
        return true;
    }
    if (s.chars().count() > t.chars().count()) | t.is_empty() {
        return false;
    }
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    let (mut i, mut j) = (0, 0);
    while (i < s.len()) && (j < t.len()) {
        let k = find_index(&t[j..], s[i]);
        if k < 0 {
            return false;
        }
        j = k as usize + j + 1;
        i += 1;
    }
    if i != s.len() {
        return false;
    }
    return true;
}

fn find_index(t: &[char], a: char) -> i32 {
    for i in 0..t.len() {
        if t[i] == a {
            return i as i32;
        };
    }
    return -1;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_is_subsequence() {
        assert_eq!(
            is_subsequence("aaaaaa".to_string(), "bbaaaa".to_string()),
            false
        );
    }
}
