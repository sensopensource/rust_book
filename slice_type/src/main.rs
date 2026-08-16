fn first_word(s: &String) -> usize {
let bytes = s.as_bytes();

for (i,&item) in bytes.iter().enumerate() {
     if item == b' ' {
         return i;
     }
}
s.len()
}


fn main() {
    let h = String::from("hello les gens");
    let hello = first_word(&h);
    println!("{hello}");
}
