mod kraken;
mod kraken_futures;

pub enum PriceType {
    Ask,
    Bid,
}

impl From<&str> for PriceType {
    fn from(s: &str) -> PriceType {
        match s {
            "buy" => PriceType::Ask,
            "sell" => PriceType::Bid,
            v => panic!(format!("Bad operation {}", v)),
        }
    }
}

pub fn create_exchange(s: &str) -> Result<Box<dyn Exchange>, String> {
    match s {
        "kraken-futures" => Ok(Box::new(kraken_futures::Exchange())),
        "kraken" => Ok(Box::new(kraken::Exchange())),
        other => Err(format!("Bad exchange type: {}", other)),
    }
}

pub trait Exchange {
    fn get_prices(
        &self,
        symbol: &str,
        price_type: &PriceType,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}
