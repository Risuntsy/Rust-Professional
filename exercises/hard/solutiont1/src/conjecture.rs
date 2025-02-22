pub fn goldbach_conjecture() -> String {
    find_first_two_exceptions()
}


fn is_prime(n: u32) -> bool {
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

fn can_be_expressed(n: u32) -> bool {
    for i in 1.. {
        let two_times_square = 2 * i * i;
        if two_times_square >= n {
            break;
        }
        let potential_prime = n - two_times_square;
        if is_prime(potential_prime) {
            return true;
        }
    }
    false
}

fn find_first_two_exceptions() -> String {
    let mut exceptions_found = 0;
    let mut result = String::new();
    let mut current_num = 9;

    while exceptions_found < 2 {
        if current_num % 2 != 0 && !is_prime(current_num) {
            if !can_be_expressed(current_num) {
                if exceptions_found > 0 {
                  result.push_str(",");
                }
                result.push_str(&current_num.to_string());
                exceptions_found += 1;
            }
        }
        current_num += 2;
    }

    result
}
