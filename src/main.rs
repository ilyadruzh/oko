pub mod cli;
pub mod database;
pub mod rest_api;
pub mod scan;
pub mod utils;

use std::str::FromStr;

use crate::cli::*;
use crate::scan::common::errors::OkoResult;
use clap::{Arg, Command};
use scan::{common::types::NetworkType, evm::evm_net_scan::check_single_debug_set_head};

#[macro_use]
extern crate log;
extern crate bitcoin;
extern crate byteorder;
extern crate chrono;
extern crate rayon;
extern crate rusty_leveldb;
extern crate seek_bufread;

fn _command() -> Command {
    let networks = ["ethereum"];
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
            Arg::new("update")
                .long("update")
                .short('u')
                .long("update")
                .value_name("NAME")
                // .value_parser(clap::value_parser!(bool))
                .help("Update chain list"),
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
    let _ = check_single_debug_set_head(&"https://testnet.rpc.abyssprotocol.ai/".to_string());
    // let options = match parse_args(command().get_matches()) {
    //     Ok(o) => o,
    //     Err(desc) => {
    //         SimpleLogger::init(log::LevelFilter::Debug).unwrap();
    //         error!(target: "main", "{}", desc);
    //         process::exit(1);
    //     }
    // };

    // let log_level = options.log_level_filter;
    // SimpleLogger::init(log_level).expect("Unable to initialize logger!");
    // info!(target: "main", "The Oko is opening... v{} ...", env!("CARGO_PKG_VERSION"));
    // debug!(target: "main", "Logging level {}", log_level);

    // if options.scan {
    //     let mut scanner = BlockchainScanner::new(&options);
    //     match scanner.start() {
    //         Ok(_) => info!(target: "main", "Done"),
    //         Err(why) => {
    //             error!("{}", why);
    //             process::exit(1);
    //         }
    //     }
    // }

    // match options.update.as_str() {
    //     "evm" => update_chain_list(),
    //     _ => update_chain_list(),
    // }
    // .await;
}

/// Parses args or panics if some requirements are not met.
fn _parse_args(matches: clap::ArgMatches) -> OkoResult<cli::CliOptions> {
    let scan = matches.get_flag("scan");
    let log_level_filter = match matches.get_count("verbosity") {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    let network = matches.get_one::<String>("network").map_or_else(
        || NetworkType::from_str("ethereum").unwrap(),
        |v| v.parse().unwrap(),
    );

    let rpc_modules = match matches.get_one::<String>("rpc_modules") {
        Some(p) => p,
        None => &"rpc_modules".to_string(),
    };

    let folder = match matches.get_one::<String>("folder") {
        Some(p) => p.to_string(),         //PathBuf::from(p),
        None => "./logs/evm".to_string(), //PathBuf::from("/"),
    };

    let update = match matches.get_one::<String>("update") {
        Some(p) => p.to_string(),   //PathBuf::from(p),
        None => "none".to_string(), //PathBuf::from("/"),
    };

    let options = CliOptions {
        log_level_filter,
        scan,
        update,
        network,
        rpc_modules: rpc_modules.to_string(),
        folder,
    };
    Ok(options)
}
