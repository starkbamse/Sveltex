use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{routes, State, options, Request, Response};
use rocket::{http::Status, get,post,launch};
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use hex;
use rusqlite::{Connection, Result};
use std::path::Path;
use std::sync::Mutex;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;


#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetData {
    site_name:String,
    credentials:Credentials
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetData {
    query:DataType
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPrivilegedData {
    credentials:Credentials
}

async fn validate_credentials(passed_username:&String,passed_password:&String,username:&String,password:&String)->bool {
    if passed_password==password && passed_username==username {
        return true;
    } 
    return false;
}

#[post("/set-data", format = "json", data = "<request>")]
async fn set_data(credentials: &State<Credentials>,connection:&State<Pool<SqliteConnectionManager>>,request:Json<SetData>) -> Status {
    //Validating credentials
    let valid_credentials=validate_credentials(&request.credentials.username, &request.credentials.password, &credentials.username, &credentials.password).await;
    if !valid_credentials {
        return Status::BadRequest;
    }

    Status::Ok
}

#[post("/get-secret-data", format = "json", data = "<get_secret_data>")]
async fn get_secret_data(connection:&State<Pool<SqliteConnectionManager>>, get_secret_data: Json<GetPrivilegedData>) -> Json<Vec<String>> {
    let locked_connection=connection.get().unwrap();
    let mut stmt = locked_connection.prepare("SELECT name FROM sveltex_database").unwrap();
    let mut rows= stmt.query([]).unwrap();
    let mut data = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        data.push(row.get(0).unwrap());
    }
    Json(data)
}


#[post("/get-data", format = "json", data = "<get_data>")]
async fn get_data(connection:&State<Pool<SqliteConnectionManager>>, get_data: Json<GetData>) -> Json<Vec<String>> {
    let locked_connection=connection.get().unwrap();
    let mut stmt = locked_connection.prepare("SELECT name FROM sveltex_database").unwrap();
    let mut rows= stmt.query([]).unwrap();
    let mut data = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        data.push(row.get(0).unwrap());
    }
    Json(data)
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
    rocket::build().attach(Cors).mount("/", routes![set_data, get_data,get_secret_data,all_get,all_options]).manage(creds).manage(pool)
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