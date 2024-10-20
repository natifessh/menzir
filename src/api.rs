pub mod api{
    use crate::{command, models::models::{Args, CoinHistory}};
    use clap::Parser;
    use reqwest::Error;
    use colored::*;

    
    pub async fn make_api_call(args:&Args)->Result<(),anyhow::Error>{
      let args=Args::parse();
      command::command::match_command(args).await?;
       
       
    
        Ok(())
    }
}