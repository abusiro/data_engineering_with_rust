use rand::seq::{IteratorRandom, SliceRandom}; // Para elegir elementos aleatorios
use rand::thread_rng; // Para generar números aleatorios
use std::io; // Para manejar la entrada del usuario

fn main() {
    let mut fruit_salad = Vec::new(); // Crear un vector vacío para la ensalada de frutas
    let predefined_fruits = vec![
        "Manzana", "Plátano", "Fresa", "Naranja", "Uva", "Piña", "Mango",
    ]; // Lista predefinida de frutas

    println!("¡Hagamos una ensalada de frutas!");

    loop {
        println!("¿Qué deseas hacer?");
        println!("1. Agregar una fruta manualmente");
        println!("2. Agregar frutas aleatorias de la lista predefinida");
        println!("3. Mostrar una fruta aleatoria de la ensalada");
        println!("4. Terminar y mostrar la ensalada");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Error al leer la entrada.");

        match choice.trim() {
            "1" => {
                println!("Ingresa una fruta:");
                let mut fruit = String::new();
                io::stdin()
                    .read_line(&mut fruit)
                    .expect("Error al leer la entrada.");
                let fruit = fruit.trim();
                if !fruit.is_empty() {
                    fruit_salad.push(fruit.to_string());
                    println!("¡{} agregado a la ensalada!", fruit);
                }
            }
            "2" => {
                println!("¿Cuántas frutas aleatorias deseas agregar?");
                let mut num_fruits = String::new();
                io::stdin()
                    .read_line(&mut num_fruits)
                    .expect("Error al leer la entrada.");
                let num_fruits: usize = match num_fruits.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Entrada inválida. Introduce un número.");
                        continue;
                    }
                };

                let mut rng = thread_rng();
                let random_fruits: Vec<_> = predefined_fruits
                    .choose_multiple(&mut rng, num_fruits)
                    .cloned()
                    .collect();

                for fruit in random_fruits {
                    fruit_salad.push(fruit.to_string());
                    println!("¡{} agregado a la ensalada!", fruit);
                }
            }
            "3" => {
                if fruit_salad.is_empty() {
                    println!("La ensalada está vacía. Agrega frutas primero.");
                } else {
                    let mut rng = thread_rng();
                    if let Some(random_fruit) = fruit_salad.choose(&mut rng) {
                        println!("Fruta aleatoria seleccionada: {}", random_fruit);
                    }
                }
            }
            "4" => {
                break;
            }
            _ => {
                println!("Opción inválida. Introduce un número del 1 al 4.");
            }
        }
    }

    // Verificar si el usuario agregó frutas
    if fruit_salad.is_empty() {
        println!("No agregaste ninguna fruta. ¡Adiós!");
        return;
    }

    // Mezclar las frutas
    let mut rng = thread_rng();
    fruit_salad.shuffle(&mut rng);

    // Imprimir la ensalada de frutas
    println!("\nTu ensalada de frutas mezclada:");
    for (i, item) in fruit_salad.iter().enumerate() {
        if i != fruit_salad.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
