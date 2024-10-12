#![no_std]  // We're using no_std since this is a Wasm contract
extern crate soroban_sdk;  // Import Soroban SDK

use soroban_sdk::{contractimpl, Env, Address, Symbol, Vec, Bytes, IntoVal};

// Data storage for our contract
struct CarbonCreditContract;

// Structure for Project Approval
#[derive(Clone)]
struct Project {
    approved: bool,
}

// Token for Carbon Credits
#[contractimpl]
impl CarbonCreditContract {
    
    // Mint carbon credits, only by approved projects
    pub fn mint(env: Env, project: Address, amount: i128) {
        // Ensure project is approved before allowing minting
        assert_eq!(
            Self::is_approved_project(&env, &project),
            true,
            "Project not approved"
        );
        
        // Mint tokens (carbon credits) to the project
        env.storage().increment(&project, amount);
    }

    // Transfer carbon credits between users
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        // Ensure sender has enough balance
        let balance = env.storage().get::<i128>(&from).unwrap_or(0);
        assert!(balance >= amount, "Insufficient balance");

        // Deduct from sender's balance
        env.storage().set(&from, balance - amount);

        // Credit to receiver
        let receiver_balance = env.storage().get::<i128>(&to).unwrap_or(0);
        env.storage().set(&to, receiver_balance + amount);
    }

    // Retire carbon credits (burning tokens)
    pub fn retire(env: Env, from: Address, amount: i128) {
        // Ensure the sender has enough tokens to burn
        let balance = env.storage().get::<i128>(&from).unwrap_or(0);
        assert!(balance >= amount, "Insufficient balance to retire");

        // Burn tokens (remove them from circulation)
        env.storage().set(&from, balance - amount);
    }

    // Approve a project to mint credits (only admin can call)
    pub fn approve_project(env: Env, admin: Address, project: Address) {
        // Check if admin is authorized (you can add your custom admin check logic)
        assert!(admin == env.current_contract_address(), "Unauthorized");

        // Approve the project
        let project_data = Project { approved: true };
        env.storage().set(&project, project_data);
    }

    // Revoke a project approval (only admin can call)
    pub fn revoke_project(env: Env, admin: Address, project: Address) {
        // Check if admin is authorized
        assert!(admin == env.current_contract_address(), "Unauthorized");

        // Revoke project approval
        let project_data = Project { approved: false };
        env.storage().set(&project, project_data);
    }

    // Check if a project is approved (internal function)
    fn is_approved_project(env: &Env, project: &Address) -> bool {
        let project_data: Option<Project> = env.storage().get(project);
        project_data.map_or(false, |p| p.approved)
    }
}