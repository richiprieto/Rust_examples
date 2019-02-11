fn main() {
    const MAX_POINTS: u32 = 100_000_000; //constantes jamas puede ser mutables y se puede agregar
                                         //guiones bajos para mejorar la lectura en literales numericos
         				 //lo nombres de constantes siempre seran en mayusculas
    println!("El valor de MAX_POINTS es: {}", MAX_POINTS);
    let mut x = 5;
    println!("El valor de x es: {}", x);
    x = 6;
    println!("El valor de x es: {}", x);
}
