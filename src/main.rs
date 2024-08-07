pub mod cli;
pub mod database;
pub mod rest_api;
pub mod scan;

use crate::cli::*;
use crate::scan::common::errors::OkoResult;
use clap::{Arg, Command};
// use scan::bitcoin::blockchain::parser::chain::ChainStorage;
// use scan::bitcoin::blockchain::parser::BlockchainParser;
use scan::common::logger::SimpleLogger;
use scan::common::types::{Ethereum, NetworkType};
use scan::evm::evm_net_scan::start_scan_evm_networks;
use scan::evm::EvmScanner;
use std::path::PathBuf;
use std::process;

#[macro_use]
extern crate log;
extern crate bitcoin;
extern crate byteorder;
extern crate chrono;
extern crate rayon;
extern crate rusty_leveldb;
extern crate seek_bufread;

fn command() -> Command {
    let networks = [
        "bitcoin",
        "testnet3",
        "namecoin",
        "litecoin",
        "dogecoin",
        "myriadcoin",
        "unobtanium",
        "noteblockchain",
        "ethereum-mainnet",
    ];
    Command::new("oko")
        .version("0.1.0")
        // Add flags
        .arg(
            Arg::new("scan")
                .long("scan")
                .action(clap::ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool))
                .help("Scanning all info from blockchain networks"),
        )
        .arg(
            Arg::new("verbosity")
                .short('v')
                .action(clap::ArgAction::Count)
                .help("Increases verbosity level. Info=0, Debug=1, Trace=2 (default: 0)"),
        )
        // Add options
        .arg(
            Arg::new("network")
                .short('n')
                .long("network")
                .value_name("NAME")
                .value_parser(clap::builder::PossibleValuesParser::new(networks))
                .help("Specify blockchain network (default: all)"),
        )
        .arg(
            Arg::new("rpc_modules")
                .short('r')
                .long("rpc_modules")
                .help("Which RPC MODULES we collect (default: rpc_modules)"),
        )
        .arg(
            Arg::new("folder")
                .short('f')
                .long("folder")
                .help("Folder to save info (default: ./)"),
        )
}

#[tokio::main]
async fn main() {

    let general_scan: bool = false;

    let options = match parse_args(command().get_matches()) {
        Ok(o) => o,
        Err(desc) => {
            // Init logger to print outstanding error message
            SimpleLogger::init(log::LevelFilter::Debug).unwrap();
            error!(target: "main", "{}", desc);
            process::exit(1);
        }
    };

    let log_level = options.log_level_filter;
    SimpleLogger::init(log_level).expect("Unable to initialize logger!");
    info!(target: "main", "The Oko is opening... v{} ...", env!("CARGO_PKG_VERSION"));
    debug!(target: "main", "Logging level {}", log_level);

    if options.scan {
        // general_scan = true;
        info!(target: "main", "Configured to collect all information from all networks");
        start_scan_evm_networks();
    }

    // choose network
    match options.network.name.as_str() {
        "ethereum" => {
            info!(target: "main", "Configured to collect all information from all networks");
        }
        _ => {
            info!(target: "main", "Configured to collect all information from all networks");
            println!("network: {:?}", options.network.name);
        }
    }

    let mut parser = EvmScanner::new(options);
    match parser.start() {
        Ok(_) => info!(target: "main", "Fin."),
        Err(why) => {
            error!("{}", why);
            process::exit(1);
        }
    }
}

/// Parses args or panics if some requirements are not met.
fn parse_args(matches: clap::ArgMatches) -> OkoResult<cli::CliOptions> {
    let scan = matches.get_flag("scan");
    let log_level_filter = match matches.get_count("verbosity") {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    let network = matches
        .get_one::<String>("network")
        .map_or_else(|| NetworkType::from(Ethereum), |v| v.parse().unwrap());

    let rpc_modules = match matches.get_one::<String>("rpc_modules") {
        Some(p) => p,
        None => &"rpc_modules".to_string(),
    };

    let folder = match matches.get_one::<String>("folder") {
        Some(p) => PathBuf::from(p),
        None => PathBuf::from("/"),
    };

    let options = CliOptions {
        log_level_filter,
        scan,
        network,
        rpc_modules: rpc_modules.to_string(),
        folder,
    };
    Ok(options)
}
