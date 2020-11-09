#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use diesel::sql_query;
use rocket_contrib::json::Json;

use crate::sample::MarketData;
use crate::schema::marketdata;
use crate::schema::marketdata::columns::{block_number, id};
use crate::schema::marketdata::dsl::*;

pub fn insert_to_db(marketdata1: InsertableMarketData, connection: &PgConnection) -> QueryResult<MarketData> {
    diesel::insert_into(marketdata::table)
        .values(&marketdata1)
        .get_result(connection)
}

pub fn insert_caller(raw_data: RawMarketData, connection: &PgConnection) -> QueryResult<MarketData> {
    let market_data_list: Vec<MarketData> = marketdata.filter(trading_pair.eq(&raw_data.trading_pair)).order(id.desc()).limit(1).load(&*connection)?;
    let marketdata1: InsertableMarketData = if market_data_list.is_empty() { InsertableMarketData::from_first(raw_data) } else { InsertableMarketData::from_latest(raw_data, market_data_list.first().unwrap()) };
    insert_to_db(marketdata1, connection)
}

pub fn get_moving_average(trading_pair_id: String, connection: &PgConnection) -> QueryResult<Vec<MovingAverage>> {
    marketdata.select((trading_pair, market_price_high_ma, market_price_low_ma, trading_volume_ma)).filter(trading_pair.eq(&trading_pair_id)).load::<MovingAverage>(&*connection)
}

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct MovingAverage {
    trading_pair: String,
    market_price_high_ma: f64,
    market_price_low_ma: f64,
    trading_volume_ma: f64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RawMarketData {
    pub block_number: i32,
    pub trading_pair: String,
    pub market_price_high: f64,
    pub market_price_low: f64,
    pub trading_volume: f64,
}


#[table_name = "marketdata"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct InsertableMarketData {
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

impl InsertableMarketData {
    fn from_first(raw_data: RawMarketData) -> Self {
        InsertableMarketData {
            block_number: raw_data.block_number,
            trading_pair: raw_data.trading_pair,
            market_price_high: raw_data.market_price_high,
            market_price_low: raw_data.market_price_low,
            trading_volume: raw_data.trading_volume,
            market_price_high_cs: raw_data.market_price_high,
            market_price_low_cs: raw_data.market_price_low,
            trading_volume_cs: raw_data.trading_volume,
            market_price_high_ma: raw_data.market_price_high,
            market_price_low_ma: raw_data.market_price_low,
            trading_volume_ma: raw_data.trading_volume,
        }
    }

    fn from_latest(raw_data: RawMarketData, previous_data: &MarketData) -> Self {
        InsertableMarketData {
            block_number: raw_data.block_number,
            trading_pair: raw_data.trading_pair,
            market_price_high: raw_data.market_price_high,
            market_price_low: raw_data.market_price_low,
            trading_volume: raw_data.trading_volume,
            market_price_high_cs: raw_data.market_price_high + previous_data.market_price_high_cs,
            market_price_low_cs: raw_data.market_price_low + previous_data.market_price_low_cs,
            trading_volume_cs: raw_data.trading_volume + previous_data.trading_volume_cs,
            market_price_high_ma: (raw_data.market_price_high + previous_data.market_price_high_cs) / raw_data.block_number as f64,
            market_price_low_ma: (raw_data.market_price_low + previous_data.market_price_low_cs) / raw_data.block_number as f64,
            trading_volume_ma: (raw_data.trading_volume + previous_data.trading_volume_cs) / raw_data.block_number as f64,
        }
    }
}








