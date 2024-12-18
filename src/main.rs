use std::{
    io,
    time::{Duration, SystemTime},
};

use tokio::time::error::Elapsed;

/*
 * tengo que crear un elevador
 * dicho elevador debe poder: bajar y subir, selecionar un piso, mientras sube o baja poder escoger
 * otro piso, si el piso seleccionado se escoge de nuevo hacer caso omiso a esa seleccion.
 *
 */

#[tokio::main]
async fn main() {
    println!("Welcome, call elevator");
    println!("1. call elevator \n2. exit");

    let seconds = Duration::from_secs(5);
    let start = SystemTime::now();
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).expect("fail");

    input = input.trim().to_string();

    loop {
        std::thread::sleep(Duration::new(2, 0));
        match start.elapsed() {
            Ok(elapsed) if elapsed > seconds => {
                return;
            }
            _ => (),
        }
    }
}
