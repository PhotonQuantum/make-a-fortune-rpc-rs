#![feature(proc_macro_hygiene, decl_macro)]

use rocket::data::Capped;
use rocket::fairing::AdHoc;
use rocket::State;
use rocket_contrib::json::{json, JsonValue};
use rocket_cors::AllowedOrigins;

use crate::config::Config;
use crate::tcp::TcpConnection;

mod config;
mod tcp;

#[rocket::get("/version")]
async fn version(config: State<'_, Config>) -> JsonValue {
    json!({
        "name": "「无可奉告」Android 版",
        "version": format!("Rust, tokio, v{}", env!("CARGO_PKG_VERSION")),
        "addr": format!("tcp://{}", config.wkfg_addr),
        "terms_of_service": "http://wukefenggao.cn/code",
        "rpc_source_code": "https://github.com/PhotonQuantum/make-a-fortune-rpc-rs",
        "rpc_terms_of_service": "https://github.com/PhotonQuantum/make-a-fortune-rpc-rs/blob/master/LICENSE.md"
    })
}

#[rocket::post("/rpc_proxy", format = "json", data = "<input>")]
async fn rpc_proxy(config: State<'_, Config>, input: Capped<&[u8]>) -> String {
    let conn = TcpConnection::connect(&config.wkfg_addr).await;
    match conn {
        Err(err) => json!({"status": "rpc error", "msg": err.to_string()}).to_string(),
        Ok(mut conn) => {
            let resp = conn.request(*input).await;
            match resp {
                Err(err) => json!({"status": "rpc error", "msg": err.to_string()}).to_string(),
                Ok(resp) => resp,
            }
        }
    }
}

#[rocket::launch]
async fn rocket() -> rocket::Rocket {
    let allowed_origins = AllowedOrigins::all();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::ignite()
        .mount("/api", rocket::routes![version, rpc_proxy])
        .attach(AdHoc::config::<Config>())
        .attach(cors)
}
