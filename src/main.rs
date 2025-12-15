use actix_web :: {App, HttpResponse, HttpServer, Responder, get, http::header::Quality, post, web::Json};
use serde::{Deserialize, Serialize};

#[actix_web::main]
 async fn main()->std::io::Result<()>{
    HttpServer::new(||{
        App::new().service(create_order).service(delete_order).service(get_depth)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Deserialize,Debug)]
struct  CreateOrderInput{
    price : u32,
    quantity : u32,
    user_id: u32,
    side : Side
}

#[derive(Serialize,Deserialize,Debug)]
struct  CreateOrderResponse {
    order_id : String
}

#[derive( Deserialize, Debug)]
enum  Side {
    Buy,
    Sell
}


#[post("/order")]
async fn create_order(body : Json<CreateOrderInput>)-> impl  Responder {
   let price = body.0.price;
   let quantity  = body.0.quantity;
   let user_id = body.0.user_id;
   let side = body.0.side; 


    return HttpResponse::Ok().json(CreateOrderResponse{
        order_id : String::from("order")
    });
}

#[post("/deleteorder")]
async  fn delete_order()-> impl  Responder{
     "delete order end point"
}

#[get("/getdepth")]
async  fn get_depth() -> impl Responder {
    "Get Depth Endpoint"
}


