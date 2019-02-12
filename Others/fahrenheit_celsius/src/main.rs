use std::io;

fn main() {
    loop{
	println!("Ingrese el valor en grados Fahrenheit");
    	let mut fahr = String::new();
    	io::stdin().read_line(&mut fahr).expect("Fallo al leer la linea");

        if fahr.trim() == "salir" { // trim() elimina el newline char al final
	    println!("Saliendo...");
	    break;
	}

	let fahr: f64 = match fahr.trim().parse(){
	    Ok(num) => num,
	    Err(_) => continue,
	};
	let cel: f64 = (fahr - 32.0) / (1.8);
	println!("Celcius {}", cel);
   }
}
