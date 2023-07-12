//RESPROVSTART
#[path = "../plugins/core/set_site_name.rs"] mod set_site_name;
#[path = "../plugins/core/get_site_name.rs"] mod get_site_name;
//RESPROVEND

use std::collections::HashMap;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rocket::State;
use std::io::Error;
use crate::TransmitData;



pub fn load_request_handlers() -> HashMap<&'static str ,fn(&State<Pool<SqliteConnectionManager>>,&TransmitData) -> Result<String,Error>> {
    let mut handlers: HashMap<&'static str,fn(&State<Pool<SqliteConnectionManager>>,&TransmitData) -> Result<String,Error>> = HashMap::new();
    handlers.insert("get_site_name",get_site_name::execute);
    handlers.insert("set_site_name", set_site_name::execute);
    handlers
}