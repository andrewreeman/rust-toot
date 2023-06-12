fn main() {
    config_max_using_match();
    config_max_using_if_let();
    if_let_with_else();
}

fn config_max_using_match() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The max is configured to be: {}", max),
        _ => (),
    }
}

fn config_max_using_if_let() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The max using if let is: {max}");
    }
}

fn if_let_with_else() {
    let thingy = Some(2391u32);
    if let None = thingy {
        ()
    } else {
        println!("Oooo we have a thingy");
    }
}
