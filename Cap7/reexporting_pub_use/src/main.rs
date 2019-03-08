// al usar "pub use" la funcion principal puede llamar a clarinete
// mediante el nuevo path

mod sound {
    pub mod instrument {
        pub fn clarinete() {
            unimplemented!();
        }
    }
}

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinete_trio() {
        instrument::clarinete();
        instrument::clarinete();
        instrument::clarinete();
    }
}
fn main() {
    performance_group::clarinete_trio();
    performance_group::instrument::clarinete();
}
