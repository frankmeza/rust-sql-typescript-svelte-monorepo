use crate::{models::Person, queries};
// use crate::ws_server;
use actix::prelude::*;
use actix::{Actor}; // , Context, StreamHandler};
use actix_web::{Error};
// use actix_web_actors::ws;

use postgres::Connection;

pub struct AppMessenger;
pub struct AppMessage(String);

impl Message for AppMessage {
    type Result = Result<AppMessage, Error>;
}

impl Actor for AppMessenger {
    // type Result = Result<String, Error>;
    type Context = SyncContext<Self>;
    // type Context = actix_web_actors::ws::WebsocketContext<Self>;
}


pub fn fetch_people_list(conn: Connection) -> Vec<Person> {
    // let system = System::new("ws");
    //     // let address = ws_server::WebSocket.start();
    // let ct = ws::WebsocketContext::create(
    //     ws_server::WebSocket,
    //     ws::Ws

    //     });
    // );
    // let address = SyncArbiter::start(2, || ws_server::WebSocket);
    let mut people = Vec::new();
    let q = queries::get_people();

    for row in &conn.query(&q, &[]).expect("ERROR: fetch_people_list") {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            ts: row.get(2),
        };

        people.push(person);
    }

    people
}

pub fn fetch_person_by_id(conn: Connection, id: &str) -> Person {
    let mut person = Person {
        id: "".to_string(),
        name: String::from(""),
        ts: 0,
    };

    let q = queries::get_name_id_person(id);

    for row in &conn.query(&q, &[]).expect("ERROR: fetch_person_by_id") {
        let p = Person {
            id: row.get(0),
            name: row.get(1),
            ts: row.get(2),
        };

        person = p;
    }

    person
}

pub fn create_person(conn: Connection, id: &str, name: &str, timestamp: u64) {
    let q = queries::create_person(id, name, timestamp);
    &conn.execute(&q, &[]).expect("ERROR: create_person");
    // handle error here
    // return eithere error, or Ok(())
}

pub fn update_person_by_id(conn: Connection, id: &str, name: &str) {
    let q = queries::update_person_by_id(id, name);
    &conn.execute(&q, &[]).expect("ERROR: update_person_by_id");
}

pub fn delete_person_by_id(conn: Connection, id: &str) {
    let q = queries::delete_person_by_id(id);
    &conn.query(&q, &[]).expect("ERROR: delete_person_by_id");
}
