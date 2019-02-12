fn main() {
    let number = 3;

    // simple if else condicion
    if number < 5 {
    	println!("Condicion fue True");
    } else {
	println!("Condicion fue False");
    }

    // condicion negada
    if number != 0 {
	println!("El numero no es igual a cero");
    }

    // condicion else if
    if number % 4 == 0 {
	println!("Numero divisible para 4");
    } else if number % 3 == 0 {
	println!("Numero divisible para 3");
    } else if number % 2 == 0 {
	println!("Numero divisible para 2");
    } else {
	println!("Numero divisible para otro número");
    }

    //Usando if en un statement let
    //Los tipos en el if tienen que ser iguales 
    let condition = true;
    let number = if condition {
	5
    } else {
	6
        //"six" //Error 
    };
    println!("El valor del numero es: {}", number);
}
