fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn another_function_thingy(x: i32) {
    println!("The value of x is {x}");
}

fn main() {
    another_function_thingy(66);
    print_labeled_measurement(48, 'c');
}
