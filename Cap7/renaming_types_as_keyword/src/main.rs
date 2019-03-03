//se puede especificar un nombre diferente
// utilizando el keyword as luego del keyword use

use std::fmt::Result;
use std::io::Result as IoResult;

fn funcion1() -> Result{

}

fn funcion2() -> IoResult <()> {

}

// Se puede observar que las dos funciones anteriores
// retornan tienen el mismo nombre para evitar errores
// renombramos la segunda como IoResult
// Este codigo por si solo no funciona
fn main() {

}
