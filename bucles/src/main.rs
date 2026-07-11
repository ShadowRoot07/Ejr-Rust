// Hay tres palabras clave de bucle en Rust:
// while, loop y for:
fn main() {
    // La palabra clave while es muy similar
    // a la de otros lenguajes y ejecuta 
    // el cuerpo del bucle
    // mientras que la condición sea valida.
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("x final: {x}");

    // El bucle for itera sobre rangos
    // de valores o las entradas de una colección:
    for x in 1..5 {
        println!("x: {x}");
    }
    for elem in [1, 2, 3, 4, 5] {
        println!("elem: {elem}");
    }

    // El bucle loop repite hasta encontrar un break 
    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break;
        }
    }

    // break y continue
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;  // salir de un bucle 
                    // antes de que termine
        }
        if i % 2 == 0 {
            continue;   // iniciar inmediatamente 
                        // la siguiente iteración
        }
        println!("{}", i);
    }

    // Etiquetas
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    print!("elementos travesados: {elements_searched}");
}
