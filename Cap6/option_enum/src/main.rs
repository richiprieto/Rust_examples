//Rust no tiene definido el valor null
//la opcion <T> es un parametro de tipo generico
//Por ahora <T> lo definimos como alguna variante de Option enum
//<T> puede ser de cualquier tipo

/*
enum Option<T> {
    Some(T),
    None,
}
*/

fn main() {
    // el compilador no puede diferenciar que tipo de Some
    // tenemos pero si puede saber cuando es None ya que es tipo unico
    let number = Some(5);
    let string = Some("string");
    let no_number: Option<i32> = None;

    //Cuando se deseen realizar operaciones hay que convertir
    // Option<T> a T
    //Option T tien multiples metodos para diferentes
    //situaciones Chequear la documentacion
}
