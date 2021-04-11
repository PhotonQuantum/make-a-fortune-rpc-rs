use std::time::Duration;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub wkfg_addr: String,
    pub timeout: Timeout,
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Timeout {
    #[serde(with = "humantime_serde")]
    pub connect: Duration,
    #[serde(with = "humantime_serde")]
    pub read: Duration,
    #[serde(with = "humantime_serde")]
    pub write: Duration,
}
