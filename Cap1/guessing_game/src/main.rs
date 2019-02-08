use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Escoja un numero!");
    let secret_number = rand::thread_rng().gen_range(1, 101);//genera num random entre 1-100
    println!("El numero secreto es: {}", secret_number); //para el juego comentar esta linea

    loop{ //genera un loop
    	println!("Ingrese su eleccion");
    	let mut guess = String::new(); //inicia la variable tipo string
    	io::stdin().read_line(&mut guess).expect("Fallo al leer la linea"); //Lee el std input
    	
	let guess: u32 = match guess.trim().parse(){ //convierte el String a u32 
	    Ok(num) => num,
	    Err(_) => continue, //maneja entradas erroneas para que no cierre el programa
		                // continue nos lleva a la siguiente iteracion del loop
	};

    	println!("Escogiste: {}", guess);

    	match guess.cmp(&secret_number){ //compara variable guess con secret_number
	    Ordering::Less => println!("Muy pequeño"), //diferente opciones de resultados
	    Ordering::Greater => println!("Muy grande"),
	    Ordering::Equal => {
		println!("Ganas");
		break; //Cierra el programa si ganas
	    }
	}
    }
}
