-- Your SQL goes here
CREATE TABLE marketdata(
    id SERIAL PRIMARY KEY,
    block_number INT NOT NULL,
    trading_pair VARCHAR NOT NULL,
    market_price_high FLOAT NOT NULL,
    market_price_low FLOAT NOT NULL,
    trading_volume FLOAT NOT NULL,
    market_price_high_cs FLOAT NOT NULL,
    market_price_low_cs FLOAT NOT NULL,
    trading_volume_cs FLOAT NOT NULL,
    market_price_high_ma FLOAT NOT NULL,
    market_price_low_ma FLOAT NOT NULL,
    trading_volume_ma FLOAT NOT NULL
)