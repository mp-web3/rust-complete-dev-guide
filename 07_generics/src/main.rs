use num_traits::ToPrimitive;
use num_traits::Float;
fn solve<T: Float, U: Float>(a: T, b: U) -> f64 {

    let a_f64: f64 = a.to_f64().unwrap();
    let b_f64: f64 = b.to_f64().unwrap();   

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn solve_any_num<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {

    let a_f64: f64 = a.to_f64().unwrap();
    let b_f64: f64 = b.to_f64().unwrap();   

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    let a: i32 = 3;
    let b: f64 = 4.0;

    let result = solve_any_num(a, b);
    println!("{}", result);

}
