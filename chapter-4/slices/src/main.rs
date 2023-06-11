fn main() {
    let s = String::from("Superduperlong word in a sentance");
    let end_of_first_word = first_word(&s);

    println!("End of first word is {}", end_of_first_word);

    string_slice_example();

    let first_word_slice = first_word_returning_slice(&s);
    println!("First word is: {}", first_word_slice);

    let mut mut_s = String::from("Mutable strings are fun");
    let slice_from_mutable_s = first_word_returning_slice(&mut_s);

    mut_s.clear();

    println!("Slice from mutable s is {}", slice_from_mutable_s);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_returning_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn string_slice_example() {
    let s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("Hello is {}", hello);
    println!("World is {}", world);
}
