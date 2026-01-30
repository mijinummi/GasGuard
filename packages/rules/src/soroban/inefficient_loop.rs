#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct InefficientLoop;

#[contractimpl]
impl InefficientLoop {
    pub fn sum(env: Env, n: u32) -> u32 {
        let mut total = 0;

        // ❌ Inefficient: loop + storage access
        for i in 0..n {
            let val: u32 = env.storage().instance().get(&i).unwrap_or(0);
            total += val;
        }

        total
    }
}
