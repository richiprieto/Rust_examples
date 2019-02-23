//Los modulos permiten organizar codigo en grupos
//el keyword es mod, el codigo esta en modulos jerarquicos
//y se pueden colocar modulos dentro de otros modulos

mod sound{
    mod instrument{
        mod woodwind{
            fn clarinete(){
                //cuerpo de funcion
            }
        }
    }
    mod voice {

    }
}
//La jerarquia de los modulos es la siguiente
//crate(root)
//|___sound
//    |___instrument
//        |___woodwind
//    |___voice
// carte es el nombre implicito para root
fn main() {

}
