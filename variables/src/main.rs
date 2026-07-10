fn main() {
    // Demostracion de variables inmutables
    let x: i32 = 10;
    println!("x: {x}");
    // x = 20; // Esto retorna error
    // println!("x: {x}");

    //=================
    // Demostracion de variables mutablesi
    //=======================
    let mut y: i32 = 20; 
    println!("y: {y}");

    y = 22; // Puede mutar
    println!("y muto: {y}");
    
    //=======================
    // Enteros con signo
    //=======================
    let ai: i8 = 1;
    let bi: i16 = 2;
    let ci: i32 = 3;
    let di: i64 = 4;
    let ei: i128 = 5;
    let fi: isize = 6;  // isize y usize tienen el ancho de un puntero

    println!("==============================");
    println!("Mostrando diferentes enteros con signo: ");
    println!("i8 = {ai}, i16 = {bi}");
    println!("i32 = {ci}, i64 = {di}");
    println!("i128 = {ei}, isize = {fi}");
    println!("==============================");

    //=======================
    // Enteros sin signo
    //=======================
    let au: u8 = 1; 
    let bu: u16 = 2;
    let cu: u32 = 3;
    let du: u64 = 4;
    let eu: u128 = 1_000;
    let fu: usize = 6; 

    println!("==============================");
    println!("Mostrando diferentes enteros sin signo: ");
    println!("u8 = {au}, u16 = {bu}");
    println!("u32 = {cu}, u64 = {du}");
    println!("u128 = {eu}, usize = {fu}");
    println!("==============================");

    //=======================
    // Valores flotantes
    //=======================
    let af: f32 = 1.11;
    let bf: f64 = -2.15;

    println!("==============================");
    println!("Mostrando los dos valores tipo flotantes: ");
    println!("f32 = {af}, f64 = {bf}");
    println!("==============================");

    //=======================
    // Valores escaladores unicode
    //=======================
    let astr: &str = "Hola";
    let ac: char = "i";

    println!("==============================");
    println!("Mostrando valores escaladores: ");
    println!("string = {astr}, char = {ac}");
    println!("==============================");

    //=======================
    // Valores Booleanos
    //=======================
    let ab: bool = true;    // bool tiene 8 bits de ancho
    let bb: bool = false;

    println!("==============================");
    println!("Mostrando valores booleanos: ");
    println!("true-bool = {ab}, false-bool = {bb}");
    println!("==============================");
}
