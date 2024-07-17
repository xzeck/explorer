mod api;
mod models;
mod service;
mod dto;

use std::env;
use actix_cors::Cors;
use actix_web::{http, middleware::{self, Logger}, App, HttpServer, web};
use api::code_runner_controller;
use reqwest::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    if cfg!(debug_assertions) {
        std::env::set_var("RUST_LOG", "debug");
   }
   else {
    std::env::set_var("RUST_LOG", "info")
   }


    env_logger::init();

    let host_address = env::var("HOST_ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port_str = env::var("PORT").unwrap_or("6000".to_string());

    let port = port_str.parse().unwrap_or(6000);



    HttpServer::new(move || {
        let logger = Logger::default();
        let cors = Cors::default()
              .allow_any_origin()
              .allowed_methods(vec!["GET", "POST", "OPTIONS"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
        
        let client = web::Data::new(Client::new());


        App::new()
        .wrap(middleware::NormalizePath::trim())
        .wrap(logger)
        .wrap(cors)
        .app_data(client)
        .service(code_runner_controller::get_scope())
    })
    .bind((host_address, port))?
    .run()
    .await
}