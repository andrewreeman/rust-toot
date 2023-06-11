fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of inner x is {x}");
    }

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of x is {x}");
    println!("The number of spaces is {spaces}");
}
