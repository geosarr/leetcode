/// Alice and Bob take turns playing a game, with Alice starting first.
///
/// Initially, there is a number n on the chalkboard.
///
/// On each player's turn, that player makes a move consisting of:
/// - Choosing any x with 0 < x < n and n % x == 0.
/// - Replacing the number n on the chalkboard with n - x.
/// - Also, if a player cannot make a move, they lose the game.
///
/// Return true if and only if Alice wins the game, assuming both players play optimally.
///
/// Example 1:
/// - Input: n = 2
/// - Output: true
/// - Explanation: Alice chooses 1, and Bob has no more moves.
///
/// Example 2:
/// - Input: n = 3
/// - Output: false
/// - Explanation: Alice chooses 1, Bob chooses 1, and Alice has no more moves.
///
/// Constraints:
/// - 1 <= n <= 1000
pub fn divisor_game(n: i32) -> bool {
    let mut h = std::collections::HashMap::<i32, bool>::new();
    h.insert(1, false);
    h.insert(2, true);
    h.insert(3, false);
    _divisor_game(n, h).1
}

pub fn _divisor_game(
    n: i32,
    mut h: std::collections::HashMap<i32, bool>,
) -> (std::collections::HashMap<i32, bool>, bool) {
    if h.contains_key(&n) {
        let res = h[&n];
        return (h, res);
    }
    let mut res = false;
    for x in 1..n {
        if n % x == 0 {
            (h, res) = _divisor_game(n - x, h);
            h.insert(n - x, res);
            if !res {
                return (h, true);
            }
        }
    }
    return (h, false);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_divisor_game() {
        assert!(!divisor_game(1));
        assert!(divisor_game(2));
        assert!(!divisor_game(3));
    }
}
