#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Map, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Escrow {
    pub freelancer: Symbol,
    pub client: Symbol,
    pub amount: u64,
    pub active: bool,
    pub released: bool,
}

#[contract]
pub struct FreelanceEscrow;

const ESCROW_COUNTER: Symbol = symbol_short!("ESRW_CONT");
const ESCROWS: Symbol = symbol_short!("ESCROWS");

#[contractimpl]
impl FreelanceEscrow {
    pub fn create_escrow(env: Env, freelancer: Symbol, client: Symbol, amount: u64) -> u32 {
        let mut counter: u32 = env.storage().instance().get(&ESCROW_COUNTER).unwrap_or(0);
        counter += 1;

        let escrow = Escrow {
            freelancer,
            client,
            amount,
            active: true,
            released: false,
        };

        let mut escrows: Map<u32, Escrow> = env
            .storage()
            .instance()
            .get(&ESCROWS)
            .unwrap_or(Map::new(&env));
        escrows.set(counter, escrow);
        env.storage().instance().set(&ESCROWS, &escrows);
        env.storage().instance().set(&ESCROW_COUNTER, &counter);

        counter
    }

    pub fn release_funds(env: Env, escrow_id: u32) {
        let mut escrows: Map<u32, Escrow> = env
            .storage()
            .instance()
            .get(&ESCROWS)
            .unwrap_or(Map::new(&env));

        if let Some(mut escrow) = escrows.get(escrow_id) {
            if !escrow.active {
                panic!("Escrow is not active anymore");
            }

            if escrow.released {
                panic!("Funds have already been released");
            }

            escrow.released = true;
            escrows.set(escrow_id, escrow);
            env.storage().instance().set(&ESCROWS, &escrows);
        } else {
            panic!("Escrow not found");
        }
    }

    pub fn cancel_escrow(env: Env, escrow_id: u32) {
        let mut escrows: Map<u32, Escrow> = env
            .storage()
            .instance()
            .get(&ESCROWS)
            .unwrap_or(Map::new(&env));

        if let Some(mut escrow) = escrows.get(escrow_id) {
            if !escrow.active {
                panic!("Escrow is already canceled");
            }

            escrow.active = false;
            escrows.set(escrow_id, escrow);
            env.storage().instance().set(&ESCROWS, &escrows);
        } else {
            panic!("Escrow not found");
        }
    }

    pub fn get_escrow(env: Env, escrow_id: u32) -> Escrow {
        let escrows: Map<u32, Escrow> = env
            .storage()
            .instance()
            .get(&ESCROWS)
            .unwrap_or(Map::new(&env));

        escrows.get(escrow_id).unwrap_or_else(|| panic!("Escrow not found"))
    }
}
