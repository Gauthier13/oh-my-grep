use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect(); // Permet de passer des arguments à cargo run

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config)
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    /**
     *@description Parse the config from user input
     */
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Missing arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn run(config: Config) {
    // Permet de lire le fichier passé en argument.
    // Expect est du type Result<T,E> et la string passé en argument permet de gérer l'erreur si le
    // fichier passé n'existe pas
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
