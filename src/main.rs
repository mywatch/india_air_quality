#[macro_use]
extern crate actix_web;

use actix_web::web::Json;
use actix_web::{App, HttpServer};
use futures::prelude::*;
use reqwest::r#async::Client;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct JsonTargetBucket {
    pub index: String,
    pub r#type: String,
    pub field: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonField {
    pub id: String,
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRecord {
    pub id: String,
    pub country: String,
    pub state: String,
    pub city: String,
    pub station: String,
    pub last_update: String,
    pub pollutant_id: String,
    pub pollutant_min: String,
    pub pollutant_max: String,
    pub pollutant_avg: String,
    pub pollutant_unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub index_name: String,
    pub title: String,
    pub desc: String,
    pub org_type: String,
    pub org: Vec<String>,
    pub sector: Vec<String>,
    pub source: String,
    pub catalog_uuid: String,
    pub visualizable: String,
    pub active: String,
    pub created: i64,
    pub updated: i64,
    pub created_date: String,
    pub updated_date: String,
    pub target_bucket: JsonTargetBucket,
    pub field: Vec<JsonField>,
    pub message: String,
    pub version: String,
    pub status: String,
    pub total: i64,
    pub count: i64,
    pub limit: String,
    pub offset: String,
    pub records: Vec<JsonRecord>,
}


#[get("/")]
fn index() -> Box<dyn Future<Item = Json<Response>, Error = ()>> {

    let cli = Client::builder().build().unwrap();
    Box::new(
        cli.get("https://api.data.gov.in/resource/3b01bcb8-0b14-4abf-b6f2-c1bfd384ba69?api-key=579b464db66ec23bdd000001cdd3946e44ce4aad7209ff7b23ac571b&format=json&offset=5&limit=10")
            .send()
            .map_err(|e| println!("Request error: {}", e))
            .and_then(|mut res| {
                res.json::<Response>()
                    .map(|r| Json(r))
                    .map_err(|e| println!("unpack error: {}", e))
            }),
    )
}

fn main() -> Result<(), ()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:4000")
        .unwrap()
        .run()
        .map_err(|e| println!("Error: {}", e))
}