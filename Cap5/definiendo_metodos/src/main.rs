#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{ //keyword impl significa implementacion
                //creamos el bloque de metodo
    //Debemos notar que aun se usa & para denotar quien mantiene propiedad
    fn area(&self) -> u32 { //el primer parametro siempre es self
        self.width * self.height
    }

}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50};

    println!("El area del rectangulo es {}", rect1.area());
}
