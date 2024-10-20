pub mod display{
    use crate::models::models::{CurrencyInfo, ExchangeHistory, CoinHistory};
    use colored::*;

    pub fn DisplayExchangeRateHistory(data:ExchangeHistory){
        println!("{}",format!("Crytocurrency Exchange detail for {}",data.name.bright_cyan()).green());
        println!(
            "name: {} \ncountry: {} \nyear {} \nmarket identifier: {} \ntrust_score {} \ntrust_score_rank {} \ncentralized {} \ntrade_volume_24h_btc {} \ntrade_volume_24h_btc_normalized {} \nVolume {:?} \ntarget {}\nbase {}",
            data.name.red(),
            data.country.unwrap().red(),
            data.year_established.to_string().red(),
           
            data.tickers[0].market.identifier.red(),
            data.trust_score.to_string().red(),
            data.trust_score_rank.to_string().red(),
            data.centralized.to_string().red(),
            data.trade_volume_24h_btc.to_string().red(),
            data.trade_volume_24h_btc_normalized.to_string().red(),
            data.tickers.get(0).unwrap().volume.to_string().red(),
            data.tickers.get(0).unwrap().target.red(),
            data.tickers.get(0).unwrap().base.red()
        );

    }
    pub fn CalculateExchangeRate(btc_rate: &CurrencyInfo, usd_rate: &CurrencyInfo, amount: Option<i32>) {
        match amount {
            Some(amt) => {
                let converted_value = (usd_rate.value as f32 / btc_rate.value as f32) * amt as f32;
                println!(
                    "{} {} = {} {}",
                    amt.to_string().blue(),
                    btc_rate.unit.blue(),
                    converted_value.to_string().green(),
                    usd_rate.unit.green()
                );
            }
            None => {
                let conversion_rate = usd_rate.value as f32 / btc_rate.value as f32;
                println!(
                    "1 {} = {:.2} {}",
                    btc_rate.unit.blue(),
                    conversion_rate,
                    usd_rate.unit.green()
                );
            }
        }
    }
    
    pub fn DisplayCoinHistory(data: CoinHistory) {
        println!("{}", data.name.bright_red().bold());
        println!("{}", "Exchange Rates On The Date You Provided".underline().yellow());
        println!();
        println!("{:<20} | {}", "Currency".blue().bold(), "Price".red().bold());
        println!("{:-<20} | {:-<10}", "", ""); // Separator line
        for (currency, price) in data.market_data.current_price {
            println!("{:<20} | ${:.2}", currency.to_string().blue(), price); // Formatting price to 2 decimal places
        }
    
        println!(); // Extra line for spacing
        println!("{}", "End of data".bright_magenta().bold());
    }
    
}