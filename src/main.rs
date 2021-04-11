#![feature(proc_macro_hygiene, decl_macro)]

use mobc::Pool;
use rocket::data::Capped;
use rocket::fairing::AdHoc;
use rocket::State;
use rocket_contrib::json::{json, JsonValue};
use rocket_cors::AllowedOrigins;

use crate::tcp::TcpManager;
use crate::types::Config;

mod tcp;
mod types;

#[rocket::get("/version")]
async fn version(config: State<'_, Config>) -> JsonValue {
    json!({
        "name": "「无可奉告」Android 版",
        "version": "Rust, tokio",
        "addr": format!("tcp://{}", config.wkfg_addr),
        "terms_of_service": "http://wukefenggao.cn/code",
        "rpc_source_code": "https://github.com/PhotonQuantum/make-a-fortune-rpc-rs",
        "rpc_terms_of_service": "https://github.com/PhotonQuantum/make-a-fortune-rpc-rs/blob/master/LICENSE.md"
    })
}

#[rocket::post("/rpc_proxy", format = "json", data = "<input>")]
async fn rpc_proxy(pool: State<'_, Pool<TcpManager>>, input: Capped<&[u8]>) -> JsonValue {
    let conn = pool.get().await;
    match conn {
        Err(err) => json!({"status": "rpc error", "msg": err.to_string()}),
        Ok(mut conn) => {
            let resp = conn.request(*input).await;
            match resp {
                Err(err) => json!({"status": "rpc error", "msg": err.to_string()}),
                Ok(resp) => JsonValue::from(resp),
            }
        }
    }
}

#[rocket::launch]
async fn rocket() -> rocket::Rocket {
    let rocket = rocket::ignite();
    let figment = rocket.figment();
    let config: Config = figment.extract().expect("config");
    let pool = Pool::builder()
        .max_open(config.max_tcp_conn)
        .build(TcpManager::new(config.wkfg_addr));

    let allowed_origins = AllowedOrigins::all();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket
        .mount("/api", rocket::routes![version, rpc_proxy])
        .attach(AdHoc::config::<Config>())
        .attach(cors)
        .manage(pool)
}
