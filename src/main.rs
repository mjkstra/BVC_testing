#![allow(non_snake_case)]

use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefMut, RefCell};
use std::rc::Rc;
use unitn_market_2022::good::consts::{DEFAULT_EUR_USD_EXCHANGE_RATE, DEFAULT_EUR_YEN_EXCHANGE_RATE, DEFAULT_EUR_YUAN_EXCHANGE_RATE};
use unitn_market_2022::good::{good::Good, good_kind::GoodKind};
use unitn_market_2022::market::{MarketGetterError, LockBuyError};
use unitn_market_2022::{market::Market};

const TRADER_NAME : &'static str = "TESTING";


fn main() {
    let market = BVC::BVCMarket::new_random();

    print_market((*market).borrow());

    let good_info = get_good_qty_and_price((*market).borrow(),0.75,GoodKind::YEN);
    /* let res = test_lock_buy( (*market).borrow_mut() ,good_info.0 , good_info.1, good_info.2);
    eprintln!("Testing lock_buy - result: {}", res);
    print_market((*market).borrow());
    eprintln!("Testing buy - result: {}", test_buy((*market).borrow_mut(), res, good_info.2));
    print_market((*market).borrow()); */

}

fn get_good_qty_and_price (m : Ref<dyn Market>, percentage_of_qty : f32, kind_to_buy : GoodKind) -> (GoodKind,f32,f32) {   
    let goods = m.get_goods();
    let mut good_qty : f32 = -1.0;
    for good in goods {
        if good.good_kind == kind_to_buy{
            good_qty = good.quantity*percentage_of_qty;
        }
    }
    let suggested_price = m.get_buy_price(kind_to_buy, good_qty).unwrap();
    (kind_to_buy,good_qty,suggested_price)
}

fn test_lock_buy(mut m: RefMut<dyn Market>, kind : GoodKind, qty : f32, price : f32) -> String{
    let res = m.lock_buy(kind, qty, price, String::from(TRADER_NAME));
    eprintln!("used price {}", price);
    match res{
        Ok(token) => token,
        Err(err) => String::from(format!("{:?}",err)),
    }
}

fn test_buy(mut m: RefMut<dyn Market>, token : String, price : f32 ) -> String{
    let res = m.buy(token, &mut Good::new(GoodKind::EUR,price));
    match res{
        Ok(good) => String::from("BuyOK"),
        Err(err) => String::from(format!("{:?}",err)),
    }
}

fn print_market(m : Ref<dyn Market>){
    let good_info = m.get_goods();

    eprintln!("Here is the good list of the market:\n");
    for good_label in good_info{
        let default_exchange_rate_buy;
        
        default_exchange_rate_buy = 1.0/match good_label.good_kind{
            GoodKind::EUR => 1.0,
            GoodKind::USD => DEFAULT_EUR_USD_EXCHANGE_RATE,
            GoodKind::YEN => DEFAULT_EUR_YEN_EXCHANGE_RATE,
            GoodKind::YUAN => DEFAULT_EUR_YUAN_EXCHANGE_RATE,
        };

        eprintln!("{}:\nQuantity: {}\nDefault exchange buy rate: {}\nExchange buy rate: {}\nExchange sell rate: {}\n-----", good_label.good_kind,
                                good_label.quantity,
                                default_exchange_rate_buy,
                                good_label.exchange_rate_buy,
                                good_label.exchange_rate_sell);
    }
} 
