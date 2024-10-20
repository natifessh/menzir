pub mod models {

    use std::collections::HashMap;

    use clap::Parser;
    use serde::Deserialize;
    
    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    pub struct Args {
        #[arg(short = 'f', long)]
        pub from_currency: Option<String>,
        #[arg(short = 't', long)]
        pub to_currency: Option<String>,
        #[arg(short = 'a', long)]
        pub amount: Option<i32>,
        #[arg(short = 'e', long)]
        pub exchange_detail: Option<String>,
        #[arg(short='d',long)]
        pub date:Option<String>
    }

    #[derive(Deserialize, Debug)]
    pub struct CurrencyInfo {
        pub name: String,
        pub unit: String,
        pub value: f64,
        pub type_: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct ExchangeHistory {
        pub name: String,
        pub year_established: i32,
        pub country: Option<String>,
        pub centralized: bool,
        pub trust_score: i32,
        pub trust_score_rank: i32,
        pub trade_volume_24h_btc: f64,
        pub trade_volume_24h_btc_normalized: f64,
        pub tickers: Vec<Tickers>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Tickers {
        pub base: String,
        pub target: String,
        pub market: Market,
        pub last_traded_at: String,
        pub trust_score: String,
        pub bid_ask_spread_percentage: f32,
        pub last_fetch_at: String,
        pub volume:f32,
        pub cost_to_move_up_usd:Option<f32>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Market {
        pub name: String,
        pub identifier: String,
        pub has_trading_incentive: bool,
    }

    #[derive(Deserialize)]
    pub struct ExchangeRates {
        pub rates: HashMap<String, CurrencyInfo>,
    }
    #[derive(Debug,Deserialize)]
    pub struct CoinHistory{
        pub id:String,
        pub name:String,
        pub symbol:String,
        pub market_data:CurrentPrice
       


    }
    #[derive(Debug,Deserialize)]
    pub struct CurrentPrice{
        pub current_price: HashMap<String, f64>, 
    }
}

