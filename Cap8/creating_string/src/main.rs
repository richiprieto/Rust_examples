fn main() {

    // Se utiliza el metodo to_string para crear un string a partir
    // de un string literal, adicionalmente se puede utilizar la funcion
    // String::from

    // Creamos un string s
    let mut s = String::new();
    // asignamos el string a la variable data
    let data = "Contenido inicial1";
    // asignamos la variable data a s
    s = data.to_string();
    println!("Contenido: {}", s);
    // Otra asignacion es pasar el string inmediatamente
    s = "contenido inicial2".to_string();
    println!("Contenido: {}", s);
    // Generar el string directamente en s
    s = String::from("Contenido Inicial3");
    println!("Contenido: {}", s);

    // Recordar que los strings son encoded a UTF-8
    // por lo que puede manejar diferentes lenguajes strings
    let hola = String::from(" ‫عليكم‬ ‫السلام‬ ");
    let hola = String::from("Dobrý den");
    let hola = String::from("Hello");
    let hola = String::from("‫שָׁלוֹם‬ ");
    let hola = String::from("नम�ते ");
    let hola = String::from("こんにちは");
    let hola = String::from("你好");
    let hola = String::from("Olá");
    let hola = String::from("Здравствуйте");
    let hola = String::from("Hola");
}
