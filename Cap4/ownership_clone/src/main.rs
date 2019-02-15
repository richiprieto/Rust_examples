fn main() {
    //clone por otra parte copia tambien la data
    //que se encuentra en el heap
    let s1 = String::from("Hola");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
