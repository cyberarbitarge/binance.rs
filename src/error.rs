//! errors wrapper 

use std::error::Error;
use std::fmt;
use std::error;
use std::fmt::Debug;
use openssl::error::ErrorStack;

#[derive(Debug, Clone)]
pub enum BinanceError {
    Openssl(ErrorStack),
    Configuration{
        cause: String,
        //FIXME: maybe need another fields 
    }
}

impl error::Error for BinanceError {}

impl fmt::Display for BinanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &BinanceError::Openssl(ref e) => std::fmt::Display::fmt(&e, f),
            &BinanceError::Configuration{ref cause} => write!(f, "{}", cause)
        }
    }
}

impl From<ErrorStack> for BinanceError {
    fn from(e: ErrorStack) -> Self {
        BinanceError::Openssl(e)
    }
}

