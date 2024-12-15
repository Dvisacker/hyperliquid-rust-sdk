use std::{borrow::Cow, str::FromStr};

use crate::exchange::{cancel::CancelRequest, modify::ModifyRequest, order::OrderRequest};
use alloy::sol;

// use alloy::dyn_abi::eip712::{encode_eip712_type, Eip712Domain, Eip712, Eip712Error};
// use alloy::dyn_abi::eip712::{encode_eip712_type, Eip712Domain, Eip712, Eip712Error};

pub use alloy::dyn_abi::eip712::{
    parser as eip712_parser, Eip712Types, PropertyDef, Resolver, TypeDef, TypedData,
};
// pub use alloy::dyn_abi::{
//     eip712::{encode_type, Domain, Error as Eip712Error},
//     eip712_parser, Eip712Types, PropertyDef, Resolver, TypeDef, TypedData,
// };
use alloy_primitives::{Address, U256};

use alloy_dyn_abi::{DynSolType, DynSolValue};
use alloy_sol_types::{Eip712Domain, SolStruct, SolValue};

use serde::{Deserialize, Serialize};

use super::cancel::CancelRequestCloid;

pub(crate) const HYPERLIQUID_EIP_PREFIX: &str = "HyperliquidTransaction:";

fn eip_712_domain(chain_id: U256) -> Eip712Domain {
    Eip712Domain {
        name: Some(Cow::Borrowed("HyperliquidSignTransaction")),
        version: Some(Cow::Borrowed("1")),
        chain_id: Some(chain_id),
        verifying_contract: Some(
            "0x0000000000000000000000000000000000000000"
                .parse()
                .unwrap(),
        ),
        salt: None,
    }
}

sol! {
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct UsdSend {
        uint256 signature_chain_id;
        string hyperliquid_chain;
        string destination;
        string amount;
        uint64 time;
    }
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct UsdSend {
//     pub signature_chain_id: U256,
//     pub hyperliquid_chain: String,
//     pub destination: String,
//     pub amount: String,
//     pub time: u64,
// }

// impl Eip712 for UsdSend {
//     type Error = Eip712Error;

//     fn domain(&self) -> Result<Eip712Domain, Self::Error> {
//         Ok(Eip712Domain {
//             name: Some("HyperliquidSignTransaction".into()),
//             version: Some("1".into()),
//             chain_id: Some(self.signature_chain_id.into()),
//             verifying_contract: Some(
//                 Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
//             ),
//             salt: None,
//         })
//     }

//     fn type_hash() -> Result<[u8; 32], Self::Error> {
//         let fields = vec![
//             ("hyperliquidChain", "string"),
//             ("destination", "string"),
//             ("amount", "string"),
//             ("time", "uint64"),
//         ];
//         Ok(encode_type(
//             format!("{HYPERLIQUID_EIP_PREFIX}UsdSend"),
//             &fields,
//         ))
//     }

//     fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
//         // Implementation using alloy types
//         // This would need to be updated to use alloy's encoding functions
//         unimplemented!("Update to use alloy encoding")
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLeverage {
    pub asset: u32,
    pub is_cross: bool,
    pub leverage: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIsolatedMargin {
    pub asset: u32,
    pub is_buy: bool,
    pub ntli: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkOrder {
    pub orders: Vec<OrderRequest>,
    pub grouping: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancel {
    pub cancels: Vec<CancelRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkModify {
    pub modifies: Vec<ModifyRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancelCloid {
    pub cancels: Vec<CancelRequestCloid>,
}

sol! {
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct ApproveAgent {
        uint256 signature_chain_id;
        string hyperliquid_chain;
        address agent_address;
        string agent_name;
        uint64 nonce;
    }
}

// impl Eip712 for ApproveAgent {
//     type Error = Eip712Error;

//     fn domain(&self) -> Result<Eip712Domain, Self::Error> {
//         Ok(eip_712_domain(self.signature_chain_id))
//     }

//     fn type_hash() -> Result<[u8; 32], Self::Error> {
//         Ok(eip712::make_type_hash(
//             format!("{HYPERLIQUID_EIP_PREFIX}ApproveAgent"),
//             &[
//                 ("hyperliquidChain".to_string(), ParamType::String),
//                 ("agentAddress".to_string(), ParamType::Address),
//                 ("agentName".to_string(), ParamType::String),
//                 ("nonce".to_string(), ParamType::Uint(64)),
//             ],
//         ))
//     }

//     fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
//         let Self {
//             signature_chain_id: _,
//             hyperliquid_chain,
//             agent_address,
//             agent_name,
//             nonce,
//         } = self;
//         let items = vec![
//             ethers::abi::Token::Uint(Self::type_hash()?.into()),
//             encode_eip712_type(hyperliquid_chain.clone().into_token()),
//             encode_eip712_type(agent_address.into_token()),
//             encode_eip712_type(agent_name.clone().unwrap_or_default().into_token()),
//             encode_eip712_type(nonce.into_token()),
//         ];
//         Ok(keccak256(encode(&items)))
//     }
// }

sol! {
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Withdraw3 {
        string hyperliquid_chain;
        uint256 signature_chain_id;
        string amount;
        uint64 time;
        string destination;
    }
}

// impl Eip712 for Withdraw3 {
//     type Error = Eip712Error;

//     fn domain(&self) -> Result<Eip712Domain, Self::Error> {
//         Ok(eip_712_domain(self.signature_chain_id))
//     }

//     fn type_hash() -> Result<[u8; 32], Self::Error> {
//         Ok(eip712::make_type_hash(
//             format!("{HYPERLIQUID_EIP_PREFIX}Withdraw"),
//             &[
//                 ("hyperliquidChain".to_string(), ParamType::String),
//                 ("destination".to_string(), ParamType::String),
//                 ("amount".to_string(), ParamType::String),
//                 ("time".to_string(), ParamType::Uint(64)),
//             ],
//         ))
//     }

//     fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
//         let Self {
//             signature_chain_id: _,
//             hyperliquid_chain,
//             amount,
//             time,
//             destination,
//         } = self;
//         let items = vec![
//             ethers::abi::Token::Uint(Self::type_hash()?.into()),
//             encode_eip712_type(hyperliquid_chain.clone().into_token()),
//             encode_eip712_type(destination.clone().into_token()),
//             encode_eip712_type(amount.clone().into_token()),
//             encode_eip712_type(time.into_token()),
//         ];
//         Ok(keccak256(encode(&items)))
//     }
// }

sol! {
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct SpotSend {
        string hyperliquid_chain;
        uint256 signature_chain_id;
        string destination;
        string token;
        string amount;
        uint64 time;
    }
}

// impl Eip712 for SpotSend {
//     type Error = Eip712Error;

//     fn domain(&self) -> Result<Eip712Domain, Self::Error> {
//         Ok(eip_712_domain(self.signature_chain_id))
//     }

//     fn type_hash() -> Result<[u8; 32], Self::Error> {
//         Ok(eip712::make_type_hash(
//             format!("{HYPERLIQUID_EIP_PREFIX}SpotSend"),
//             &[
//                 ("hyperliquidChain".to_string(), ParamType::String),
//                 ("destination".to_string(), ParamType::String),
//                 ("token".to_string(), ParamType::String),
//                 ("amount".to_string(), ParamType::String),
//                 ("time".to_string(), ParamType::Uint(64)),
//             ],
//         ))
//     }

//     fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
//         let Self {
//             signature_chain_id: _,
//             hyperliquid_chain,
//             destination,
//             token,
//             amount,
//             time,
//         } = self;
//         let items = vec![
//             ethers::abi::Token::Uint(Self::type_hash()?.into()),
//             encode_eip712_type(hyperliquid_chain.clone().into_token()),
//             encode_eip712_type(destination.clone().into_token()),
//             encode_eip712_type(token.clone().into_token()),
//             encode_eip712_type(amount.clone().into_token()),
//             encode_eip712_type(time.into_token()),
//         ];
//         Ok(keccak256(encode(&items)))
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotUser {
    pub class_transfer: ClassTransfer,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClassTransfer {
    pub usdc: u64,
    pub to_perp: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VaultTransfer {
    pub vault_address: Address,
    pub is_deposit: bool,
    pub usd: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetReferrer {
    pub code: String,
}
