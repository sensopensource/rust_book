fn main() {
let config_max = Some(3u8);

if let Some(max) = config_max {
    println!("the maximumn is configured to be {max}");
}
/* impl UsState {
 *  fn existed_in(&self,year: u16) -> book {
 *  match self {
 *  UsState::Alabama => year >= 1819,
 *  UsState::Alaska => year >= 1959,
 *  }
 *  } */

   fn describe_state_quarter(coin: Coin) -> Option<String> {
       let Coin::Quarter(state) = coin else {
           return None;
       }

       if state.existed_in(1900) {
           Some(format!("{state:? is pretty olf. dor America!lol"))
       } else {
           Some(format!("{state:?} is new"))
       }
   }
 * 
 *
 * }
}
