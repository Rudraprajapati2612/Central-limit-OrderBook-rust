use std::sync::{Arc, Mutex};

use actix_web :: { HttpResponse,  Responder, get,  post, web::{Data, Json}};

use crate::{input::{CreateOrderInput, DeleteOrder}, orderbook::{self, OrderBook}, output::{CreateOrderResponse, DeleteOrderResponse, Depth}};
#[post("/order")]
pub async fn create_order(body : Json<CreateOrderInput>,orderbook : Data<Arc<Mutex<OrderBook>>> )-> impl  Responder {
   let price = body.0.price;
   let quantity  = body.0.quantity;
   let user_id = body.0.user_id;
   let side = body.0.side; 

    let orderbook = orderbook.lock().unwrap();
    
    return HttpResponse::Ok().json(CreateOrderResponse{
        order_id : String::from("order")
    });
}

#[post("/deleteorder")]
pub async  fn delete_order(Json(body ): Json<DeleteOrder>, orderbook : Data<Arc<Mutex<OrderBook>>>)-> impl  Responder{
     let order_id = body.order_id;
     HttpResponse::Ok().json(DeleteOrderResponse{
        filled_qty : 3,
        avergae_price : 100
     })
}

#[get("/getdepth")]
pub async  fn get_depth(orderbook : Data<Arc<Mutex<OrderBook>>>) -> impl Responder {
    HttpResponse::Ok().json(Depth{
        bids : vec![],
        asks : vec![],
        lastupdatedid : String::from("UpdatedID_123")
    })
}

