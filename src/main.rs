use reqwest::Error;
use serde::{de, Deserialize};
use std::{collections::HashMap, time::Duration};
use colored::*;
use clap::Parser;
mod models;
mod api;
mod display;
mod command;
use models::models::{Args, ExchangeHistory, ExchangeRates, CoinHistory};
use api::api::make_api_call;



#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    make_api_call(&args).await?;
    Ok(())
 
}
