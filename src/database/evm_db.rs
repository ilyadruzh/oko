use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{collections::HashMap, fs};
use warp::{http, Filter};

// from https://chainid.network/chains.json

#[derive(Debug, Deserialize, Serialize, Clone)]

struct _ChainIDFeatures {
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]

struct _ChainIDNativeCurrency {
    name: String,
    symbol: String,
    decimals: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct _ChainIDEns {
    registry: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct _ChainIDExplorer {
    name: String,
    // icon: String,
    url: String,
    standard: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChainIDNodeInfo {
    pub name: String,
    // icon: String,
    pub rpc: Vec<String>,
    // features: Vec<ChainIDFeatures>,
    faucets: Vec<String>,
    // native_currency: ChainIDNativeCurrency,
    // info_url: String,
    // short_name: String,
    // chain_id: String,
    // network_id: String,
    // slip44: u64,
    // ens: ChainIDEns,
    // explorers: Vec<ChainIDExplorer>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct NodeInfo {
    name: String,
    client_version: String, // web3_clientVersion
}

// type EVMNodeInfo = Vec<NodeInfo>;
type EVMNodes = HashMap<String, NodeInfo>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EVMNode {
    url: String,
    info: NodeInfo,
}

#[derive(Clone)]
pub struct Store {
    evm_nodes: Arc<RwLock<EVMNodes>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            evm_nodes: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub fn get_nodes_from_chainid_list() -> Vec<ChainIDNodeInfo> {
    let res = fs::read_to_string("src/database/chainid_network.json").expect("Can't read file"); // read file into string

    // let s = match res { // get value from Result object
    //     Ok(s) => s,
    //     Err(_) => panic!("Can't read file")
    // };

    let nodes = serde_json::from_str::<Vec<ChainIDNodeInfo>>(&res).expect("Can't parse json"); //.unwrap();

    // for node in &nodes {
    //     for url in &node.rpc {
    //         println!("node rpc: {}", url);
    //     }
    // }

    // // change values
    // json_data[0]["name"] = serde_json::json!(123);
    // json_data[0]["name"] = serde_json::json!("mascai");

    // std::fs::write("output.json", serde_json::to_string_pretty(&json_data).unwrap())
    //     .expect("Can't write to file");

    nodes
}

pub async fn add_node_to_evm_nodes(
    node: EVMNode,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let r = store.evm_nodes.write().insert(node.url, node.info);
    println!("add_node_to_evm_nodes");
    Ok(warp::reply::json(&r))
}

pub async fn get_evm_nodes(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let result = store.evm_nodes.read();
    Ok(warp::reply::json(&*result)) // &* - разыменование RwLockReadGuard в отображение
}

pub async fn update_node_to_evm_nodes(
    // node: EVMNode,
    // store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    // let r = store.evm_nodes.write().insert(node.url, node.info);
    println!("update_node_to_evm_nodes");
    Ok(warp::reply::with_status(
        "Updated node from EVM nodes",
        http::StatusCode::CREATED,
    ))
}

pub async fn delete_node_to_evm_nodes(
    // node: EVMNode,
    // store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    // let _r = store.evm_nodes.write().remove(&node.url);
    println!("delete_node_to_evm_nodes");
    Ok(warp::reply::with_status(
        "Removed node from evm nodes",
        http::StatusCode::OK,
    ))
}

pub fn post_json() -> impl Filter<Extract = (EVMNode,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn delete_json() -> impl Filter<Extract = (EVMNode,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn json_body() -> impl Filter<Extract = (EVMNode,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

// положить в json
pub fn add_to_json() {}

// fn check_if_exist() {}
