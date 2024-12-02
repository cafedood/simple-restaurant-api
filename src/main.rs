use crate::api::restaurant_api::{add_items, get_item, get_all_items, get_table, remove_item};
use actix_request_identifier::RequestIdentifier;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use log::info;
use mysql::Pool;
use std::env;

mod common;
mod api;
mod model;
mod persist;

const ENV_EXPECT_LOG:&str = "ENV var is NOT valid!";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Initialize logger");
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(
        env::var("LOG_LEVEL").expect(&format!("LOG_LEVEL {}", ENV_EXPECT_LOG))
    ));

    info!("Config DB and connection pools");
    let db_host = env::var("MYSQL_HOST").expect(&format!("MYSQL_HOST {}", ENV_EXPECT_LOG));
    let db_port:u16 = env::var("MYSQL_PORT").unwrap().parse().expect(&format!("MYSQL_PORT {}", ENV_EXPECT_LOG));
    let db_name = env::var("MYSQL_DBNAME").expect(&format!("MYSQL_DBNAME {}", ENV_EXPECT_LOG));
    let db_user = env::var("MYSQL_USER").expect(&format!("MYSQL_USER {}", ENV_EXPECT_LOG));
    let db_passwd = env::var("MYSQL_PASSWORD").expect(&format!("MYSQL_PASSWORD {}", ENV_EXPECT_LOG));

    let builder = mysql::OptsBuilder::new()
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port)
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_passwd));

    let pool = Pool::new(builder).unwrap();
    let web_data = web::Data::new(pool);
  
    info!("Config server");
    let workers = env::var("WORKERS").unwrap().parse().expect(&format!("WORKERS {}", ENV_EXPECT_LOG));
    let host = env::var("HOST").expect(&format!("HOST {}", ENV_EXPECT_LOG));
    let port:u16 = env::var("PORT").unwrap().parse().expect(&format!("PORT {}", ENV_EXPECT_LOG));

    HttpServer::new(move || {
        App::new().app_data(web_data.clone())
            .service(add_items)
            .service(remove_item)
            .service(get_table)
            .service(get_all_items)
            .service(get_item)
            .wrap(Logger::default())
            .wrap(RequestIdentifier::with_uuid())
    }).bind((host, port))?
      .workers(workers)
      .run()
      .await
}