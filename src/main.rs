use std::{
    i32, io,
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
    let seconds = Duration::from_secs(5);
    let start = SystemTime::now();
    let stdin = io::stdin();

    println!("Welcome, call elevator");

    'menu: loop {
        println!("1. call elevator \n2. exit");
        let mut input = String::new();
        stdin.read_line(&mut input).expect("fail");

        let choice = input.trim().parse::<u32>();

        match choice {
            Ok(1) => {
                std::thread::sleep(Duration::new(2, 0));
                let mut actual_floor = "";
                loop {
                    println!("Select floor");
                    println!("1\n2\n3\n4");
                    println!("actual floor: {}", actual_floor);

                    let mut floor = String::new();
                    stdin.read_line(&mut floor).expect("fail");

                    let floor_choice = floor.trim().parse::<u32>();
                    match floor_choice {
                        Ok(1) => {
                            println!("{:?}", floor_choice);
                        }
                        Ok(2) => {
                            println!("{:?}", floor_choice);
                        }
                        Ok(3) => {
                            println!("{:?}", floor_choice);
                        }
                        Ok(4) => {
                            println!("See you later");
                            break 'menu;
                        }
                        _ => print!("Option not allowed"),
                    };
                }
            }
            Ok(2) => {
                println!("See you later");
                break;
            }
            _ => {
                println!("fail")
            }
        };
    }
}
