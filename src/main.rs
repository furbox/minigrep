//Crearemos un programa que busque una palabra en un archivo

use std::env; //de la libreria estandar usaremos args para obtener los argumentos de la linea de comandos
use minigrep::Config; //importamos la estructura Config que creamos en lib.rs
fn main() {
    //Los argumentos de la linea de comandos se almacenan en un vector
    // args devielve un iterador, por lo que debemos convertirlo a un vector
    //Vec<String> es un vector de cadenas
    //let args es una variable inmutable
    let args: Vec<String> = env::args().collect();

    //imprimimos los argumentos
    //{:?} es un placeholder para imprimir el contenido de un vector
    //args es un vector de cadenas y devuelve un vector de cadenas
    //println!("{:?}", args);

    let config= Config::new(&args);

    minigrep::run(config);
}
