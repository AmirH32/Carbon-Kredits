#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol, Vec, Val, TryFromVal, symbol_short};


#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    TokenData,
    ContractData,
}

pub struct TokenData {
    pub total_supply: u128,
    pub balances: Vec<(Address, u128)>,
}

// Implement TryFromVal for TokenData
impl TryFromVal<Env, Val> for TokenData {
    type Error = Err
    fn try_from_val(env: &Env, val: Val) -> Result<Self, soroban_sdk::Error> {
        if let Val::Object(obj) = val {
            let total_supply: u128 = obj.get(&Symbol::from_str("total_supply"))?.try_into_val(env)?;
            let balances: Vec<(Address, u128)> = obj.get(&Symbol::from_str("balances"))?.try_into_val(env)?;
            Ok(TokenData { total_supply, balances })
        } else {
            Err(soroban_sdk::Error::UnexpectedType)
        }
    }
}

// Implement IntoVal for TokenData
impl From<TokenData> for Val {
    fn from(token_data: TokenData) -> Val {
        let mut obj = Val::object();
        obj.set(&Symbol::from_str("total_supply"), token_data.total_supply.into());
        obj.set(&Symbol::from_str("balances"), token_data.balances.into());
        obj
    }
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    const EVENT_TOKENS_MINTED: Symbol = symbol_short!("Minted");
    const EVENT_TOKENS_TRANSFERRED: Symbol = symbol_short!("Xfered");

    pub fn initialise(e: Env, initial_supply: u128) {
        let mut token_data = TokenData {
            total_supply: initial_supply,
            balances: Vec::new(&e),
        };

        let caller = e.invoker();
        Self::mint(e, token_data, caller, initial_supply); // Mutable reference to update state
        
        e.storage().instance().set(&DataKey::TokenData, &token_data);
    }

    pub fn mint(e: Env, token_data: TokenData, recipient: Address, amount: u128) {
        if amount == 0 {
            panic!("Cannot mint zero tokens");
        }

        let mut balances = token_data.balances.clone();
        let mut total_supply = token_data.total_supply;

        let mut found = false;
        for (addr, balance) in balances.iter_mut() {
            if addr == recipient {
                *balance += amount;
                found = true;
                break;
            }
        }

        if !found {
            balances.push((recipient, amount));
        }

        total_supply += amount;

        let updated_data = TokenData {
            total_supply,
            balances,
        };

        e.storage().instance().set(&DataKey::TokenData, &updated_data);

        e.events().publish(
            (Self::EVENT_TOKENS_MINTED,),
            (recipient, amount),
        );
    }

    pub fn transfer(e: Env, recipient: Address, amount: u128) {
        let caller = e.invoker();
        let mut token_data = load_token_data(&e);

        if amount == 0 {
            panic!("Cannot transfer zero tokens");
        }

        let mut caller_balance = 0;
        for (addr, balance) in token_data.balances.iter_mut() {
            if addr == caller {
                caller_balance = *balance;
                *balance -= amount;
                break;
            }
        }

        if caller_balance < amount {
            panic!("Insufficient balance");
        }

        let mut found = false;
        for (addr, balance) in token_data.balances.iter_mut() {
            if addr == recipient {
                *balance += amount;
                found = true;
                break;
            }
        }

        if !found {
            token_data.balances.push((recipient, amount));
        }

        e.storage().instance().set(&DataKey::TokenData, &token_data);

        e.events().publish(
            (Self::EVENT_TOKENS_TRANSFERRED,),
            (caller, recipient, amount),
        );
    }

    pub fn total_supply(e: Env) -> u128 {
        let token_data = load_token_data(&e);
        token_data.total_supply
    }

    pub fn balance_of(e: Env, address: Address) -> u128 {
        let token_data = load_token_data(&e);
        for (addr, balance) in token_data.balances.iter() {
            if addr == address {
                return *balance;
            }
        }
        0
    }
}

fn load_token_data(e: &Env) -> TokenData {
    e.storage().instance().get(&DataKey::TokenData).unwrap_or(TokenData {
        total_supply: 0,
        balances: Vec::new(&e),
    })
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
