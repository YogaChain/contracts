use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub balances: HashMap<String, u64>,
}

impl Token {
    /// Initializes the CAKRA token with a total supply
    pub fn new(name: &str, symbol: &str, total_supply: u64) -> Self {
        let mut balances = HashMap::new();
        balances.insert("genesis".to_string(), total_supply);
        Self {
            name: name.to_string(),
            symbol: symbol.to_string(),
            total_supply,
            balances,
        }
    }

    /// Transfers tokens between accounts
    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> bool {
        if let Some(balance) = self.balances.get_mut(from) {
            if *balance >= amount {
                *balance -= amount;
                *self.balances.entry(to.to_string()).or_insert(0) += amount;
                return true;
            }
        }
        false
    }

    /// Retrieves the balance of an account
    pub fn get_balance(&self, account: &str) -> u64 {
        *self.balances.get(account).unwrap_or(&0)
    }
}
