use crate::balances::Pallet;

#[test]
fn init_balances() {
    let _balances = Pallet::new();
}

#[test]
fn test_balance() {
    let mut _balances = Pallet::new();
    assert_eq!(_balances.get_balance(&"Rodrigo".to_string()), 0);

    _balances.set_balance("Rodrigo".to_string(), 10);
    assert_eq!(_balances.get_balance(&"Rodrigo".to_string()), 10);
    assert_eq!(_balances.get_balance(&"Nikas".to_string()), 0);
}

#[test]
fn transfer_balance() {
    let mut _balances = Pallet::new();

    assert_eq!(
        _balances.transfer("Rodrigo".to_string(), "Nikas".to_string(), 10),
        Err("Insufficient balance")
    );

    _balances.set_balance("Rodrigo".to_string(), 10);
    assert_eq!(
        _balances.transfer("Rodrigo".to_string(), "Nikas".to_string(), 3),
        Ok(())
    );

    assert_eq!(_balances.get_balance(&"Rodrigo".to_string()), 7);
    assert_eq!(_balances.get_balance(&"Nikas".to_string()), 3);

    _balances.set_balance("Nikas".to_string(), u128::MAX);
    assert_eq!(
        _balances.transfer("Rodrigo".to_string(), "Nikas".to_string(), 3),
        Err("Overflow")
    );
}