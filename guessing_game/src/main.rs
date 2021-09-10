fn random_int() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    seed ^= seed << 21;
    seed ^= seed >> 35;
    seed ^= seed << 4;
    seed
}

fn main() {
    let hidden_number = (random_int() % 100 + 1) as u8;

    println!("Hi! Try to guess the number:");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: u8 = input.trim().parse().unwrap();

        if hidden_number == input {
            println!("Congratulations! The answer is {}.", hidden_number);
            break;
        }
        println!(
            "Hidden number is {}, try again:",
            if hidden_number > input { "greater" } else { "less" }
        );
    }
}
