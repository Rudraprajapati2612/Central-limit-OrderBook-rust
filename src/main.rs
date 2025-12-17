use std::sync::{Arc, Mutex};

// use::actix_web
use::actix_web::{App , HttpServer,web};

use crate::{orderbook::OrderBook, routes::{create_order, delete_order, depth}};
pub mod orderbook;
pub mod output;
pub mod routes;
pub mod input;


#[actix_web::main]

async  fn main () -> std::io::Result<()> {

    let orderbook = Arc::new(Mutex::new(OrderBook::new()));

  HttpServer::new(move || {
    App::new()
    .app_data(web::Data::new(orderbook.clone()))
    .service(create_order)
    .service(delete_order)
    .service(depth)
  })
  .bind(("127.0.0.1",8080))?
  .run()
  .await
}