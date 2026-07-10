fn main() {
    //demostracion de variables inmutables 
    let x: i32 = 10;
    println!("x: {x}");
    // x = 20; //esto retorna errorr
    // println!("x: {x}");

    //=================
    //Demostracion de variables mutables 
    mut y: i32 = 20;
    println("y: {y}");

    y = 22; //puede mutar 
    println("y muto: {y}");

    //=======================
    // Enteros con signo:
    //=======================
    let ai: i8 = 1;
    let bi: i16 = 2;
    let ci: i32 = 3;
    let di: i64 = 4;
    let ei: i128 = 5;
    let fi: isize = 6;  // isize y usize tienen el ancho de un puntero
    
    println("==============================");
    println("Mostrando diferentes enteros con signo: ");
    println("i8 = {ai}, i16 = {bi}");
    println("i32 = {ci}, i64 = {di}");
    println("i128 = {ei}, isize = {fi}");
    println("==============================");

    //=======================
    // Enteros sin signo
    //=======================
    let au: u8 = 1_000; // Esto favilita la lectura
    let bu: u16 = 2;
    let cu: u32 = 3;
    let du: u64 = 4;
    let eu: u128 = 5;
    let fu: usoze = 6;

    println("==============================");
    println("Mostrando diferentes enteros sin signo: ");
    println("u8 = {au}, u16 = {bu}");
    println("u8 = {au}, u16 = {bu}");
    println("u8 = {au}, u16 = {bu}");
    println("==============================");

    //=======================
    // Valores flotantes
    //=======================
    let af: f32 = 1.11;
    let bf: f64 = -2.15;

    println("==============================");
    println("Mostrandovloa dos valores tipo flotantes: ");
    println("f32 = {af}, f63 = {bf}");
    println("==============================");

    //=======================
    // Valores escaladores unicode 
    //=======================
    let ac: char = "Hola"   // char tiene un tamaño de 32 bits
    
    println("==============================");
    println("Mostrando valores escaladores: ");
    println("char = {ac}");
    println("==============================");

    //=======================
    // Valores Boleanos
    //=======================
    
    let ab: bool = true;    // bool tiene 8 bits de ancho
    let bb: bool = false;

    println("==============================");
    println("Mostrando valores booleanos: ");   
    println("true-bool = {ab}, false-bool = {bb}");
    println("==============================");

}
