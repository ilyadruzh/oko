use super::callbacks::Callback;
use super::errors::{OpError, OpResult};
use crate::scan::blockchain::parser::types::CoinType;
use std::boxed::Box;
use std::fmt;
use std::path::PathBuf;

extern crate byteorder;
extern crate chrono;
extern crate rayon;
extern crate rusty_leveldb;
extern crate seek_bufread;

#[derive(Copy, Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct BlockHeightRange {
    pub start: u64,
    pub end: Option<u64>,
}

impl BlockHeightRange {
    pub fn new(start: u64, end: Option<u64>) -> OpResult<Self> {
        if end.is_some() && start >= end.unwrap() {
            return Err(OpError::from(String::from(
                "--start value must be lower than --end value",
            )));
        }
        Ok(Self { start, end })
    }

    pub fn is_default(&self) -> bool {
        self.start == 0 && self.end.is_none()
    }
}

impl fmt::Display for BlockHeightRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let end = match self.end {
            Some(e) => e.to_string(),
            None => String::from("HEAD"),
        };
        write!(f, "{}..{}", self.start, end)
    }
}

/// Holds all available user arguments
pub struct ParserOptions {
    // Name of the callback which gets executed for each block. (See callbacks/mod.rs)
    pub callback: Box<dyn Callback>,
    // Holds the relevant coin parameters we need for parsing
    pub coin: CoinType,
    // Enable this if you want to check the chain index integrity and merkle root for each block.
    pub verify: bool,
    // Path to directory where blk.dat files are stored
    pub blockchain_dir: PathBuf,
    // Verbosity level, 0 = Error, 1 = Info, 2 = Debug, 3+ = Trace
    pub log_level_filter: log::LevelFilter,
    // Range which is considered for parsing
    pub range: BlockHeightRange,
}
