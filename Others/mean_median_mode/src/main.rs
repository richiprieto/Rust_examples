/*
Given a list of integers, use a vector and return the mean (the average value),
median (when sorted, the value in the middle position), and mode (the value
that occurs most often; a hash map will be helpful here) of the list.
*/
use std::collections::HashMap;

fn main() {
    // creamos el vector o lista que sea mutable para poder usar el sort
    let mut vector = vec![11, -2, 5, 23, 6, 423];
    //let mut vector = [11, -2, 5, 23, 6, 423];
    // Obtenemos la media
    println!("Media: {}", media(&vector));
    // obtenemos la mediana
    println!("Mediana: {}", mediana(&mut vector));
    // obtenemos la moda
    println!("Moda: {}", moda(&vector));

}

fn media (vector: &[i32]) -> f32 {
    // sumamos los valores del vector y los almacenamos en suma
    let suma: i32 = vector.iter().sum();
    // la division debe hacerse trasformando a float32 para que sea exacta
    suma as f32/vector.len() as f32
}

fn mediana (vector: &mut [i32]) -> f32 {
    //organizamos el vector para la moda de menor a mayor
    vector.sort();
    let mitad = vector.len() / 2;
    let residuo = vector.len() % 2;

    if residuo == 0{
        media(&vector[(mitad - 1)..(mitad + 1)])
    } else {
        vector[mitad] as f32
    }
}

fn moda (vector: &[i32]) -> i32 {
    let mut repeticiones = HashMap::new();
    //Creamos el hashmap a partir del vector
    for &valor in vector {
        *repeticiones.entry(valor).or_insert(0) += 1;
    }
    //Separamos el valor mas repetido en el vector
    repeticiones.into_iter()
    .max_by_key(|&(_, count)| count)
    .map(|(val, _)| val)
    .expect("No se puede obtener moda de numeros cero")
}
