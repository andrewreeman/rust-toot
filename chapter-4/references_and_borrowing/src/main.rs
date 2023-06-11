fn main() {
    let s1 = String::from("I am a superlong string");
    let length = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, length);

    let mut s2 = String::from("I am a string that will be changed.");
    change_mut(&mut s2);

    println!("The value of S2 is {s2}");

    let mut only_can_borrow_once = String::from("I can only be borrowed once");
    let borrow_1 = &mut only_can_borrow_once;

    //The below does not compile because we can only borrow a value once
    //let borrow_2 = &mut only_can_borrow_once;

    println!("Value of borrow 1 is {}", borrow_1);

    let mut also_only_borrowed_once = String::from("I can also only be borrwed once");
    {
        let borrow_1 = &mut also_only_borrowed_once;
        println!("Value of borrow 1 is {}", borrow_1);
    }

    let borrow_2 = &mut also_only_borrowed_once;
    println!(
        "Value of borrow 2, now borrow_1 is out of scope, is {}",
        borrow_2
    );

    borrowing_as_mutable_when_borrowed_from_immutable();

    let ownership_moved = no_dangle();
    println!(
        "no_dangle function will move ownership to caller: {}",
        ownership_moved
    );
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// The below function will compile as we cannot modify this borrowed value
//fn change(some_string: &String) {
//   some_string.push_str("I will not work");
//}
//
fn change_mut(some_string: &mut String) {
    some_string.push_str(" I will work");
}

fn borrowing_as_mutable_when_borrowed_from_immutable() {
    let mut s = String::from("String that will be borrowed");

    let r1 = &s;
    let r2 = &s;

    // Below will not compile because s is already referenced by r1 (and r2) which is immutable. So
    // r3 cannot borrow as a mutable
    //let r3 = &mut s;

    println!("Value of r1 is {r1} and r2 is {r2}");

    let r3 = &mut s;
    println!("But can I use r3 here now and r1 and r2 have gone out of scope? {r3}");
}

/* The below will not compile because we are returning a reference to a string that has now gone
 * out of scope. Maybe some magic feature called 'lifetimes' is a way around this
fn returning_a_dangling_reference() -> &String {
    let s = String::from("I will not work");
    &s
}
*/

fn no_dangle() -> String {
    let s = String::from("I do not dangle!");
    s
}
