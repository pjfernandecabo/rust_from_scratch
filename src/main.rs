use std::io;

fn main() {
    println!("¡Hola! ¿Cómo te llamas?");

    // Crear un String vacío para guardar el nombre
    let mut nombre = String::new();
    io::stdin()
        .read_line(&mut nombre)
        .expect("Error al leer el nombre");

    // Eliminar salto de línea
    let nombre = nombre.trim();

    println!("Mucho gusto, {}. ¿Cuántos años tienes?", nombre);

    // Crear un String para guardar la edad
    let mut edad = String::new();
    io::stdin()
        .read_line(&mut edad)
        .expect("Error al leer la edad");

    // Convertir el texto a número (u32 = entero sin signo)
    let edad: u32 = match edad.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, introduce un número válido.");
            return;
        }
    };

    let edad_proximo_anio = edad + 1;
    println!(
        "¡Genial, {}! El próximo año tendrás {} años.",
        nombre, edad_proximo_anio
    );

    // Comparación con mi edad (ejemplo: 30 años)
    let mi_edad = 30;
    if edad > mi_edad {
        println!("Wow, eres mayor que yo 😅");
    } else if edad < mi_edad {
        println!("¡Eres más joven que yo!");
    } else {
        println!("¡Tenemos la misma edad! 😄");
    }
}


