#[macro_use]
extern crate diesel;

pub mod db;

use diesel::{PgConnection, Connection};
use dotenv::dotenv;
use diesel::prelude::*;

use self::db::models::*;
use self::db::schema::users::dsl::*;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main(){
  let database_connection = &mut establish_connection();
  let result = users.load::<User>(database_connection);
  for user in result.unwrap() {
    println!("{} {} {} {} {} {} {} {}", user.u_id, user.u_nickname, user.u_email, user.u_password, user.u_avatar, user.u_is_online, user.u_creattime, user.u_lastlogin);
  }
}