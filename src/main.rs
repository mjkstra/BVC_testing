#![allow(non_snake_case)]

use std::cell::Ref;
use unitn_market_2022::good::consts::{DEFAULT_EUR_USD_EXCHANGE_RATE, DEFAULT_EUR_YEN_EXCHANGE_RATE, DEFAULT_EUR_YUAN_EXCHANGE_RATE};
use unitn_market_2022::good::good_kind::GoodKind;
use unitn_market_2022::{market::Market};
fn main() {
    let market = BVC::BVCMarket::new_random();

    let m = (*market).borrow();

    print_market(m);
}


fn print_market(m : Ref<dyn Market>){
    let good_info = m.get_goods();

    println!("Here is the good list of the market:\n");
    for good_label in good_info{
        let default_exchange_rate_buy;
        
        default_exchange_rate_buy = match good_label.good_kind{
            GoodKind::EUR => 1.0,
            GoodKind::USD => DEFAULT_EUR_USD_EXCHANGE_RATE,
            GoodKind::YEN => DEFAULT_EUR_YEN_EXCHANGE_RATE,
            GoodKind::YUAN => DEFAULT_EUR_YUAN_EXCHANGE_RATE,
        };

        println!("{}:\nQuantity: {}\nDefault exchange buy rate: {}\nExchange buy rate: {}\nExchange sell rate: {}\n-----", good_label.good_kind,
                                good_label.quantity,
                                default_exchange_rate_buy,
                                good_label.exchange_rate_buy,
                                good_label.exchange_rate_sell);
    }
} 
