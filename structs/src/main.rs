fn main() {

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("senso"),
        email : String::from("emai@gmail.com"),
        sign_in_count: 42,
    };

     user1.email = String::from("senso@senso.com");
     user1.username = String::from("senso@senso.com");
     let username2 = String::from("ayoub");
     let email2 = String::from("email.com");
     let user2 = build_user(username2,email2);
     println!("{}",user2.email);


fn build_user(username: String,email: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
  
        }
   }

let user3 = User {
    email: String::from("another@example.com"),
    ..user2 
};

println!("{}",user3.username);
}
