pub mod utils;
pub mod models;
use std::env;
pub use dotenvy::dotenv;

use diesel::{ Connection, PgConnection};
use utils::{ hash_password, verify_password };

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let data_base_url = env::var("DATABASE_URL").expect("no env file found");
    PgConnection::establish(&data_base_url).unwrap_or_else(|_| panic!("Nothing Found"))
}

async fn signup() {}

fn main() {
    establish_connection();
    let hash = hash_password("my_password").unwrap();
    println!("HASH = {}", hash);

    let valid = verify_password("my_password", &hash).unwrap();
    println!("VALID = {}", valid);
}
