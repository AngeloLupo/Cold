#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::NewTemperature;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn record_temperature<'a>(conn: &PgConnection, temperature: &'a f64, device: &'a str) {
    use schema::temperatures;

    let new_temperature = NewTemperature{
        temperature: temperature,
        device: device,
    };

    diesel::insert_into(temperatures::table)
        .values(&new_temperature)
        .get_result(conn)
        .expect("Error recording temperature")
}


#[derive(Serialize, Deserialize)]
struct TemperatureReading {
    temperature: f64,
    device: String,
}

#[put("/record", format = "json", data = "<temperatureReading>")]
fn record(temperatureReading: Json<TemperatureReading>) -> Option<JsonValue> {

    //print!("{:?}", message.0.temperature);
    //print!("{:?}", message.0.device);
    let connection = establish_connection();
    record_temperature(&connection, &temperatureReading.0.temperature, &temperatureReading.0.device);

    Some(json!({ "status": "ok" }))
}

fn main() {
    rocket::ignite().mount("/", routes![record]).launch();
}
