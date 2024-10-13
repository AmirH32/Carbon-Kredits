#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol, Vec, symbol_short};

#[derive(Clone)]
#[contracttype]
pub struct TokenData {
    pub total_supply: u128,
    pub balances: Vec<(Address, u128)>,
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    const EVENT_TOKENS_MINTED: Symbol = symbol_short!("Minted");
    const EVENT_TOKENS_TRANSFERRED: Symbol = symbol_short!("Transferred");
    
    // Creates a new token contract with an initial supply of tokens
    pub fn initialize(e: Env, initial_supply: u128) {
        let token_data = TokenData {
            total_supply: initial_supply,
            balances: Vec::new(),
        };
        
        // Mint the initial tokens to the caller
        let caller = e.invoker();
        mint(&e, &token_data, &caller, initial_supply);
    }

    // Mints new tokens to a specified address
    pub fn mint(e: Env, token_data: &TokenData, recipient: &Address, amount: u128) {
        // Validate the amount
        if amount == 0 {
            panic!("Cannot mint zero tokens");
        }
        
        // Load existing balances and update total supply
        let mut balances = token_data.balances.clone();
        let mut total_supply = token_data.total_supply;

        // Check if the recipient already has a balance
        let mut found = false;
        for (addr, balance) in balances.iter_mut() {
            if *addr == *recipient {
                *balance += amount;
                found = true;
                break;
            }
        }

        // If the recipient does not exist in the balances, add them
        if !found {
            balances.push((*recipient, amount));
        }

        total_supply += amount;

        // Store updated token data
        let updated_data = TokenData {
            total_supply,
            balances,
        };

        e.storage().instance().set(&symbol_short!("TokenData"), &updated_data);

        // Emit event for minting tokens
        e.events().publish(
            (Self::EVENT_TOKENS_MINTED,),
            (recipient, amount),
        );
    }

    // Transfers tokens from the caller to a recipient
    pub fn transfer(e: Env, recipient: Address, amount: u128) {
        let caller = e.invoker();
        let mut token_data = load_token_data(&e);

        // Validate the transfer amount
        if amount == 0 {
            panic!("Cannot transfer zero tokens");
        }

        // Load caller's balance
        let mut caller_balance = 0;
        for (addr, balance) in token_data.balances.iter_mut() {
            if *addr == caller {
                caller_balance = *balance;
                *balance -= amount; // Subtract amount from caller's balance
                break;
            }
        }

        // Check if caller has sufficient balance
        if caller_balance < amount {
            panic!("Insufficient balance");
        }

        // Update recipient's balance
        let mut found = false;
        for (addr, balance) in token_data.balances.iter_mut() {
            if *addr == recipient {
                *balance += amount; // Add amount to recipient's balance
                found = true;
                break;
            }
        }

        // If recipient doesn't exist, initialize their balance
        if !found {
            token_data.balances.push((recipient, amount));
        }

        // Store updated token data
        e.storage().instance().set(&symbol_short!("TokenData"), &token_data);

        // Emit event for transferring tokens
        e.events().publish(
            (Self::EVENT_TOKENS_TRANSFERRED,),
            (caller, recipient, amount),
        );
    }

    // Returns the total supply of tokens
    pub fn total_supply(e: Env) -> u128 {
        let token_data = load_token_data(&e);
        token_data.total_supply
    }

    // Returns the balance of a specified address
    pub fn balance_of(e: Env, address: Address) -> u128 {
        let token_data = load_token_data(&e);
        for (addr, balance) in token_data.balances.iter() {
            if *addr == address {
                return *balance;
            }
        }
        0 // If address not found, return 0
    }
}

// Helper function to load token data from storage
fn load_token_data(e: &Env) -> TokenData {
    e.storage().instance().get(&symbol_short!("TokenData")).unwrap_or(TokenData {
        total_supply: 0,
        balances: Vec::new(),
    })
}
