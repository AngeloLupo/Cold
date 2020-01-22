#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;


use rocket_contrib::json::{Json, JsonValue};


#[derive(Serialize, Deserialize)]
struct Message {
    contents: String
}

#[put("/record", format = "json", data = "<message>")]
fn record(message: Json<Message>) -> Option<JsonValue> {
    print!("{:?}", message.0.contents);
    Some(json!({ "status": "ok" }))
}

fn main() {
    rocket::ignite().mount("/", routes![record]).launch();
}
