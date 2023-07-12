//RESPROVSTART
#[path = "../role_result_provider/set_site_name.rs"] mod set_site_name;
//RESPROVEND



use std::collections::HashMap;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rocket::State;




pub fn load_role_request_handlers() -> HashMap<String,fn(&State<Pool<SqliteConnectionManager>>,&String) -> String> {
    let mut handlers: HashMap<String,fn(&State<Pool<SqliteConnectionManager>>,&String) -> String> = HashMap::new();
    handlers.insert("set_site_name".to_string(),set_site_name::execute);
    handlers
}