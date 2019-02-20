//Se define un enum mediante la keyword enum

//El siguiente bloque de codigo se puede reemplazar
//con enum Message
/*
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); //struct tupla
struct ChangeColorMessage(i32, i32, i32);
*/

enum Message {
    //se puede colocar los tipos de datos
    //directamente en cada variante de enum
    //cada variante pude tener difentes tipos
    //de datos asociados
    // se puede colocar multiples tipos de datos dentro de
    // enum Strings, structs, num type
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

//una similitud entre enum y estructs es que puede definirse
//metodos en enum usando impl

impl Message{
    fn call(&self) {
        //Cuerpo del metodo
    }
}

fn main() {
    let m = Message::Write(String::from("Hola")); //Llamamos a enum con ::
    m.call();
}
