//RESPROVSTART
#[path = "../result_provider/get_site_name.rs"] mod get_site_name;
//RESPROVEND

use std::collections::HashMap;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rocket::State;



pub fn load_request_handlers() -> HashMap<String,fn(&State<Pool<SqliteConnectionManager>>,&String) -> String> {
    let mut handlers: HashMap<String,fn(&State<Pool<SqliteConnectionManager>>,&String) -> String> = HashMap::new();
    handlers.insert("get_site_name".to_string(),get_site_name::execute);

    handlers
}