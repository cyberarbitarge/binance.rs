//! errors wrapper 

use std::fmt;
use std::error;
use std::fmt::Debug;
use openssl::error::ErrorStack;

pub type OpensslError = BinanceError<ErrorStack>;

pub type ConfigurationError = BinanceError<CommonError>;


pub enum Cause {
    OpensslError,
    ConfigError
}

impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Cause::ConfigError => write!(f, "{}", "configuration error"),
            &Cause::OpensslError => write!(f, "{}", "openssl error")
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinanceError<T> where T : error::Error + Clone + fmt::Display + Debug{
    cause: Cause, 
    inner: T,
}

impl<T> error::Error for BinanceError<T> where T: error::Error + Clone {} 

unsafe impl<T> Send for BinanceError<T>{}
unsafe impl<T> Sync for BinanceError<T>{}

impl<T> From<ErrorStack> for BinanceError<T> {

    fn from(e: ErrorStack) -> Self {
        Self{
            cause: Cause::OpensslError,
            inner: e,
        }
    }
}

impl<T> fmt::Display for BinanceError<T> where T:  fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.cause, self.inner);
        Ok(())
    }
}

pub struct CommonError {
    cause: String 
}

impl error::Error for CommonError {}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",self.cause);
        Ok(())
    }
}

impl Clone for CommonError {
    fn clone(&self) -> Self {
        Self{
            cause: self.cause,
        }
    }
}