use std::fs::{File, OpenOptions};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use serde::Deserialize;

trait Pricing {
    fn fetch_price(&self) -> Option<f32>;
    fn save_to_file(&self, price: f32);
}

// Bitcoin Struct
#[derive(Debug)]
struct Bitcoin {
    api_address: String,
    file_name: String,
}

#[derive(Debug, Deserialize)]
struct BitcoinResponse {
    bitcoin: BitcoinPrice,
}

#[derive(Debug, Deserialize)]
struct BitcoinPrice {
    usd: f32,
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Option<f32> {
        let response = ureq::get(&self.api_address).call();
        match response {
            Ok(res) => {
                let api_response: Result<BitcoinResponse, _> = res.into_json();
                match api_response {
                    Ok(data) => Some(data.bitcoin.usd),
                    Err(e) => {
                        println!("Bitcoin price data parse failed: {:?}", e);
                        None
                    }
                }
            }
            Err(e) => {
                println!("Bitcoin price data fetch failed: {:?}", e);
                None
            }
        }
    }

    fn save_to_file(&self, price: f32) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_name)
            .expect("Could not open file");
        writeln!(file, "Bitcoin price: ${}", price).expect("Writing to file failed");
        println!("Updated Bitcoin price, {}", self.file_name);
    }
}

// Ethereum Struct
#[derive(Debug)]
struct Ethereum {
    api_address: String,
    file_name: String,
}

#[derive(Debug, Deserialize)]
struct EthereumResponse {
    ethereum: EthereumPrice,
}

#[derive(Debug, Deserialize)]
struct EthereumPrice {
    usd: f32,
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> Option<f32> {
        let response = ureq::get(&self.api_address).call();
        match response {
            Ok(res) => {
                let api_response: Result<EthereumResponse, _> = res.into_json();
                match api_response {
                    Ok(data) => Some(data.ethereum.usd),
                    Err(e) => {
                        println!("Ethereum price data parse failed: {:?}", e);
                        None
                    }
                }
            }
            Err(e) => {
                println!("Ethereum price data fetch failed: {:?}", e);
                None
            }
        }
    }

    fn save_to_file(&self, price: f32) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_name)
            .expect("Could not open file");
        writeln!(file, "Ethereum price: ${}", price).expect("Writing to file failed");
        println!("Updated Ethereum price, {}", self.file_name);
    }
}

// S&P 500 Struct
#[derive(Debug)]
struct SP500 {
    api_address: String,
    file_name: String,
}

#[derive(Debug, Deserialize)]
struct SP500Response {
    chart: Chart,
}

#[derive(Debug, Deserialize)]
struct Chart {
    result: Vec<ChartResult>,
}

#[derive(Debug, Deserialize)]
struct ChartResult {
    meta: Meta,
}

#[derive(Debug, Deserialize)]
struct Meta {
    regularMarketPrice: f32,
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> Option<f32> {
        let response = ureq::get(&self.api_address).call();
        match response {
            Ok(res) => {
                let api_response: Result<SP500Response, _> = res.into_json();
                match api_response {
                    Ok(data) => Some(data.chart.result[0].meta.regularMarketPrice),
                    Err(e) => {
                        println!("S&P 500 price data parse failed: {:?}", e);
                        None
                    }
                }
            }
            Err(e) => {
                println!("S&P 500 price data fetch failed: {:?}", e);
                None
            }
        }
    }

    fn save_to_file(&self, price: f32) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_name)
            .expect("Could not open file");
        writeln!(file, "S&P 500 price: ${}", price).expect("Writing in file failed");
        println!("Updated S&P 500 price, {}", self.file_name);
    }
}

fn main() {
    let bitcoin = Bitcoin {
        api_address: "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd".to_string(),
        file_name: "bitcoin_price.txt".to_string(),
    };

    let ethereum = Ethereum {
        api_address: "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd".to_string(),
        file_name: "ethereum_price.txt".to_string(),
    };

    let sp500 = SP500 {
        api_address: "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d".to_string(),
        file_name: "sp500_price.txt".to_string(),
    };

    let assets: Vec<&dyn Pricing> = vec![&bitcoin, &ethereum, &sp500];

    loop {
        for asset in &assets {
            if let Some(price) = asset.fetch_price() {
                asset.save_to_file(price);
            }
        }
        println!("Waiting for 10 seconds...");
        sleep(Duration::from_secs(10));
    }
}
