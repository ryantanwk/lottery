use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen,Promise};

// 1 NEAR in yoctoNEAR
const PRIZE_AMOUNT: u128 = 1_000_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    winning_number: String,
    is_solved: bool,
}

#[near_bindgen]
impl Contract {
    // INITIALIZE CONTRACT, PREVENTS CHANGES TO THE CONTRACT
    #[init]
    pub fn new(winner: String) -> Self {
            Self{winning_number: winner, is_solved: false}
    }

    // ADD CONTRACT METHODS HERE
    pub fn get_winning_number(&self) -> String {
        self.winning_number.clone()
    }

    pub fn guess_number(&mut self, guess: String) -> bool {
        let hashed_input = env::sha256(guess.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);

        if hashed_input_hex == self.winning_number {

            // payout and update status if winning number found
            if !self.is_solved {
              env::log_str("You're the first to find the winning number!");
              self.is_solved = true;
              Promise::new(env::predecessor_account_id()).transfer(PRIZE_AMOUNT);

            }
            else {
              env::log_str("You've found the winning number! But someone found it before you!");
            }
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
    fn debug_get_hash() {
        // Basic set up for a unit test
        testing_env!(VMContextBuilder::new().build());

        // Using a unit test to rapidly debug and iterate
        let debug_solution = "1234";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string);
    }

    #[test]
    fn check_guess_number() {
        // Set test account ID
        let dummy = AccountId::new_unchecked("ryantan.testnet".to_string());

        // Set up the testing context and unit test environment
        let context = get_context(dummy);
        testing_env!(context.build());

        // Set up contract object and call the new method
        let mut contract = Contract::new
            ("03ac674216f3e15c761ee1a5e255f067953623c8b388b4459e13f978d7c846f4".to_string(),);

        let mut guess_result = contract.guess_number("4321".to_string());
        assert!(!(guess_result), "Expected a failure from the wrong guess");
        assert_eq!(get_logs(), ["Better luck next time!"],
            "Expected a failure log.");

        guess_result = contract.guess_number("1234".to_string());
        assert!(guess_result, "Expected the correct answer to return true.");
        assert_eq!(get_logs(), ["Better luck next time!", "You're the first to find the winning number!"],
            "Expected a successful log after the previous failed log.");

        guess_result = contract.guess_number("1234".to_string());
        assert!(guess_result, "Expected the correct answer to return true.");
        assert_eq!(get_logs(), ["Better luck next time!", "You're the first to find the winning number!",
           "You've found the winning number! But someone found it before you!"],
           "Expected a successful log w/o payout after the previous successful log.");
    }
}
