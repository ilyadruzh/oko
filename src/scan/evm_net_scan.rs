extern crate web3;
use crate::database::evm_db::{get_nodes_from_chainid_list, ChainIDNodeInfo};
use bitcoin::taproot::NodeInfo;
use futures::TryFutureExt;
use log::warn;
use reqwest::{header::CONTENT_TYPE, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::thread;
use tokio::runtime::Handle;

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

pub fn start_scan_evm_networks(asynchronous: bool) {
    let nodes = get_nodes_from_chainid_list();

    if (asynchronous) {
        println!("async true");

        for n in nodes {
            for u in n.rpc {
                if (u.contains("wss://") || u.contains("https://")) {
                } else {
                    println!("rpc_url: {}", &u);
                    thread::spawn(move || {
                        let _ = get_rpc_modules_async((&u).to_string());
                    })
                    .join()
                    .expect("Thread panicked")
                }
            }
        }
    } else {
        println!("async false");
        get_rpc_modules_sync(nodes);
    }
}
// get RPC modules from node
// pub async fn get_rpc_modules(handle: &Handle, uri:  &str) {
#[tokio::main]
pub async fn get_rpc_modules_async(uri: String) -> Result<(), Box<dyn Error>> {
    let client: reqwest::Client = reqwest::Client::new();

    let mut map = HashMap::new();

    let mut nodes_errors = File::options()
        .write(true)
        .append(true)
        .open("nodes_errors.log") // TODO: called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
        .unwrap();

    let mut node_info = OpenOptions::new()
        .write(true)
        .append(true)
        .open("nodes_rpc.log")
        .unwrap();

    map.insert("jsonrpc", "2.0");
    map.insert("method", "rpc_modules");
    map.insert("params", "[]");
    map.insert("id", "67");

    let resp = client
        .post(uri)
        .header(CONTENT_TYPE, "application/json")
        .json(&map)
        .send()
        .await;

    // if let Some(resp) = client
    // .post(uri)
    // .header(CONTENT_TYPE, "application/json")
    // .json(&map)
    // .send()
    // .await? {

    match resp {
        Ok(r) => {
            match r.status() {
                // 200
                reqwest::StatusCode::OK => {
                    println!("Success!");
                    let resp_200 = r.text().await?; //.json::<POSTAPIResponse>().await?;

                    let json_val: serde_json::Value = serde_json::from_str(&resp_200).unwrap();

                    if let Some(r) = json_val.get("result") {
                        let val = json_val.get("result").unwrap();
                        writeln!(node_info, "{}", val).unwrap();
                    }

                    if let Some(v) = json_val.get("error") {
                        let status = json_val.get("error").unwrap();
                        let error_code = status.get("code").unwrap();
                        let error_message = status.get("message").unwrap();

                        match error_code.to_string().as_str() {
                            "-32602" => {
                                println!("Error message: {}", error_message);
                                return Ok(());
                            }
                            "-32601" => {
                                println!("Error message: {}", error_message);
                                return Ok(());
                            }
                            _ => {
                                println!("Error code: {}", error_code);
                                println!("Error message: {}", error_message);
                                return Ok(());
                            }
                        }
                    }

                    Ok(())
                }
                // 400
                reqwest::StatusCode::BAD_REQUEST => {
                    warn!("Got 400! BAD_REQUEST");
                    let resp_400 = r.json::<POSTAPIResponse>().await;

                    println!("{:#?}", resp_400);

                    Ok(())
                }
                // 401
                reqwest::StatusCode::UNAUTHORIZED => {
                    warn!("Got 401! UNAUTHORIZED");
                    let resp_401 = r.json::<POSTAPIResponse>().await;

                    println!("{:#?}", resp_401);

                    Ok(())
                }

                // 403
                reqwest::StatusCode::FORBIDDEN => {
                    warn!("Got 403! FORBIDDEN");
                    let resp_403 = r.json::<POSTAPIResponse>().await;

                    println!("{:#?}", resp_403);

                    Ok(())
                }
                // 404
                reqwest::StatusCode::NOT_FOUND => {
                    warn!("Got 404! Haven't found resource!");
                    let resp_404 = r.json::<POSTAPIResponse>().await;
                    println!("{:#?}", resp_404);

                    Ok(())
                }
                // 405
                reqwest::StatusCode::METHOD_NOT_ALLOWED => {
                    warn!("Got 405! METHOD_NOT_ALLOWED");
                    let resp_405 = r.json::<POSTAPIResponse>().await;
                    println!("{:#?}", resp_405);

                    Ok(())
                }
                // 406
                reqwest::StatusCode::NOT_ACCEPTABLE => {
                    warn!("Got 406! NOT_ACCEPTABLE");
                    let resp_406 = r.json::<POSTAPIResponse>().await;

                    println!("{:#?}", resp_406);

                    Ok(())
                }

                // 410
                reqwest::StatusCode::GONE => {
                    warn!("Got 410! GONE");
                    let resp_410 = r.json::<POSTAPIResponse>().await;
                    println!("{:#?}", resp_410);
                    Ok(())
                }
                // 429
                reqwest::StatusCode::TOO_MANY_REQUESTS => {
                    warn!("Got 429! ");
                    let resp_429 = r.json::<POSTAPIResponse>().await;
                    println!("Got {:#?} TOO_MANY_REQUESTS", resp_429);

                    Ok(())
                }
                // 500
                reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                    warn!("Status 500! Internal server error!");
                    let resp_500 = r.json::<POSTAPIResponse>().await;
                    println!("{:#?}", resp_500);

                    Ok(())
                }
                _ => {
                    let line = r.status().to_string();

                    let file = File::open("nodes_errors.log")?;
                    let reader = BufReader::new(file);

                    for line in reader.lines() {
                        let line = line.unwrap();
                        if line.contains(&line) {
                            println!("{}", line);
                        } else {
                            writeln!(nodes_errors, "{}", line).unwrap();
                        }
                    }

                    Ok(())
                }
            }
        }
        Err(e) => {
            println!("REQUEST ERROR: {}", e);
            Ok(())
        }
    }
}

// save RPC methods to DB
// get node uri from DB
#[tokio::main]
pub async fn get_rpc_modules_sync(nodes: Vec<ChainIDNodeInfo>) {
    let mut client: reqwest::Client = reqwest::Client::new();
    let mut map = HashMap::new();

    let mut f = File::options()
        .append(true)
        .open("nodes_errors.log")
        .unwrap();

    map.insert("jsonrpc", "2.0");
    map.insert("method", "rpc_modules");
    map.insert("params", "[]");
    map.insert("id", "67");

    for n in nodes {
        for u in n.rpc {
            let resp = client
                .post(u)
                .header(CONTENT_TYPE, "application/json")
                .json(&map)
                .send()
                .await
                .unwrap();

            println!("{:#?}", resp.status());

            match resp.status() {
                // 200
                reqwest::StatusCode::OK => {
                    println!("Success!");
                    let resp_200 = resp.text().await; //.json::<POSTAPIResponse>().await?;
                    println!("resp_200: {:#?}", resp_200);

                    let json_val: serde_json::Value =
                        serde_json::from_str(&resp_200.unwrap()).unwrap();
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

                    ()
                    // Ok(())
                }
                // 403
                reqwest::StatusCode::FORBIDDEN => {
                    warn!("Got 403! FORBIDDEN");
                    let resp_403 = resp.json::<POSTAPIResponse>().await;

                    println!("{:#?}", resp_403);
                    ()
                    // Ok(())
                }
                // 404
                reqwest::StatusCode::NOT_FOUND => {
                    warn!("Got 404! Haven't found resource!");
                    let resp_404 = resp.json::<POSTAPIResponse>().await;
                    println!("{:#?}", resp_404);
                    ()
                    // Ok(())
                }
                // 405
                reqwest::StatusCode::METHOD_NOT_ALLOWED => {
                    warn!("Got 405! METHOD_NOT_ALLOWED");
                    let resp_405 = resp.json::<POSTAPIResponse>().await;
                    println!("{:#?}", resp_405);
                    ()
                    // Ok(())
                }
                // 429
                reqwest::StatusCode::TOO_MANY_REQUESTS => {
                    warn!("Got 429! ");
                    let resp_429 = resp.json::<POSTAPIResponse>().await;
                    println!("Got {:#?} TOO_MANY_REQUESTS", resp_429);
                    ()
                    // Ok(())
                }
                // 500
                reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                    warn!("Status 500! Internal server error!");
                    let resp_500 = resp.json::<POSTAPIResponse>().await;
                    println!("{:#?}", resp_500);
                    ()
                    // Ok(())
                }
                _ => {
                    let line = resp.status().to_string();
                    writeln!(&mut f, "{}\n", line);
                    ()
                    // Ok(())

                    // panic!("this shouldn't happen");
                }
            }
        }
    }
}

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
