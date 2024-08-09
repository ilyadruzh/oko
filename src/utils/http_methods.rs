extern crate web3;
use crate::scan::evm::types::{GETAPIResponse, JSONResponse};
use reqwest::{header::CONTENT_TYPE, Client};
use std::collections::HashMap;

#[tokio::main]
pub async fn get(client: &Client, uri: &str) -> Result<GETAPIResponse, Box<dyn std::error::Error>> {
    let resp = client
        .get(uri)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
    //  .json::<GETAPIResponse>()
    //  .await?;

    // println!("{:#?}", resp);

    match resp.status() {
        // "OK - 200" — все прошло хорошо
        reqwest::StatusCode::OK => {
            println!("Success!");
            let resp_json = resp.json::<GETAPIResponse>().await?;
            Ok(resp_json)
        }
        // "NOT_FOUND - 404" — ресурс не найден
        reqwest::StatusCode::NOT_FOUND => {
            println!("Got 404! Haven't found resource!");
            let resp_404 = resp.json::<GETAPIResponse>().await?;
            println!("{:#?}", resp_404);
            Ok(resp_404)
        }
        // Любой другой код состояния, не совпадающий с приведенными выше
        _ => {
            panic!("Okay... this shouldn't happen...");
        }
    }
}

#[tokio::main]
pub async fn post(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    // Создается карта со строковыми парами «ключ — значение»
    // — полезной нагрузкой тела запроса
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    // Выполняется POST-запрос, а также парсинг ответа в структуру JSONResponse
    let resp_json = client
        .post("https://httpbin.org/anything")
        .json(&map)
        .send()
        .await?
        .json::<JSONResponse>()
        .await?;

    println!("{:#?}", resp_json);

    Ok(())
}
