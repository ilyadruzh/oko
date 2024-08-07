use bitcoin::hashes::{sha256d, Hash};
use std::path::PathBuf;

// use crate::scan::bitcoin::blockchain::parser::types::CoinType;

use super::types::NetworkType;

/// Calculates merkle root for the whole block
/// See: https://en.bitcoin.it/wiki/Protocol_documentation#Merkle_Trees
pub fn merkle_root(hashes: Vec<sha256d::Hash>) -> sha256d::Hash {
    let mut hashes = hashes;

    while hashes.len() > 1 {
        // Calculates double sha hash for each pair. If len is odd, last value is ignored.
        let mut new_hashes = hashes
            .chunks(2)
            .filter(|c| c.len() == 2)
            .map(|c| sha256d::Hash::hash(&[c[0], c[1]].concat()))
            .collect::<Vec<sha256d::Hash>>();

        // If the length is odd, take the last hash twice
        if hashes.len() % 2 == 1 {
            let last_hash = hashes.last().unwrap();
            new_hashes.push(sha256d::Hash::hash(
                &[&last_hash[..], &last_hash[..]].concat(),
            ));
        }
        hashes = new_hashes;
    }
    *hashes
        .first()
        .expect("unable to calculate merkle root on empty hashes")
}

pub fn arr_to_hex(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x?}", b)).collect()
}

pub fn hex_to_vec(hex_str: &str) -> Vec<u8> {
    if hex_str.len() % 2 != 0 {
        panic!("string length is not even");
    }

    (0..hex_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).unwrap())
        .collect()
}

/// Returns default directory. TODO: test on windows
pub fn get_absolute_blockchain_dir(network: &NetworkType) -> PathBuf {
    dirs::home_dir()
        .expect("Unable to get home path from env!")
        .join(&network.default_folder)
}

/// Get mean value from u32 slice
pub fn get_mean(slice: &[u32]) -> f64 {
    if slice.is_empty() {
        return 0.00;
    }
    let sum = slice.iter().sum::<u32>();
    sum as f64 / slice.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arr_to_hex() {
        let test = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0xd6, 0x68, 0x9c, 0x08, 0x5a, 0xe1, 0x65, 0x83,
            0x1e, 0x93, 0x4f, 0xf7, 0x63, 0xae, 0x46, 0xa2, 0xa6, 0xc1, 0x72, 0xb3, 0xf1, 0xb6,
            0x0a, 0x8c, 0xe2, 0x6f,
        ];
        let expected = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";
        assert_eq!(arr_to_hex(&test), expected);
    }

    #[test]
    fn test_merkle_root() {
        let hashes = Vec::from([
            sha256d::Hash::from_byte_array([
                0x8c, 0xb1, 0xdf, 0x74, 0xdb, 0xe9, 0x80, 0xc6, 0xb9, 0x20, 0x2e, 0x91, 0x95, 0x97,
                0xa5, 0xea, 0xbe, 0xb2, 0xd3, 0x2e, 0x4d, 0xe0, 0x21, 0x4a, 0x39, 0xf8, 0x0c, 0x5f,
                0xab, 0x9e, 0x45, 0x3a,
            ]),
            sha256d::Hash::from_byte_array([
                0xb7, 0xa6, 0x06, 0x8e, 0x58, 0x14, 0x73, 0x84, 0x22, 0x76, 0x8b, 0x92, 0xb7, 0xff,
                0x81, 0xb8, 0x07, 0xfd, 0x51, 0x58, 0x71, 0xed, 0x6a, 0x41, 0x72, 0xba, 0xcc, 0x0e,
                0x6f, 0xf4, 0x38, 0xbe,
            ]),
            sha256d::Hash::from_byte_array([
                0xbe, 0x32, 0x73, 0x29, 0xc9, 0x6d, 0x01, 0xbb, 0x0e, 0xf9, 0x39, 0x77, 0xd0, 0x26,
                0xb8, 0x02, 0xdb, 0x0b, 0x59, 0xbb, 0x7b, 0xfe, 0xd9, 0x77, 0x3a, 0xf6, 0x6f, 0x2b,
                0xa1, 0xf2, 0x73, 0xd1,
            ]),
            sha256d::Hash::from_byte_array([
                0x2f, 0x05, 0xc7, 0x5f, 0x38, 0x82, 0x9e, 0xee, 0xaf, 0x84, 0x34, 0x55, 0xdf, 0x87,
                0xaa, 0xc0, 0xa7, 0xf2, 0xbb, 0x3c, 0xf2, 0x4f, 0x23, 0x91, 0xb4, 0xbb, 0x68, 0x52,
                0x3e, 0xe8, 0xd1, 0x59,
            ]),
            sha256d::Hash::from_byte_array([
                0x0c, 0xc6, 0x7a, 0x79, 0xdd, 0x56, 0x4d, 0x24, 0x55, 0xdf, 0x58, 0xb3, 0x71, 0xaf,
                0xde, 0xb1, 0xa3, 0x1f, 0x44, 0xff, 0xa0, 0x08, 0x3b, 0x9e, 0xb7, 0xef, 0x06, 0x9d,
                0xa6, 0x77, 0xce, 0xf1,
            ]),
            sha256d::Hash::from_byte_array([
                0xe0, 0x52, 0xdf, 0x8e, 0x7d, 0x50, 0xda, 0x4b, 0xe4, 0x74, 0xcd, 0x50, 0x5b, 0x21,
                0x99, 0x6b, 0x74, 0xe3, 0xd0, 0x2f, 0xbf, 0xa1, 0xaf, 0xd3, 0x9f, 0x65, 0xfe, 0x91,
                0xba, 0x3c, 0x05, 0x84,
            ]),
        ]);

        let expected = sha256d::Hash::from_byte_array([
            0x52, 0xed, 0x57, 0x8c, 0xb6, 0xed, 0x9a, 0xe5, 0xf5, 0x31, 0x6d, 0x45, 0x42, 0x9b,
            0xf6, 0x9c, 0xfd, 0xde, 0x2b, 0xe3, 0x94, 0x97, 0xba, 0x31, 0x57, 0x01, 0x64, 0xeb,
            0x22, 0x77, 0xdf, 0x9c,
        ]);

        let merkle_hash = merkle_root(hashes);
        assert_eq!(merkle_hash, expected);
    }
}
