pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|i| is_multiple_of(i, factors))
        .sum()
}

fn is_multiple_of(n: &u32, factors: &[u32]) -> bool {
    for i in factors {
        if *i != 0 && n % i == 0 {
            return true;
        }
    }
    false
}
