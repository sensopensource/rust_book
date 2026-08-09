fn main() {
  //  let number = 3;
  //
  //  if number != 0 {
  //      println!("number doesnt equal 0");
  //  } 
  // let condition = true;
  //let number = if condition {5} else {6};

  //println!("{number}");
  

   // loop {
   //     println!("again");
   // }
   let mut counter = 0;

   let result = loop {
       counter+=1;

       if counter == 10 {
           break counter * 2;
       }
   } ;

   println!("the result is {result}");
}
