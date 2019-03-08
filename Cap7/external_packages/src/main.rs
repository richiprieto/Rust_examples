// se va a utilizar un paquete externo para generar numeros
// random para eso debemos agregar el paquete en el archivo
// Cargo.toml en la parte de dependencias
// [dependencies]
// rand = "0.5.5"
// descarga el paquete rand de https://crates.io

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("El numero es: {}", secret_number);
}
