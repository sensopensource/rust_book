fn main() {
    const THREE_HOURS: u32 = 3 * 60 * 60;
    println!("{THREE_HOURS}");
    let x = 5;
    let x = x + 1;
    
    {
        let x = x * 2;
        println!("The calue of x in the inner scope is: {x}");
    } 
    
    println!("the value of x is {x}")
}
