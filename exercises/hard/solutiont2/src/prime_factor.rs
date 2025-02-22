pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut n = number;
    
    let mut max_factor = 0;

    // Remove all factors of 2
    while n % 2 == 0 {
        max_factor = 2;
        n /= 2;
    }

    // Check for odd factors from 3 to sqrt(n)
    let mut factor = 3;
    while factor * factor <= n {
        while n % factor == 0 {
            max_factor = factor;
            n /= factor;
            if is_prime(n) {
                return n;
            }
        }
        factor += 2;
    }

    if n > 2 {
        max_factor = n;
    }

    max_factor
}

fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
