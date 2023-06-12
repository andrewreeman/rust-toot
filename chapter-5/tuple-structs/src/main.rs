struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct EmptyStruct;

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black red value is {}", black.0);
    println!("Origin x value is {}", origin.0);

    let Color(r, g, b) = black;
    println!("Red value is {r}, Green value is {g} and blue value is {b}");

    let _ = EmptyStruct;
}
