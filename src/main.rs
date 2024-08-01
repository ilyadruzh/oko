pub mod evm;

fn main() {
    let client: reqwest::Client = reqwest::Client::new();

    let good_uri = "https://celo.drpc.org";
    let bad_uri = "https://services.tokenview.io/vipapi/nodeservice/eth?apikey=qVHq2o6jpaakcw3lRstl";
    let bnb_uri = "https://bsc.blockpi.network/v1/rpc/public";

    let _res = evm::get_all_rpc_methods(
        &client,
        bnb_uri,
    );
}
