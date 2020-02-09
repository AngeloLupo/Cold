pub use super::schema::temperatures;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct Temperatures {
    pub id: i32,
    pub temperature: f64,
    pub device: String,
    pub timestamp: SystemTime,
}

#[derive(Insertable)]
#[table_name="temperatures"]
pub struct NewTemperature<'a>{
    pub temperature: &'a f64,
    pub device: &'a str
}