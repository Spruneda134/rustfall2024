use ureq;
use serde::Deserialize;
use std::error::Error;

// #[derive(Debug, Deserialize)]
// struct Crypt {
//     "bitcoin": {
//         "usd": 66129
//     },
//     "ethereum": {
//         "usd": 2543.21
//     }
// }

#[derive(Debug, Deserialize)]
struct Price {
    usd: f32,
}

#[derive(Debug, Deserialize)]
struct Crypt {
    bitcoin: Price,
    ethereum: Price,
}


fn main() {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd";

    let req = ureq::get(url).call().unwrap();
    let content = req.into_json::<Crypt>();

    print!("{:?}", content);
}
