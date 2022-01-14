//! binance websocket impl
//!
//! conver binance websocket protocol from Java

pub enum Payload {
    AggTrade {
        e: String,
        E: u32,
        s: String,
        a: u32,
        p: String,
        q: String,
        f: u32,
        l: u32,
        T: u32,
        m: bool,
        M: bool,
    },
    Trade {
        e: String,
        E: u32,
        s: String,
        t: u32,
        p: String,
        q: String,
        b: u32,
        a: u32,
        T: u32,
        m: bool,
        M: bool,
    },
    Kline {
        e: String,
        E: u32,
        s: String,
        kt: u32,
        kT: u32,
        ks: String,
        ki: String,
        kf: u32,
        kL: u32,
        ko: String,
        kc: String,
        kh: String,
        kl: String,
        kv: String,
        kn: u32,
        kx: bool,
        kq: String,
        kV: String,
        kQ: String,
        kB: String,
    },
}

#[cfg(test)]
mod tests {}
