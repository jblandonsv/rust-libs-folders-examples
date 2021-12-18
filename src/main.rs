// use paquete::uno::Libro;
// mod libreria;
use probando::libreria::clases;
fn main() {
    
    let l = clases::Libro {
        nombre: String::from("Hola"),
        autor: String::from("Mundo"),
    };

    println!("Libro: {:?}", l);
}
