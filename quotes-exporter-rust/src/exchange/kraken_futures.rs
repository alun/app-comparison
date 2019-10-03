use super::PriceType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CryptoFacilitiesResponse {
    #[serde(rename = "orderBook")]
    order_book: BidsAsks,
}

#[derive(Debug, Deserialize)]
struct DomRow(f64, i32);

#[derive(Debug, Deserialize)]
struct BidsAsks {
    asks: Vec<DomRow>,
    bids: Vec<DomRow>,
}

pub struct Exchange();

impl super::Exchange for Exchange {
    fn get_prices(
        &self,
        symbol: &str,
        price_type: &PriceType,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let resp: CryptoFacilitiesResponse = reqwest::get(&format!(
            "https://www.cryptofacilities.com/derivatives/api/v3/orderbook?symbol={}",
            symbol
        ))?
        .json()?;
        let rows = match price_type {
            PriceType::Ask => &resp.order_book.asks,
            PriceType::Bid => &resp.order_book.bids,
        };
        let result: Vec<String> = rows.iter().map(|r| r.0.to_string()).collect();
        Ok(result)
    }
}
