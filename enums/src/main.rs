struct v8 {
    field: u32,
    field2: u16
}
enum ipAddr {
    v4(u8,u8,u8,u8),
    v6(String),
    v8_(v8),
}

fn main() { 
let home = ipAddr::v4(127,0,0,1);
let loopback = ipAddr::v6(String::from("::1"));
}



