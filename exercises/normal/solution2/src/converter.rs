
pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let chars = num_str.chars().collect::<Vec<_>>();

    let mut i = chars.len() - 1;

    while chars[i] != '(' {
        i -= 1;
    }
    

    let base = num_str[i + 1..num_str.len() - 1].parse::<usize>().unwrap();
    let mut b = 1;

    i-=1;

    let mut num = 0;
    loop {
        num += to_dight(chars[i]) * b;
        b *= base;

        if i == 0 {
            break;
        }
        i -= 1;
    }

    convert(num, to_base)
}

fn to_dight(c: char) -> usize {
    match c {
        '0'..='9' => c as usize - '0' as usize,
        'a'..='z' => c as usize - 'a' as usize + 10,
        _ => panic!("Invalid character"),
    }
}

fn to_char(num: u8) -> char {
    match num {
        0..=9 => (num + '0' as u8)  as char,
        10..=35 => (num -10 + 'a' as u8) as char,
        _ => panic!("Invalid number"),
    }
}

fn convert(mut num: usize, base: u32) -> String {
    let base = base as usize;
    let mut result = Vec::new();
    
    while num != 0 {
        let remainder = num % base;
        result.push(to_char(remainder as u8));
        num /= base;
    }

    result.into_iter().rev().collect()
}