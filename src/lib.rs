pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum: u32 = 0;

    for i in factors {
        let mut j = 1;

        loop {
            if i * j < limit {
                sum += i * j;
                j += 1;
            }
            else {
                break;
            }
        }
    }
    sum
}
