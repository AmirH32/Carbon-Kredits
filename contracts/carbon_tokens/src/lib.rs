#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};
// Floats and floating point math are not supported.
#[contract]
pub struct CarbonTokenContract;

#[contractimpl]
impl CarbonTokenContract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

mod test;
