use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, account: String, amount: u128) {
        self.balances.insert(account, amount);
    }

    pub fn get_balance(&self, account: &str) -> u128 {
        *self.balances.get(account).unwrap_or(&0)
    }

    /// Transfere `amount` de uma conta para outra.
    /// Esta função verifica se `caller` tem pelo menos `amount` de saldo para transferir,
    /// e se não ocorrem overflow/underflow matemáticos.
    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let caller_balance = self.get_balance(&caller);
        let to_balance = self.get_balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("Insufficient balance")?;

        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);
        /* TODO:
            - Obter o saldo da conta `caller`.                              - Check
            - Obter o saldo da conta `to`.                                  - Check
            - Usar matemática segura para calcular um `new_caller_balance`. - Check
            - Usar matemática segura para calcular um `new_to_balance`.     - Check
            - Inserir o novo saldo de `caller`.                             - Check
            - Inserir o novo saldo de `to`.                                 - Check
        */

        Ok(())
    }
}
