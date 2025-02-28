use std::collections::HashMap;

pub struct SmartContract {
    pub name: String,
    pub code: String,
}

impl SmartContract {
    pub fn call_function(&self, function: &str, params: Vec<String>) -> String {
        format!("Executing `{}` on contract `{}` with params {:?}", function, self.name, params)
    }
}

pub struct ContractLoader {
    contracts: HashMap<String, SmartContract>,
}

impl ContractLoader {
    /// Initializes contract storage
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
        }
    }

    /// Loads an existing contract
    pub fn load_contract(&mut self, name: &str) -> Option<&SmartContract> {
        self.contracts.get(name)
    }

    /// Deploys a new contract
    pub fn deploy_contract(&mut self, name: &str, code: &str) {
        self.contracts.insert(name.to_string(), SmartContract {
            name: name.to_string(),
            code: code.to_string(),
        });
    }
}
