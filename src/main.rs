
use serde::{self, Deserialize};
use reqwasm::http::Request;
use reqwasm::Error;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const ROOT_URL: &str = "http://0.0.0.0:1317";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum SellOfferStatus {
    Open,
    Sold,
    Removed
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum CardStatus {
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
struct Card {
    owner: String,
    artist: String,
    content: String,
    image: String,
    full_art: bool,
    notes: String,
    status: CardStatus,
    vote_pool: String,
    voters: Vec<String>,
    fair_enough_votes: String,
    overpowered_votes: String,
    underpowered_votes: String,
    inappropriate_votes: String,
    nerflevel: String
}

#[derive(Deserialize, Debug)]
struct Price {
    denom: String,
    amount: String
}

#[derive(Deserialize, Debug)]
struct SellOffer {
    seller: String,
    buyer: String,
    card: String,
    price: Price,
    status: SellOfferStatus
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SellOffersResponse {
    sell_offers_ids: Vec<String>,
    sell_offers: Vec<SellOffer>,
}

#[function_component(App)]
fn app() -> Html {
    let resp = use_state(|| SellOffersResponse{sell_offers_ids: vec![], sell_offers: vec![]});
    {
        let resp = resp.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                let fetched_resp = get_sell_offers().await.unwrap();
                resp.set(fetched_resp)
            });
            || ()
        }, ());
    }

    let offers = resp.sell_offers.iter().map(|offer| html! {
        <div class="col-md-4">
            <div class="card mb-4 box-shadow">
                <div class="card-body">
                    <div class="card-text">
                        <p>{format!("Cardid: {}", offer.card)}</p>
                        <p>{format!("Seller: {}", offer.seller)}</p>
                        <p>{format!("Price: {}", parse_price(&offer.price))}</p>
                        <p>{format!("Status: {:?}", offer.status)}</p>
                    </div>
                    <div class="d-flex justify-content-between align-items-center">
                        <div class="btn-group">
                            <button type="button" class="btn btn-sm btn-outline-secondary">{"View card"}</button>
                            <button type="button" class={ format!("btn btn-sm btn-outline-secondary {}", match offer.status {SellOfferStatus::Open => "", _ => "disabled"}) }>{"Buy"}</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }).collect::<Html>();

    html! {
        <body>
            { get_header() }
            <main role="main">
                <div class="album py-5 bg-light">
                    <div class="container">
                        <div class="row">
                            {offers}
                        </div>
                    </div>
                </div>
            </main>
            { get_footer() }
        </body>
    }
}

fn get_header() -> Html {
    html! {
        <>
            <div class="p-5 bg-light text-center" style="margin-bottom:0; background-image:url('http://images.designtrends.com/wp-content/uploads/2016/03/29085517/Solar-Lights-Bright-Background.jpg')">
                <h1>{"My First Bootstrap Page"}</h1>
                <p>{"Resize this responsive page to see the effect!"}</p>
            </div>
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                <div class="container-fluid">
                    <a class="navbar-brand" href="#">{"Navbar"}</a>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarNav">
                        <ul class="navbar-nav">
                            <li class="nav-item">
                                <a class="nav-link active" aria-current="page" href="#">{"Home"}</a>
                            </li>
                            <li class="nav-item">
                                <a class="nav-link" href="#">{"Features"}</a>
                            </li>
                            <li class="nav-item">
                              <a class="nav-link" href="#">{"Pricing"}</a>
                            </li>
                            <li class="nav-item">
                              <a class="nav-link disabled">{"Disabled"}</a>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
        </>
    }
}

fn get_footer() -> Html {
    html! {
        <>
            <footer class="text-muted">
                <div class="container">
                    <p class="float-right">
                        <a href="#">{"Back to top"}</a>
                    </p>
                    <p>{"Album example is &copy; Bootstrap, but please download and customize it for yourself!"}</p>
                </div>
            </footer>
        </>
    }
}

fn parse_price(Price {denom, amount}: &Price) -> String {
    format!("{}{}", amount, denom)
}

async fn get_sell_offers() -> Result<SellOffersResponse, Error> {
    let request_url = format!("{}/DecentralCardGame/cardchain/cardchain/q_sell_offers/%22%22/%22%22/%22%22/%22%22/open?ignore.status=false&ignore.price=true&ignore.seller=true&ignore.buyer=true&ignore.card=true", ROOT_URL);
    let sell_offers_response: SellOffersResponse = Request::get(&request_url).send()
                                                                             .await?
                                                                             .json()
                                                                             .await?;
    Ok(sell_offers_response)
}

fn main() {
    yew::start_app::<App>();
}
