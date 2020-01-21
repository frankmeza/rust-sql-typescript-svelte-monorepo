use crate::{models::Person, queries};
use postgres::Connection;

pub fn fetch_people_list(conn: Connection) -> Vec<Person> {
    let mut people = Vec::new();
    let q = queries::get_people();
    let rows = &conn.query(&q, &[]);

    match rows {
        Ok(rows) => {
            for row in rows {
                let person = Person {
                    id: row.get(0),
                    name: row.get(1),
                    ts: row.get(2),
                };

                people.push(person);
            }
        }
        Err(err) => {
            println!("rows in fetch_people_list very virus: {:?}", err);
        }
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
    let rows = &conn.query(&q, &[]);

    match rows {
        Ok(rows) => {
            for row in rows {
                let p = Person {
                    id: row.get(0),
                    name: row.get(1),
                    ts: row.get(2),
                };

                person = p;
            }
        }
        Err(err) => {
            println!("rows in fetch_person_by_id very virus: {:?}", err);
        }
    }

    person
}

pub fn create_person(conn: Connection, id: &str, name: &str, timestamp: u64) {
    let q = queries::create_person(id, name, timestamp);
    let rows = &conn.query(&q, &[]);

    match rows {
        Ok(_rows) => (),
        Err(err) => {
            println!("rows in create_person very virus: {:?}", err);
        }
    }
}

pub fn update_person_by_id(conn: Connection, id: &str, name: &str) {
    let q = queries::update_person_by_id(id, name);
    let rows = &conn.query(&q, &[]);

    match rows {
        Ok(_rows) => (),
        Err(err) => {
            println!("rows in update_person_by_id very virus: {:?}", err);
        }
    }
}

pub fn delete_person_by_id(conn: Connection, id: &str) {
    let q = queries::delete_person_by_id(id);
    let rows = &conn.query(&q, &[]);

    match rows {
        Ok(_rows) => (),
        Err(err) => {
            println!("rows in delete_person_by_id very virus: {:?}", err);
        }
    }
}
