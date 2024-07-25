mod models;
use models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
mod schema;
use crate::models::users::{NewUser, Users};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    use schema::users::dsl::*;
    let connection = &mut establish_connection();

    let new_example = NewUser {
        name: Some("Zee".to_string()), // Wrapped in Some
        email: Some("Zee@zee.com".to_string()),     // Wrapped in Some
        sessiontoken: Some(1),                      // Wrapped in Some
    };

    let database_record = diesel::insert_into(schema::users::table)
        .values(new_example)
        .get_result::<Users>(connection)
        .expect("Error saving new user");

    println!("Inserted example: {:?}", database_record);
}
