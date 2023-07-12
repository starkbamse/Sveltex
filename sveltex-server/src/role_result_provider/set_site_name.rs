use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rocket::State;


pub fn execute(connection:&State<Pool<SqliteConnectionManager>>,query:&String)->String{
    let locked_connection=connection.get().unwrap();
    locked_connection.execute("CREATE TABLE IF NOT EXISTS sveltex_database (
        id              INTEGER PRIMARY KEY,
        name            TEXT NOT NULL
    )", []).unwrap();

    locked_connection.execute("DELETE FROM sveltex_database", []).unwrap();
    locked_connection.execute(
        "INSERT INTO sveltex_database (id, name) VALUES (?1, ?2)",
        (&0, query),
    ).unwrap();
    "true".to_string()
}