fn main() {
    // basic_way();
    // tuple_struct_way();
    // struct_way();
    method_way();
    // left off https://doc.rust-lang.org/book/ch05-03-method-syntax.html#methods-with-more-parameters
}

// -------------------------------------------------------------
fn basic_way() {
    let width1 = 30;
    let height1 = 50;

    println!("The area is {}", area(width1, height1))
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// -------------------------------------------------------------
fn tuple_struct_way() {
    let rect1 = (30, 50);

    println!("The area is {}", area_tuple(rect1))
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// -------------------------------------------------------------
#[derive(Debug)] // add ability to print Debug output formatting
struct Rectangle {
    width: u32,
    height: u32,
}

fn struct_way() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area is {}", area_struct(&rect1));

    // can keep using the rect1 (area_struct didn't take ownership bc we used a reference)
    println!("{}", rect1.height);

    println!(
        "rect1 is {:?}", // use Debug output formatter (also {:#?} which makes it easier to read)
        rect1,
    );

    // using !dbg macro --------------------------------------------------------------
    // takes owndership and then gives it back
    // so can use it on the fly like width here
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // dbg
        height: 50,
    };

    // use ref here so dbg! doesn't take ownership here (we want to keep using rect2)
    dbg!(&rect2);
}

// use a reference to Rectangle so caller can keep using the variable
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn method_way() {
    let my_rect = Rectangle {
        width: 30,
        height: 50,
    };

    // use the area method
    let area = my_rect.area();

    println!("The area is {}", area);

    println!("my_rect has width? {}", my_rect.width());
    
}

// add method "area" on Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        // &self is short for "self: &Self"
        self.width * self.height
    }

    // method that has same name as a field (width)
    fn width(&self) -> bool {
        self.width > 0
    }
}
