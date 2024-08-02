pub mod database;
pub mod rest_api;
pub mod scan;
pub mod utils;

use tokio::*;

async fn our_async_program() {
    todo!();
}

fn fib_cpu_intensive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib_cpu_intensive(n - 1) + fib_cpu_intensive(n - 2),
    }
}

async fn app() {
    let concurrent_future = task::spawn(our_async_program());

    let threadpool_future = task::spawn_blocking(|| fib_cpu_intensive(30));

    todo!()
}

#[tokio::main]
async fn main() {
    rest_api::start_server();

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let future = app();
    rt.block_on(future);

    // bank::test_transfer();

    // let client: reqwest::Client = reqwest::Client::new();

    // let good_uri = "https://celo.drpc.org";
    // let bad_uri =
    //     "https://services.tokenview.io/vipapi/nodeservice/eth?apikey=qVHq2o6jpaakcw3lRstl";
    // let bnb_uri = "https://bsc.blockpi.network/v1/rpc/public";

    // let _res = evm::get_all_rpc_methods(&client, bnb_uri);
}
