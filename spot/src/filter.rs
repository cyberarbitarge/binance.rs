use bigdecimal::BigDecimal;
use serde::Serializer;
use serde::{Deserialize, Serialize};

//TODO: the whole filter logic need refactor 

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Filter {
    PriceFilter {
        min_price: BigDecimal,
        max_price: BigDecimal,
        tick_size: BigDecimal,
    },
    PercentPrice {
        multiplier_up: BigDecimal,
        multiplier_down: BigDecimal,
        avg_price_mins: i32,
    },
    PercentPriceBySide {
        bid_multiplier_up: BigDecimal,
        bid_multiplier_down: BigDecimal,
        ask_multiplier_up: BigDecimal,
        ask_multiplier_down: BigDecimal,
        avg_price_mins: i32,
    },
    LotSize {
        min_qty: BigDecimal,
        max_qty: BigDecimal,
        step_size: BigDecimal,
    },
    MinNotional {
        min_notional: BigDecimal,
        apply_to_market: BigDecimal,
        avg_price_mins: i32,
    },
    IcebergParts {
        limit: i32,
    },
    MarketLotSize {
        min_qty: BigDecimal,
        max_qty: BigDecimal,
        step_size: BigDecimal,
    },
    MaxNumOrders {
        max_num_orders: i32,
    },
    MaxNumAlgoOrders {
        max_num_algo_orders: i32,
    },
    MaxNumIcebergOrders {
        max_num_algo_orders: i32,
    },
    MaxPosition {
        max_position: BigDecimal,
    },
    ExchangeMaxNumOrders {
        max_num_orders: i32,
    },
    ExchangeMaxAlgoOrders {
        max_num_algo_orders: i32,
    },
}

// FIXME: maybe simple impl for this method like dynamical create a json map and then insert ?
impl Serialize for Filter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        match self {
            Filter::PriceFilter {
                min_price,
                max_price,
                tick_size,
            } => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("filterType", "PRICE_FILTER")?;
                map.serialize_entry("minPrice", &min_price)?;
                map.serialize_entry("maxPrice", &max_price)?;
                map.serialize_entry("tickSize", &tick_size)?;
                map.end()
            }
            Filter::PercentPrice {
                multiplier_up,
                multiplier_down,
                avg_price_mins,
            } => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("filterType", "PERCENT_PRICE")?;
                map.serialize_entry("multiplierUp", &multiplier_up)?;
                map.serialize_entry("multiplierDown", &multiplier_down)?;
                map.serialize_entry("avgPriceMins", &avg_price_mins)?;
                map.end()
            }
            Filter::PercentPriceBySide {
                bid_multiplier_up,
                bid_multiplier_down,
                ask_multiplier_up,
                ask_multiplier_down,
                avg_price_mins,
            } => {
                let mut map = serializer.serialize_map(Some(6))?;
                map.serialize_entry("filterType", "PERCENT_PRICE_BY_SIDE")?;
                map.serialize_entry("bidMultiplierUp", &bid_multiplier_up)?;
                map.serialize_entry("bidMultiplierDown", &bid_multiplier_down)?;
                map.serialize_entry("askMultiplierUp", &ask_multiplier_up)?;
                map.serialize_entry("askMultiplierDown", &ask_multiplier_down)?;
                map.serialize_entry("avgPriceMins", &avg_price_mins)?;
                map.end()
            }
            Filter::LotSize {
                min_qty,
                max_qty,
                step_size,
            } => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("filterType", "LOT_SIZE")?;
                map.serialize_entry("minQty", &min_qty)?;
                map.serialize_entry("maxQty", &max_qty)?;
                map.serialize_entry("stepSize", &step_size)?;
                map.end()
            },
            Filter::MinNotional{ min_notional, apply_to_market, avg_price_mins } => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("minNotional", &min_notional)?;
                map.serialize_entry("applyToMarket", &apply_to_market)?;
                map.serialize_entry("avgPriceMins", &avg_price_mins)?;
                map.end()
            },
            Filter::IcebergParts{ limit } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("filterType", "ICEBERG_PARTS")?;
                map.serialize_entry("limit", &limit)?;
                map.end()
            },
            Filter::MarketLotSize{ min_qty, max_qty, step_size } => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("filterType", "MARKET_LOT_SIZE")?;
                map.serialize_entry("minQty", &min_qty)?;
                map.serialize_entry("maxQty", &max_qty)?;
                map.serialize_entry("stepSize", &step_size)?;
                map.end()
            },
            
            _ => serializer.serialize_map(None)?.end(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use core::str::FromStr;
    use serde_json;

    #[test]
    fn test_self_defined_serialize() {
        let filter = Filter::PriceFilter {
            min_price: BigDecimal::from_str("0.2245").unwrap(),
            max_price: BigDecimal::from_str("0.2333").unwrap(),
            tick_size: BigDecimal::from_str("1").unwrap(),
        };
        println!("{}", serde_json::to_string_pretty(&filter).unwrap());
    }


    //TODO: maybe we don't need the deserialize from response ?
    #[test]
    fn test_ser_de() {
        let filter = Filter::PriceFilter {
            min_price: BigDecimal::from_str("0.23333").unwrap(),
            max_price: BigDecimal::from_str("0.33333").unwrap(),
            tick_size: BigDecimal::from_str("0.1").unwrap(),
        };
        let serialized = serde_json::to_string(&filter).unwrap();
        let deserialized: Filter = serde_json::from_str(&serialized).unwrap();
        if let Filter::PriceFilter{ max_price, .. } = deserialized {
           assert_eq!(max_price, BigDecimal::from_str("0.23333").unwrap());
        } else {
            panic!("fail to convert from str")
        }
    }
}
