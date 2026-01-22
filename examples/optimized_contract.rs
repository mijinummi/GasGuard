// Example of an optimized Soroban contract with no unused state variables

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub struct OptimizedTokenContract {
    pub owner: Address,
    pub total_supply: u64,
    pub balances: soroban_sdk::Map<Address, u64>,
}

#[contractimpl]
impl OptimizedTokenContract {
    pub fn new(env: Env, owner: Address, initial_supply: u64) -> Self {
        let mut contract = Self {
            owner: owner.clone(),
            total_supply: initial_supply,
            balances: soroban_sdk::Map::new(&env),
        };
        
        // Initialize the owner's balance
        contract.balances.set(env, &owner, initial_supply);
        
        contract
    }
    
    pub fn balance_of(&self, env: Env, account: Address) -> u64 {
        self.balances.get(env, &account).unwrap_or(0)
    }
    
    pub fn transfer(&mut self, env: Env, from: Address, to: Address, amount: u64) -> bool {
        let from_balance = self.balances.get(env, &from).unwrap_or(0);
        
        if from_balance < amount {
            return false;
        }
        
        let to_balance = self.balances.get(env, &to).unwrap_or(0);
        
        self.balances.set(env, &from, from_balance - amount);
        self.balances.set(env, &to, to_balance + amount);
        
        true
    }
    
    pub fn get_owner(&self) -> &Address {
        &self.owner
    }
    
    pub fn get_total_supply(&self) -> u64 {
        self.total_supply
    }
    
    pub fn mint(&mut self, env: Env, to: Address, amount: u64) {
        let current_balance = self.balances.get(env, &to).unwrap_or(0);
        self.balances.set(env, &to, current_balance + amount);
        self.total_supply += amount;
    }
    
    pub fn burn(&mut self, env: Env, from: Address, amount: u64) -> bool {
        let from_balance = self.balances.get(env, &from).unwrap_or(0);
        
        if from_balance < amount {
            return false;
        }
        
        self.balances.set(env, &from, from_balance - amount);
        self.total_supply -= amount;
        
        true
    }
}
