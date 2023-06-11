fn my_func() -> i32 {
    return 3 + 4;
}

fn my_func_2() -> i32 {
    3 + 4
}

fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");

    let result = my_func();
    println!("Result of my_func is {result}");

    let result: i32 = my_func_2();
    println!("Result of my_func_2 is {result}");
}
