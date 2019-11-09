use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use postgres::{Connection, TlsMode};

mod handlers;
mod models;
mod queries;
mod responders;
mod ws_server;

extern crate env_logger;
extern crate ws;

pub fn get_connection() -> Connection {
    Connection::connect("postgres://postgres@localhost:5432", TlsMode::None)
        .expect("wut happen postgres")
}

fn start_ws() {
    ws_server::start();
}

fn main() {
    env_logger::init();

    println!("Server did something");
    start_ws();

    // HttpServer::new(|| {
    //     App::new()
    //         .wrap(
    //             Cors::new()
    //                 .allowed_origin("http://localhost:10001")
    //                 .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
    //                 .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
    //                 .allowed_header(header::CONTENT_TYPE)
    //                 .max_age(3600),
    //         )
    //         .route("/people", web::get().to(responders::get_people_list))
    //         .route("/people", web::post().to(responders::create_person))
    //         .route("/people", web::put().to(responders::update_person_by_id))
    //         .route("/people/{id}", web::get().to(responders::get_person_by_id))
    //         .route(
    //             "/people/{id}",
    //             web::delete().to(responders::delete_person_by_id),
    //         )
    // })
    // .bind("127.0.0.1:8088")
    // .unwrap()
    // .run()
    // .unwrap();

    println!("Server got here");
    println!("Server started");
}

// .route("/ws/", web::get().to(ws_server::start))
// // .service(web::resource("/ws").to(ws_server::start))