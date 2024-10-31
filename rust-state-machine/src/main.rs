use crate::balances::Pallet;

mod balances;
mod tests;

fn main() {
    let mut pallet = Pallet::new();
    pallet.set_balance("rodrigo".to_string(), 2);
    let balance = pallet.get_balance("rodrigo");
    println!("Balance: {:?}", balance);

    println!("Hello, world!");
}
