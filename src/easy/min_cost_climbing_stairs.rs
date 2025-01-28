/// You are given an integer array cost where cost[i] is the cost of i<sup>th</sup> step on a staircase.
/// Once you pay the cost, you can either climb one or two steps.
///
/// You can either start from the step with index 0, or the step with index 1.
///
/// Return the minimum cost to reach the top of the floor.
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let candidates = [min_cost(&cost, 0), min_cost(&cost, 1)];
    std::cmp::min(candidates[0], candidates[1])
}

fn min_cost(cost: &[i32], source: i32) -> i32 {
    // A variation of the topological sort algorithm, on the directed acyclic graph
    // with weigthed edges defined by:
    // for i in 0..cost.len()+ 1:
    //      i -> i+1; i -> i+2;
    //      weight[i -> i+1]=weight[i -> i+2]=cost[i]
    let n = cost.len();
    let mut cost_to = vec![i32::MAX; n + 1];
    cost_to[source as usize] = 0;
    let mut flag_source = false;
    for v in 0..n {
        if (v as i32) == source {
            flag_source = true;
        }
        if flag_source {
            let neighbors = if v <= n - 2 {
                vec![v + 1, v + 2]
            } else {
                vec![n]
            };
            for u in neighbors {
                let new_cost = cost_to[v] + cost[v]; // cost[v] is the weight of the edge v -> u
                if cost_to[u] > new_cost {
                    cost_to[u] = new_cost;
                }
            }
        }
    }
    cost_to[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_climbing_stairs() {
        println!("{}", min_cost_climbing_stairs(vec![10, 15, 20]));
        println!(
            "{}",
            min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
    }
}
