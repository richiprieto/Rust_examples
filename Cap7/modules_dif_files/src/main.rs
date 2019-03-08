// se puede llamar un modulo que se encuentra en un archivo
// distinto mediante el keyword mod seguido del nombre del
// archivo del modulo, de esta manera se puede llamar al item
// mediante su ruta absoluta y relativa como se vio anteriormente

mod sound;

fn main() {
    // Absolute path
    crate::sound::instrument::clarinete();

    // Relative path
    sound::instrument::clarinete();
}
