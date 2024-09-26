use std::{borrow::Borrow, io::stdin};

use file_manager::read_file;
use logic::Logic;

pub mod arithmetic;
pub mod file_manager;
pub mod huffman;
pub mod logic;
pub mod structs;

fn main() {
    use std::time::Instant;

    let mut data = read_file("Enter the path to the .csv file(relative path)");
    while let Err(e) = data.borrow() {
        println!("Error: {e}");
        data = read_file("Enter the path to the .csv file(relative path)");
    }
    let now = Instant::now();

    let data = data.unwrap();
    let mut manager = Logic::new();
    match manager.execute_methods(data) {
        Ok(_) => {
            let elapsed = now.elapsed();
            println!("Execution time: {:.2?}", elapsed);

            let mut search = read_file("Enter the path with the search methods(relative path)");
            while let Err(e) = search {
                println!("Error: {e}");
                search = read_file("Enter the path with the search methods(relative path)");
            }
            let now = Instant::now();
            let search = search.unwrap();
            if let Err(e) = manager.search(search) {
                println!("Error: {e}");
            }
            let elapsed = now.elapsed();
            println!("Searching time: {:.2?}", elapsed);
        }
        Err(e) => println!("Error: {e}"),
    }
    println!("Generated file output.txt. Press Enter to exit");
    let mut a = String::from("a");
    _ = stdin().read_line(&mut a);
}
