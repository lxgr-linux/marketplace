
use serde::Deserialize;
use reqwasm::http::Request;
use reqwasm::http::Response;
use reqwasm::Error;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Deserialize, Debug)]
enum Status {
    open,
    sold,
    removed
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
    status: Status
}

#[derive(Deserialize, Debug)]
struct SellOffersResponse {
    sellOffersIds: Vec<String>,
    sellOffers: Vec<SellOffer>,
}

#[function_component(App)]
fn app() -> Html {
    let resp = use_state(|| SellOffersResponse{sellOffersIds: vec![], sellOffers: vec![]});
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

    let offers = resp.sellOffers.iter().map(|offer| html! {
        <div class="container">
            <p>{format!("Cardid: {}", offer.card)}</p>
            <p>{format!("Seller: {}", offer.seller)}</p>
            <p>{format!("Price: {}", parse_price(&offer.price))}</p>
            <p>{format!("Status: {:?}", offer.status)}</p>
        </div>
    }).collect::<Html>();

    html! {
        <body>
            <div class="p-5 mb-4 bg-light rounded-3" style="margin-bottom:0">
                <h1>{"My First Bootstrap Page"}</h1>
                <p>{"Resize this responsive page to see the effect!"}</p>
            </div>
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                <div class="container-fluid">
                    <div class="container-fluid">
                        <button type="button" class="navbar-toggler" data-toggle="collapse" data-target="#myNavbar">
                            <a class="navbar-brand" href="#">{"WebSiteName"}</a>
                        </button>
                    </div>
                    <div class="collapse navbar-collapse" id="myNavbar">
                        <ul class="nav navbar-nav">
                            <li class="active"><a href="#">{"Home"}</a></li>
                        </ul>
                    </div>
                </div>
            </nav>
            <h1>{ "Hello World" }</h1>
            {offers}
        </body>
    }
}

fn parse_price(Price {denom, amount}: &Price) -> String {
    format!("{}{}", amount, denom)
}

async fn get_sell_offers() -> Result<SellOffersResponse, Error> {
    let request_url = "http://0.0.0.0:1317/DecentralCardGame/cardchain/cardchain/q_sell_offers/%22%22/%22%22/%22%22/%22%22/open?ignore.status=false&ignore.price=true&ignore.seller=true&ignore.buyer=true&ignore.card=true";
    let response: Response = Request::get(request_url).send()
                                                     .await
                                                     .expect("Hier");
    let sell_offers_response: SellOffersResponse = response.json().await.unwrap();
    Ok(sell_offers_response)
}

fn main() {
    yew::start_app::<App>();
}
