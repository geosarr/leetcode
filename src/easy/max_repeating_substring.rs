pub fn max_repeating(sequence: String, word: String) -> i32 {
    let sequence = sequence.chars().collect::<Vec<char>>();
    let mut word = word.chars().collect::<Vec<char>>();
    if sequence.len() < word.len() {
        return 0;
    }
    let mut k = 0;
    let wd = word.clone();
    while is_substring(&sequence, &word) {
        k += 1;
        for c in &wd {
            word.push(*c);
        }
    }
    k
}

pub fn is_substring(seq: &[char], wd: &[char]) -> bool {
    if (seq.len() < wd.len()) || (wd.len() == 0) {
        return false;
    }
    let c = &wd[0];
    if wd.len() == 1 {
        return &seq[0] == c;
    }
    for (pos, s) in seq.iter().enumerate() {
        if s == c {
            if pos + 1 < seq.len() {
                // return is_substring(&seq[pos + 1..], &wd[1..]);
                return is_subsequence(&seq[pos + 1..], &wd[1..]);
            } else {
                return true;
            }
        }
    }
    return false;
}

pub fn is_subsequence(t: &[char], s: &[char]) -> bool {
    if s.is_empty() {
        return true;
    }
    if (s.len() > t.len()) | t.is_empty() {
        return false;
    }
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
mod tests {
    use super::*;
    #[test]
    fn test_max_repeating() {
        assert_eq!(1, max_repeating("ababc".to_owned(), "ba".to_owned()));
        let seq = "aaabaaaabaaabaaaabaaaabaaaabaaaaba"
            .to_owned()
            .chars()
            .collect::<Vec<_>>();
        // aaaba
        let wd = "aaabaaaabaaaabaaaabaaaabaaaaba"
            .to_owned()
            .chars()
            .collect::<Vec<_>>();
        assert!(is_subsequence(&seq, &wd));
        // assert_eq!(
        //     5,
        //     max_repeating(
        //         "aaabaaaabaaabaaaabaaaabaaaabaaaaba".to_owned(),
        //         "aaaba".to_owned()
        //     )
        // );
        let seq = "ababc".to_owned().chars().collect::<Vec<_>>();
        let wd = "ba".to_owned().chars().collect::<Vec<_>>();
        // assert!(is_substring(&seq, &wd));
    }
}
