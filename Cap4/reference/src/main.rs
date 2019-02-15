fn main() {
    let s1 = String::from("Hola");
    //ampersand sirve para referenciar la variable
    //no le quita ownership a la variable s1
    // por lo que no es dropeada al terminar la
    //funcion
    let len = calculate_lenght(&s1);

    println!("La longitud de '{}' es {}.", s1, len);
}

//en el parametro de la funcion tambien se debe
//especificar que es una referencia
fn calculate_lenght(s1: &String) -> usize{
    s1.len()
}
