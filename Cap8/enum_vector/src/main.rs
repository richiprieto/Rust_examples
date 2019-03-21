// Se puede almacenar lista de diferentes tipos de items
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Hola")),
        SpreadsheetCell::Float(10.12),
    ];
}
