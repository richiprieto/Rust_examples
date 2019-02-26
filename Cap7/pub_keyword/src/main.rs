mod sound {
    pub mod instrument{ // se debe hace publica tanto el modulo
        pub fn clarinete() { // como la funcion
            //Body
        }
    }
}

fn main () {
    crate::sound::instrument::clarinete();

    sound::instrument::clarinete();
}
