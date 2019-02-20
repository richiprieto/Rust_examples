#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
    Etc,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("25 ctvs del estado {:?}", state);
            25
        },
    }
}

fn main() {
    //LLamamos a la funcion
    let valor = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("El valor es: {}", valor);
}
