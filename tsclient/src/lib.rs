use {
  poise::serenity_prelude::Token,
  std::{
    str::FromStr,
    sync::LazyLock
  },
  tokenservice_client::{
    TokenService,
    TokenServiceApi
  },
  tokio::sync::Mutex
};

pub struct TSClient(TokenService);

impl Default for TSClient {
  fn default() -> Self { Self::new() }
}

impl TSClient {
  pub fn new() -> Self {
    let args: Vec<String> = std::env::args().collect();
    let service = if args.len() > 1 { &args[1] } else { "pgbot" };
    Self(TokenService::new(service))
  }

  pub async fn get(&self) -> Result<TokenServiceApi, Box<dyn std::error::Error + Send + Sync>> {
    match self.0.connect().await {
      Ok(api) => Ok(api),
      Err(e) => Err(e)
    }
  }
}

static TSCLIENT: LazyLock<Mutex<TSClient>> = LazyLock::new(|| Mutex::new(TSClient::new()));

pub async fn token_path() -> TokenServiceApi { TSCLIENT.lock().await.get().await.unwrap() }

pub async fn discord_token() -> Token { Token::from_str(&token_path().await.main).expect("Serenity couldn't parse the bot token!") }
