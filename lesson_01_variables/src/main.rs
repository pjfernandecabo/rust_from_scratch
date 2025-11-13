fn main() {
    println!("🦀 Lección 1: Variables, Mutabilidad y Tipos Básicos");

    // 1️⃣ Declaración de una variable inmutable
    let nombre = "Pedro";
    println!("Hola, {}!", nombre);

    // 2️⃣ Variable mutable (con `mut`)
    let mut edad = 41;
    println!("Tienes {} años.", edad);

    edad += 1; // puedes modificarla porque es mutable
    println!("El próximo año tendrás {} años.", edad);

    // 3️⃣ Constante (siempre debe tener tipo explícito y se define con `const`)
    const PI: f64 = 3.14159;
    println!("El valor de PI es: {}", PI);

    // 4️⃣ Inferencia de tipos
    let temperatura = 22.5; // Rust infiere que es f64
    println!("Temperatura actual: {}°C", temperatura);

    // 5️⃣ Especificar tipo manualmente
    let numero: i32 = 100;
    println!("Número entero: {}", numero);

    // 6️⃣ Booleans y chars
    let es_programador = true;
    let inicial: char = 'P';
    println!("¿Eres programador? {}. Tu inicial es '{}'", es_programador, inicial);

    // 7️⃣ Shadowing (redefinir variable con el mismo nombre)
    let edad = "cuarenta y dos"; // mismo nombre, pero distinto tipo
    println!("Edad en texto: {}", edad);
}
// Ejecuta este código con `cargo run` para ver los resultados en la consola.
