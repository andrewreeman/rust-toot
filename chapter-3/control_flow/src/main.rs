fn if_statements() {
    let number = 3;

    // In rust we call branches 'arms'. We also use the term 'arms' when using 'match'
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Number after condition is {number}");
}

fn simple_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result value after loop is {result}");
}

fn labeled_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count is {count}");
}

fn while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFT OFF!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
}

fn for_loops() {
    let a = [10, 20, 30, 40];

    for element in a {
        println!("The value in a using fancy for loop is {element}");
    }
}

fn for_loop_using_range() {
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("TAKE OFF! Wooo");

    for number_in_inclusive_range in (1..=4).rev() {
        println!("Number in inclusive range: {number_in_inclusive_range}");
    }
}

fn main() {
    if_statements();
    simple_loop();
    labeled_loops();
    while_loops();
    for_loops();
    for_loop_using_range();
}
