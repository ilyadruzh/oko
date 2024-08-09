use crate::scan::common::types::{NetworkType};

#[derive(Debug, Clone)]
pub struct CliOptions {
    // Name of the callback which gets executed for each block. (See callbacks/mod.rs)
    // pub callback: Box<dyn Callback>,
    // Holds the relevant coin parameters we need for parsing
    // Enable this if you want to check the chain index integrity and merkle root for each block.
    pub scan: bool,
    // Path to directory where blk.dat files are stored
    pub update: String,
    // Path to directory where blk.dat files are stored
    pub network: NetworkType,
    // Path to directory where blk.dat files are stored
    pub rpc_modules: String,
    // Verbosity level, 0 = Error, 1 = Info, 2 = Debug, 3+ = Trace
    pub log_level_filter: log::LevelFilter,
    // Range which is considered for parsing
    // pub range: BlockHeightRange,
    pub folder: String
}
