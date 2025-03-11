mod calc;

fn main() {
    println!("add(1, 2)   = {}", calc::plus(1, 2));
    println!("minus(5, 3) = {}", calc::minus(5, 3));
    println!("mul(5, 2)   = {}", calc::mul(5, 2));
    println!("div(12, 2)  = {}", calc::div(12, 2).unwrap());

    let result = calc::div(12, 0);
    match result {
        Ok(n) => println!("div(12, 0)  = {}", n),
        Err(_) =>  println!("Div Failed"),
    }
}
