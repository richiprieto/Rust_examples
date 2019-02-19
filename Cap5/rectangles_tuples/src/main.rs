fn main() {
    /*
    let width1 = 30;
    let height1 = 80;

    println!("El area es {} ", area(width1, height1));
    */

    //refactorizando
    let rect1 = (30, 50);
    println!("El area del rectangulo es: {}", area(rect1));
}
// Funcion normal de codigo para calcular el área

/*
fn area(width: u32, height1: u32) -> u32{
    width * height1
}
*/

//refactorizando con tuplas
fn area(dimensiones: (u32, u32)) -> u32{
    dimensiones.0 * dimensiones.1
}
