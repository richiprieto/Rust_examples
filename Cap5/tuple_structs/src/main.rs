fn main() {
    //las estructuras tupla no tienen nombres
    //asociados a colos campos, solo tienen tipos
    //son utiles cuando se necesita dar un nombre
    //a la tupla y que sea diferente a otras especificas
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(1, 0, 0);
    let origin = Point(0, 0, 0);

    // se puede acceder mediante el uso de "." seguido
    // del index
    println!("black en 0 es {}", black.0)
}
