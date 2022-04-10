use serde::{self, Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SellOfferStatus {
    Open,
    Sold,
    Removed
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CardStatus {
    Scheme,
    Prototype,
    Trial,
    Permanent,
    Suspended,
    Banned,
    BannedSoon,
    BannedVerySoon,
    None
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub owner: String,
    pub artist: String,
    pub content: String,
    pub image: String,
    pub full_art: bool,
    pub notes: String,
    pub status: CardStatus,
    pub vote_pool: String,
    pub voters: Vec<String>,
    pub fair_enough_votes: String,
    pub overpowered_votes: String,
    pub underpowered_votes: String,
    pub inappropriate_votes: String,
    pub nerflevel: String
}

#[derive(Deserialize, Debug)]
pub struct Price {
    pub denom: String,
    pub amount: String
}

#[derive(Deserialize, Debug)]
pub struct SellOffer {
    pub seller: String,
    pub buyer: String,
    pub card: String,
    pub price: Price,
    pub status: SellOfferStatus
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SellOffersResponse {
    pub sell_offers_ids: Vec<String>,
    pub sell_offers: Vec<SellOffer>,
}
