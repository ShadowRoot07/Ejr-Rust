fn main() {
    let x = 10;
    if x == 0 {
        println!("cero!");
    } else if x < 100 {
        println!("muy grande");
    } else {
        println!("enorme");
    }

    let x = 10;
    let size = if x < 20 { "pequeño" } else { "grande" };
    println!("tamaño del número: {}", size);
}
