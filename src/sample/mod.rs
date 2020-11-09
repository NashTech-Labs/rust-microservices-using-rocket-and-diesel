#![allow(proc_macro_derive_resolution_fallback)]

use super::schema::marketdata;

pub mod handler;
pub mod router;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, QueryableByName, Debug)]
#[table_name = "marketdata"]
pub struct MarketData {
    id: i32,
    block_number: i32,
    trading_pair: String,
    market_price_high: f64,
    market_price_low: f64,
    trading_volume: f64,
    market_price_high_cs: f64,
    market_price_low_cs: f64,
    trading_volume_cs: f64,
    market_price_high_ma: f64,
    market_price_low_ma: f64,
    trading_volume_ma: f64,
}