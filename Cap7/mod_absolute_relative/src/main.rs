//se puede llamar al path de dos fomras
//absoluta, inicia en crate root usando literal crate
//relativa, inicia desde el modulo actual usando self,
//super o un idenfiticador en el modulo actual

//los idenficadores de modulos son separados mediante ::

mod sound {
    mod instrument {
        fn clarinete() {
            //cuerpo de la funcion
        }
    }
}
fn main() {
    // Ruta absoluta
    crate::sound::instrument::clarinete();

    // Ruta relativa
    sound::instrument::clarinete();
}
// el software no se ejecuta debido a que los modulos
// son privados por default para solventar ese compile_error!
// debemos usar el keyword pub
