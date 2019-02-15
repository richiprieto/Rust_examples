struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    //No se necesita agregar datos en el mismo orden de como
    //se creo la estructura
    //si se desea cambiar el valor de una estructura
    //toda la estructura debe ser mutable

    let mut user1 = User {
        email: String::from("richi@ejemplo.com"),
        username: String::from("richi"),
        active: true,
        sign_in_count: 1,
    };

    //cambio de valor de correo
    user1.email = String::from("richi@otroejemplo.com");

    //Construimos un usuario a partir de la funcion
    // en este caso el user2 no es mutable
    let user2 = build_user(String::from("manuel@ejemplo.com"), String::from("manuel"));

    //creamos un usuario3 pero con los mismos parametros
    //de user2 excepto los que sean necesarios cambiar
    // en este caso active y sign_in_count seran igual
    //al user2
    let user3 = User{
        email: String::from("andres@ejemplo.com"),
        username: String::from("andres"),
        ..user2
    };

}

//funcion para crear usuarios
fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
