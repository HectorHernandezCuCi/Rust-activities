
fn main() {
    println!("📌 Ejemplos de Funciones en Rust");

    // 1. Función básica sin parámetros ni retorno
    saludo();

    // 2. Función con parámetros
    imprimir_numero(42);

    // 3. Función con varios parámetros
    suma(10, 20);

    // 4. Función con retorno (usa return)
    let resultado = cuadrado(5);
    println!("El cuadrado de 5 es: {}", resultado);

    // 5. Función con retorno implícito (sin return)
    let triple = triple(7);
    println!("El triple de 7 es: {}", triple);

    // 6. Función que devuelve múltiples valores (tuplas)
    let (a, b) = dividir_y_resto(17, 5);
    println!("División: 17 / 5 = {}, Resto = {}", a, b);

    // 7. Funciones anidadas (llamando dentro de otra función)
    mostrar_mensaje();

    // 8. Función con expresiones y scopes
    let valor = calcular_valor();
    println!("El valor calculado es: {}", valor);
}

// 1. Función básica
fn saludo() {
    println!("Hola desde una función!");
}

// 2. Función con parámetros
fn imprimir_numero(num: i32) {
    println!("El número es: {}", num);
}

// 3. Función con varios parámetros
fn suma(a: i32, b: i32) {
    println!("La suma de {} + {} = {}", a, b, a + b);
}

// 4. Función con retorno explícito
fn cuadrado(x: i32) -> i32 {
    return x * x;
}

// 5. Función con retorno implícito (última expresión)
fn triple(x: i32) -> i32 {
    x * 3
}

// 6. Función que devuelve múltiples valores (tupla)
fn dividir_y_resto(x: i32, y: i32) -> (i32, i32) {
    (x / y, x % y)
}

// 7. Función que llama a otra
fn mostrar_mensaje() {
    println!("Estoy dentro de mostrar_mensaje()");
    saludo(); // Llama a otra función
}

// 8. Función con expresiones y scopes
fn calcular_valor() -> i32 {
    let x = 3;
    let y = {
        let z = 2;
        x + z // esta es la última expresión, no lleva ;
    };
    y * 2
}
