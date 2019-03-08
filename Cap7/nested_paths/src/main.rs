// Cuando usa muchos items definidos por el mismo paquete
// se puede proceder a anidar

// Codigo sin anidar
// use std::cmp::Ordering;
// use std::io;

//Codigo anidado
use std::{cmp::Ordering, io};

fn main() {
    unimplemented!();
}

// tambien se puede definir de la siguiente manera
// use std::io;
// use std::io::Write;

// use std::io::{self, Write};
