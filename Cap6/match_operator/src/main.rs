//match  es un operador que permite comparar valores
//con una serie de patrones y ejecutar el codigo
//si un patron se empareja con el valor evaluado

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => { // si se necesita mas codigo se puede utilizar {}
            println!("Centavo");
            1 //retorna este valor
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let valor = value_in_cents(Coin::Penny);
    println!("El valor es {}", valor);
}
