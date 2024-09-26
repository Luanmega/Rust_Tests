/**
 * Ownership:
 *  Every value is 'owned' by a single variable, struct, vector, etc at a time.
 *  Reasiagnar el valor a otra variable, pasarla a una function, agregandola a unvector etc, mueve el valor. La vieja variable ya no puede ser usada.
 *  
 * Borrowing: 
 *  Puedes crear muchas referencias de "solo lectura" a un valor que existe al mismo tiempo
 *  No puedes mover un valor mientras la referencia del valor aun existe
 *  Puedes crear una referencia modificable(mut) a un valor solo si no existen referencias de solo lectura en uso. Una referencia modificable de un valor solo puede existe una a la vez.
 *  No puedes modificar un valor a travez del propietario cuando cualquier referencia(mut o no mut) a el valor existe.
 *  Algunos tipos de valores con copiados en vez de movidos(numeros, bools, chars, arrays/tuples with copyable elements)
 * 
 * Lifetimes: 
 *  Cuando una varuable sale del alcance(scope) el valor que le pertenece es borrado(limpiado en memoria)
 *  Valores no pueden ser eliminados si aun tienen referencias activas
 *  Referencias a un valor no pueden vivir mas que el valor a las que refiere
 */
#[derive(Debug)]
struct Account {
    id: u32, 
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_account(account: Account) {
    println!("{:#?}", account);
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(1, String::from("Me"));

    //println!("{:#?}", bank);
    print_account(account);
    print_account(account);
}

