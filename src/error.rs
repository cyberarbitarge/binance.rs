//! errors wrapper 
//! TODO: need refactor the inner design pattern 

use std::fmt;
use std::error;
use std::fmt::Debug;
use openssl::error::ErrorStack;
use reqwest;


#[derive(Debug)]
pub enum BinanceError {
    Openssl(ErrorStack),
    Configuration{
        cause: String,
        //FIXME: maybe need another fields 
    },
    Client(reqwest::Error),
}

impl error::Error for BinanceError {}

impl fmt::Display for BinanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &BinanceError::Openssl(ref e) => std::fmt::Display::fmt(&e, f),
            &BinanceError::Configuration{ref cause} => write!(f, "{}", cause),
            &BinanceError::Client(ref e) => std::fmt::Display::fmt(&e, f),
        }
    }
}

impl From<ErrorStack> for BinanceError {
    fn from(e: ErrorStack) -> Self {
        BinanceError::Openssl(e)
    }
}


impl From<reqwest::Error> for BinanceError {
    fn from(e: reqwest::Error) -> Self {
        BinanceError::Client(e)
    }
}
