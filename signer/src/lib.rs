use anyhow::Result;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sign::Signer as OpensslSigner;

/// provide the binance api secret
pub trait SecretProvider<'a> {
    /// return current binance secret key
    fn secret(&'a self) -> &'a [u8];
}

/// parameters provider
/// FIXME: should change the parameter to AsRef<[u8]> then could use the Vec<u8> to generate the 
/// parameter 
pub trait ParameterProvider<'a> {
    /// return the parameters wait to sign
    fn wait_to_sign(&'a self) -> &'a [u8];
}

/// sign a message witin the secret provider
pub struct Signer<S> {
    secret_provider: S,
}

impl<S> Signer<S> {
    pub fn new(s: S) -> Self {
        Signer { secret_provider: s }
    }
}

impl<'a, S> Signer<S>
where
    S: SecretProvider<'a>,
{
    pub fn sign<P: ParameterProvider<'a>>(&'a self, parameter: &'a P) -> Result<Vec<u8>> {
        let secret = PKey::hmac(self.secret_provider.secret())?;
        let mut signer = OpensslSigner::new(MessageDigest::sha256(), &secret)?;
        signer.update(parameter.wait_to_sign())?;
        Ok(signer.sign_to_vec()?)
    }
}

impl ParameterProvider<'_> for &str {
    fn wait_to_sign(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl ParameterProvider<'_> for String {
    fn wait_to_sign(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct MockProvider<'a> {
        secret: &'a [u8],
        parameter: &'a [u8],
    }

    impl<'a> MockProvider<'a> {
        fn new(secret: &'a [u8], parameter: &'a [u8]) -> Self {
            MockProvider { secret, parameter }
        }
    }

    impl<'a> SecretProvider<'a> for MockProvider<'a> {
        fn secret(&'a self) -> &'a [u8] {
            self.secret
        }
    }

    impl<'a> ParameterProvider<'a> for MockProvider<'a> {
        fn wait_to_sign(&'a self) -> &'a [u8] {
            self.parameter
        }
    }

    #[test]
    fn test_signature_algo() {
        let target =
            hex::decode("c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71")
                .unwrap();
        let mock = MockProvider::new(
            b"NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j", 
            b"symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559");

        let signer = Signer::new(mock.clone());
        let result = signer.sign(&mock);
        assert!(result.is_ok());
        assert!(openssl::memcmp::eq(&target, &result.unwrap()));
    }

    #[test]
    fn test_auto_impl() {
        let r#str = "symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";
        let string = String::from("symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559");
        let target = b"symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";
        assert!(openssl::memcmp::eq(r#str.wait_to_sign(), target));
        assert!(openssl::memcmp::eq(string.wait_to_sign(), target));
    }
}
