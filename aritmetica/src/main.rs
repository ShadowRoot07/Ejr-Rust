fn takes_u32(x: u32) {
    println!("u32: {x}");
}   

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn mult(a: i32, b: i32) -> i32 {
    return a * b;
}

fn main() {
    let x = 10;
    let y = 20;
    takes_u32(x);
    takes_i8(y);
    // takes_u32(y);

    let n = 20;
    println!("fib({n}) = {}", fib(n));

    // multiplicacion

    let m1: i32 = 13;    let m2: i32 = 23;

    println!("mult({m1}, {m2}) = {}", mult(m1, m2));
}
