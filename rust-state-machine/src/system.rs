/* TODO: Talvez seja necessário atualizar suas importações. */
use std::collections::BTreeMap;

/// Este é o Pallet do Sistema.
/// Ele lida com o estado de baixo nível necessário para seu blockchain.
#[derive(Debug)]
pub struct Pallet {
    block_number: u64,
    pub(crate) nonce: BTreeMap<String, u32>,
    /* TODO:
        O número do bloco atual.
        Crie um campo `block_number` que armazena um `u32`.
        Um mapa de uma conta para seu nonce.
    */
    /*TODO:
        Crie um campo `nonce` que seja um `BTreeMap` de `String` para `u32`.
    */
}

impl Pallet {
    /// Cria uma nova instância do System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
        /* TODO: Retorne uma nova instância da struct `Pallet`. */
    }

    pub fn block_number(&self) -> u64 {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn inc_nonce(&mut self, account: &String) {
        let nonce = self.nonce.get(account).unwrap_or(&0) + 1;
        self.nonce.insert(account.clone(), nonce);
    }
}
