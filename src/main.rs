use reqwest;
use serde_json::Value;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://contract.mexc.com/api/v1/contract/ticker";
    let response = reqwest::get(url).await?.text().await?;
    let data: Value = serde_json::from_str(&response)?;

    let mut coins: Vec<String> = Vec::new();

    if let Some(tickers) = data["data"].as_array() {
        for ticker in tickers {
            if let Some(symbol) = ticker["symbol"].as_str() {
                if symbol.ends_with("USDT") {
                    let cleaned_symbol = symbol.trim_end_matches("USDT").replace("_", "");
                    coins.push(format!("MEXC:{}USDT.P", cleaned_symbol));
                }
            }
        }
    }

    coins.sort_by(|a, b| {
        let a_name = a.split(':').nth(1).unwrap_or("").trim_end_matches("USDT.P");
        let b_name = b.split(':').nth(1).unwrap_or("").trim_end_matches("USDT.P");

        let a_numeric = a_name
            .chars()
            .take_while(|c| c.is_numeric())
            .collect::<String>();
        let b_numeric = b_name
            .chars()
            .take_while(|c| c.is_numeric())
            .collect::<String>();

        if !a_numeric.is_empty() && !b_numeric.is_empty() {
            let a_num: u32 = a_numeric.parse().unwrap_or(0);
            let b_num: u32 = b_numeric.parse().unwrap_or(0);
            if a_num != b_num {
                return b_num.cmp(&a_num);
            }
        }

        if a_numeric.is_empty() != b_numeric.is_empty() {
            return if a_numeric.is_empty() {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        }

        a_name.cmp(b_name)
    });

    let mut file = File::create("mexc_usdt_perpetual_futures.txt")?;
    for coin in coins {
        writeln!(file, "{}", coin)?;
    }

    println!("Coin listesi başarıyla oluşturuldu ve kaydedildi.");
    Ok(())
}
