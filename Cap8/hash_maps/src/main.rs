/*
Un hash map es de tipo HashMap<K, V>
mapea palabras claves tipo K a valores tipo V
*/
fn main() {
    // Se puede crear un hash map con new y agregar elementos con insert
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Azul"), 10);
    scores.insert(String::from("Rojo"), 20);

    println!("{:?}", scores);

    // Otro metodo para construir un hashmap es usando el metodo collect
    // en un vector de tuplas, donde cada tupla contiene su key y valor
    let teams = vec![String::from("Azul"), String::from("Rojo")];
    let initial_scores = vec![30, 40];

    // la anotacion HashMap<_, _> es necesario porque es posible collect diferentes estructuras de
    // de datos, usando los guiones bajos Rust puede inferir los tipos que el hashmap contiene
    // basado en los tipos de datos de los vectores
    let scores: HashMap<_, _> =
    teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);

    // Hashmaps y Ownership
    let nombre = String::from("Color favorito");
    let valor = String::from("Negro");

    let mut map = HashMap::new();
    map.insert(nombre, valor);
    // en este punto las variables nombre y valor han sido movido al hashmap mediante insert
    // si insertamos valores por referencia al hashmap los valores no seran movidos en propiedad
    // dentro del hashmap
    
}
