use std::io;

fn main() {
    loop {
	println!("Elija el n numero fibonacci");
   	let mut n_fibonacci = String::new();
   	io::stdin().read_line(&mut n_fibonacci).expect("Fallo al leer la linea");

	if n_fibonacci.trim() == "salir" {
	    println!("Saliendo...");
	    break;
	}
    
   	let n_fibonacci: u32 = match n_fibonacci.trim().parse(){
	    Ok(num) => num,
	    Err(_) => continue,
   	};

   	let mut a = 0;
   	let mut b = 1;
   	let mut c;
   	for _k in 0..n_fibonacci {
       	   c = b + a;
           a = b;
           b = c;
     	}
      	println!("El valor fibonacci para n {} es: {}", n_fibonacci, a);
    }
}
