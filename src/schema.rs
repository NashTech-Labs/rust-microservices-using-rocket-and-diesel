table! {
    marketdata (id) {
        id -> Int4,
        block_number -> Int4,
        trading_pair -> Varchar,
        market_price_high -> Float8,
        market_price_low -> Float8,
        trading_volume -> Float8,
        market_price_high_cs -> Float8,
        market_price_low_cs -> Float8,
        trading_volume_cs -> Float8,
        market_price_high_ma -> Float8,
        market_price_low_ma -> Float8,
        trading_volume_ma -> Float8,
    }
}
