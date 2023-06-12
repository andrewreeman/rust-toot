fn main() {
    let some_number = Some(5);
    let some_char = Some('c');
    let absent_number: Option<i32> = None;

    println!("Some number: {:?}", some_number);
    println!("Some char: {:?}", some_char);
    println!("No number: {:?}", absent_number);
}
