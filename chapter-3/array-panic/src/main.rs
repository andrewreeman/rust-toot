use std::io;

fn main() {
    let a = [1, 6, 1, 6];
    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Failed to parse index as a number");

    let element = a[index];

    print!("The value of element at index {index} is {element}");
}
