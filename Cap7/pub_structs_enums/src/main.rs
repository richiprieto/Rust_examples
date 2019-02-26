mod plant {
    pub struct Vegetable { //estructura publica
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    pub enum Appetizer { // enum publico
        Soup,
        Salad,
    }
}

fn main () {
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("lechucha");
    println!("{} es delicioso", v.name);

    // si se desea imprimir el id obtenemos un compile_error!
    // debido a que es un parametro privado

    // si hacemos un enum publico todos los parametros incluidos
    // tambien cambian a publico

    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;

}
