   mod types;
   use types::StockData;
  
   use discord_flows::{
       model::Message,
       ProvidedBot, Bot,
   };
  use flowsnet_platform_sdk::logger;
  use anyhow::bail;
  use http_req::request;
  use regex::Regex;
 
  #[no_mangle]
  #[tokio::main(flavor = "current_thread")]
  pub async fn run() -> anyhow::Result<()> {
 
      let discord_token = std::env::var("discord_token").unwrap();
      let bot = ProvidedBot::new(discord_token);
      bot.listen(|msg| handler(&bot, msg)).await;
      Ok(())
  }
 
  async fn handler(bot: &ProvidedBot, msg: Message) {
      logger::init();
      let discord = bot.get_client();
 
      if msg.author.bot {
          log::debug!("ignored bot message");
          return;
      }
      if msg.member.is_some() {
          log::debug!("ignored channel message");
          return;
      }
 
      let channel_id = msg.channel_id;
 
      let stock_ticker_pattern = Regex::new(r"!stock\s*([\w-]+)").unwrap();
      if let Some(matched_ticker) = stock_ticker_pattern.captures(&msg.content) {
	   let ticker = matched_ticker.get(1).map_or("N/A", |m| m.as_str());
 
      match fetch_stock_price(&ticker) {
          Ok(price) => {
                  let resp = format!("The current market price for {} is {}. Do not take this as a financial advice", ticker, price);
                  _ = discord.send_message(
                      channel_id.into(),
                      &serde_json::json!({
                          "content": resp
                      }),
                  ).await;
          },
          Err(err) => {
          log::debug!("Failed to fetch stock price fpr {}: {}", ticker, err);
                  _ = discord.send_message(
                      channel_id.into(),
                      &serde_json::json!({
                          "content": format!("I couldn't fetch the stock price for {}. you could be entering a wrong ticker, the stock may be outside of our coverage markets, or we could be tracking it with a somewhat different ticker if it's a stock listed outside the U.S markets", ticker)
                      }),
                  ).await;
          }
       }
  } else {
	log::debug!("Stock ticker pattern could not be fetched from message");
                  _ = discord.send_message(
                      channel_id.into(),
                      &serde_json::json!({
                          "content": "I couldn't fetch the ticker from your message, please make sure to include the pattern !stock <ticker>, e.g !stock MSFT in your message."
                      }),
                  ).await;
    }
}
 
 fn fetch_stock_price(ticker: &str) -> anyhow::Result<String> {
      let api_key = std::env::var("alpha_vantage_api_key").unwrap();
      let request_url = format!("https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol={}&interval=5min&apikey={}", ticker, api_key);
      
      let mut writer = Vec::new();
      request::get(request_url, &mut writer).map_err(|e| anyhow::Error::from(e))?; 
      let market_data: StockData = serde_json::from_slice(&writer)?;
      let last_refreshed_time = &market_data.meta_data.last_refreshed;
      if let Some(last_data) = market_data.time_series.get(last_refreshed_time) {
          Ok(last_data.close.clone())
      } else {
          bail!("Stock data not found")
      }
  }
 

