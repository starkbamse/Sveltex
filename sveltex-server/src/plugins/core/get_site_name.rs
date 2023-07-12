use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rocket::State;
use std::io::Error;

use crate::{TransmitData};


pub fn execute(connection:&State<Pool<SqliteConnectionManager>>,_query:&TransmitData,)->Result<String,Error>{
    let locked_connection=connection.get().unwrap();
    let mut stmt = locked_connection.prepare("SELECT name FROM sveltex_database").unwrap();
    let mut rows= stmt.query([]).unwrap();
    let mut site_name:String="".to_string();
    while let Some(row) = rows.next().unwrap() {
        site_name=row.get(0).unwrap();
    }
    Ok(site_name)
}