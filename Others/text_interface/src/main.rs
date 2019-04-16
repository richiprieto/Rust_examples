/*
Using a hash map and vectors, create a text interface to allow a user to add
employee names to a department in a company. For example, “Add Sally to
Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people
in a department or all people in the company by department, sorted
alphabetically.
*/
use std::io;

fn main() {
    loop {
        intro(); // Introduccion del software
        let input_value = input_text(); // Valida que el valor sea un numero
        match input_value { // Validamos las opciones
            3 => break, // Salir con valor 3
            _ => println!("valor {}", input_value),
        };
    }
}

fn intro(){
    println!("Por favor ingrese la opcion:");
    println!("1) Agregar un empleado");
    println!("2) Consultar empleados");
    println!("3) Salir");
}

fn input_text() -> u8 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Falla a leer stdin");

    let trimmed = input_text.trim();

    match trimmed.parse::<u8>(){
        Ok(i) => return i,
        Err(..) => {
            println!("No es un numero");
            return 3;
        },
    };
}
