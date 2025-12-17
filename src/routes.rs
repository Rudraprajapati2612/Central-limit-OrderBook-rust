use std::{sync::{Arc, Mutex}, vec};

use actix_web::{HttpResponse, Responder,  get, post, web::{Data, Json}};

use crate::{input::{CreateOrderInput, DeleteOrder}, orderbook::{ OrderBook}, output::{CreateOrderOutput, Depth}};



#[post("/order")]
pub async  fn create_order( orderbook : Data<Arc<Mutex<OrderBook>>> , input: Json<CreateOrderInput>)-> impl Responder{
    
    // let price = body.0.price;
    // let user_id = body.0.user_id;
    // let qty = body.0.quantity;
    // let side = body.0.side;
    let mut orderbook =  orderbook.lock().unwrap();
    // orderbook.create_order(CreateOrderInput);
    let input  = input.into_inner();
    orderbook.create_order(&input);


    return  HttpResponse::Ok().json(CreateOrderOutput{
        user_id : input.user_id.to_string()
    })
}

#[post("/deleteorder")]

pub async fn delete_order() -> impl Responder{
    HttpResponse::Ok().json(DeleteOrder{
        order_id : String::from("Delete_Id 1")
    })
}
#[get("/depth")]
pub  async  fn depth() -> impl Responder{
    HttpResponse::Ok().json(Depth{
        bids : vec![],
        asks : vec![],
        lastupdatedid : String::from("lastUpdated123")
    })
}