fn main() {
    // para hacer referencias que puedan mutar
    // utilizamos mut en la variable
    let mut s = String::from("Hola");
    change(&mut s);
    println!("Mensaje: {}", s);

    let mut s1 = String::from("Saludos");
    let r1 = &mut s1;
    println!("{}", r1);
    //Las referencias mutables pueden darser solo una vez
    //si se descomentan las lineas de abajo
    //obtenemos un error de compilacion
    //esto permite evitar data races en tiempo
    //de compilacion

    /*
    let r2 = &mut s1;
    println!("{}, {}", r1, r2);
    */

    //Si por otro lado el borrowed (tomar prestado) se
    //se realiza de manera inmutable no existe problema de
    //referenciar multiples variables

    let x1 = &s;
    let x2 = &s;

    println!("{},{}", x1, x2);

    //si se deseea tener multiples referencia mutables
    //siempre se debe encerrar por {} para que realize
    //el drop al terminar la ejecucion

    {
        let y1 = &mut s;
        y1.push_str(", mutado y1");
        println!("{}", r1);
    }

    let y2 = &mut s;
    y2.push_str(", y ahora mutado y2");
    println!("{}", y2);

    //NOTA: tampoco se puede tener una referencia mutable
    //si ya tenemos una inmutable
    //el siguiente bloque nos devuelve un error en compilacion

    /*
    let z1 = &s;
    let z2 = &s;
    let z3 = &mut s;
    */

}

fn change(s: &mut String){
    s.push_str(", Mundo");
}
