fn show_stack_vs_heap_strings() {
    let s_stack = "hello";
    println!("Greetings from a string on the stack {s_stack}");

    let mut s = String::from("hey");

    s.push_str(" world");

    println!("My string is {}", s);
}

fn show_move() {
    // moving variables on stack will copy
    let x = 5;
    let y = x;

    println!("X is {x} and y is {y}");

    let s1 = String::from("Hello");
    println!("S1 pre-move is {s1}");
    let s2 = s1;

    //Does not compile because s2 has been moved from s1. s1 no longer valid  println!("S1 is {s1}, and s2 is {s2}");
    println!("S2 has been moved from S1, S2 is {s2}");

    let s3 = s2.clone();
    println!("S2 has been clonsed to s3 and we can access both, look... s2: {s2} and s3: {s3}");
}

fn main() {
    show_stack_vs_heap_strings();
    show_move();
}
