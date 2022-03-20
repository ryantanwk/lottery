use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    winning_number: u32,
}

#[near_bindgen]
impl Contract {
    // INITIALIZE CONTRACT, PREVENTS CHANGES TO THE CONTRACT
    #[init]
    pub fn new(winner: u32) -> Self {
            Self{winning_number: winner,}
    }

    // ADD CONTRACT METHODS HERE
    pub fn get_winning_number(&self) -> u32 {
        self.winning_number
    }

    pub fn guess_number(&mut self, guess: u32) -> bool {
        if guess == self.winning_number {
            env::log_str("You've found the winning number!");
            return true;
        } else {
            env::log_str("Better luck next time!");
            return false;
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    fn check_guess_number() {
        // Set test account ID
        let dummy = AccountId::new_unchecked("ryantan.testnet".to_string());

        // Set up the testing context and unit test environment
        let context = get_context(dummy);
        testing_env!(context.build());

        // Set up contract object and call the new method
        let mut contract = Contract::new(1234,);

        let mut guess_result = contract.guess_number(4321);
        assert!(!(guess_result), "Expected a failure from the wrong guess");
        assert_eq!(get_logs(), ["Better luck next time!"],
            "Expected a failure log.");

        guess_result = contract.guess_number(1234);
        assert!(guess_result, "Expected the correct answer to return true.");
        assert_eq!(get_logs(), ["Better luck next time!", "You've found the winning number!"],
            "Expected a successful log after the previous failed log.");
    }
}
