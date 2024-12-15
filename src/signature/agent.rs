use alloy_primitives::{Address, B256}; // Instead of H256
use alloy_sol_types::{eip712_domain, sol, Eip712Domain}; // For EIP-712 support

pub(crate) mod l1 {
    use std::str::FromStr;

    use alloy_primitives::Address;
    use serde::Serialize;

    use super::*;

    sol! {
        #[derive(Debug, Serialize)]
        struct Agent {
            string source;
            bytes32 connection_id;
        }
    }

    pub(crate) fn domain() -> Eip712Domain {
        eip712_domain! {
            name: "Exchange",
            version: "1",
            chain_id: 1337,
            verifying_contract: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
        }
    }
}
