pub mod utils;
use utils::{hash_password, verify_password };

fn main() {
    let hash = hash_password("my_password").unwrap();
    println!("HASH = {}", hash);

    let valid = verify_password("my_password", &hash).unwrap();
    println!("VALID = {}", valid);
}
