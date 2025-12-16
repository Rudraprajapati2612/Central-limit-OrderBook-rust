use std::collections::HashMap;

use crate::input::Side;

pub struct OrderBook{
    pub bids : HashMap<u32 , Vec<UserOrder>>,
    pub asks : HashMap<u32 , Vec<UserOrder>>,
    pub order_id_index : u32
}

pub struct  UserOrder{
    pub user_id : u32,
    pub qty : u32,
    pub order_id : u32
}

impl OrderBook {
   pub fn new () -> Self{
        Self {
        bids: HashMap::new(),    
        asks: HashMap::new(),
        order_id_index : 0
        }
    }
}


impl  OrderBook {
    pub fn create_order( self , price : u32,quantity : u32, user_id : String , side :Side ) {
        
    }
}