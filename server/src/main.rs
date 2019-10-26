use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use postgres::{Connection, TlsMode};
#[macro_use()]
extern crate serde_derive;
use serde_derive::{Deserialize, Serialize};

mod queries;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    id: i32,
    name: String,
    // data: Option<Vec<u8>>,
}

fn get_connection() -> Connection {
    Connection::connect("postgres://postgres@localhost:5432", TlsMode::None)
        .expect("wut happen postgres")
}

// fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

fn index_2_method(conn: Connection) -> Vec<Person> {
    let mut people = Vec::new();

    for row in &conn.query(queries::GET_NAME_ID, &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
        };

        println!("Found person {}: {}", person.id, person.name);
        people.push(person);
    }

    people
}

fn index2() -> impl Responder {
    let conn = get_connection();
    let people = index_2_method(conn);

    // HttpResponse::Ok().body("Hello world again!")
    HttpResponse::Ok().json(people) // <- send response
}


fn main() {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:10001")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            // .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
