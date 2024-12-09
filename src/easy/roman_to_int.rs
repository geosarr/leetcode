/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
/// _________________________________
/// |    Symbol    |     Value      |
/// |              |                |
/// |      I       |       1        |
/// |              |                |
/// |      V       |       5        |
/// |              |                |
/// |      X       |       10       |
/// |              |                |
/// |      L       |       50       |
/// |              |                |
/// |      C       |       100      |
/// |              |                |
/// |      D       |       500      |
/// |              |                |
/// |      M       |       1000     |
///
///
/// For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
///
/// Roman numerals are usually written largest to smallest from left to right.
///
/// However, the numeral for four is not IIII. Instead, the number four is written as IV.
///
/// Because the one is before the five we subtract it making four.
///
/// The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
///
/// - I can be placed before V (5) and X (10) to make 4 and 9.
/// - X can be placed before L (50) and C (100) to make 40 and 90.
/// - C can be placed before D (500) and M (1000) to make 400 and 900.
/// - Given a roman numeral, convert it to an integer.
///
/// Example 1:
///
/// - Input: s = "III"
/// - Output: 3
///
/// Explanation: III = 3.
///
/// Example 2:
///
/// - Input: s = "LVIII"
/// - Output: 58
///
/// Explanation: L = 50, V= 5, III = 3.
///
/// Example 3:
///
/// - Input: s = "MCMXCIV"
/// - Output: 1994
///
/// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
///
/// Constraints:
///
/// - 1 <= s.length <= 15
/// - s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
///
/// It is guaranteed that s is a valid roman numeral in the range [1, 3999].
pub fn roman_to_int(s: String) -> i32 {
    let numerals = s.chars().collect::<Vec<char>>();
    if numerals.is_empty() {
        panic!("Empty string");
    }
    if ROMAN_SYMBOLS.contains(&numerals[0]) && numerals.len() == 1 {
        return symbol_to_int(numerals[0]);
    }
    let mut res = 0;
    let mut i = 0;
    while i < numerals.len() - 1 {
        let current = symbol_to_int(numerals[i]);
        let next = symbol_to_int(numerals[i + 1]);
        if current >= next {
            res += current;
            i += 1;
        } else {
            res += next - current;
            i += 2;
        }
    }
    if i == numerals.len() - 1 {
        res += symbol_to_int(numerals[i]);
    }
    println!("{}", res);
    return res;
}
const ROMAN_SYMBOLS: [char; 7] = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
pub fn symbol_to_int(s: char) -> i32 {
    if s == 'I' {
        1
    } else if s == 'V' {
        5
    } else if s == 'X' {
        10
    } else if s == 'L' {
        50
    } else if s == 'C' {
        100
    } else if s == 'D' {
        500
    } else if s == 'M' {
        1000
    } else {
        panic!("Not implemented")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int("I".to_owned()), 1);
        assert_eq!(roman_to_int("V".to_owned()), 5);
        assert_eq!(roman_to_int("X".to_owned()), 10);
        assert_eq!(roman_to_int("L".to_owned()), 50);
        assert_eq!(roman_to_int("C".to_owned()), 100);
        assert_eq!(roman_to_int("D".to_owned()), 500);
        assert_eq!(roman_to_int("M".to_owned()), 1000);
        assert_eq!(roman_to_int("III".to_owned()), 3);
        assert_eq!(roman_to_int("LVIII".to_owned()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_owned()), 1994);
    }
}
