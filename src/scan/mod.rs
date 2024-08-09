pub mod bitcoin;
pub mod common;
pub mod evm;
use crate::CliOptions;
use common::{errors::OkoResult, types::NetworkType};
use evm::evm_net_scan::start_scan_evm_networks;

// /// Small struct to hold statistics together
// struct WorkerStats {
//     pub started_at: Instant,
//     pub last_log: Instant,
//     pub measure_frame: Duration,
// }

// impl WorkerStats {
//     fn new() -> Self {
//         Self {
//             started_at: Instant::now(),
//             last_log: Instant::now(),
//             measure_frame: Duration::from_secs(10),
//         }
//     }
// }

pub struct BlockchainScanner {
    blockchain_network: NetworkType,
    blockchain_rpc_modules: String,
    folder: String,
    // stats: WorkerStats, // struct for thread management & statistics
}

impl BlockchainScanner {
    /// Instantiates a new scanner.
    pub fn new(options: &CliOptions) -> Self {
        info!(target: "scanner", "Scanning {} blockchain ...", options.network.name);

        Self {
            blockchain_network: options.network.clone(),
            blockchain_rpc_modules: options.rpc_modules.clone(),
            folder: options.folder.clone(),
            // stats: WorkerStats::new(),
        }
    }

    pub fn start(&mut self) -> OkoResult<()> {
        debug!(target: "scanner", "Starting worker ...");

        println!("blockchain_network.name: {}", self.blockchain_network.name);
        println!("self.folder: {}", self.folder);
        println!("blockchain_rpc_modules: {}", self.blockchain_rpc_modules);

        self.on_start()
    }

    fn on_start(&mut self) -> OkoResult<()> {
        match self.get_blockchain_name() {
            "ethereum" => {
                info!(target: "main", "Start scan Ethereum");

                start_scan_evm_networks(
                    self.blockchain_rpc_modules.to_string(),
                    self.folder.clone(),
                );
            }
            "" => {
                println!("choose");
            }
            _ => {
                println!("error with: {:?}", self.blockchain_network);
            }
        }
        Ok(())
    }

    fn get_blockchain_name(&self) -> &str {
        let res = self.blockchain_network.name.as_str();
        return res;
    }

    // Triggers the on_complete() callback and updates statistics.
    // fn on_complete(&mut self) -> OkoResult<()> {
    //     info!(target: "scanner", "Done. Processed in {:.2} minutes.",
    //     (Instant::now() - self.stats.started_at).as_secs_f32() / 60.0);

    //     // self.on_complete()?;
    //     trace!(target: "scanner", "on_complete() called");
    //     Ok(())
    // }

    // fn print_progress(&mut self) {
    //     todo!()
    // }
}
