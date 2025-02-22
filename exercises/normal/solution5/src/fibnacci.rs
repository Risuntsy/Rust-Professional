pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    if threshold <=1 {
        return threshold;
    }

    std::iter::successors(Some((0, 1)), |&(a, b)| {
        let next_fib = a + b;
        if next_fib < threshold {
            Some((b, next_fib))
        } else {
            None 
        }
    })
    .map(|(_, b)| b) 
    .filter(|&fib| fib % 2 != 0) 
    .sum() 
}
