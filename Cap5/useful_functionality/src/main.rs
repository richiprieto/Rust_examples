#[derive(Debug)] //especificamos formato de salida
                 //llamado Debug

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50};
    println!("rect1 is {:?}", rect1); //para imprimir la salida del struct
                                      // utilizamos {:?} o {:#?}
}
