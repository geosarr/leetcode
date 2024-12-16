/// Pascal Triangle.
pub fn generate_pascal(n: i32) -> Vec<Vec<i32>> {
    if n == 1 {
        return vec![vec![1]];
    }
    if n == 2 {
        return vec![vec![1], vec![1, 1]];
    }
    let mut new = vec![0; n as usize];
    new[0] = 1;
    new[n as usize - 1] = 1;
    let mut old = generate_pascal(n - 1);
    for k in 1..(n as usize - 1) {
        new[k] = old.last().as_ref().unwrap()[k] + old.last().as_ref().unwrap()[k - 1];
    }
    old.extend_from_slice(&[new]);
    return old;
}
