#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol, Vec, symbol_short}; // Removed unused imports

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    ContractData,
}

#[derive(Clone)]
#[contracttype]
pub struct ContractData {
    pub buyer: Address,
    pub price_per_token: u32,
    pub total_value: i128, // Total number of tokens required
    pub assigned_tokens: i128, // Number of tokens already assigned
}

#[contract]
pub struct CarbonCreditContract;

#[contractimpl]
impl CarbonCreditContract {

    // Event name constants for emitting blockchain events
    const EVENT_CONTRACT_CREATED: Symbol = symbol_short!("CrtCrd"); // Shortened
    const EVENT_TOKENS_ASSIGNED: Symbol = symbol_short!("TknAsg"); // Shortened

    // Creates a contract initiated by the buyer (carbon-positive company)
    pub fn create(
        e: Env,
        buyer: Address,
        price_per_token: u32,
        total_value: i128,
    ) {
        if e.storage().instance().has(&DataKey::ContractData) {
            panic!("Contract already exists");
        }
        if price_per_token == 0 || total_value <= 0 {
            panic!("Invalid price or total value");
        }

        // Authorize the buyer
        buyer.require_auth();

        let contract_data = ContractData {
            buyer: buyer.clone(),
            price_per_token,
            total_value,
            assigned_tokens: 0,
        };

        // Store the contract data in the contract's state
        write_contract_data(&e, &contract_data);

        // Emit event: Contract created
        e.events().publish(
            (Self::EVENT_CONTRACT_CREATED,),
            (buyer, price_per_token, total_value),
        );
    }

    // Allows the seller (carbon-negative company) to assign tokens to the contract and retire them
    pub fn assign_tokens(
        e: Env,
        seller: Address,
        token: Address,   // Address of the carbon token contract
        token_amount: i128,
    ) {
        // Load the current contract data
        let mut contract_data = load_contract_data(&e);

        if token_amount <= 0 {
            panic!("Token amount must be greater than zero");
        }

        // Authorize the seller to assign tokens
        seller.require_auth();

        // Prepare the token client and perform the token transfer
        let token_client = token::Client::new(&e, &token);

        // Burn the tokens from the seller to retire them
        token_client.burn(&seller, &token_amount);

        // Update the assigned token amount
        contract_data.assigned_tokens += token_amount;
        let outstanding_tokens = contract_data.total_value - contract_data.assigned_tokens;

        // Store the updated contract data
        write_contract_data(&e, &contract_data);

        // Emit event: Tokens assigned and the outstanding balance
        e.events().publish(
            (Self::EVENT_TOKENS_ASSIGNED,),
            (seller, token_amount, outstanding_tokens),
        );

        // Optionally check if the contract has been fulfilled
        if outstanding_tokens <= 0 {
            // All tokens have been assigned, the contract is fulfilled
            // Handle fulfillment logic if needed (e.g., notify buyer, issue rewards, etc.)
        }
    }

    // Returns the current contract data
    pub fn get_contract_data(e: Env) -> ContractData {
        load_contract_data(&e)
    }
}

// Helper function to load contract data from storage
fn load_contract_data(e: &Env) -> ContractData {
    e.storage().instance().get(&DataKey::ContractData).unwrap()
}

// Helper function to write contract data to storage
fn write_contract_data(e: &Env, contract_data: &ContractData) {
    e.storage().instance().set(&DataKey::ContractData, contract_data);
}

mod test;
