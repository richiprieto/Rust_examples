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

    // Accediendo a valores en hash maps

    let nombre_color = String::from("Azul");
    let score = scores.get(&nombre_color);
    println!("{:?}", score); // devuelve Some como respuesta

    // Otra forma es mediante iteracion
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Sobre escribiendo un valor en hash map
    let mut puntaje = HashMap::new();

    puntaje.insert(String::from("Azul"), 10);
    puntaje.insert(String::from("Azul"), 667); // sobre escribimos el valor
    println!("{:?}", puntaje);

    // Insertar un valor si el key no tiene valor
    let mut scores = HashMap::new();
    scores.insert(String::from("Azul"), 10);

    scores.entry(String::from("Azul")).or_insert(500);
    scores.entry(String::from("Amarillo")).or_insert(100);

    println!("{:?}", scores);
    // El key entry inserta el parametro si la key no existe
    // caso contrario el valor no varia

    // Actualizar un valor basado en el antiguo valor
    let text = "hola mundo maravilloso mundo";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    // El metodo or_insert retorna una referencia mutable &mut V al valor de esa key.
    // Almacenams esa referencia mutable en la variable count en orden de asignacion de ese valor
    // pero primero debemos derefenciar count usando (*)
}
