#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Temperature, NewTemperature}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn record_temperature<'a>(conn: &PgConnection, temperature &'a f64, device &'a str) -> Temperature {
    use schema::temperature;

    let new_temperature = NewTemperature{
        temperature: temperature,
        device: device,
    };

    diesel::insert_into(temperature::table)
        .values(&new_temperature)
        .get_result(conn)
        .expect("Error recording temperature")
}