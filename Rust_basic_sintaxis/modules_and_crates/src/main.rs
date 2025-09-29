fn main() {
    println!("📌 Ejemplos de Modules & Crates en Rust");

    // ======================
    // 1. Usando módulo con funciones públicas
    // ======================
    saludo::hola();
    saludo::adios();
    // saludo::secreto(); // ❌ ERROR: función privada

    // ======================
    // 2. Structs públicos y privados
    // ======================
    let libro = biblioteca::crear_libro("1984", "George Orwell");
    println!("Título: {}", libro.titulo);
    // println!("Autor: {}", libro.autor); // ❌ ERROR: campo privado

    // ======================
    // 3. Módulos anidados
    // ======================
    biblioteca::libro::leer();

    // ======================
    // 4. Uso de `use` para simplificar paths
    // ======================
    use biblioteca::sumar as suma;
    let resultado = suma(10, 15);
    println!("10 + 15 = {}", resultado);
}

// ======================
// Módulos
// ======================

mod saludo {
    pub fn hola() {
        println!("¡Hola desde el módulo saludo!");
    }

    pub fn adios() {
        println!("¡Adiós desde el módulo saludo!");
    }

    fn secreto() {
        println!("Este es privado y no se puede usar fuera");
    }
}

mod biblioteca {
    pub struct Libro {        // struct público
        pub titulo: String,   // campo público
        autor: String,        // campo privado
    }

    pub fn crear_libro(titulo: &str, autor: &str) -> Libro {
        Libro {
            titulo: String::from(titulo),
            autor: String::from(autor),
        }
    }

    // Módulo anidado
    pub mod libro {
        pub fn leer() {
            println!("Leyendo un libro desde módulo anidado...");
        }
    }

    // Función para usar con `use` y alias
    pub fn sumar(a: i32, b: i32) -> i32 {
        a + b
    }
}
