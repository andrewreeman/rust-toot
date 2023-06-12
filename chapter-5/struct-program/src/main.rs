#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 20;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );

    let rect1 = (30, 20);

    println!(
        "The area of the rectangle using tuples is {} square pixels",
        area_tuples(rect1)
    );

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    println!(
        "The area of the rectangle using structs is {} square pixels",
        area_struct(&rect2)
    );

    println!("My rect is {:?}", rect2);
    println!("My pretty rect is {:#?}", rect2);

    // dbg statement example
    let rect1 = Rectangle {
        width: dbg!(30 * 2),
        height: 50,
    };

    dbg!(&rect1);

    println!(
        "And we can use our area method instead of the funtions: {}",
        rect1.area()
    );

    println!(
        "Method call syntax is short for (&rect1).area() like so {}",
        (&rect1).area()
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
