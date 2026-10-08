#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

#[contract]
pub struct RemitLoopContract;

#[contractimpl]
impl RemitLoopContract {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&symbol_short!("ADMIN"), &admin);
    }

    pub fn record(env: Env, actor: Address, value: i128) {
        actor.require_auth();
        let key = symbol_short!("VALUE");
        env.storage().instance().set(&key, &value);
    }

    pub fn read(env: Env) -> i128 {
        env.storage().instance().get(&symbol_short!("VALUE")).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn records_value() {
        let env = Env::default();
        let id = env.register(RemitLoopContract, ());
        let client = RemitLoopContractClient::new(&env, &id);
        let actor = env.accounts().generate();
        client.initialize(&actor);
        client.record(&actor, &42);
        assert_eq!(client.read(), 42);
    }
}
