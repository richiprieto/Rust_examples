//el guion bajo "_" es un caso especial que se refiere
//a todos los casos que no han sido vistos en el match

fn main() {
    let some_u8_value = 3u8; // para probar cambiar el valor de x en xu8
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        7 => println!("seven"),
        _ => println!("Ni idea no esta en el caso"),
    }
}
