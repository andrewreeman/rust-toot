fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("String created in s2");
    let s3 = takes_and_gives_back_ownership(s2);

    println!("S1 is {s1}");

    //Below will not work as s2 has been move into takes_and_gives_back_ownership function
    //println!("S2 is {s2}");

    println!("S3 is {s3}");
}

fn gives_ownership() -> String {
    let some_string = String::from("A string");
    some_string
}

fn takes_and_gives_back_ownership(a_string: String) -> String {
    a_string
}
