fn main() {
    //shadowing es declarar una nueva variable con el mismo nombre de una variable previa
    // la primera variable es "shadowed" por la segunda
    // para realizar shadowing se usa el keyword let repetidas veces
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("El valor de x es: {}", x);

    //shadowing permite cambia el tipo de valor reusando el mismo nombre
    let chars = "abcdef"; //string type
    println!("El valor de chars es: {}", chars);
    let chars = chars.len(); //number type
    println!("El valor de chars es: {}", chars);

}
