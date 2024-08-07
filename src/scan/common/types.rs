
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

// use hex_literal::hex;
use sha3::{Digest, Sha3_256};

use super::errors::{OkoError, OkoErrorKind};

/// Trait to specify the underlying coin of a blockchain
/// Needs a proper magic value and a network id for address prefixes
pub trait Network {
    // Human readable coin name
    fn name(&self) -> String;
    // https://en.bitcoin.it/wiki/List_of_address_prefixes
    fn chain_id(&self) -> u8;
    fn network_id(&self) -> u8;
    // Returns genesis hash
    fn genesis(&self) -> Sha3_256;
    // Default working directory to look for datadir, for example .bitcoin
    fn default_folder(&self) -> PathBuf;
}

pub struct Ethereum;

impl Network for Ethereum {
    fn name(&self) -> String {
        String::from("ethereum")
    }
    fn chain_id(&self) -> u8 {
        0x00
    }
    fn network_id(&self) -> u8 {
        0x00
    }
    fn genesis(&self) -> Sha3_256 {
        let mut hasher = Sha3_256::new();
        // write input message
        hasher.update(b"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f");
        hasher
    }
    fn default_folder(&self) -> PathBuf {
        Path::new(".oko").join("ethereum")
    }
}

// Holds the selected coin type information
#[derive(Debug, Clone)]
pub struct NetworkType {
    pub name: String,
    pub chain_id: u8,
    pub network_id: u8,
    pub genesis_hash: Sha3_256,
    pub default_folder: PathBuf,
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
            default_folder: network.default_folder(),
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
