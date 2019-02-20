// se puede combinar if y let para evaluar un patron e ignorar el resto
// Necesita mucha menos identacion y codigo
// pero pierde el exahustive checking que match provee

fn main() {
    let some_u8_value = Some(3u8);

    /*
    match some_u8_value{
        Some(3) => println!("Tres"),
        _ => (),
    }
    */

    //el bloque anterior puede ser reemplazado por
    if let Some(3) = some_u8_value {
        println!("Tres");
    }

    //se puede utilizar con else en expresiones similares al
    //siguiente bloque
    // Tomado del ejemplo de las monedas suma al contador
    // si la moneda no es Quarter.
    
    /*
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("El estado de la moneda de 25 es: {:?}", state);
    } else {
        count += 1;
    }
    */
}
