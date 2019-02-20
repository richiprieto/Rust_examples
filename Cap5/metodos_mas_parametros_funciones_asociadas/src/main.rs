#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    //pasamos otra variable como parametro rect1 es &self
    //mientras que other: &Rectangle toma los valores de rect2 y rect3
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    //tambien es permitido bloques impl que no toman self
    //como parametro y se les conoce comu funicones asociadas
    fn square(size: u32) -> Rectangle{
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50};
    let rect2 = Rectangle { width: 10, height: 40};
    let rect3 = Rectangle { width: 60, height: 45};

    println!("Puede rect1 contener a rect2? {} ", rect1.can_hold(&rect2));
    println!("Puede rect2 contener a rect3? {} ", rect1.can_hold(&rect3));

    //la sintaxis es impl::fn tal y como vemos a continuacion
    //de esa manera llamamos la funcion asociadas
    //retorna una nueva instancia de la estructura
    let sq = Rectangle::square(3);
    println!("Valor de sq es {:?}", sq);
}
