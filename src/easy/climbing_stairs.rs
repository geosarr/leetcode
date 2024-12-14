/// You are climbing a staircase. It takes n steps to reach the top.
///
/// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
///
/// Example 1:
///
/// - Input: n = 2
/// - Output: 2
///
/// Explanation: There are two ways to climb to the top.
///
/// 1. 1 step + 1 step
///
/// 2. 2 steps
///
/// Example 2:
///
/// - Input: n = 3
/// - Output: 3
///
/// Explanation: There are three ways to climb to the top.
///
/// 1. 1 step + 1 step + 1 step
///
/// 2. 1 step + 2 steps
///
/// 3. 2 steps + 1 step
///  
///
/// Constraints:
///
/// - 1 <= n <= 45
pub fn climb_stairs(n: i32) -> i32 {
    let mut map = std::collections::HashMap::new();
    let kmax = n / 2 + 1;
    (0..kmax).map(|k| f(n, k, &mut map)).sum::<i32>()
}
pub fn f(n: i32, k: i32, map: &mut std::collections::HashMap<(i32, i32), i32>) -> i32 {
    // if map.contains_key(&(n, k)) {
    //     return map[&(n, k)];
    // }
    if (k == 0) || (n == 1 && k == 1) || (n == 2 * k) {
        map.insert((n, k), 1);
        return 1;
    }
    if k == 1 {
        map.insert((n, k), n - 1);
        return n - 1;
    }
    let val = (1..n - 2 * k + 2)
        .map(|j| {
            let (a, b) = (n - j - 1, k - 1);
            if map.contains_key(&(a, b)) {
                map[&(a, b)]
            } else {
                f(a, b, map)
            }
        })
        .sum::<i32>();
    map.insert((n, k), val);
    return val;
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_climb_stairs() {
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
        assert_eq!(climb_stairs(5), 8);
    }
}
