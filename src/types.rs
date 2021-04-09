use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub wkfg_addr: String,
    pub max_tcp_conn: u64,
}
