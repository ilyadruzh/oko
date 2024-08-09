use crate::database::evm_db::Store;

use warp::Filter;

#[tokio::main]
pub async fn start_server() {
    let store = Store::new();
    let _store_filter = warp::any().map(move || store.clone());

    // let add_evm_node = warp::post()
    //     .and(warp::path("v1"))
    //     .and(warp::path("evm_nodes"))
    //     .and(warp::path::end())
    //     .and(json_body())
    //     .and(store_filter.clone())
    //     .and_then(add_node_to_evm_nodes);

    // let get_evm_node = warp::get()
    //     .and(warp::path("v1"))
    //     .and(warp::path("evm_nodes"))
    //     .and(warp::path::end())
    //     .and(store_filter.clone())
    //     .and_then(get_evm_nodes);

    // let update_evm_node = warp::put()
    //     .and(warp::path("v1"))
    //     .and(warp::path("evm_nodes"))
    //     .and(warp::path::end())
    //     .and(post_json())
    //     .and(store_filter.clone())
    //     .and_then(update_node_to_evm_nodes);

    // let delete_evm_node = warp::delete()
    //     .and(warp::path("v1"))
    //     .and(warp::path("evm_nodes"))
    //     .and(warp::path::end())
    //     .and(delete_json())
    //     .and(store_filter.clone())
    //     .and_then(delete_node_to_evm_nodes);

    // let routes = add_evm_node
    //     .or(get_evm_node)
    //     .or(update_evm_node)
    //     .or(delete_evm_node);

    // warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
