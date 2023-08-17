use std::fs; //de la libreria estandar usaremos fs para leer el archivo

//Creamos una estructura para almacenar los argumentos
//pub significa que la estructura es publica
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config{
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("No se proporcionaron suficientes argumentos");
        }
        let filename = args[1].clone();
        let query = args[2].clone();

        Config {
            query,
            filename,
        }
    }
}

pub fn run(config: Config) {
    println!("Buscando {} en el archivo {}", config.query, config.filename);
    //leemos el archivo
    let contents = fs::read_to_string(config.filename).expect("Algo salio mal al leer el archivo");
    //println!("El contenido del archivo es: \n{}", &contents[..10]);
    let found = search(&config.query, &contents);

    for line in found {
        println!("{}", line);
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}