struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 40, height: 50};

    println!("El area del rectangulo es {}", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}
