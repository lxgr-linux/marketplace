
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
                let fetched_resp = getSellOffers().await.unwrap();
                resp.set(fetched_resp)
            });
            || ()
        }, ());
    }

    let offers = resp.sellOffers.iter().map(|offer| html! {
        <p>{format!("{}: {}", offer.seller, offer.card)}</p>
    }).collect::<Html>();

    html! {
        <>
            <h1>{ "Hello World" }</h1>
            {offers}
        </>
    }
}

async fn getSellOffers() -> Result<SellOffersResponse, Error> {
    let request_url = "http://0.0.0.0:1317/DecentralCardGame/cardchain/cardchain/q_sell_offers/%22%22/%22%22/%22%22/%22%22/open?ignore.status=false&ignore.price=true&ignore.seller=true&ignore.buyer=true&ignore.card=true";
    let response: Response = Request::get(request_url).send()
                                                     .await
                                                     .expect("Hier");
    let sellOffersResponse: SellOffersResponse = response.json().await.unwrap();
    println!("AVC{:?}", sellOffersResponse);
    Ok(sellOffersResponse)
}

fn main() {
    yew::start_app::<App>();
}
