fn show_integer_types() {
    let integer_8_bit: i8 = 127;
    let unsigned_8_bit: u8 = 254;

    let integer_16_bit: i16 = 32766;
    let unsigned_16_bit: u16 = 65534;

    let integer_32_bit: i32 = 2147483646;
    let unsigned_32_bit: u32 = 4294967295;

    let integer_64_bit: i64 = 9223372036854775807;
    let unsigned_64_bit: u64 = 18446744073709551615;

    let integer_128_bit: i128 = 170141183460469231731687303715884105727;
    let unsigned_128_bit: u128 = 340282366920938463463374607431768211455;

    let integer_arch_bit: isize = 9223372036854775807;
    let unsigned_arch_bit: usize = 18446744073709551615;

    let hex_literal: u64 = 0xDEADBEEF;
    let octal_literal_but_octal_sucks = 0o77;
    let binary_literal = 0b1111_0101;
    let byte_literal = b'A';

    println!("My 8 bits are {integer_8_bit} and {unsigned_8_bit}");
    println!("My 16 bits are {integer_16_bit} and {unsigned_16_bit}");
    println!("My 32 bits are {integer_32_bit} and {unsigned_32_bit}");
    println!("My 64 bits are {integer_64_bit} and {unsigned_64_bit}");
    println!("My 128 bits are {integer_128_bit} and {unsigned_128_bit}");
    println!("My arch bits are {integer_arch_bit} and {unsigned_arch_bit}");

    println!("My hex literal is {hex_literal}");
    println!("My octal literal is {octal_literal_but_octal_sucks}");
    println!("My binary literal is {binary_literal}");
    println!("My byte is {byte_literal}");
}

fn show_floating_point_types() {
    let x = 2.0; // f64
    let y: f32 = 4.0;

    println!("Default float size is 64 bit: {x}");
    println!("Set y to 32 bit: {y}");
}

fn show_bool_types() {
    let t = true;
    let f: bool = false;

    println!("T = {t}");
    println!("F = {f}");
}

fn show_char_types() {
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c is {c}");
    println!("z is {z}");
    println!("cat is {heart_eyed_cat}");
}

fn show_tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("X tup is {x}");
    println!("Y tup is {y}");
    println!("Z tup is {z}");

    let implicit_typed_tup = (300, 'a', true);
    let (x, y, z) = implicit_typed_tup;

    println!("X from implicit typed tup is {x}");
    println!("Y from implicit typed tup is {y}");
    println!("Z from implicit typed tup is {z}");

    let x = tup.0;
    println!("X directly from first tupl is {x}");
}

fn show_arrays() {
    let a = [1, 5, 1, 6, 1];
    let b: [i64; 3] = [7, 1, 3];
    let all_fives = [5; 8];

    let x = a[0];
    let y = b[2];
    let z = all_fives[1];

    println!("X from array a is {x}");
    println!("Y from 64 bit array b is {y}");
    println!("Z from an array containing the value 5 8 times is {z}");
}

fn main() {
    show_integer_types();
    show_floating_point_types();
    show_bool_types();
    show_char_types();
    show_tuples();
    show_arrays();
}
