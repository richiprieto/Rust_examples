fn main() {
    //las tuplas son de tama;o fijo
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; //asigna cada valor en la tupla variables distintas
			 //se conoce como destrucutrar
    println!("El valor de y es: {}", y);
    let one = tup.2; //Accesando a valores de la tupla (inicia indice en 0)
    println!("El valor de variable one es: {}", one);
}
