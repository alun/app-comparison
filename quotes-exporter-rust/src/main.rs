use quotes_exporter::exchange::{create_exchange, PriceType};
use quotes_exporter::sheets::Doc;
use serde::Deserialize;
use std::collections::HashMap;
use std::{fs, thread, time};

#[derive(Deserialize, Debug)]
struct Config {
    sheet: String,
    timeout: u32,
    data: HashMap<String, Vec<String>>,
}

fn export_quotes(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let doc: Doc = From::from(config.sheet.as_ref());
    for (conf, cells) in config.data.iter() {
        let conf: Vec<&str> = conf.split(":").collect();
        let (exchange, symbol, price_type) =
            (create_exchange(conf[0])?, conf[1], PriceType::from(conf[2]));
        for cell in cells {
            let prices = exchange.get_prices(symbol, &price_type)?;
            doc.write(cell, &prices[0])?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting quotes updater");
    let config: Config = serde_json::from_str(
        &fs::read_to_string("./config/config.json").or(Err("Expected config/config.json file"))?,
    )?;
    println!("Config: {:?}", &config);
    loop {
        export_quotes(&config)?;
        println!("Exported quotes: {:?}", &config.data);
        thread::sleep(time::Duration::from_millis(config.timeout as u64))
    }
}
