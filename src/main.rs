mod builtins;
mod stack;
mod parser;
mod interpreter; 

use std::env;
use std::fs;
use std::io::Write;
use std::process;
use crate::interpreter::Interpreter;

fn main() {
    // Obtener argumentos de línea de comandos
    let args: Vec<String> = env::args().collect();

    // Verificar que se pasó al menos la ruta al archivo
    if args.len() < 2 {
        eprintln!("Uso: {} <ruta/.fth> [stack-size]", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    // Intentar leer el contenido del archivo
    let input = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error: No se pudo leer el archivo {}", file_path);
            process::exit(1);
        }
    };

    // Opcionalmente, obtener el tamaño del stack
    let stack_size = args.get(2).and_then(|s| s.parse::<usize>().ok());

    // Crear el intérprete
    let mut interpreter = Interpreter::new(stack_size);

    // Ejecutar el programa Forth
    if let Err(e) = interpreter.run(&input) {
        eprintln!("Error en ejecución: {}", e);
        process::exit(1);
    }

    // Guardar el stack restante en stack.fth
    if let Err(e) = save_stack(interpreter) {
        eprintln!("Error guardando el stack: {}", e);
        process::exit(1);
    }
}

/// Guarda el estado actual del stack en stack.fth
fn save_stack(mut interpreter: Interpreter) -> std::io::Result<()> {
    let contents = interpreter.get_stack_contents(); // Mueve los datos
    let mut file = fs::File::create("stack.fth")?;

    let stack_data: String = contents.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");
    writeln!(file, "{}", stack_data)
}
