use super::{
    errors::{OkoError, OkoErrorKind},
    networks::Ethereum,
};
use sha3::{Digest, Sha3_256};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
pub enum NetworkMode {
    Mainnet,
    _Devnet,
    _Testnet,
}
pub trait Network {
    fn name(&self) -> String;
    fn r#type(&self) -> NetworkMode;
    fn chain_id(&self) -> u8;
    fn network_id(&self) -> u8;
    fn genesis(&self) -> Sha3_256;
    fn folder(&self) -> PathBuf;
    fn rpcs(&self) -> Vec<String>;
}

impl Network for Ethereum {
    fn name(&self) -> String {
        String::from("Ethereum")
    }
    fn r#type(&self) -> NetworkMode {
        NetworkMode::Mainnet
    }
    fn chain_id(&self) -> u8 {
        1
    }
    fn network_id(&self) -> u8 {
        1
    }
    fn genesis(&self) -> Sha3_256 {
        let mut hasher = Sha3_256::new();
        hasher.update(b"d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3"); // 0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3
        hasher
    }
    fn folder(&self) -> PathBuf {
        Path::new(".oko").join("ethereum")
    }
    fn rpcs(&self) -> Vec<String> {
        vec![String::from("")]
    }
}

#[derive(Debug, Clone)]
pub struct NetworkType {
    pub name: String,
    pub chain_id: u8,
    pub network_id: u8,
    pub genesis_hash: Sha3_256,
    pub folder: PathBuf,
    pub rpcs: Vec<String>,
}

impl Default for NetworkType {
    fn default() -> Self {
        NetworkType::from(Ethereum)
    }
}

impl<T: Network> From<T> for NetworkType {
    fn from(network: T) -> Self {
        NetworkType {
            name: network.name(),
            chain_id: network.chain_id(),
            network_id: network.network_id(),
            genesis_hash: network.genesis(),
            folder: network.folder(),
            rpcs: network.rpcs(),
        }
    }
}

impl FromStr for NetworkType {
    type Err = OkoError;
    fn from_str(network_name: &str) -> Result<Self, OkoError> {
        match network_name {
            "ethereum" => Ok(NetworkType::from(Ethereum)),
            n => {
                let e = OkoError::new(OkoErrorKind::InvalidArgsError)
                    .join_msg(&format!("There is no impl for `{}`!", n));
                Err(e)
            }
        }
    }
}
