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


use std::{io, ops::RangeInclusive};
use std::fmt::Display;
use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{fork, getpid, getppid};


fn main() {
    unsafe{}
}

unsafe fn fork_process() {
    let pid = fork();

    match pid.expect("Fork Failed: Unable to create child process!") {
        Child => println!("Hello from child process with pid: {} and parent pid:{}", getpid(), getppid()),
        Parent { child } => {
            println!("Hello from parent process with pid: {} and child pid:{}",getpid(), child
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

// Se valida el formato de la linea de comando ingresada
fn validador() {
    println!("Ingresar comando");
    let mut line = String::new();
    io::stdin().read_line(&mut line);

    let commands: Vec<String> = dividor(&line);
    validador_contenedor(commands);
}
// ======================================
mod system_call_names;

use linux_personality::personality;
use nix::sys::ptrace;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, Pid};
use std::os::unix::process::CommandExt;
use std::process::{exit, Command};

fn trace_main() {
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            run_tracee();
        }

        Ok(ForkResult::Parent { child }) => {
            run_tracer(child);
        }

        Err(err) => {
            panic!("[main] fork() failed: {}", err);
        }
    }
}

fn run_tracer(child: Pid) {
    loop {
        wait().unwrap();

        match ptrace::getregs(child) {
            Ok(x) => println!(
                "Syscall name: {:?}",
                system_call_names::SYSTEM_CALL_NAMES[(x.orig_rax) as usize]
            ),
            Err(_) => break,
        };

        match ptrace::syscall(child, None) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
}

fn run_tracee() {
    ptrace::traceme().unwrap();
    personality(linux_personality::ADDR_NO_RANDOMIZE).unwrap();

    Command::new("ls").exec();

    exit(0)
}
