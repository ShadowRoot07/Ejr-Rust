fn main() {
    // Bloques
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");

    // Ámbitos
    let a = 10;
    println!("antes: {a}");
    {
        let a = "hola";
        println!("ámbito interno: {a}");
        let a = true;
        println!("sombreado en el ámbito interno: {a}");
    }
    println!("después: {a}");
}
