/**
 * Super: es una referencia al modulo padre en este caso el archivo mod.rs
 */
use super::media::Media;

//Catalogo que tendra libros, peliculas y audiolibros
#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: vec![] }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            //Good! We have something to return
            // MightHaveAValue::ThereIsAValue(&self.items[index])
            Some(&self.items[index])
        } else {
            //Bad! We dont have anything to return!
            // MightHaveAValue::NoValueAvailable
            None
        }
    }
}
