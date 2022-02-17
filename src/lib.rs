use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct HelloWorld {}

#[near_bindgen]
impl HelloWorld {
    pub fn greet(&self, name: String) -> String {
        let log_message = format!("Hello {}!", name.to_string());
        env::log(log_message.as_bytes());
        return log_message;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "john.testnet".to_string(),
            signer_account_id: "alex.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jean.testnet".to_string(),
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

    #[test]
    fn greet_individual() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = HelloWorld {};
        let name = "Alexander".to_string();
        assert_eq!(format!("Hello {}!", name), contract.greet(name));
    }
}
