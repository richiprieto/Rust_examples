/*
Convert strings to pig latin. The first consonant of each word is moved to the end
of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with
a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
Keep in mind the details about UTF-8 encoding!
*/
fn main() {
    let mut palabra = String::from("first");
    // extraemos la primera letra
    let primera_letra = &palabra[..1];
    // lista de vocales
    let vocales = ["a", "e", "i", "o", "u"];
    // comparamos si vocales contiene la primera letra
    if vocales.contains(&primera_letra){
        println!("{}-{}", palabra, "hay");
    } else {
        println!("{}-{}{}",&palabra[1..],primera_letra,"ay");
    }
}
