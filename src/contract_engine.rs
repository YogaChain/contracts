use crate::contract_loader::ContractLoader;
use crate::security::ContractSecurity;
use crate::gas::GasMeter;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractExecutionResult {
    pub success: bool,
    pub gas_used: u64,
    pub output: Option<String>,
}

pub struct ContractEngine {
    contract_loader: ContractLoader,
}

impl ContractEngine {
    /// Initializes the contract engine
    pub fn new(contract_loader: ContractLoader) -> Self {
        Self { contract_loader }
    }

    /// Executes a contract function call with gas metering
    pub fn execute_contract(
        &self,
        contract_name: &str,
        function: &str,
        params: Vec<String>,
    ) -> ContractExecutionResult {
        if let Some(contract) = self.contract_loader.load_contract(contract_name) {
            let gas_cost = GasMeter::calculate_gas(&contract.code, function);

            let output = contract.call_function(function, params);
            ContractExecutionResult {
                success: true,
                gas_used: gas_cost,
                output: Some(output),
            }
        } else {
            ContractExecutionResult {
                success: false,
                gas_used: 0,
                output: None,
            }
        }
    }
}
