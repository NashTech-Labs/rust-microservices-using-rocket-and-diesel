use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample;
use crate::sample::MarketData;
use crate::sample::repository::{MovingAverage, RawMarketData};

#[get("/<trading_pair_id>")]
pub fn all(trading_pair_id: String,connection: DbConn) -> Result<Json<Vec<MovingAverage>>, Status> {
    sample::repository::get_moving_average(trading_pair_id,&connection)
        .map(|ma| Json(ma))
        .map_err(|error| error_status(error))
}

#[post("/", format ="application/json", data = "<marketdata>")]
pub fn post(marketdata: Json<RawMarketData>, connection: DbConn) ->  Result<status::Created<Json<MarketData>>, Status> {
    println!("here 0 {}",&marketdata.block_number);
    sample::repository::insert_caller(marketdata.into_inner(), &connection)
        .map(|marketdata| market_data_created(marketdata))
        .map_err(|error| error_status(error))

}

fn market_data_created(marketdata: MarketData) -> status::Created<Json<MarketData>> {
    println!("here final");
    status::Created(
        format!("{host}:{port}/marketdata/{id}", host = host(), port = port(), id = marketdata.block_number).to_string(),
        Some(Json(marketdata)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}