fn main() {
    println!("🦀 Lección 2: Funciones y Control de Flujo");

    // 1️⃣ Llamada a una función sin parámetros ni retorno
    saludar();

    // 2️⃣ Función con parámetros
    presentarse("Pedro", 41);

    // 3️⃣ Función que devuelve un valor
    let resultado = sumar(10, 20);
    println!("El resultado de 10 + 20 es: {}", resultado);

    // 4️⃣ Uso de if / else
    let edad = 17;
    if edad >= 18 {
        println!("Eres mayor de edad");
    } else {
        println!("Eres menor de edad");
    }

    // 5️⃣ Uso de match (más elegante que muchos if/else)
    let nota = 8;
    match nota {
        10 => println!("Excelente 💯"),
        7..=9 => println!("Notable 👏"),
        5..=6 => println!("Aprobado 😌"),
        _ => println!("Suspenso 😞"),
    }

    // 6️⃣ Uso de bucles (loop, while, for)
    // loop infinito controlado por break
    let mut contador = 0;
    loop {
        contador += 1;
        if contador == 3 {
            println!("loop -> rompemos con contador = {}", contador);
            break;
        }
    }

    // while
    let mut n = 0;
    while n < 3 {
        println!("while -> n = {}", n);
        n += 1;
    }

    // for
    for i in 1..=3 {
        println!("for -> iteración {}", i);
    }

    // 7️⃣ Ejemplo con funciones + control de flujo
    let temp = 9.5;
    println!("Temperatura actual: {}°C", temp);
    //describir_temperatura(temp);
    let result = describir_temperatura(temp);
    println!("Resultado de describir_temperatura: {}", result);

    // 8️⃣ Función recursiva
    let numero = 5;
    println!("Contando atrás para el factorial de {}:", numero);
    contar_atras(numero);

}

// ------- FUNCIONES -------

// Función sin parámetros ni retorno
fn saludar() {
    println!("¡Hola desde la función saludar()!");
}

// Función con parámetros
fn presentarse(nombre: &str, edad: i32) {
    println!("Me llamo {} y tengo {} años.", nombre, edad);
}

// Función que devuelve un valor (sin punto y coma al final del retorno)
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

// Función con control de flujo interno
fn describir_temperatura(temp: f64) -> f64{
    if temp < 10.0 {
        //println!("Hace bastante frío 🥶");
        //"Hace bastante frío 🥶"
        temp
    } else if temp < 25.0 {
        println!("El clima es templado 🌤️");
        //"Hace bastante frío 🥶" 
        temp   
    } else {
        //println!("Hace calor ☀️");
        //"Hace bastante frío 🥶"
        temp
    }
}

fn contar_atras_factorial(n: i32) {
    for i in (1..=n).rev() {
        if i == 1 {
            println!("{}!", i);
        } else {
            contar_atras_factorial(i - 1);
            println!("{}!", i);
        }
    }
    println!("¡Despegue! 🚀");
}

fn contar_atras_working(n: i32) {
    //if n < 0 {
    //    return;
    //}

    println!("{}!", n);

    if n <= 0 {
        println!("¡Despegue! 🚀");
        return;
    }

    contar_atras(n - 1);
}

fn contar_atras(n: i32) {
    match n {
        0 => {
            println!("0!");
            println!("¡Despegue! 🚀");
        }
        x if x > 0 => {
            println!("{}!", x);
            contar_atras(x - 1);
        }
        _ => {}
    }
}