//Comparar diferentes variantes de Option<T> con
//match
// La opcion match es exahustive por lo que siempre
//debe cubrirse el caso de None, evitando que se 
//tenga errores con el manejo de null
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("La respuesta de plus_one(five) es {:?}", six);
    println!("y la resp de plus_one(None) es {:?}", none);
}
