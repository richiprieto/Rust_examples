fn main() {
    // Uso del break para romper el loop
    let mut counter = 0;
    let result = loop {
	counter += 1;
	
	if counter == 10 {
	    break counter * 2;
	}
    };

    // While mientras una conedicion es True
    assert_eq!(result, 20);
    println!("El resultado es: {}", result);

    let mut number = 3;
    while number != 0 {
	println!("{}!", number);
	number = number -1;
    }
    println!("Culmino!");

    // For, loop sobre elemento de una coleccion
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
	println!("El valor es: {}", element);
    }

    for number in (1..4).rev() {
	println!("{}!", number);
    }
    println!("Termina!");
}
