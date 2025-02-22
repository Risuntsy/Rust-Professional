const COINS: [u32;8] = [100, 50, 30, 20, 10, 5, 2, 1];


pub fn dp_rec_mc(amount: u32) -> u32 {
    let mut amount = amount;

    let mut count = 0;

    let mut i = 0;

    while amount > 0 {
        if amount >= COINS[i] {
            amount -= COINS[i];
            count += 1;
        } else {
            i += 1;
        }
    }

    count
}
