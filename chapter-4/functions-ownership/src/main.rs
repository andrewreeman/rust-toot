fn main() {
    let s = String::from("Hello");

    takes_ownership(s);
    //this is an error: println!("Trying to use s after ownership has been taken: {s}");

    let x = 5;
    makes_copy(x);
    println!("Using x after a copy has been done: {x}");
}

fn takes_ownership(some_string: String) {
    println!("I now own {some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("This integer has been copied and is still accessible: {some_integer}");
}
