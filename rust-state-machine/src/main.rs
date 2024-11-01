mod balances;
mod system;
mod tests;

// Este é o nosso Runtime principal.
// Ele acumula todos os diferentes pallets que queremos usar.
#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet,
    system: system::Pallet,
    /* TODO: cria um campo `system` que é do tipo `system::Pallet`. */
    /* TODO: cria um campo `balances` que é do tipo `balances::Pallet`. */
}

impl Runtime {
    // Cria uma nova instância do Runtime principal,
    // criando uma nova instância de cada pallet.
    fn new() -> Self {
        Runtime {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }
}

fn main() {
    /*
    let mut pallet = Pallet::new();
    pallet.set_balance("rodrigo".to_string(), 2);
    let balance = pallet.get_balance("rodrigo");
    println!("Balance: {:?}", balance);
    */

    /* TODO: Cria uma variável mutável `runtime`, que é uma nova instância de `Runtime`. */
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();
    /* TODO: Define o saldo de `alice` para 100, permitindo-nos executar outras transações. */
    runtime
        .balances
        .set_balance((&alice.to_string()).parse().unwrap(), 100);
    // começa a emular um bloco
    /* TODO: Aumenta o número do bloco no sistema. */
    runtime.system.inc_block_number();
    /* TODO: Afirmar que o número do bloco é o que esperamos. */
    assert!(runtime.system.block_number() == 1);

    // primeira transação
    /* TODO: Aumenta o nonce de `alice`. */
    runtime.system.inc_nonce(&alice);
    /* TODO: Executa uma transferência de `alice` para `bob` por 30 tokens.
        - A transferência _poderia_ retornar um erro. Deveríamos usar `map_err` para imprimir o erro, se houver.
        - Devemos capturar o resultado da transferência em uma variável não utilizada como `_res`.
    */
    let _res = runtime
        .balances
        .transfer(alice.clone(), bob.clone(), 30)
        .map_err(|e| println!("{}", e));

    // segunda transação
    /* TODO: Aumenta o nonce de `alice` novamente. */
    runtime.system.inc_nonce(&alice);
    /* TODO: Executa outra transferência de saldo, desta vez de `alice` para `charlie` por 20. */
    let _res = runtime
        .balances
        .transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e| println!("{}", e));

    println!("Hello, world!");
    println!("{:?}", runtime);
}
