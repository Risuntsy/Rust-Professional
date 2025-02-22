pub fn new_birthday_probability(n: u32) -> f64 {
    if n > 365 {
        return 1_f64;
    }

    if n < 2 {
        return 0_f64;
    }

    let mut p = 1_f64;

    for i in 0..n {
        p *= (365 - i) as f64 / 365_f64;
    }

    ((1_f64 - p) * 10000.0).round() / 10000.0
}
