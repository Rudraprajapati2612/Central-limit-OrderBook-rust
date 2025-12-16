use std::sync::{Arc, Mutex};

use actix_web :: {App,  HttpServer};


use crate::{orderbook::OrderBook, routes::{create_order, delete_order, get_depth}};
pub mod input;
pub mod  routes;
pub mod output;
pub mod orderbook;
#[actix_web::main]
 async fn main()->std::io::Result<()>{
    let orderbook = Arc::new(Mutex::new(OrderBook::new()));

  
    HttpServer::new(move||{
        App::new()
        .app_data(orderbook.clone())
        .service(create_order)
        .service(delete_order)
        .service(get_depth)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}







