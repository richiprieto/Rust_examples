fn main() {
    //let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
}

// El siguiente codigo da un error debido a que no se tiene
// un valor borrowed debido a que al ser dropped se elimina
// de la memoria

/*
fn dangle() -> &String {
    let s = String::from("HOLA");
    &s
}
*/

//en este caso no existe problema por que el ownership
// se mueve al return fuera de la funcion
fn no_dangle() -> String {
    let s = String::from("HOLA");
    s
}
