mod basket;

use basket::Basket;

fn main() {
    let basket_1 = Basket::new(String::from("apple"));
    let basket_2 = Basket::new(1);
    let basket_3 = Basket::new(1.0);
}
