// un string puede aumentar su tama;o y cambiar su contenido
// se puede usar push_str similar a Vect<T> o usar el operador
// + o el macro format!, para concatenar multiples strings
fn main() {
    // Agregar un string a otro con push_str
    let mut s = String::from("Hola ");
    s.push_str("Mundo");
    println!("{}",s);

    // Usar string slice luego de agregar ese contenido a un string
    let mut s1 = String::from("Como");
    let s2 = " te veo";
    s1.push_str(s2);
    println!("El valor de s2 es '{}'", s2);

    // El metodo push a diferencia de push_str solo toma un caracter
    s1.push('A');
    println!("Agregando el caracter A al final: {}", s1);

    // Concaternar con el operador +
    let s1 = String::from("Hola ");
    let s2 = String::from("Mundo");
    let s3 = s1 + &s2; //s1 es movido de a s3 y no puede ser usado nuevamente
    println!("{}",s3);
    let s4 = String::from(" Bello");
    let s5 = s3 + &s2 + &s4; // Concatenando multiples strings
    println!("{}", s5);

    // Otra forma de concatenar es mediante el macro format!
    let s1 = String::from("uno");
    let s2 = String::from("dos");
    let s3 = String::from("tres");
    // esta forma es mas sencilla y no toma ownership de ningun parametro
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}
