mod balances;
use std::collections::BTreeMap;

use balances::Pallet;
fn main() {
    let mut pallet: Pallet = Pallet::new();
    pallet.set_balance("Guilherme".to_string(), 3);

    let balance: u128 = pallet.balance("Guilherme".to_string());
    println!("Balance: {}",balance);
}
