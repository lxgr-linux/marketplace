use crate::types::*;
use serde::de::DeserializeOwned;
use reqwasm::http::Request;
use reqwasm::Error;

const ROOT_URL: &str = "http://0.0.0.0:1317";

pub async fn get_card_image(id: &str) -> String {
    let path = format!("/DecentralCardGame/cardchain/cardchain/q_card/{}", id);
    let resp:Card = get_from_cardchain(&path).await.unwrap();
    resp.image
}

pub async fn get_from_cardchain<T: DeserializeOwned>(request_path: &str) -> Result<T, Error> {
    let request_url = format!("{}{}", ROOT_URL, request_path);
    let sell_offers_response: T = Request::get(&request_url).send()
                                                            .await?
                                                            .json()
                                                            .await?;
    Ok(sell_offers_response)
}
