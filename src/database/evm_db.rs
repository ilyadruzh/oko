use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use warp::{http, Filter};

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
    node: EVMNode,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let r = store.evm_nodes.write().insert(node.url, node.info);
    println!("update_node_to_evm_nodes");
    Ok(warp::reply::with_status(
        "Updated node from EVM nodes",
        http::StatusCode::CREATED,
    ))
}

pub async fn delete_node_to_evm_nodes(
    node: EVMNode,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let r = store.evm_nodes.write().remove(&node.url);
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

fn check_if_exist() {}
