use std::{io, ops::RangeInclusive};
use std::fmt::Display;
use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{fork, getpid, getppid};


fn main() {
    let pid = fork();

    match pid.expect("Fork Failed: Unable to create child process!") {
        Child => println!(
            "Hello from child process with pid: {} and parent pid:{}",
            getpid(),
            getppid()
        ),
        Parent { child } => {
            wait();
            println!(
                "Hello from parent process with pid: {} and child pid:{}",
                getpid(),
                child
            );
        }
    }
}

// Descompone la linea de comando a ingresar
fn dividor(opcion: &str) -> Vec<String>{
    let secciones = opcion.split_whitespace().map(str::to_string).collect();
    return secciones; 
}
// Se valida que las opciones para rastreador sean las adecuadas
fn validador_contenedor(elementos: Vec<String>) -> bool{
    if (elementos.len()) == 4 && (elementos[0]== "-v" || elementos[0]== "-V") {
        return true;
    }else {
        return false;
    }
}

fn validador() {
    println!("Ingresar comando");
    let mut line = String::new();
    io::stdin().read_line(&mut line);

    let commands: Vec<String> = dividor(&line);
    validador_contenedor(commands);
}