fn main() {
    // ======================
    // if / else
    // ======================
    let edad = 18;

    if edad >= 18 {
        println!("Eres mayor de edad ✅");
    } else {
        println!("Eres menor de edad ❌");
    }

    // ======================
    // if / else if / else
    // ======================
    let nota = 85;

    if nota >= 90 {
        println!("Calificación: A");
    } else if nota >= 80 {
        println!("Calificación: B");
    } else {
        println!("Necesitas mejorar 😅");
    }

    // ======================
    // if como expresión
    // ======================
    let es_par = true;
    let numero = if es_par { 2 } else { 3 }; 
    println!("El número es {}", numero);

    // ======================
    // loop (bucle infinito con break)
    // ======================
    let mut contador = 0;

    loop {
        contador += 1;
        println!("Contador = {}", contador);

        if contador == 3 {
            break; // salir del bucle
        }
    }

    // ======================
    // while
    // ======================
    let mut n = 0;
    while n < 5 {
        println!("n = {}", n);
        n += 1;
    }

    // ======================
    // for con rango
    // ======================
    for i in 1..5 {  // 1 hasta 4
        println!("for simple i = {}", i);
    }

    for i in 1..=5 { // 1 hasta 5
        println!("for con igual i = {}", i);
    }

    // ======================
    // for iterando colecciones
    // ======================
    let numeros = [10, 20, 30, 40];
    for n in numeros {
        println!("Número = {}", n);
    }

    // ======================
    // match (switch)
    // ======================
    let direccion = "izquierda";

    match direccion {
        "arriba" => println!("Vas hacia arriba ⬆️"),
        "abajo" => println!("Vas hacia abajo ⬇️"),
        "izquierda" => println!("Vas hacia la izquierda ⬅️"),
        "derecha" => println!("Vas hacia la derecha ➡️"),
        _ => println!("Dirección desconocida ❓"),
    }

    // ======================
    // match con rangos
    // ======================
    let edad_persona = 16;

    match edad_persona {
        0..=12 => println!("Niño 👶"),
        13..=17 => println!("Adolescente 👦"),
        18..=64 => println!("Adulto 👨"),
        _ => println!("Adulto mayor 👴"),
    }
}
