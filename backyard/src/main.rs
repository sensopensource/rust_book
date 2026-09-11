use crate::garden::vegetables::Carrot ;

pub mod garden;

fn main() {
    let plant = Carrot {};
    println!("im growing a {plant:?}")
}
