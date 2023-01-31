#![allow(non_snake_case)]

use std::cell::{Ref, RefMut};
use unitn_market_2022::good::consts::{
    DEFAULT_EUR_USD_EXCHANGE_RATE, DEFAULT_EUR_YEN_EXCHANGE_RATE, DEFAULT_EUR_YUAN_EXCHANGE_RATE,
};
use unitn_market_2022::good::{good::Good, good_kind::GoodKind};
use unitn_market_2022::market::Market;

const TRADER_NAME: &'static str = "TESTING";

fn main() {
    let mut market = BVC::BVCMarket::new_random();

    print_market((*market).borrow());

    // * buy and lock_buy
    let good_info = get_good_qty_and_buy_price((*market).borrow(), 0.74, GoodKind::YEN);
    let res = test_lock_buy(
        (*market).borrow_mut(),
        good_info.0,
        good_info.1,
        good_info.2,
    );
    eprintln!("Testing lock_buy - result: {}", res);
    print_market((*market).borrow());
    eprintln!(
        "Testing buy - result: {}",
        test_buy((*market).borrow_mut(), res, good_info.2)
    );
    print_market((*market).borrow());

    // * sell and lock sell
    market = BVC::BVCMarket::new_random();
    let good_info = get_good_qty_and_sell_price((*market).borrow(), 0.79, GoodKind::YEN);
    let res = test_lock_sell(
        (*market).borrow_mut(),
        good_info.0,
        good_info.1,
        good_info.2,
    );
    eprintln!("Testing lock_sell - result: {}", res);
    print_market((*market).borrow());
    eprintln!(
        "Testing sell - result: {}",
        test_sell((*market).borrow_mut(), res, good_info.0, good_info.1)
    );
    print_market((*market).borrow());
}

fn get_good_qty_and_sell_price(
    m: Ref<dyn Market>,
    percentage_of_qty: f32,
    kind_to_sell: GoodKind,
) -> (GoodKind, f32, f32) {
    let goods = m.get_goods();
    let mut good_qty: f32 = -1.0;
    for good in goods {
        if good.good_kind == GoodKind::EUR {
            good_qty = good.quantity
                * percentage_of_qty
                * match kind_to_sell {
                    GoodKind::EUR => 1.0,
                    GoodKind::USD => DEFAULT_EUR_USD_EXCHANGE_RATE,
                    GoodKind::YEN => {
                        eprintln!("\n\nEXCHANGE RATE: {}\n\n", DEFAULT_EUR_YEN_EXCHANGE_RATE);
                        DEFAULT_EUR_YEN_EXCHANGE_RATE
                    }
                    GoodKind::YUAN => DEFAULT_EUR_YUAN_EXCHANGE_RATE,
                };
        }
    }
    let suggested_price = m.get_sell_price(kind_to_sell, good_qty).unwrap();
    (kind_to_sell, good_qty, suggested_price)
}

fn test_lock_sell(mut m: RefMut<dyn Market>, kind: GoodKind, qty: f32, price: f32) -> String {
    let res = m.lock_sell(kind, qty, price, String::from(TRADER_NAME));
    //eprintln!("used sell price {}", price);
    match res {
        Ok(token) => token,
        Err(err) => String::from(format!("{:?}", err)),
    }
}

fn test_sell(mut m: RefMut<dyn Market>, token: String, kind_to_sell: GoodKind, qty: f32) -> String {
    let res = m.sell(token, &mut Good::new(kind_to_sell, qty));
    match res {
        Ok(good) => String::from(format!("BuyOK with {} eur", good.get_qty())),
        Err(err) => String::from(format!("{:?}", err)),
    }
}

fn get_good_qty_and_buy_price(
    m: Ref<dyn Market>,
    percentage_of_qty: f32,
    kind_to_buy: GoodKind,
) -> (GoodKind, f32, f32) {
    let goods = m.get_goods();
    let mut good_qty: f32 = -1.0;
    for good in goods {
        if good.good_kind == kind_to_buy {
            good_qty = good.quantity * percentage_of_qty;
        }
    }
    let suggested_price = m.get_buy_price(kind_to_buy, good_qty).unwrap();
    (kind_to_buy, good_qty, suggested_price)
}

fn test_lock_buy(mut m: RefMut<dyn Market>, kind: GoodKind, qty: f32, price: f32) -> String {
    let res = m.lock_buy(kind, qty, price, String::from(TRADER_NAME));
    //eprintln!("used price {}", price);
    match res {
        Ok(token) => token,
        Err(err) => String::from(format!("{:?}", err)),
    }
}

fn test_buy(mut m: RefMut<dyn Market>, token: String, price: f32) -> String {
    let res = m.buy(token, &mut Good::new(GoodKind::EUR, price));
    match res {
        Ok(good) => String::from("BuyOK"),
        Err(err) => String::from(format!("{:?}", err)),
    }
}

fn print_market(m: Ref<dyn Market>) {
    let good_info = m.get_goods();

    eprintln!("Here is the good list of the market:\n");
    for good_label in good_info {
        let default_exchange_rate_buy;

        default_exchange_rate_buy = 1.0
            / match good_label.good_kind {
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
