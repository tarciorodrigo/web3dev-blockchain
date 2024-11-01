//use crate::balances::Pallet;

#[test]
fn init_balances() {
    use crate::balances::Pallet;

    let _balances = Pallet::new();
}

#[test]
fn test_balance() {
    use crate::balances::Pallet;

    let mut _balances = Pallet::new();
    assert_eq!(_balances.get_balance(&"Rodrigo".to_string()), 0);

    _balances.set_balance("Rodrigo".to_string(), 10);
    assert_eq!(_balances.get_balance(&"Rodrigo".to_string()), 10);
    assert_eq!(_balances.get_balance(&"Nikas".to_string()), 0);
}

#[test]
fn transfer_balance() {
    use crate::balances::Pallet;

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

#[cfg(test)]
mod tests {
    use crate::system::Pallet;

    #[test]
    fn init_system() {
        let mut _system = Pallet::new();

        assert_eq!(_system.block_number(), 0);
        assert_eq!(_system.nonce.get(&"Rodrigo".to_string()), None);

        _system.inc_block_number();
        assert_eq!(_system.block_number(), 1);

        _system.inc_nonce(&"Rodrigo".to_string());
        assert_eq!(_system.nonce.get(&"Rodrigo".to_string()).unwrap(), &1);
    }
}
