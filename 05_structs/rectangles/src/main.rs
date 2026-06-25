fn main() {
    basic_way();
    tuple_struct_way();
    struct_way();

    // left off here: https://doc.rust-lang.org/book/ch05-02-example-structs.html#adding-functionality-with-derived-traits
}

// -------------------------------------------------------------
fn basic_way() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area is {}",
        area(width1, height1)
    )
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}


// -------------------------------------------------------------
fn tuple_struct_way() {
    let rect1 = (30, 50);

    println!(
        "The area is {}",
        area_tuple(rect1)
    )
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// -------------------------------------------------------------
struct Rectangle {
    width: u32,
    height: u32,
}

fn struct_way() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area is {}",
        area_struct(&rect)
    );

    // can keep using the rect (area_struct didn't take ownership bc we used a reference)
    println!("{}", rect.height);
}

// use a reference to Rectangle so caller can keep using the variable
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
