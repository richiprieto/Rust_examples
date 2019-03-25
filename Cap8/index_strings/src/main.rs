// Se debe recalcar que Rust no permite acceder a un caracter individual
// por referencia del index, esto debido a la forma en la que Rust almacena los strings

fn main() {
    // El siguiente codigo retorna un error
    /*
    let s1 = String::from("Hola");
    let h = s1[0];
    */

    // Slicing Strings
    // Para el ejemplo tenemos dos string uno con utf-8 que es español
    // Mientras que el ruso que utiliza cyrillic
    // un char para utf es 1 byte de almacenamiento
    // un char para utf es 2 byte de almacenamiento
    let ruso = "Здравствуйте";
    let espanol = "hola";
    let r = &ruso[0..4]; // slice dos char
    let e = &espanol[0..2]; // slice dos car
    println!("{}",r);
    println!("{}",e);

    // Metodos para iterar sobre Strings
    // si imprimimos el string caracter a caracter observamos
    // que tiene 4 chars debido a que el ultimo es un diacritico
    println!("Iterar chars");
    for c in "नमते".chars() {
        println!("{}", c);
    }

    // Otra manera de imprimir es mediante bytes
    println!("Iterar bytes");
    for c in "नमते".bytes() {
        println!("{}", c);
    }

    // El metodo grapheme clusters no se encuentra en la libreria estandar
    // pero se encuentra en crates.io 
}
