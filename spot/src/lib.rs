//! Binance Spot Connector
//!
//! Supplies the primitives to connect to Binance and trade any token.

mod error;
mod filter;
mod request;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
