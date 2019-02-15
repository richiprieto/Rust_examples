fn main() {
    let s = String::from("Hol4 Mund0");

    let hola = &s[0..4];
    let hola = &s[0..=3]; // el = indica que esta incluido
    let hola = &s[..4]; //si inicia en cero se puede evitar su colocacion

    let mundo = &s[5..10];
    let mundo = &s[5..=9];

    println!("{},{}", hola, mundo);

    //si el slice va hasta el final se puede evitar el ultimo
    //valor del index similar al incial 0
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    println!("{}", slice);

    //si se recorre el string copleto solo se coloca ..
    let total = &s[..];

    println!("{}", total);

    //funcion first_word
    let palabra = first_word(&s);
    println!("la primera palabra es: {}", palabra);

    //slice de otros tipos
    let numeros = [1, 2, 3, 4, 5];
    let slice = &numeros[1..3];
}

//el parametro &str permite manejar tipos de datos str y 'String'

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
