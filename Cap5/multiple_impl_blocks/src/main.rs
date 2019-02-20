#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
}

//se puede contener multiples bloques impl como se puede observar
//esta utilidad se vera a mayor detalle en el capitulo 10
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50};
    let rect2 = Rectangle { width: 10, height: 40};
    let rect3 = Rectangle { width: 60, height: 45};

    println!("Puede rect1 contener a rect2? {} ", rect1.can_hold(&rect2));
    println!("Puede rect2 contener a rect3? {} ", rect1.can_hold(&rect3));

}
