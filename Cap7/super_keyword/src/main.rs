// la funcion de super es mantener una ruta relativa
// similar al filesystem path .. o sea sube un item
// de la posicion actual

mod sound {
    pub mod instrument {
        pub fn clarinete() {
            super::breathe_in();
        }
    }

    fn breathe_in() {
        //func body
    }
}

fn main () {
    // body
    sound::instrument::clarinete();
}
