use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Asset {
    Uknown(String),
    Btc,
    Ada,
    Eth,
    Dot,
    Bnb,
    Xrp,
    Usdt,
    Ltc,
}
