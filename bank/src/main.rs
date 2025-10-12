/**
 * Ownership:
 *  Every value is 'owned' by a single variable, struct, vector, etc at a time.
 *  Cada valor le pertenece a una sola variable, struct, vector etc  en su momento
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
 *
 * Argument types:
 *  Necesita guardar un argumento en algun lugar? -> Es mejor tomar posesion(recibir un valor)
 *  Necesita hacer un calculo con el valor? -> Mejor recibir una referncia de solo lectura
 *  Necesita cambiar el valor de alguna manera? -> Mejor recibir una referencia mutable
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

    fn summary(&self) -> String {
        format!("{} has a balance {}", self.holder, self.balance)
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
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

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum() //Casi como lambda!
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("me"));

    account.deposit(500);
    account.withdraw(250);

    bank.add_account(account);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
}
