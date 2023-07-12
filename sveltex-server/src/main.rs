use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{routes, State, options, Request, Response, catch};
use rocket::{http::Status, get,post,launch};
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use hex;
use rusqlite::{Connection, Result};
use std::collections::HashMap;
use std::io::Error;
use std::path::Path;
use std::sync::Mutex;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::request_handler::load_request_handlers;
#[path = "request_handlers/request_handler.rs"] mod request_handler;

pub struct RequestHandler{
    handler:HashMap<&'static str,fn(&State<Pool<SqliteConnectionManager>>,&TransmitData) -> Result<String,Error>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuery {
    request_type: String,
    request_query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransmitData {
    query:RequestQuery,
    credentials:Credentials
}

async fn validate_credentials(passed_username:&String,passed_password:&String,username:&String,password:&String)->bool {
    if passed_password==password && passed_username==username {
        return true;
    } 
    return false;
}

#[post("/set-data", format = "json", data = "<request>")]
async fn set_data(credentials: &State<Credentials>,
    connection:&State<Pool<SqliteConnectionManager>>,
    secret_handler:&State<RequestHandler>,
    request:Json<TransmitData>) -> Json<String>  {
        //Validating credentials
        let valid_credentials=validate_credentials(&request.credentials.username, &request.credentials.password, &credentials.username, &credentials.password).await;
        if !valid_credentials {
            return Json("bad request".to_string())
        }
        let request_type = &request.query.request_type;
        let assumed_handler = secret_handler.handler.get(request_type.as_str());
        match assumed_handler {
            Some(handler_function) => {
                handler_function(connection,&request);
            },
            None => {
                return Json("bad request".to_string())
            }
        }

        return Json("ok".to_string())
    }


#[post("/get-data", format = "json", data = "<request>")]
async fn get_data(connection:&State<Pool<SqliteConnectionManager>>,
    handler:&State<RequestHandler>, 
    request: Json<TransmitData>) -> Json<String> {
        let request_type = &request.query.request_type;
        let assumed_handler = handler.handler.get(request_type.as_str());
        match assumed_handler {
            Some(handler_function) => {
                let data=handler_function(connection,&request).ok();
                match data {
                    Some(data)=>{
                        return Json(data)
                    }, None=>{
                        return Json("bad request".to_string())
                    }
                }
            },
            None => {
                return Json("bad request".to_string())
            }
        }

}


use rocket::response::content;

#[catch(422)]
fn unprocessable_entity(_req: &Request<'_>) -> Status {
    Status::BadRequest
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() -> Status{
    /* Intentionally left empty */
    Status::Ok

}

#[get("/<_..>")]
fn all_get() ->  &'static str {
    "GET Requests are not allowed on this resource."
}



#[launch]
fn rocket() -> _ {
    let manager = SqliteConnectionManager::file("sveltex.db");
    println!("Connected to database");
    let pool = Pool::new(manager).expect("Failed to create connection pool");
    println!("Created SQLite Connection Pool");
    let data = std::fs::read_to_string("credentials.json").expect("Unable to read file");
    println!("Loaded credentials");
    let creds: Credentials = serde_json::from_str(&data).unwrap();
    println!("Parsed credentials!");
    let request_handlers:RequestHandler=RequestHandler {handler:load_request_handlers()};
    rocket::build()
    .attach(Cors)
    .mount("/", routes![set_data, get_data,all_get,all_options])
    .manage(creds)
    .manage(pool)
    .manage(request_handlers)
}


pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods","POST, OPTIONS, GET"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Max-Age", "86400"));
    }
}  