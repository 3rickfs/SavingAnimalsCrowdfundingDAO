use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedSet, Vector};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, Duration, Promise};
use std::collections::HashMap;


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Animalito{
    animaltype: String,
    name: String,
}

#[near_bindgen]
impl Animalito{
    pub fn get_animalito_info(&self) -> Animalito {
        self.animalitos.get(id).expect("Animalito not found")
    }
    pub fn insert_animalito(&mut self, anityp: String, nm: String) {
        self.animaltype = anityp;
        self.name = nm;

        let log_message = format!("Animalito type inserted {}", self.animaltype);
        env::log(log_message.as_bytes());
        let log_message = format!("Animalito name type inserted {}", self.name);
        env::log(log_message.as_bytes());
    }  
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct SavingAnimalsCrowdfundingDAO {
    animalitos: Vector<Animalito>,
    rescuername: String,
    arrivaldate: String,
}

impl Default for SavingAnimalsCrowdfundingDAO {
    fn default() -> Self {
        env::panic_str("SavingAnimalsCrowdfundingDAO should be initialized before usage")
    }
}

#[near_bindgen]
impl SavingAnimalsCrowdfundingDAO {
    #[init]
    pub fn new(
       resnam: String,
       arrdat: String,
    ) -> Self {
       animalitos: Vector::new(b"a".to_vec()),
       resname,
       arrdat,
    };

   pub fn add_animalito(&mut self, animalitoname: String, animalitotype: String) -> u64{
      let a = Animalito {
          animaltype: animalitoname,
          name: animalitotype,
      };
      self.animalitos.push(&a);
      self.animalitos.len() - 1
   }

   pub fn get_animalito(&self, id: u64) -> Animalito {
      let log_message = format!("Searching for Animalito");
      env::log(log_message.as_bytes());
      self.animalitos.get(id).expect("Animalito not found")
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use near_sdk::MockedBlockchain;
   use near_sdk::{testing_env, VMContext};

   fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
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
            epoch_height: 19,
        }
    }

   fn test_basics() {
      // instantiate a contract
      let mut animalitodao = SavingAnimalsCrowdfundingDAO::new(
         "Erick".to_string(),
         "18-12-2021".to_string()
      );

      //context setting up
      let context = get_context(vec![], false);
      testing_env!(context);
      //add an Animalito
      let id = animalitodao.add_animalito("Rocky".to_string(),"Perro".to_string())
      //get and display an Animalito
      println!("Searching for animalito")
      let animalitotype = animalitodao.get_animalito(id).animaltype
      let animalitoname = animalitodao.get_animalito(id).name
      println!("Searching results in... 0.0001 seconds")
      println!("Animalito ID: {}",id)
      println!("Animalito type: {}",animalitotype)
      println!("Animalito name: {}",animalitoname)
   }

}




















