struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 40, height: 50};

    println!("El area del rectangulo es {}", area(&rect1));
}

//Se usa & debido a que se desea mantener el ownership de
//la funcion 
fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}
