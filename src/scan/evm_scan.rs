extern crate web3;
use log::{warn};
use reqwest::{header::CONTENT_TYPE, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

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

// get RPC methods from node
#[tokio::main]
pub async fn get_all_rpc_methods(client: &Client, uri: &str) -> Result<(), Box<dyn Error>> {
    let mut map = HashMap::new();

    map.insert("jsonrpc", "2.0");
    map.insert("method", "rpc_modules");
    map.insert("params", "[]");
    map.insert("id", "67");

    let resp = client
        .post(uri)
        .header(CONTENT_TYPE, "application/json")
        .json(&map)
        .send()
        .await?;

    println!("{:#?}", resp);

    match resp.status() {
        // 200
        reqwest::StatusCode::OK => {
            println!("Success!");
            let resp_200 = resp.text().await?; //.json::<POSTAPIResponse>().await?;
            println!("resp_200: {:#?}", resp_200);

            let json_val: serde_json::Value = serde_json::from_str(&resp_200).unwrap();
            println!("json_val: {:#?}", json_val);

            let status = json_val.get("error").unwrap();
            let error_code = status.get("code").unwrap();
            println!("error_code: {}", error_code);

            match error_code.to_string().as_str() {
                "-32602" => {
                    println!("-32602");
                }

                _ => {
                    println!("fun");
                }
            }

            let val = json_val.get("result").unwrap();
            println!("rpc_modules: {}", val);

            Ok(())
        }
        // 403
        reqwest::StatusCode::FORBIDDEN => {
            warn!("Got 403! FORBIDDEN");
            let resp_403 = resp.json::<POSTAPIResponse>().await?;
            println!("{:#?}", resp_403);
            Ok(())
        }
        // 404
        reqwest::StatusCode::NOT_FOUND => {
            warn!("Got 404! Haven't found resource!");
            let resp_404 = resp.json::<POSTAPIResponse>().await?;
            println!("{:#?}", resp_404);
            Ok(())
        }
        // 405
        reqwest::StatusCode::METHOD_NOT_ALLOWED => {
            warn!("Got 405! METHOD_NOT_ALLOWED");
            let resp_405 = resp.json::<POSTAPIResponse>().await?;
            println!("{:#?}", resp_405);
            Ok(())
        }
        // 429
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            warn!("Got 429! ");
            let resp_429 = resp.json::<POSTAPIResponse>().await?;
            println!("Got {:#?} TOO_MANY_REQUESTS", resp_429);
            Ok(())
        }
        // 500
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
            warn!("Status 500! Internal server error!");
            let resp_500 = resp.json::<POSTAPIResponse>().await?;
            println!("{:#?}", resp_500);
            Ok(())
        }
        _ => {
            panic!("this shouldn't happen");
        }
    }
}

// save RPC methods to DB

// get node uri from DB

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_rpc_methods_works() {
        let test_client: reqwest::Client = reqwest::Client::new();
        const uri: &str =
            "https://services.tokenview.io/vipapi/nodeservice/eth?apikey=qVHq2o6jpaakcw3lRstl";

        let result = get_all_rpc_methods(&test_client, uri);

        // assert_eq!(result, 4);
    }
}
