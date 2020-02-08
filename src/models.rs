pub use super::schema::temperatures;

#[derive(Insertable)]
#[table_name="temperatures"]
pub struct NewTemperature<'a>{
    pub temperature: &'a f64,
    pub device: &'a str
}