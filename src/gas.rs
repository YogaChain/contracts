use std::collections::HashMap;
use crate::validator::Validator;

/// Represents a gas fee system for smart contract execution
pub struct GasMeter {
    pub base_fee: u64,
    pub congestion_multiplier: f64,
    pub validator_discounts: HashMap<String, f64>,
}

impl GasMeter {
    /// Initializes the gas system with a base fee
    pub fn new(base_fee: u64) -> Self {
        Self {
            base_fee,
            congestion_multiplier: 1.0,
            validator_discounts: HashMap::new(),
        }
    }

    /// Calculates gas cost based on contract complexity & network congestion
    pub fn calculate_gas(contract_code: &str, function: &str) -> u64 {
        let base_gas_cost = Self::estimate_base_gas(contract_code, function);
        let congestion_factor = Self::network_congestion_factor();
        (base_gas_cost as f64 * congestion_factor) as u64
    }

    /// Estimates base gas cost based on contract complexity
    fn estimate_base_gas(contract_code: &str, function: &str) -> u64 {
        let complexity_factor = contract_code.len() as u64 / 100;
        let function_factor = function.len() as u64 * 10;
        21000 + (complexity_factor * 5) + function_factor
    }

    /// Adjusts gas pricing based on network congestion levels
    fn network_congestion_factor() -> f64 {
        let simulated_network_load = rand::random::<u8>() % 100;
        if simulated_network_load < 30 {
            0.8 // Low congestion, reduced fees
        } else if simulated_network_load < 70 {
            1.0 // Normal load, standard fees
        } else {
            1.5 // High congestion, increased fees
        }
    }

    /// Applies validator discount based on eco-score & reputation
    pub fn apply_validator_discount(&mut self, validator: &Validator) {
        let discount = if validator.eco_score > 90 {
            0.3 // 30% discount for eco-friendly validators
        } else if validator.reputation > 0.9 {
            0.2 // 20% discount for high-reputation validators
        } else {
            0.0 // No discount
        };
        self.validator_discounts.insert(validator.id.clone(), discount);
    }

    /// Calculates the final gas fee after validator discount
    pub fn calculate_final_gas(&self, validator_id: &str, base_gas: u64) -> u64 {
        if let Some(discount) = self.validator_discounts.get(validator_id) {
            (base_gas as f64 * (1.0 - discount)) as u64
        } else {
            base_gas
        }
    }
}
