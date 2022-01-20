//! spot api 

use crate::{config::Config, error::{self, BinanceError}};
use reqwest::{header::HeaderMap};


pub struct Client<'a> {
    config: &'a crate::config::Config,
    executor: reqwest::Client,
}

impl <'a> Client<'a> {

    pub fn new(config: &'a Config) -> Self {
        let mut headers = HeaderMap::new();
        headers.append("X-MBX-APIKEY", config.api_key.parse().unwrap());
        Client{
            config,
            executor: reqwest::ClientBuilder::new()
            .default_headers(headers).build().unwrap(),
        }
    }

    // FIXME: sapi fialed when run unit test ?
    pub async fn get_system_status(&self) -> Result<String, error::BinanceError> {
        match self.executor
        .get(format!("{}{}",self.config.spot_api_url, "/sapi/v1/system/status"))
        .send()
        .await? 
        .text_with_charset("utf-8")
        .await {
            Ok(body) => Ok(body),
            Err(e) => Err(BinanceError::Client(e))
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::config::Builder;
    use tokio;
    #[test]
    fn test_normal_request() {
        let config = Builder::new()
        .api_key("qe2vwkaoae7rsGf87S2ZrbHTul4g0pZdxaETizzvz553MuOJPdri1ofpoFraQZO7")
        .api_secret("BrGL5TXeezl9YgA6GSvIr7U6zkQQRGgkw2aGU9cLbgVy7c36Sx6X0dsAmuyICTI3")
        .spot_api_url("https://testnet.binance.vision")
        .build().unwrap();

        let client = super::Client::new(&config);

        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            match client.get_system_status().await {
                Ok(body) => println!("{}", body),
                Err(e) => println!("{:?}", e.to_string())
            }
        })
    }
}