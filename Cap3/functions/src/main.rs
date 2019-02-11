fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = {
	let x = 3;
	x + 1
    };

    another_function(x, y);

    let z = retorna_cinco();
    println!("El valor de z es: {}", z);

    let a = mas_uno(29);
    println!("El valor de {} mas uno es: {}", 29, a);
}

/*
{
    let x = 3;
    x + 1
}
Es una expresion que evalua 4 
la expresion no termina con punto y coma ; ya que al agregar ; no retorna el valor
*/

//recordar siempre declarar el tipo de cada parametro
fn another_function(x: i32, y: i32){
    println!("Valores de x, y son: {}, {}", x,y);

}

fn retorna_cinco() -> i32 {
    // retorna la ultima expression implicita
    5 
    //return 5 //otra opcion de return
}

fn mas_uno(x: i32) -> i32 {
    x + 1 // no colocar el punto y coma para obtener el return
}
