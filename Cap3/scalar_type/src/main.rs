fn main() {
    //tipo escalar representa un valor simple
    //integers, floating-point, booleans, characters

    //tipo isize / usize depende de la arquitectura donde el programa funcione 32/64 bits
    let int_value: u128 = 9_000;
    let x = 2.43; //float 64
    let y: f32 = 3.21; //float 32
    let t = true;
    let f: bool = false;
    let c = 'R';

    //Operaciones
    let sum = 1 + 1;
    let diff = 2 - 5;
    let product = 3 * 4;
    let quotient = 10 / 5;
    let remainder = 22.2 % 3.45; 
    println!("Print sobrante: {}", remainder);
    //Vamos a tener multiples warnings por que variables no estan siendo utilizadas    
}
