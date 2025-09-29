fn main() {
    // ======================
    // Variables inmutables
    // ======================
    let x = 5;
    println!("El valor de x es: {}", x);
    // x = 6; ❌ Error: no se puede reasignar a una variable inmutable

    // ======================
    // Variables mutables
    // ======================
    let mut y = 10;
    println!("y = {}", y);
    y = 20; // ✅ permitido porque es mutable
    println!("y actualizado = {}", y);

    // ======================
    // Constantes
    // ======================
    const PI: f64 = 3.1416;
    println!("PI = {}", PI);

    // ======================
    // Shadowing (enmascaramiento)
    // ======================
    let x = 5;
    let x = x + 1; // ahora vale 6
    let x = x * 2; // ahora vale 12
    println!("x con shadowing = {}", x);

    // ======================
    // Tipos escalares
    // ======================

    // Enteros
    let a: i32 = -10;
    let b: u32 = 20;
    println!("Enteros -> a = {}, b = {}", a, b);

    // Flotantes
    let pi: f64 = 3.14159;
    let e: f32 = 2.71;
    println!("Flotantes -> pi = {}, e = {}", pi, e);

    // Booleanos
    let is_active: bool = true;
    println!("Booleano -> ¿Activo? {}", is_active);

    // Caracteres
    let c: char = 'A';
    let emoji: char = '😎';
    println!("Caracteres -> c = {}, emoji = {}", c, emoji);

    // ======================
    // Tipos compuestos
    // ======================

    // Tupla
    let person: (&str, i32, f64) = ("Alice", 25, 1.70);
    println!("Tupla -> {} tiene {} años y mide {} m", person.0, person.1, person.2);

    // Array
    let numbers: [i32; 3] = [1, 2, 3];
    println!("Array -> Primer número = {}", numbers[0]);

    // Vector
    let mut numbers_vec = vec![10, 20, 30];
    numbers_vec.push(40);
    println!("Vector -> {:?}", numbers_vec);

    // ======================
    // Inferencia de tipos
    // ======================
    let x = 42;      // Rust infiere que es i32
    let pi = 3.14;   // Rust infiere que es f64
    println!("Inferencia -> x = {}, pi = {}", x, pi);

    // ======================
    // Conversión de tipos
    // ======================
    let a: i32 = 10;
    let b: f64 = a as f64 / 3.0;
    println!("Conversión -> b = {}", b);
}
