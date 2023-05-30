pub mod models;
pub mod schema;

use diesel::{PgConnection, Connection};
use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main(){
  println!("Hello, world!");
}