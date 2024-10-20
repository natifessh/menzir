pub mod command{
    use crate::models::models::{Args, CoinHistory};

    use crate::{display::display::{CalculateExchangeRate, DisplayCoinHistory, DisplayExchangeRateHistory}, models::models::{ExchangeHistory, ExchangeRates}};

    pub async fn match_command(args:Args)->Result<(),reqwest::Error>{
        //this url will give cryto exchange rates in bitcoin for many fiat and crypto currencies so we will need to get from and to fmanually rom the response using get 
        let mut url = format!("https://api.coingecko.com/api/v3/exchange_rates");
        let mut response = reqwest::get(&url).await?;
        let data: ExchangeRates = response.json().await?;
        match ( args.from_currency.as_ref().map(|c| c.to_lowercase()), 
        args.to_currency.as_ref().map(|c| c.to_lowercase()), 
        args.exchange_detail.as_ref().map(|c| c.to_lowercase()), 
        &args.date) {
            (Some(from), Some(to), _,_) if args.amount.is_some() => {
                if let (Some(from),Some(to))=(data.rates.get(&from),data.rates.get(&to)){
                    let amt=args.amount.unwrap();
                    CalculateExchangeRate(from,to, Some(amt));
                }else {
                    eprintln!("ERR Currencie not found")
                }   
            }
            (Some(from), Some(to), _,_) => {
                if let (Some(from),Some(to))=(data.rates.get(&from),data.rates.get(&to)){
                    CalculateExchangeRate(from, to,None);
                }else{
                    eprintln!("Error Currencies not found")
                }
            }
            (_, _, Some(exchange_detail),_) => {
                url = format!("https://api.coingecko.com/api/v3/exchanges/{}", exchange_detail.to_string());
                response = reqwest::get(&url).await?;
                let data: ExchangeHistory = response.json().await?;
                DisplayExchangeRateHistory(data);
            }
            (btc_rate,_,__,Some(history))=>{
                let id=args.from_currency.clone().unwrap();
                let history=args.date.clone().unwrap();
                url = format!("https://api.coingecko.com/api/v3/coins/{}/history?date={}",id,history);
                response = reqwest::get(&url).await?;
                let data:CoinHistory= response.json().await?;
                DisplayCoinHistory(data);
            }
            _ => (),
        }
        Ok(())


    }
}