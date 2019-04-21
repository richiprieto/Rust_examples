/*
Using a hash map and vectors, create a text interface to allow a user to add
employee names to a department in a company. For example, “Add Sally to
Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people
in a department or all people in the company by department, sorted
alphabetically.
*/

use std::collections::HashMap;
extern crate colored;
use colored::*;

fn intro(){
    println!("{}", "Por favor ingrese la opcion:".green());
    println!("1) Agregar un empleado");
    println!("2) Consultar empleados");
    println!("3) Salir");
}

fn input_number() -> u8 {
    use std::io;
    // variable tipo string
    let mut input_text = String::new();
    // ingreso el valor
    io::stdin()
        .read_line(&mut input_text)
        .expect("Falla a leer stdin");

    let trimmed = input_text.trim();

    match trimmed.parse::<u8>(){
        Ok(i) => return i,
        Err(..) => {
            println!("{}", "No es un numero, Saliendo...".red());
            return 3;
        },
    };
}

fn input_text() -> String {
    use std::io;
    // variable tipo string
    let mut input_text = String::new();
    // ingreso el valor
    io::stdin()
        .read_line(&mut input_text)
        .expect("Falla a leer stdin");

    let trimmed = input_text.trim();

    trimmed.to_string()
}

fn add_user() {

    let mut book_reviews = HashMap<&str, &str>::new();
    loop {
        println!("{}", "Ejemplo de Ingreso: Add Amir to Sales or Add Sally to Engineering".green());
        let input_text = input_text(); // Ingreso de texto
        let v: Vec<&str> = input_text.split(' ').collect(); // Lo spliteamos y agregamos a un vector
        if v[0].to_lowercase() == "add" && v[2].to_lowercase() == "to"{
            println!("simon");
        }
        break;
    }
}

fn main() {
    loop {
        intro(); // Introduccion del software
        let input_number = input_number(); // Valida que el valor sea un numero
        match input_number { // Validamos las opciones
            1 => add_user(),
            3 => break, // Salir con valor 3
            _ => println!("No es una opcion valida."),
        };
    }
}
