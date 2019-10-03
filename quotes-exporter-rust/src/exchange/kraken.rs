use super::PriceType;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct KrakenResponse {
    result: HashMap<String, BidAsk>,
}

#[derive(Deserialize, Debug)]
struct DomRow(String, String, i64);

#[derive(Deserialize, Debug)]
struct BidAsk {
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
        let resp: KrakenResponse = reqwest::get(&format!(
            "https://api.kraken.com/0/public/Depth?pair={}",
            symbol
        ))?
        .json()?;
        let upper_symbol = symbol.to_uppercase();
        let (base, quote) = upper_symbol.split_at(3);
        let dom = resp
            .result
            .get(&format!("X{}Z{}", base, quote))
            .map_or(Err("Expected result"), |v| Ok(v))?;
        let rows = match price_type {
            PriceType::Ask => &dom.asks,
            PriceType::Bid => &dom.bids,
        };
        let result: Vec<String> = rows.iter().map(|e| e.0.clone()).collect();
        Ok(result)
    }
}
