fn main() {
    //Funciona igual que en otros lenguajes
    //tanto x como y tienen el valor de 5 debido
    //a que son tipos fijos y trabajan directamente
    //en el stack
    let x = 5;
    let y = x;

    //estamos creando un string de tama;o dinamico
    //que sera colocado en el heap de memoria
    //cuando se asigna el valor de s1 a s2
    //automaticamente s2 toma los valores
    //del puntero longitud y capacidad, más no
    //hace una copia de la variable en el heap
    //por otra parte s1 es invalidado como variable
    //para interactuar y sirve para evitar errores como double
    //free memory que puede ser causa potencial de
    //fallos de seguridad

    let s1 = String::from("Hola");
    let s2 = s1;

    //println!("{}, Mundo", s1); // obtenemos un error
    println!("{}, Mundo", s2); //correcto
}
