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

fn add_user(persona_departamento: &mut HashMap<String,String>) { // pasamos un hashmap mutable como parametro
    loop {
        println!("{}", "Ejemplo de ingreso: 'Add Amir to Sales' or 'Add Sally to Engineering'".green());
        println!("{}", "Para regresar escriba 'regresar'".green());

        let input_text = input_text(); // Ingreso de texto
        let v: Vec<&str> = input_text.split(' ').collect(); // Lo spliteamos y agregamos a un vector

        if v.len() == 4 && v[0].to_lowercase() == "add" && v[2].to_lowercase() == "to"{ //comparamos que cumpla con el protocolo para agregar
            persona_departamento.insert(v[1].to_string(), v[3].to_string()); // inserta data
            println!("{}", "Persona insertada".yellow());
            println!("{:?}", persona_departamento);
            break;
        } else if v[0].to_lowercase() == "regresar" { // opcion para regresar
            break;
        } else {
            println!("{}", "El texto ingresado no es valido por favor verifique".red()); // en caso que no cumpla el protocolo de agregar
        }
    }
}

fn intro_consulta(){
    println!("{}", "Escoja como observar los datos:".green());
    println!("1) Personas en departamento especifico");
    println!("2) Personas en la compañia por departamento");
    println!("3) Salir");
}

fn consulta_departamento(persona_departamento: & HashMap<String,String>){
    println!("{}", "Ingrese departamento:".green());
    let departamento = input_text();
    let mut vec = Vec::new();

    for (persona, dep) in &*persona_departamento {
        if departamento == dep.to_owned(){
            vec.push(persona);
            vec.sort_by(|a,b| a.to_lowercase().cmp(&b.to_lowercase()));
        }
    }

    if vec.len() != 0 {
        println!("Las personas que trabajan en el departamento: {} son {:?}", departamento, vec);
    } else {
        println!("El departamento {} no existe", departamento.red());
    }
}

fn consulta_compania(persona_departamento: & HashMap<String,String>) {
    let mut count_vec: Vec<_> = persona_departamento.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("{:?}", count_vec);
}

fn consulta(persona_departamento: & HashMap<String,String>){

    loop {
        intro_consulta();
        let input_number = input_number();
        match input_number { // Validamos las opciones
            1 => consulta_departamento(& persona_departamento),
            2 => consulta_compania(& persona_departamento),
            3 => break, // Salir con valor 3
            _ => println!("{}", "No es una opcion valida.".red()),
        };
    }
}

fn main() {
    let mut persona_departamento: HashMap<String, String> = HashMap::new();

    loop {
        intro(); // Introduccion del software
        let input_number = input_number(); // Valida que el valor sea un numero
        match input_number { // Validamos las opciones
            1 => add_user(&mut persona_departamento),
            2 => consulta(&persona_departamento),
            3 => break, // Salir con valor 3
            _ => println!("{}", "No es una opcion valida.".red()),
        };
    }
}
