use pqcrypto_dilithium::dilithium2::{sign, verify, keypair as dilithium_keypair};

pub struct ContractSecurity;

impl ContractSecurity {
    /// Generates contract security keys
    pub fn generate_contract_keys() -> (Vec<u8>, Vec<u8>) {
        let (public_key, secret_key) = dilithium_keypair();
        (public_key.as_bytes().to_vec(), secret_key.as_bytes().to_vec())
    }

    /// Signs contract execution
    pub fn sign_execution(execution_data: &[u8], private_key: &[u8]) -> Vec<u8> {
        sign(execution_data, private_key)
    }

    /// Verifies contract execution integrity
    pub fn verify_execution(execution_data: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        verify(execution_data, signature, public_key).is_ok()
    }
}
