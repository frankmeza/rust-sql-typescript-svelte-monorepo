use crate::{models::Person, queries};
use postgres::Connection;

pub fn fetch_people_list(conn: Connection) -> Vec<Person> {
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
        id: String::from(""),
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
}

pub fn update_person_by_id(conn: Connection, id: &str, name: &str) {
    let q = queries::update_person_by_id(id, name);
    &conn.execute(&q, &[]).expect("ERROR: update_person_by_id");
}

pub fn delete_person_by_id(conn: Connection, id: &str) {
    let q = queries::delete_person_by_id(id);
    &conn.query(&q, &[]).expect("ERROR: delete_person_by_id");
}
