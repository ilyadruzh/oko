pub mod evm_net_scan;
pub mod types;

// Small struct to hold statistics together
// struct WorkerStats {
//     pub started_at: Instant,
//     pub last_log: Instant,
//     pub measure_frame: Duration,
// }

// impl WorkerStats {
//     // fn new() -> Self {
//     //     Self {
//     //         started_at: Instant::now(),
//     //         last_log: Instant::now(),
//     //         measure_frame: Duration::from_secs(10),
//     //     }
//     // }
// }

pub struct EvmScanner {
    // chain_storage: ChainStorage, // Hash storage with the longest chain
    // stats: WorkerStats, // struct for thread management & statistics
}

impl EvmScanner {
    // Instantiates a new scanner.
    // pub fn new(options: CliOptions) -> Self {
    //     info!(target: "scanner", "Scanning {} blockchain ...", options.network.name);
    //     Self {
    //         // chain_storage,
    //         stats: WorkerStats::new(),
    //     }
    // }

    // pub fn start(&mut self) -> OkoResult<()> {
    //     debug!(target: "scanner", "Starting worker ...");
    //     self.on_start()
    // }

    // fn on_start(&mut self) -> OkoResult<()> {
    //     start_scan_evm_networks(self.);
    //     Ok(())
    // }

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
