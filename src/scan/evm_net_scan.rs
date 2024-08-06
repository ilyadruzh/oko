extern crate web3;
use crate::database::evm_db::{get_nodes_from_chainid_list, ChainIDNodeInfo};
use bitcoin::taproot::NodeInfo;
use futures::TryFutureExt;
use log::warn;
use reqwest::{header::CONTENT_TYPE, Client};
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;
use tokio::runtime::Handle;

const TIMEOUT: Duration = Duration::from_secs(10);

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

// impl fmt::Display for RPCStruct {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{} {} {} {} {} {}",
//             self.eth, self.debug, self.net, self.web3, self.txpool, self.trace
//         )
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct POSTAPIResponse {
    result: RPCStruct,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JSONResponse {
    json: HashMap<String, String>,
}

pub fn start_scan_evm_networks() {
    let nodes = get_nodes_from_chainid_list();

    let all_urls = nodes.len() + 1;
    let mut count_urls = 0;

    for n in nodes {
        for u in n.rpc {
            if false {
            } else {
                count_urls += 1;

                println!("{}", all_urls - count_urls);

                thread::spawn(move || {
                    let _ = get_rpc_modules_async((&u).to_string());
                })
                .join()
                .expect("Thread panicked")
            }
        }
    }
}
// get RPC modules from node
// pub async fn get_rpc_modules(handle: &Handle, uri:  &str) {
#[tokio::main]
pub async fn get_rpc_modules_async(uri: String) -> Result<(), Box<dyn Error>> {
    let client: reqwest::Client = reqwest::Client::new();

    let mut map = HashMap::new();

    let mut node_error = File::options()
        .write(true)
        .append(true)
        .open("nodes_errors.log") // TODO: called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
        .unwrap();

    let mut http_error = File::options()
        .write(true)
        .append(true)
        .open("http_errors.log") // TODO: called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
        .unwrap();

    let mut node_info = OpenOptions::new()
        .write(true)
        .append(true)
        .open("nodes_rpc_modules.log")
        .unwrap();

    let mut node_response = OpenOptions::new()
        .write(true)
        .append(true)
        .open("nodes_response_error.log")
        .unwrap();

    map.insert("jsonrpc", "2.0");
    map.insert("method", "rpc_modules");
    // map.insert("params", "[]");
    map.insert("id", "67");

    if is_processed(&uri) {
        return Ok(());
    }

    let resp = client
        .post(&uri)
        .timeout(TIMEOUT)
        .header(CONTENT_TYPE, "application/json")
        .json(&map)
        .send()
        .await;

    match resp {
        Ok(r) => {
            match r.status() {
                // 200
                reqwest::StatusCode::OK => {
                    let resp_200 = r.text().await?; //.json::<POSTAPIResponse>().await?;
                    if resp_200 == "" {
                        let record = uri.clone() + " - NULL: " + &resp_200;
                        if !is_exist("nodes_rpc_modules.log".to_string(), &record) {
                            writeln!(node_response, "{} - NULL: {:?}", uri, resp_200).unwrap();
                        }
                        return Ok(());
                    }

                    if resp_200.contains("<!doctype html>")
                        || resp_200.contains("<html>")
                        || resp_200.contains("<!DOCTYPE html>")
                        || resp_200.contains("<html lang=")
                    {
                        let record = uri.clone() + " - NULL: " + &resp_200;
                        if !is_exist("nodes_rpc_modules.log".to_string(), &record) {
                            writeln!(node_response, "{} - NULL: {:?}", uri, resp_200).unwrap();
                        }
                        return Ok(());
                    }

                    if resp_200.contains("LBRY") {
                        return Ok(());
                    }

                    // let json_val: POSTAPIResponse = serde_json::from_str(&resp_200).unwrap();
                    // if let Ok(json_val: serde_json::Value) = serde_json::from_str(&resp_200) {}

                    let json_val: serde_json::Value = serde_json::from_str(&resp_200).unwrap();

                    if let Some(rpc_modules) = json_val.get("result") {
                        let record: String = uri.clone() + " - " + &rpc_modules.to_string();
                        // println!("record: {}", &record);
                        if !is_exist("nodes_rpc_modules.log".to_string(), &record) {
                            call_rpc_method(&uri, "txpool_status".to_string());
                            writeln!(node_info, "{} - {}", &uri.to_string(), rpc_modules).unwrap();
                        }
                    }

                    if let Some(status) = json_val.get("error") {
                        aggregate_error(
                            uri,
                            status.get("code").unwrap().to_string(),
                            status.get("message").unwrap().to_string(),
                        );
                        // writeln!(node_response, "{} - ERROR: - {}", uri, status).unwrap();
                    }

                    Ok(())
                }
                _ => {
                    write_error_to_file(node_error, uri.to_string(), r.status(), r).await;
                    Ok(())
                }
            }
        }
        Err(e) => {
            let url = e.url();
            let source = e.source();
            let record: String = url.unwrap().to_string() + " - " + &source.unwrap().to_string();

            if !is_exist("http_errors.log".to_string(), &record) {
                writeln!(http_error, "{} - {:?}", url.unwrap().to_string(), source).unwrap();
            }

            Ok(())
        }
    }
}

pub fn get_all_debug_methods(uri: String) {
    todo!()
}

#[tokio::main]
pub async fn call_rpc_method(uri: &String, method: String) {
    let client: reqwest::Client = reqwest::Client::new();
    let mut map = HashMap::new();

    map.insert("jsonrpc", "2.0");
    map.insert("method", &method);
    // map.insert("params", "[]");
    map.insert("id", "67");

    let resp = client
        .post(uri)
        .timeout(TIMEOUT)
        .header(CONTENT_TYPE, "application/json")
        .json(&map)
        .send()
        .await;

    match resp {
        Ok(r) => {
            match r.status() {
                // 200
                reqwest::StatusCode::OK => {
                    println!("Response for method - {}: {:?}", method, r);
                    println!("");
                }
                _ => {
                    println!("Response status for method - {}: {}", method, r.status());
                    println!("");
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            let url = e.url();
            let source = e.source();
            println!("");

            // let record: String = url.unwrap().to_string() + " - " + &source.unwrap().to_string();

            // println!("{} - {:?}", url.unwrap().to_string(), source);

            // if !is_exist("http_errors.log".to_string(), &record) {
            //     println!("{} - {:?}", url.unwrap().to_string(), source);
            // }
        }
    }
}

fn is_exist(file_path: String, record: &String) -> bool {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line.unwrap().contains(record) {
            return true;
        } else {
            return false;
        }
    }
    false
}

fn is_processed(uri: &String) -> bool {
    let files = Vec::from([
        "0_rpc_errors.log",
        "http_errors.log",
        "nodes_errors.log",
        "nodes_response_error.log",
        "nodes_rpc_modules.log",
        "32001_rpc_errors.log",
        "32601_rpc_errors.log",
    ]);

    let mut res = false;

    for f in files {
        let file = File::open(f).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if line.unwrap().contains(uri) {
                res = true;
            } else {
            }
        }
    }
    res
}

async fn write_error_to_file(file_path: File, uri: String, status: StatusCode, response: Response) {
    if let Err(resp_error) = response.json::<POSTAPIResponse>().await {
        let record: String = uri + " - " + &status.to_string() + " - " + &resp_error.to_string();

        if !is_exist("nodes_errors.log".to_string(), &record) {
            writeln!(&file_path, "{}", record).unwrap();
        }
    } else {
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!")
    }
}

fn aggregate_error(uri: String, status_code: String, status_message: String) {
    println!("status_code: {}", status_code);

    let mut file_0 = OpenOptions::new()
        .write(true)
        .append(true)
        // .truncate(true)
        .open("0_rpc_errors.log")
        .unwrap();

    let mut file_32001 = OpenOptions::new()
        .write(true)
        .append(true)
        // .truncate(true)
        .open("32001_rpc_errors.log")
        .unwrap();

    let mut file_32601 = OpenOptions::new()
        .write(true)
        .append(true)
        // .truncate(true)
        .open("32601_rpc_errors.log")
        .unwrap();

    if status_code == "-32601".to_string() {
        if !is_exist("32601_rpc_errors.log".to_string(), &uri) {
            writeln!(&file_32601, "{} - {}", uri, status_message).unwrap();
        }
    } else if status_code == "-32001".to_string() {
        if !is_exist("32001_rpc_errors.log".to_string(), &uri) {
            writeln!(&file_32001, "{} - {}", uri, status_message).unwrap();
        }
    } else {
        if !is_exist("0_rpc_errors.log".to_string(), &uri) {
            writeln!(&file_0, "{} - {}", uri, status_message).unwrap();
        }
    }
}

pub fn get_correct_nodes_from_file() -> Vec<String> {
    let mut nodes: Vec<String> = Vec::from(["".to_string()]);

    let file = File::open("nodes_rpc_modules.log").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        let (a, b) = l.split_once(char::is_whitespace).unwrap();
        nodes.push(a.clone().to_string().to_owned());
    }

    return nodes;
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
