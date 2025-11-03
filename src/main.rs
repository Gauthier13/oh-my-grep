use std::{env, fs}; // Apporte dans le scope le module env

fn main() {
    let args: Vec<String> = env::args().collect(); // Permet de passer des arguments à cargo run

    let (query, file_path) = parse_config(&args);

    println!("Searching for {query}");
    println!("In file {file_path}");

    // Permet de lire le fichier passé en argument.
    // Expect est du type Result<T,E> et la string passé en argument permet de gérer l'erreur si le
    // fichier passé n'existe pas
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
