fn main() {
    let s1 = String::from("Hola");

    let (s2, len) = calculate_len(s1);

    println!("La longitud de '{}' es {}.", s2, len);
}

fn calculate_len(s: String) -> (String, usize){
    let length = s.len(); //retorna long del string

    (s, length) // multiples valores pueden retornar
                //mediante una tupla
}
