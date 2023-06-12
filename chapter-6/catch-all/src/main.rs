use rand::prelude::*;

fn main() {
    for _ in 1..10 {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1..=10);
        dice_roll(value);
    }
}

fn dice_roll(value: u8) {
    match value {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        2 => move_spaces(2),
        _ => (),
    }
}

fn add_fancy_hat() {
    println!("We're giving you a fancy hat to wear");
}

fn remove_fancy_hat() {
    println!("Sorry, you lost your fancy hat");
}

fn move_spaces(spaces: u8) {
    println!("You are moving {spaces} spaces");
}
