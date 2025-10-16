/**
 * Enum: permite definir modelo con datos similares
 * Se define una sola estructura que puede almacenar tipos de datos similares(title en este ejemplo)
 */
/**
 * Como decidir cuando usar Enums o Structs
 * Cuando los modelos tienen el mismo set de metodos es mejor usar Enmus
 * Si la aplicacion fuera mas compleja y tuviera metodos mas complejos entonces es mejor usar structs(ej:book.read, movie.play)
 *   por ejemplo una pelicula no puede tener un metodo "leer" igual si un modelo tiene demasiados campos
 */
/**
 * Option Enum, Rust no tiene un null, nil o undefined
 *   Pero se tiene un enum llamado Option construido dentro de Rust
 *   Tiene dos variantes 'Some' y 'None'
 *   Si se quiere trabajar con Option se tiene que usar coincidencia de patrones(if let) o una declaracion de match
 *   Te fuerza a manejar la situacion donde tienes un valor y la situacion donde no lo tienes
 *  item.unwrap(): si el item es un Some, regresa el valor dentro de Some. Si item es none, "panics!"(error). Se puede usar para un rapido debugg o ejemplos
 *  item.expect("Mensaje de error"): si el item es un Some, regresa el valor dentro de Some. Si item es none, imprime el mensaje de debug enviado y "panics!".
 *    Se usa cuando se busca "crashear" si no existe un valor.
 *  item.unwrap_or(&placeholder): si el item es Some, regresa el valor dentro de Some. Si Item es None, regresa el valor enviado por default y no entra en panico. Se usa cuando hace sentido proveer un valor por error.
 */
/**
 * Module: es posible crear dentro de un mismo archivo un modulo para englobar estructuras, funciones o enums relacionados.
 *  Toda funcion, structs, enums deben tener 'pub' para hacerlo visibles fuera del modulo.
 *  Tambien es posible crear otro archivo.rs para englobarlos en un modulo dentro del mismo folder donde son importados. Es lo mas apropiado cuando se quiere separar un modulo para organizar el codigo pero no necesita expanderse a varios archivos.
 *  Se puede importar como - mod content; y accederse como content::Catalog::new(); o use content::Catalog;
 *  Otra opcion es esparcir el codigo en varios archivos dentro de una nueva carpeta y unirlos en un solo modulo el cual puede ser importado en main.rs. De esta forma no seria posible saltarse niveles, X:main>media-module. Ok:main>content module>media-module
 */
//importacion de modulos:
mod content;
use content::catalog::Catalog;
use content::media::Media;
//lifetime anotation <'a>
//se utiliza para manejar errores donde el valor existe o no existe
// enum MightHaveAValue<'a> {
//     ThereIsAValue(&'a Media),
//     NoValueAvailable,
// }

fn main() {
    //importar media desde content de forma mas extense que catalog
    let audiobook = content::media::Media::Audiobook {
        title: String::from("An Audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };
    let podcast = Media::Podcast((10));
    let placeholder = Media::Placeholder;

    // println!("{}", audiobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_book.description());

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    //manejar cuando exista o no el indice
    let item = catalog.get_by_index(40);
    let placeholder = Media::Placeholder;
    //println!("{:#?}", item.unwrap());
    println!("{:#?}", item.unwrap_or(&placeholder));
}

//Ejercicio 57:
// #[derive(Debug)]
// struct Account {
//     balance: i32
// }

// fn main() {
//     let mut accounts: Vec<Account> = vec![
//         Account { balance: 0 },
//         Account { balance: 10 }
//     ];

//     // Add code here:
//     println!("{:#?}", accounts.first_mut());

//     match accounts.first_mut() {
//         Some(value) => {
//             println!("{:#?}", "Has a value");
//             value.balance = value.balance + 30;
//             println!("{:#?}", value)
//         }
//         None => {
//             println!("{:#?}", "No account found");
//         }
//     }
// }
