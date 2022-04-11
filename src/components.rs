use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::types::*;
use crate::cardchain::*;

pub fn get_footer() -> Html {
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

pub fn get_header() -> Html {
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

pub fn get_offers() -> Html {
    let html_resp = use_state(|| Html::default());
    {
        let html_resp = html_resp.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                let mut fetched_html_resp = html!{};
                let path = "/DecentralCardGame/cardchain/cardchain/q_sell_offers/%22%22/%22%22/%22%22/%22%22/open?ignore.status=false&ignore.price=true&ignore.seller=true&ignore.buyer=true&ignore.card=true";
                let resp:SellOffersResponse = get_from_cardchain(path).await.unwrap();
                for offer in resp.sell_offers.iter() {
                    let pic = get_card_image(&offer.card).await;
                    fetched_html_resp = html! {
                        <>
                            { fetched_html_resp }
                            <div class="col-md-4">
                                <div class="card mb-4 box-shadow">
                                    <img class="card-img-top" src={pic} alt="Card image cap"/>
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
                        </>
                    }
                };

                html_resp.set(fetched_html_resp);
            });
            || ()
        }, ());
    };
    (*html_resp).clone()
}

fn parse_price(Coin {denom, amount}: &Coin) -> String {
    format!("{}{}", amount, denom)
}
