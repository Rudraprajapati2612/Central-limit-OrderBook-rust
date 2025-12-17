use std::vec;

use serde::{ Deserialize , Serialize} ;

#[derive(Deserialize,Serialize)]
pub struct CreateOrderOutput{
    pub user_id : String
}
#[derive(Deserialize,Serialize)]
pub struct  Depth{
    pub bids : Vec<[u32;2]>,
    pub asks : Vec<[u32;2]>,
    pub lastUpdatedId : String

}