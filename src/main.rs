extern crate postgres;
extern crate "rustc-serialize" as rustc_serialize;

use rustc_serialize::json;

use postgres::{Connection, SslMode};

fn main() {
    let conn = Connection::connect("postgres://postgres@localhost", &SslMode::None)
            .unwrap();

    let stmt = conn.prepare("SELECT id, name, time_created, data FROM person")
            .unwrap();

    let json: json::Json = "{}".parse().unwrap();

    stmt.query(&[&json]).unwrap()
}
