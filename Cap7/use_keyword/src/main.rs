// Como se ve anteriormente llamar a funciones puede
// llegar a ser un proceso largo y repetitivo
// agregar el keyword use es similar a crear un link simbolico

mod sound {
    pub mod instrument {
        pub fn clarinete() {
            //cuerpo de la funcion
        }
    }
}

//Ruta absoluta
//use crate::sound::instrument;

//Ruta relativa
use self::sound::instrument;

fn main() {
    instrument::clarinete();
    instrument::clarinete();
}
