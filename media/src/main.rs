#[derive(Debug)]
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
 */

enum Media {
    //Book, movie, audiobook son llamados Variants
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        //Es self un libro, una pelicula, un audiolibro?
        //if tenemos un libro=> format un libro con datos
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie: {} {}", title, director)
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Media description")
        // }

        match self {
            //Si self es igual a Media::Book dame acceso a title y author
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

//Catalogo que tendra libros, peliculas y audiolibros
#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook {
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

    match catalog.items.get(100) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("Nothing at that index");
        }
    }
}
