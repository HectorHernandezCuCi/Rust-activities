fn main() {
    println!("📌 Ownership & Borrowing en Rust");

    // ======================
    // 1. Ownership básico
    // ======================
    let s1 = String::from("Hola");
    let s2 = s1; // s1 se mueve a s2
    // println!("{}", s1); // ❌ ERROR: s1 ya no es válido
    println!("s2 = {}", s2);

    // ======================
    // 2. Borrowing (referencias inmutables)
    // ======================
    let s3 = String::from("Mundo");
    imprimir(&s3); // prestamos s3
    println!("s3 todavía válido: {}", s3);

    // ======================
    // 3. Mutable Borrowing (referencias mutables)
    // ======================
    let mut s4 = String::from("Hola");
    agregar_mundo(&mut s4); // prestamos mutable
    println!("s4 modificado: {}", s4);

    // ======================
    // 4. Ownership al pasar por valor a funciones
    // ======================
    let s5 = String::from("Hola");
    tomar_propiedad(s5);
    // println!("{}", s5); // ❌ ERROR: s5 ya no es válido

    // ======================
    // 5. Borrowing al pasar por referencia a funciones
    // ======================
    let s6 = String::from("Hola");
    usar_referencia(&s6);
    println!("s6 todavía válido: {}", s6);

    // ======================
    // 6. Retorno de Ownership
    // ======================
    let s7 = crear_string();
    println!("s7 = {}", s7);
}

// ======================
// Funciones para los ejemplos
// ======================

// 2. Borrowing
fn imprimir(texto: &String) {
    println!("Texto prestado: {}", texto);
}

// 3. Mutable Borrowing
fn agregar_mundo(texto: &mut String) {
    texto.push_str(" Mundo");
}

// 4. Ownership al pasar por valor
fn tomar_propiedad(str_param: String) {
    println!("Valor recibido: {}", str_param);
}

// 5. Borrowing al pasar por referencia
fn usar_referencia(str_ref: &String) {
    println!("Valor prestado: {}", str_ref);
}

// 6. Retorno de Ownership
fn crear_string() -> String {
    let nuevo = String::from("Hola Mundo");
    nuevo // ownership se transfiere al llamador
}
