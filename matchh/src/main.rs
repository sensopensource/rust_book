fn main() { /*
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
     Coin::Penny => 1,
     Coin::Nickel => 5,
     Coin::Dime => 10,
     Coin::Quarter => 2    }
} */ 
fn plus_one(option: Option<i32>) -> Option<i32> {
    match option {
        None => None,
        Some(i) => Some(i+ 1), 
    }
}
    let a = Some(5);
    let b = plus_one(a);
    let none = plus_one(None);

    println!("{:#?}",a);
    println!("{:#?}",b);
    println!("{:#?}",none);
}
