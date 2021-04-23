//!
//! The depth ticker GET response.
//!

use serde::Deserialize;

use rust_decimal::Decimal;

///
/// The `https://www.binance.com/api/v3/ticker/bookTicker` GET response.
///
pub type Response = Vec<DepthTicker>;

///
/// A type representing the best bid and ask in the order book.
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepthTicker {
    /// The name of the trading pair
    pub symbol: String,
    /// The best (highest) bid price in the order book
    pub bid_price: Decimal,
    /// The corresponding bid quantity
    pub bid_qty: Decimal,
    /// The best (lowest) ask price in the order book
    pub ask_price: Decimal,
    /// The corresponding ask quantity
    pub ask_qty: Decimal,
}
