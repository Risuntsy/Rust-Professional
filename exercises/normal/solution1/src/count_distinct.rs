pub fn new_count_distinct(input_str: &str) -> usize {
    let mut parts = input_str.split(",").collect::<Vec<&str>>();

    parts.sort_unstable();

    let mut count = 0;

    let mut i = 0;
    while i < parts.len() {
        count += 1;

        while i< parts.len() - 1 && parts[i] == parts[i+1] {
            i+=1;
        }

        i+=1;
    }

    count
}
