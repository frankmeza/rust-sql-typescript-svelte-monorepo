use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// extern crate postgres;
use postgres::{Connection, TlsMode};

mod queries;

fn get_connection() -> Connection {
    Connection::connect("postgres://postgres@localhost:5432", TlsMode::None)
        .expect("wut happen postgres")
}

// fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

fn index_2_method(conn: Connection) {
    for row in &conn.query(queries::GET_NAME_ID, &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
        };

        println!("Found person {}: {}", person.id, person.name);
    }
}

fn index2() -> impl Responder {
    let conn = get_connection();
    index_2_method(conn);
    // for row in &conn.query(queries::GET_NAME_ID, &[]).unwrap() {
    //     let person = Person {
    //         id: row.get(0),
    //         name: row.get(1),
    //     };

    //     println!("Found person {}: {}", person.id, person.name);
    // }

    HttpResponse::Ok().body("Hello world again!")
}

struct Person {
    id: i32,
    name: String,
    // data: Option<Vec<u8>>,
}

fn main() {
    // let conn = get_connection();

    // let me = Person {
    //     id: 0,
    //     name: "Frank2".to_string(),
    //     data: None,
    // };

    // conn.execute(
    //     "INSERT INTO person (name, data) VALUES ($1, $2)",
    //     &[&me.name, &me.data],
    // )
    // .unwrap();

    HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
