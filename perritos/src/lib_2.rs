use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{env, near_bindgen};
use std::fmt::Debug;

near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum Estados {
    Vencio,
    Activa,
    Lograda,
}
// impl BorshIntoStorageKey for State {}
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub struct Proyecto {
    balance_actual: u128,
    descripcion: String,
    duracion: u128,
    meta: u128,
    estado: Estados,
    perrito: String,
}

// impl BorshIntoStorageKey for Proyecto {}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Proyectos {
    list: UnorderedMap<String, Proyecto>,
}

impl Default for Proyectos {
    fn default() -> Self {
        env::panic(b"The contract should be initialized before usage")
    }
}

#[near_bindgen]
impl Proyectos {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            list: UnorderedMap::new(b"s".to_vec()),
        }
    }
    pub fn create_project(
        &mut self,
        perrito: String,
        descripcion: String,
        duracion: u128,
        meta: u128,
    ) {
        let creator = env::predecessor_account_id();
        self.list.insert(
            &creator,
            &Proyecto {
                balance_actual: 0,
                descripcion,
                duracion,
                meta,
                estado: Estados::Activa,
                perrito,
            },
        );
    }

    pub fn get_details(&self, creator: String) -> Option<Proyecto> {
        return self.list.get(&creator);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use std::u128;

    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn create_project() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Proyectos::new();
        contract.create_project(
            "Test Project".to_string(),
             "This is the descripcion of the project".to_string(),
             123,
             1234,
        );
        assert_eq!(
            Proyecto {
                balance_actual: 0,
                descripcion: "This is the descripcion of the Proyecto".to_string(),
                duracion: 123 as u128,
                meta: 1234 as u128,
                estado: Estados::Activa,
                perrito: "Test Proyecto".to_string(),
            },
            contract.get_details(env::predecessor_account_id()).unwrap()
        );
    }
}