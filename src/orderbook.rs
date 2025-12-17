use std::{cmp::{max, min_by, min_by_key}, collections::{BTreeMap, HashMap},  path::PrefixComponent};

use actix_web::http::header::Quality;

use crate::{input::{CreateOrderInput, Side}, orderbook};

pub struct OrderBook {
    asks: BTreeMap<u32, Vec<UserOrder>>,
    bids: BTreeMap<u32, Vec<UserOrder>>,
    order_id_index: u32,
}

pub struct UserOrder {
    pub order_id: u32,
    pub price: u32,
    pub side: Side,
    pub user_id: u32,
    pub original_qty: u32,
    pub remainig_qty: u32,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            order_id_index: 0,
        }
    }
}

impl OrderBook {
    pub fn create_order(&mut self, order: CreateOrderInput ) {
        let order_id = self.order_id_index;
        
        let order =  UserOrder{
            order_id : self.order_id_index,
            price : order.price,
            user_id : order.user_id,
            side : order.side,
            original_qty : order.quantity,
            remainig_qty:order.quantity
        };
        self.order_id_index = self.order_id_index + 1;
        match order.side {
            Side::Buy => self.match_buy(order),
            Side::Sell => self.match_sell(order)
        }
    }


    // pub fn match_buy(&mut self, mut buy_order: UserOrder) {

    //     while buy_order.remainig_qty > 0 {
    
    //         // 1. Get best ask price
    //         let best_sell_price = match self.asks.keys().next().cloned() {
    //             Some(p) => p,
    //             None => break, // no asks
    //         };
    
    //         // 2. Price check
    //         if best_sell_price > buy_order.price {
    //             break;
    //         }
    
    //         // 3. Take oldest sell order at this price
    //         let ask_queue = self.asks.get_mut(&best_sell_price).unwrap();
    //         let mut sell_order = ask_queue.remove(0);
    
    //         // 4. Execute trade
    //         let trade_qty = std::cmp::min(
    //             buy_order.remainig_qty,
    //             sell_order.remainig_qty,
    //         );
    
    //         buy_order.remainig_qty -= trade_qty;
    //         sell_order.remainig_qty -= trade_qty;
    
    //         println!(
    //             "TRADE: Buy {} Sell {} Qty {} @ {}",
    //             buy_order.order_id,
    //             sell_order.order_id,
    //             trade_qty,
    //             best_sell_price
    //         );
    
    //         // 5. Handle resting sell order
    //         if sell_order.remainig_qty > 0 {
    //             // partial fill → put back at same price (front = FIFO)
    //             ask_queue.insert(0, sell_order);
    //         }
    
    //         // 6. Remove price level if empty
    //         if ask_queue.is_empty() {
    //             self.asks.remove(&best_sell_price);
    //         }
    //     }
    
    //     // 7. Insert remaining BUY order ONCE
    //     if buy_order.remainig_qty > 0 {
    //         self.bids
    //             .entry(buy_order.price)
    //             .or_default()
    //             .push(buy_order);
    //     }
    // }
    

    pub fn match_buy ( &mut self, mut buy_order:UserOrder) {

        while buy_order.remainig_qty>0 {
        // get best sell price 
        let best_sell_price = match self.asks.keys().next().cloned(){
            Some(p)=>p,
            None => break, 
        };


        // now check for if sell price is to high then user current price then  order will not match 
            if best_sell_price > buy_order.price {
                break ;
            }
                        // type of ask order is 
                        // asks ={
                        //  100=>[orderA,orderB]
                        //  105=>[orderC] 
                        // }
 
            // so asks order is stored in this way and for it best sell order we get best price because list is in sorted order 
            // and from the we will remove OrderA because of time priority 
            let ask_queue = self.asks.get_mut(&best_sell_price).unwrap();
            let mut sell_order =   ask_queue.remove(0);
            let trade_qty = std::cmp::min(buy_order.remainig_qty, sell_order.remainig_qty);

            buy_order.remainig_qty-=trade_qty;
            sell_order.remainig_qty-=trade_qty;

            println!(
                "TRADE: Buy {} Sell {} Qty {} @ {}",
                buy_order.order_id,
                sell_order.order_id,
                trade_qty,
                best_sell_price
            );
             // after it sell order quantity is remaning  means 
            // let suppose user want to buy only 5 Solana 
            // and total solana for sell is 10 then remaning 5 sol will get in the orderbook again 
            if sell_order.remainig_qty>0{
                ask_queue.insert(0, sell_order);
            }

            if ask_queue.is_empty() {
                self.asks.remove(&best_sell_price);
            }
        }
        
        if buy_order.remainig_qty>0{
            self.bids.entry( buy_order.price).or_default().push(buy_order);
        }


          
    }  


    pub fn match_sell(&mut self, mut sell_order:UserOrder){
        while sell_order.remainig_qty>0 {
            let best_buy_price =   match  self.bids.keys().next_back().cloned(){
                Some(p)=>p,
                None => break
            };

            if best_buy_price < sell_order.price {
                break;
            }

            let bid_queue = self.bids.get_mut(&best_buy_price).unwrap();
            let mut  buy_order = bid_queue.remove(0);

            let trade_qty = std::cmp::min(sell_order.remainig_qty, buy_order.remainig_qty);


            sell_order.remainig_qty-=trade_qty;
            buy_order.remainig_qty-=trade_qty;

            if buy_order.remainig_qty>0{
                bid_queue.insert(0, buy_order);
            }

            if bid_queue.is_empty(){
                self.bids.remove(&best_buy_price);
            }
            
        }

       if sell_order.remainig_qty>0 {
        self.asks.entry(sell_order.price).or_default().push(sell_order);
       }
    }
}



