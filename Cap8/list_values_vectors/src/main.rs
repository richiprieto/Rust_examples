/*
Crear un vector
*/
fn main() {
    // Crear un vector
    // Vec<T> es un tipo provisto por la libreria estandar y contine cualquier tipo
    let mut v: Vec<i32> = Vec::new();
    // Rust tiene el macro vec! para dar valores tipo i32
    let v1 = vec![11, 22, 33, 7, 293];
    // para agregar elemento al vector utilizamos el metodo push
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(901);

    // Si queremos eliminar un vector, podemos utilizar el mismo tratamiento de una estructura
    {
        let _v2 = vec![1, 5, 9];
    }// Se elimina el vector v2

    // Los valores de indices son valores numericos que inician en cero
    // Las dos maneras de obtener el tercer elemento es mediante & y [] (mediante referencia)
    // o usando get el cual pasa directamente el indice como argumento
    // si usamos referencia y apuntamos a un indice excedido del maximo nos dara como resultado
    // un panic error
    // sin embargo si usamos get y sobrepasamos el valor maximo del indice el resultado sera None

    let third: &i32 = &v[2];
    println!("El tercer elemento de v es {}", third);

    match v1.get(2) {
        Some(third) => println!("El tercer elemento de v1 es {}", third),
        None => println!("No es el tercer elemento"),
    }

    //Iterando valores
    let v = vec![93,643,1];
    for i in &v {
        println!("Numero {}", i);
    }
    // Iterando valores haciendo cambios en los elementos
    let mut v = vec![193,392,48];
    for i in &mut v {
        *i *= 10; // *i hace referencia al operador de deferencia *
    }
    // Verificar valores mutados multiplicados por 10
    for i in &v {
        println!("Numero {}", i);
    }
}
