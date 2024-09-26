/**
 * Programa baraja de cartas
 * Se guaradaran datos y se le agregara funcionalidad
 * Se usaran struct que son como clases en otros lenguajes
 * 
 */
/**
 * Mutable vs Immutable
 * let numbers = vec![]; no se puede agregar valores
 * let mut number = vec![]; permite agregar valores
 */
use rand::{ thread_rng, seq::SliceRandom };

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    // probably need to add error handling!!!
    let cards = deck.deal(3);
    println!("Heres your hand: {:#?}", cards);
    // ! (signo admiracion) significa que es un macro
    println!("Heres your deck: {:#?}", deck);
}

/**
 * derive(debug)
 * Define atributos para el struct Deck
 * Le da instrucciones extras al compilador rust
 * Se le llama atributo 'derive', especifica que caracteristicas se implementaran automaticamente para este struc
 * Debug es un atributo, caracteristicas de un conjunto de funciones
 * 
 */
#[derive(Debug)] 
struct Deck {
    //:Definicion de struct, se pueden generar instancias del mismo
    //Lista de campos o datos que tendra este struct
    //Vec: Vector que contendra Strings, vectores son como arrays que pueden modificar(crecer/reducir) su tamano
    cards: Vec<String>, 

}

/**
 * inherit implementation - Agrega funciones a un struct
 * Se le llama "inherent implementation" porque hereda de un struct
 * Tambien existen funciones "Addociated function" que es un metodo de clase
 * Associated functions = Use then you have functionality not tied to a specific instance
 * Methods = Use when you need to read or change fields on a specific instance
 */
impl Deck {
    //-> return type annotation
    //Self regresa lo mismo que este en la funcion impl
    fn new() -> Self {
        //List of "suits" = 'hearts', 'spades'
        //List of values = 'ace', 'two', 'three'
        //Double nestes for loop
        //let suits = ["Diamonds", "Clubs"]; //crea un array de string con tamaño fijo(es mas rapido en comparacion a un vector, poca diferencia)
        //let suits = vec!["Hearts", "Spades", "Diamonds"]; //Crea un vector con tamano dinamico
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace","Two", "Three"];

        let mut cards = vec![];
        
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit); //concatena strings
                cards.push(card);
            }
        }

        //Regresa Deck, implicit return, no lleva punto y coma ejemplo: (return value; | value)
        //regresa la ultima expresion encontrada sin punto y coma
        //deck: Deck (TypeAnnotation) = {instancia de un struct};
        Deck { cards } //vec::new() otra opcion
        
    }

    //method
    //Utiliza un "External crate" se agregan con cargo add project(cargo add rand)
    //Docs en docs.rs
    //thread_rng() funcion que crea un generador de numeros al azar
    //SliceRandom trait: agrega un metodo shuffle() a todos los vectores
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }    

    //usize u64
    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}



