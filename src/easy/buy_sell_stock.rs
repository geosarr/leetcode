/// You are given an array prices where prices[i] is the price of a given stock on the ith day.
///
/// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
///
/// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.
///
/// Example 1:
///
/// - Input: prices = [7,1,5,3,6,4]
/// - Output: 5
///
/// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
///
/// Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
///
/// Example 2:
///
/// - Input: prices = [7,6,4,3,1]
/// - Output: 0
///
/// Explanation: In this case, no transactions are done and the max profit = 0.
///
/// Constraints:
///
/// - 1 <= prices.length <= 10<sup>5</sup>
/// - 0 <= prices[i] <= 10<sup>4</sup>
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 {
        return 0;
    }
    let mut max = prices[1] - prices[0];
    let mut min_price = std::cmp::min(prices[1], prices[0]);
    for i in 2..prices.len() {
        max = std::cmp::max(max, prices[i] - min_price);
        min_price = std::cmp::min(prices[i], min_price);
    }
    return std::cmp::max(0, max);
}
pub fn _naive_max_profit(prices: Vec<i32>) -> i32 {
    let mut res = 0;
    for k in 0..prices.len() {
        let sell_price = if let Some(val) = prices[k + 1..].iter().max() {
            *val
        } else {
            0
        };
        if sell_price > prices[k] {
            res = std::cmp::max(sell_price - prices[k], res);
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        println!("{}", max_profit(vec![6, 8, 1, 5, 9]))
    }
}
