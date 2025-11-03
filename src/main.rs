use std::{env, fs}; // Apporte dans le scope le module env

fn main() {
    let args: Vec<String> = env::args().collect(); // Permet de passer des arguments à cargo run

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Permet de lire le fichier passé en argument.
    // Expect est du type Result<T,E> et la string passé en argument permet de gérer l'erreur si le
    // fichier passé n'existe pas
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

/**
*@description Parse the config from user input
*@return a tuple
*/
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
