use std::env; // Apporte dans le scope le module env

fn main() {
    let args: Vec<String> = env::args().collect(); // Permet de passer des arguments à cargo run

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
