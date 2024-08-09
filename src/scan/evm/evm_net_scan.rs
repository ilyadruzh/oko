extern crate web3;
use super::types::POSTAPIResponse;
use crate::database::evm_db::get_nodes_from_chainid_list;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Response, StatusCode};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(10);
const HTTP_ERRORS: &str = "http_errors.log";
const NET_ERRORS: &str = "net_errors.log";
const NODE_INFO: &str = "node_info.log";
const UNTYPICAL_RESPONSE: &str = "untypical_response.log";
const DEBUG_NODES: &str = "debug_nodes.log";
const RPC_ERROR_32001: &str = "rpc_error_32001.log";
const RPC_ERROR_32601: &str = "rpc_error_32061.log";
const RPC_ERROR_OTHER: &str = "rpc_error_others.log";

pub fn start_scan_evm_networks(rpc_modules: String, folder: String) {
    println!("start_scan_evm_networks");
    let nodes = get_nodes_from_chainid_list();

    // let all_urls = nodes.len() + 1;
    // let mut count_urls = 0;

    for n in nodes {
        for _u in n.rpc {
            if false {
            } else {
                if rpc_modules == "rpc_modules" {
                    // count_urls += 1;
                    // println!("{}", all_urls - count_urls);
                    let _val = folder.clone();

                    println!("this 1");

                    let _ = check_debug_set_head(&"".to_string(), "".to_string());

                    thread::spawn(move || {
                        // let _ = get_rpc_modules_async((&u).to_string(), val);
                    })
                    .join()
                    .expect("Thread panicked")
                }
            }
        }
    }
}

#[tokio::main]
pub async fn get_rpc_modules_async(uri: String, folder: String) -> Result<(), Box<dyn Error>> {
    let client: reqwest::Client = reqwest::Client::new();

    let mut map = HashMap::new();

    let http_errors = open_file(folder.clone() + "/" + HTTP_ERRORS, true, false, true);
    let mut net_errors = open_file(folder.clone() + "/" + NET_ERRORS, true, false, true);
    let mut node_info = open_file(folder.clone() + "/" + NODE_INFO, true, false, true);
    let mut untypical_response =
        open_file(folder.clone() + "/" + UNTYPICAL_RESPONSE, true, false, true);
    let mut debug_nodes = open_file(folder.clone() + "/" + DEBUG_NODES, true, false, true);

    map.insert("jsonrpc", "2.0");
    map.insert("method", "rpc_modules");
    // map.insert("params", "[]");
    map.insert("id", "67");

    if is_processed(folder.clone(), &uri) {
        is_processed(folder.clone(), &uri);
        return Ok(());
    }

    let folder_dup = folder.clone();

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
                                        &"".to_string(),
                                        &folder.clone(),
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
                        if status.to_string().eq_ignore_ascii_case("invalid method") {
                            return Ok(());
                        } else {
                            if let Some(status_code) = status.get("code") {
                                let status_message = status.get("message").unwrap().to_string();
                                aggregate_error(
                                    &folder_dup,
                                    new_uri,
                                    status_code.to_string(),
                                    status_message,
                                );
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
pub async fn call_rpc_method(
    uri: &String,
    method: &String,
    _payload: &str,
    folder: &String,
) -> Result<Value, Box<dyn Error>> {
    let client: reqwest::Client = reqwest::Client::new();
    let mut map = HashMap::new();
    let mut value = json!(null);
    let filepath = folder.to_owned() + "/" + method + ".log";
    let file = open_file(filepath.to_owned(), true, false, true);

    // let pload: Vec<&str>;

    // if payload != "" {
    //     pload = Vec::new();
    // } else {
    //     pload = vec![payload]
    // }

    map.insert("jsonrpc", "2.0");
    map.insert("method", &method);
    // map.insert("params", pload.to_owned().to_owned());
    map.insert("id", "67");

    println!("map: {:?}", map);

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
                            println!(
                                "CALL {} from {} with RESULT: {}",
                                method,
                                uri,
                                res.to_string()
                            );
                            if !is_exist(filepath.clone(), &res.to_string()) {
                                write_to_file(file, uri, &res.to_string()).await;
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
                    } else if method == "eth_blockNumber" {
                        if let Some(block_number) = json_val.get("result") {
                            if !is_exist(filepath.clone(), &block_number.to_string()) {
                                write_to_file(file, uri, &block_number.to_string()).await;
                            }

                            return Ok(block_number.clone());
                        }
                    } else if method == "debug_setHead" {
                        if let Some(block_number) = json_val.get("result") {
                            if !is_exist(filepath.clone(), &block_number.to_string()) {
                                write_to_file(file, uri, &block_number.to_string()).await;
                            }

                            return Ok(block_number.clone());
                        }
                    }

                    println!("Method doesn't allowed: {:?}", resp_200);
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
            let _url = e.url();
            let _source = e.source();
            println!("");
            return Ok(value);
        }
    }
}

fn is_exist(file_path: String, record: &String) -> bool {
    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if line.unwrap().contains(record) {
                return true;
            } else {
                return false;
            }
        }
        false
    } else {
        false
    }
}

fn is_processed(folder: String, uri: &String) -> bool {
    let mut files = Vec::new();

    let mut folder_path = &folder;
    let default = "./logs/evm".to_string();

    if folder == "".to_string() {
        folder_path = &default;
    }

    let paths = fs::read_dir(folder_path).unwrap();
    for path in paths {
        // println!("Name: {}", path.unwrap().path().display());
        files.push(path.unwrap().path());
    }

    let mut res = false;

    for f in files {
        let file = open_file(f.display().to_string(), false, true, true);
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if line.unwrap().contains(uri) {
                res = true;
            }
        }
    }
    res
}

fn open_file(name: String, write: bool, read: bool, append: bool) -> File {
    let file = File::options()
        .create(true)
        .write(write)
        .read(read)
        .append(append)
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

pub async fn update_chain_list() {
    let resp = reqwest::get("https://chainid.network/chains.json")
        .await
        .expect("request failed");
    let body = resp.text().await.expect("body invalid");
    let mut file = open_file(
        "src/database/chainid_network.json".to_string(),
        true,
        false,
        false,
    );

    // let mut out = File::create("src/database/chainid_network.json").expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut file).expect("failed to copy content");
}

fn aggregate_error(folder: &String, uri: String, status_code: String, status_message: String) {
    let file_0 = open_file(folder.clone() + "/" + RPC_ERROR_OTHER, true, false, true);
    let file_32001 = open_file(folder.clone() + "/" + RPC_ERROR_32601, true, false, true);
    let file_32601 = open_file(folder.clone() + "/" + RPC_ERROR_32001, true, false, true);

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
    println!("this 3");

    let mut nodes: Vec<String> = Vec::from(["".to_string()]);

    let file = File::open("logs/evm/node_info.log").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        let (a, _b) = l.split_once(char::is_whitespace).unwrap();
        nodes.push(a.to_string().to_owned());
    }

    return nodes;
}

pub fn check_debug_set_head(uri: &String, _folder: String) {
    let nodes = get_correct_nodes_from_file();

    info!("check_debug_set_head for: {}", uri);

    // for nethermind - debug_resetHead

    for n in nodes {
        println!("this node: {} ", n);
        thread::spawn(move || {
            let block_number_raw = call_rpc_method(
                &n,
                &"eth_blockNumber".to_string(),
                &"".to_string(),
                &"logs/evm".to_string().clone(),
            );

            println!("block_number_raw: {:?}", block_number_raw);
        })
        .join()
        .expect("Thread panicked");
    }

    // let block_number_raw = call_rpc_method(uri, &"debug_setHead".to_string(), &"payload".to_string(), folder);

    // 1/ send eth_blockNumber
    // 2. send debug_setHead(blockNumber)
}

pub fn check_single_debug_set_head(uri: &String) {
    let new_uri = uri.to_owned();

    thread::spawn(move || {
        let block_number_raw = call_rpc_method(
            &new_uri,
            &"eth_blockNumber".to_string(),
            &"[]".to_string(),
            &"logs/evm".to_string().clone(),
        );

        let block_number_string = block_number_raw.unwrap().to_string().trim().to_string(); //trim_start_matches("0x");
        println!("block_number_string: {}", block_number_string);

        // let (block_number_trim, last) = block_number_string
        //     .trim_start_matches("\"")
        //     .split_at(block_number_string.len() - 2);

        // let debug_setHead = call_rpc_method(
        //     &new_uri,
        //     &"debug_setHead".to_string(),
        //     &"block_number_string".to_string(),
        //     &"logs/evm".to_string().clone(),
        // );

        // println!("debug_setHead: {}", debug_setHead.unwrap().to_string());
    })
    .join()
    .expect("Thread panicked");

    // let block_number_raw = call_rpc_method(uri, &"debug_setHead".to_string(), &"payload".to_string(), folder);

    // 1/ send eth_blockNumber
    // 2. send debug_setHead(blockNumber)
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
