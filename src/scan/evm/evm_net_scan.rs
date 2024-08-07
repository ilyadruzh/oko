extern crate web3;
use super::types::POSTAPIResponse;
use crate::database::evm_db::get_nodes_from_chainid_list;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Response, StatusCode};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(10);
const HTTP_ERRORS: &str = "logs/http_errors.log";
const NET_ERRORS: &str = "logs/net_errors.log";
const NODE_INFO: &str = "logs/node_info.log";
const UNTYPICAL_RESPONSE: &str = "logs/untypical_response.log";
const DEBUG_NODES: &str = "logs/debug_nodes.log";
const RPC_ERROR_32001: &str = "logs/rpc_error_32001.log";
const RPC_ERROR_32601: &str = "logs/rpc_error_32061.log";
const RPC_ERROR_OTHER: &str = "logs/rpc_error_others.log";

pub fn start_scan_evm_networks() {
    let nodes = get_nodes_from_chainid_list();

    let all_urls = nodes.len() + 1;
    let mut count_urls = 0;

    for n in nodes {
        for u in n.rpc {
            if false {
            } else {

                // count_urls += 1;
                // println!("{}", all_urls - count_urls);
                // // print!("{esc}c", esc = 27 as char);

                thread::spawn(move || {
                    let _ = get_rpc_modules_async((&u).to_string());
                })
                .join()
                .expect("Thread panicked")
            }
        }
    }
}

#[tokio::main]
pub async fn get_rpc_modules_async(uri: String) -> Result<(), Box<dyn Error>> {
    let client: reqwest::Client = reqwest::Client::new();

    let mut map = HashMap::new();

    let http_errors = open_file(HTTP_ERRORS.to_string(), true, false);
    let mut net_errors = open_file(NET_ERRORS.to_string(), true, false);
    let mut node_info = open_file(NODE_INFO.to_string(), true, false);
    let mut untypical_response = open_file(UNTYPICAL_RESPONSE.to_string(), true, false);
    let mut debug_nodes = open_file(DEBUG_NODES.to_string(), true, false);

    map.insert("jsonrpc", "2.0");
    map.insert("method", "rpc_modules");
    // map.insert("params", "[]");
    map.insert("id", "67");

    if is_processed(&uri) {
        is_processed(&uri);
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
                    let new_uri: String = uri.clone();

                    if resp_200 == "" {
                        let record = uri.clone() + " - NULL: " + &resp_200;
                        if !is_exist(UNTYPICAL_RESPONSE.to_string(), &record) {
                            writeln!(untypical_response, "{} - NULL: {:?}", uri, resp_200).unwrap();
                        }
                        return Ok(());
                    }

                    if resp_200.contains("<!doctype html>")
                        || resp_200.contains("<html>")
                        || resp_200.contains("<!DOCTYPE html>")
                        || resp_200.contains("<html lang=")
                    {
                        let record = uri.clone() + " - NULL: " + &resp_200;
                        if !is_exist(UNTYPICAL_RESPONSE.to_string(), &record) {
                            writeln!(untypical_response, "{} - NULL: {:?}", uri, resp_200).unwrap();
                        }
                        return Ok(());
                    }

                    if resp_200.contains("LBRY") {
                        return Ok(());
                    }

                    let json_val: serde_json::Value = serde_json::from_str(&resp_200).unwrap();

                    if let Some(rpc_modules) = json_val.get("result") {
                        let record: String = uri.clone() + " - " + &rpc_modules.to_string();

                        if !is_exist(NODE_INFO.to_string(), &record) {
                            if rpc_modules.to_string().contains("debug") {
                                let (network_name, network_type) = get_network_info(uri.to_owned());

                                thread::spawn(move || {
                                    let web3_client = call_rpc_method(
                                        &uri.to_owned(),
                                        &"web3_clientVersion".to_string(),
                                    );
                                    writeln!(
                                        debug_nodes,
                                        "{} - {} - {} - {}",
                                        &network_name,
                                        network_type,
                                        web3_client.unwrap().to_string(),
                                        &uri.to_string()
                                    )
                                    .unwrap();
                                })
                                .join()
                                .expect("Thread panicked");
                            }
                            writeln!(node_info, "{} - {}", &new_uri.to_string(), rpc_modules)
                                .unwrap();
                        }
                    }

                    if let Some(status) = json_val.get("error") {
                        println!("status: {}", status.to_string());

                        if status.to_string().eq_ignore_ascii_case("invalid method") {
                            println!("invalid method");
                            return Ok(());
                        } else {
                            if let Some(status_code) = status.get("code") {
                                let status_message = status.get("message").unwrap().to_string();
                                aggregate_error(new_uri, status_code.to_string(), status_message);
                            }
                        }
                    }

                    Ok(())
                }
                _ => {
                    write_error_to_file(http_errors, uri.to_string(), r.status(), r).await;
                    Ok(())
                }
            }
        }
        Err(e) => {
            let url = e.url();
            let source = e.source();
            let record: String = url.unwrap().to_string() + " - " + &source.unwrap().to_string();

            if !is_exist(NET_ERRORS.to_string(), &record) {
                writeln!(net_errors, "{} - {:?}", url.unwrap().to_string(), source).unwrap();
            }

            Ok(())
        }
    }
}

#[tokio::main]
pub async fn call_rpc_method(uri: &String, method: &String) -> Result<Value, Box<dyn Error>> {
    let client: reqwest::Client = reqwest::Client::new();
    let mut map = HashMap::new();

    let mut value = json!(null);

    let filepath = method.to_owned() + ".log";

    let method_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&filepath)
        .unwrap();

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
                    let resp_200 = r.text().await?;
                    if resp_200 == "" {
                        return Ok(value);
                    }

                    let json_val: serde_json::Value = serde_json::from_str(&resp_200).unwrap();

                    if method == "web3_clientVersion" {
                        if let Some(res) = json_val.get("result") {
                            println!("CALL {} with RESULT: {}", method, res.to_string());
                            if !is_exist(filepath, &res.to_string()) {
                                write_to_file(method_file, uri, &res.to_string()).await;
                            }

                            value = res.clone();
                            return Ok(value);
                        }
                    } else if method == "txpool_status" {
                        if let Some(txpool_status) = json_val.get("result") {
                            let pending = txpool_status.get("pending");
                            println!("pending: {}", pending.unwrap().to_string());
                            return Ok(value);
                        }
                    }

                    println!("Method doesn't allowed");
                    return Ok(value);
                }
                _ => {
                    println!("Response status for method - {}: {}", method, r.status());
                    println!("");
                    return Ok(value);
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            let url = e.url();
            let source = e.source();
            println!("");
            return Ok(value);
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
    let mut files = Vec::new();

    let paths = fs::read_dir("./logs").unwrap();
    for path in paths {
        // println!("Name: {}", path.unwrap().path().display());
        files.push(path.unwrap().path());
    }

    let mut res = false;

    for f in files {
        let file = open_file(f.display().to_string(), false, true);
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if line.unwrap().contains(uri) {
                res = true;
            }
        }
    }
    res
}

fn open_file(name: String, write: bool, read: bool) -> File {
    let file = File::options()
        .create(true)
        .write(write)
        .read(read)
        .append(true)
        .open(name) // TODO: called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
        .unwrap();

    return file;
}
async fn write_error_to_file(file_path: File, uri: String, status: StatusCode, response: Response) {
    if let Err(resp_error) = response.json::<POSTAPIResponse>().await {
        let record: String = uri + " - " + &status.to_string() + " - " + &resp_error.to_string();

        if !is_exist(HTTP_ERRORS.to_string(), &record) {
            writeln!(&file_path, "{}", record).unwrap();
        }
    } else {
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!")
    }
}

async fn write_to_file(file_path: File, uri: &String, record: &String) {
    if !is_exist(HTTP_ERRORS.to_string(), &record) {
        writeln!(&file_path, "{} - {}", uri, record).unwrap();
    }
}

fn aggregate_error(uri: String, status_code: String, status_message: String) {
    println!("status_code: {}", status_code);

    let file_0 = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        // .truncate(true)
        .open(RPC_ERROR_OTHER)
        .unwrap();

    let file_32001 = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        // .truncate(true)
        .open(RPC_ERROR_32601)
        .unwrap();

    let file_32601 = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        // .truncate(true)
        .open(RPC_ERROR_32001)
        .unwrap();

    if status_code == "-32601".to_string() {
        if !is_exist(RPC_ERROR_32601.to_string(), &uri) {
            writeln!(&file_32601, "{} - {}", uri, status_message).unwrap();
        }
    } else if status_code == "-32001".to_string() {
        if !is_exist(RPC_ERROR_32001.to_string(), &uri) {
            writeln!(&file_32001, "{} - {}", uri, status_message).unwrap();
        }
    } else {
        if !is_exist(RPC_ERROR_OTHER.to_string(), &uri) {
            writeln!(&file_0, "{} - {}", uri, status_message).unwrap();
        }
    }
}

fn get_network_info(uri: String) -> (String, String) {
    let nodes = get_nodes_from_chainid_list();

    let mut network_name: String = "".to_string();
    let mut network_type: String = "mainnet".to_string();

    for n in nodes {
        for r in n.rpc {
            if r.contains(&uri) {
                network_name = n.name.to_owned();

                if r.contains("test")
                    || r.contains("Test")
                    || r.contains("Devnet")
                    || r.contains("devnet")
                {
                    network_type = "testnet".to_owned();
                }
            }
        }
    }

    return (network_name, network_type);
}

pub fn get_correct_nodes_from_file() -> Vec<String> {
    let mut nodes: Vec<String> = Vec::from(["".to_string()]);

    let file = File::open("nodes_rpc_modules.log").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        let (a, b) = l.split_once(char::is_whitespace).unwrap();
        nodes.push(a.to_string().to_owned());
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
