//! binance api config 
//! TODO: complete the document 

use serde::{Serialize, Deserialize};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use crate::error;


#[derive(Serialize, Deserialize,Debug)]
pub struct Config{
    api_key: &'static str,
    api_secret: Vec<u8>,
    spot_api_url: &'static str,
    spot_ws_url: &'static str,
    spot_stream_url: &'static str,
    spot_order_recv_window: u32,
}

impl Config {

    // sign the message to signature used in binance apis 
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, error::BinanceError> {
        let secret = PKey::hmac(&self.api_secret)?;
        let mut signer = Signer::new(MessageDigest::sha256(), &secret)?;
        signer.update(message)?;
        match signer.sign_to_vec() {
            Ok(v) => Ok(v),
            Err(e) => Err(error::BinanceError::Openssl(e))
        }
    }
}

pub struct Builder {
    api_key: &'static str,
    api_secret: Vec<u8>,
    spot_api_url: &'static str,
    spot_ws_url: &'static str,
    spot_stream_url: &'static str,
    spot_order_recv_window: u32,
}

impl Builder {
    
    pub fn new() -> Self {
        Self {
            api_key: "",
            api_secret: vec![],
            spot_api_url: "https://api.binance.com/api",
            spot_ws_url: "wss://stream.binance.com:9443/ws",
            spot_stream_url: "wss://stream.binance.com:9443/stream",
            spot_order_recv_window: 5000,
        }
    }

    //TODO: use Result 
    pub fn build(self) -> Result<Config, error::BinanceError> {
        if self.api_key.len() == 0 || self.api_secret.is_empty() {
            return Err(error::BinanceError::Configuration{cause: "api key or secret if invalid ".to_string()});
        }
        Ok(
            Config{
                api_key: self.api_key,
                api_secret: self.api_secret,
                spot_api_url: self.spot_api_url,
                spot_ws_url: self.spot_ws_url,
                spot_stream_url: self.spot_stream_url,
                spot_order_recv_window: self.spot_order_recv_window,
            }
        )
    }

    pub fn api_key(mut self, key: &'static str) -> Builder {
        self.api_key = key;
        self
    }

    pub fn api_secret(mut self, secret: &'static str) -> Builder {
        self.api_secret = secret.as_bytes().to_vec();
        self 
    }

    pub fn spot_api_url(mut self, url: &'static str) -> Builder {
        self.spot_api_url = url;
        self
    }

    pub fn spot_ws_url(mut self, url: &'static str) -> Builder {
        self.spot_ws_url = url;
        self
    }

    pub fn spot_stream_url(mut self, url: &'static str) -> Builder {
        self.spot_stream_url = url;
        self 
    }

    pub fn spot_order_recv_window(mut self, window: u32) -> Builder {
        self.spot_order_recv_window = window;
        self 
    }

}

#[cfg(test)]
mod tests {
    use openssl::hash::MessageDigest;
    use openssl::memcmp;
    use openssl::pkey::PKey;
    use openssl::sign::Signer;
    use hex;

    #[test]
    fn test_openssl_usage() {
        let target = hex::decode("c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71").unwrap();
        let secret = PKey::hmac(b"NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j").unwrap();
        let mut signer = Signer::new(MessageDigest::sha256(), &secret).unwrap();
        signer.update(b"symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559").unwrap();
        let signature = signer.sign_to_vec().unwrap();
        assert!(memcmp::eq(&target, &signature))
    }

    #[test]
    fn test_config_sign() {
        let target = hex::decode("c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71").unwrap();
        let config = super::Builder::new()
        .api_key("key")
        .api_secret("NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j")
        .build().unwrap();

        let signature = config.sign(b"symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559").unwrap();

        assert!(memcmp::eq(&target, &signature))
    }
}