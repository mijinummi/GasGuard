#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct InefficientStorage;

#[contractimpl]
impl InefficientStorage {
    pub fn get_sum(env: Env) -> i32 {
        let key = Symbol::short("count");

        // ❌ Inefficient: multiple storage reads
        let a: i32 = env.storage().instance().get(&key).unwrap_or(0);
        let b: i32 = env.storage().instance().get(&key).unwrap_or(0);

        a + b
    }
}
