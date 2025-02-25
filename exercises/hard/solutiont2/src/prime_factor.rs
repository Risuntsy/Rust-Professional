use std::time::Instant;

pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut n = number;
    
    let mut max_factor = 0;

    while n % 2 == 0 {
        max_factor = 2;
        n /= 2;
    }

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
    
    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    let (s, d) = {
        let mut d = n - 1;
        let mut s = 0;
        while d % 2 == 0 {
            d /= 2;
            s += 1;
        }
        (s, d)
    };
    
    'outer: for &a in bases.iter() {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

// 分步模乘法防溢出
fn mod_pow(base: u128, exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = mod_mul(result, base, modulus);
        }
        exp >>= 1;
        base = mod_mul(base, base, modulus);
    }
    result
}

// 核心：将乘法分解为加法避免溢出 <button class="citation-flag" data-index="7">
fn mod_mul(mut a: u128, mut b: u128, modulus: u128) -> u128 {
    let mut result = 0;
    a %= modulus;
    b %= modulus;
    while b > 0 {
        if b % 2 == 1 {
            result = (result + a) % modulus;
        }
        a = (a << 1) % modulus;
        b >>= 1;
    }
    result
}
