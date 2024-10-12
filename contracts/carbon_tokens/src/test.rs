#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as TestAddress, Env, Symbol};
    use soroban_sdk::testutils::{MockAuth, Events};

    #[test]
    fn test_create_contract() {
        // Initialize the test environment
        let e = Env::default();

        // Create test buyer address
        let buyer = TestAddress::random(&e);

        // Set contract parameters
        let price_per_token = 10;
        let total_value = 100;

        // Call the create function
        CarbonKreditContract::create(e.clone(), buyer.clone(), price_per_token, total_value);

        // Verify the contract data has been saved correctly
        let contract_data = CarbonKreditContract::get_contract_data(e.clone());
        assert_eq!(contract_data.buyer, buyer);
        assert_eq!(contract_data.price_per_token, price_per_token);
        assert_eq!(contract_data.total_value, total_value);
        assert_eq!(contract_data.assigned_tokens, 0); // No tokens assigned yet

        // Verify that the ContractCreated event was emitted
        let events = e.events().all();
        assert_eq!(events.len(), 1);
        let event = events.first().unwrap();
        assert_eq!(event.data.len(), 3);
        assert_eq!(event.data[0], buyer.clone().into());
        assert_eq!(event.data[1], price_per_token.into());
        assert_eq!(event.data[2], total_value.into());
    }

    #[test]
    fn test_assign_tokens_and_retire() {
        let e = Env::default();

        // Create test buyer and seller addresses
        let buyer = TestAddress::random(&e);
        let seller = TestAddress::random(&e);

        // Set contract parameters
        let price_per_token = 10;
        let total_value = 100;

        // Create contract by the buyer
        CarbonKreditContract::create(e.clone(), buyer.clone(), price_per_token, total_value);

        // Assume token has been set up; using the address as token identifier
        let token_address = TestAddress::random(&e);

        // Mock initial token balance for the seller
        let initial_token_amount = 50;

        // Assign tokens (simulate burning or retiring)
        CarbonKreditContract::assign_tokens(e.clone(), seller.clone(), token_address.clone(), initial_token_amount);

        // Verify the contract data is updated correctly
        let contract_data = CarbonKreditContract::get_contract_data(e.clone());
        assert_eq!(contract_data.assigned_tokens, initial_token_amount);
        assert_eq!(contract_data.total_value - contract_data.assigned_tokens, 50); // Outstanding tokens

        // Verify that the TokensAssigned event was emitted
        let events = e.events().all();
        assert_eq!(events.len(), 2); // One for create, one for assign
        let event = &events[1]; // Second event for token assignment
        assert_eq!(event.data.len(), 3);
        assert_eq!(event.data[0], seller.clone().into());
        assert_eq!(event.data[1], initial_token_amount.into());
        assert_eq!(event.data[2], 50.into()); // Outstanding tokens

        // You can further verify the token was "burned" depending on how the burn method is implemented in the token contract
    }

    #[test]
    fn test_fully_assign_tokens() {
        let e = Env::default();

        // Create test buyer and seller addresses
        let buyer = TestAddress::random(&e);
        let seller = TestAddress::random(&e);

        // Set contract parameters
        let price_per_token = 10;
        let total_value = 100;

        // Create contract by the buyer
        CarbonKreditContract::create(e.clone(), buyer.clone(), price_per_token, total_value);

        // Assume token has been set up; using the address as token identifier
        let token_address = TestAddress::random(&e);

        // Mock initial token balance for the seller
        let token_amount = 100; // Full amount

        // Assign tokens and burn them all
        CarbonKreditContract::assign_tokens(e.clone(), seller.clone(), token_address.clone(), token_amount);

        // Verify the contract data is updated correctly
        let contract_data = CarbonKreditContract::get_contract_data(e.clone());
        assert_eq!(contract_data.assigned_tokens, total_value);
        assert_eq!(contract_data.total_value - contract_data.assigned_tokens, 0); // Fully assigned

        // Verify that the TokensAssigned event was emitted correctly
        let events = e.events().all();
        assert_eq!(events.len(), 2); // One for create, one for assign
        let event = &events[1]; // Second event for token assignment
        assert_eq!(event.data.len(), 3);
        assert_eq!(event.data[0], seller.clone().into());
        assert_eq!(event.data[1], token_amount.into());
        assert_eq!(event.data[2], 0.into()); // No outstanding tokens

        // Optionally, you can also check for additional logic (e.g., rewards) upon full assignment
    }
}