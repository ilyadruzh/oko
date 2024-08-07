use std::collections::HashMap;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct GETAPIResponse {
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RPCStruct {
    eth: String,
    debug: String,
    net: String,
    web3: String,
    txpool: String,
    trace: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct POSTAPIResponse {
    result: RPCStruct,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JSONResponse {
    json: HashMap<String, String>,
}