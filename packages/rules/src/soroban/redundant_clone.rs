#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Vec};

#[contract]
pub struct RedundantClone;

#[contractimpl]
impl RedundantClone {
    pub fn duplicate(env: Env, data: Vec<i32>) -> Vec<i32> {
        
        let copy = data.clone();
        copy
    }
}

