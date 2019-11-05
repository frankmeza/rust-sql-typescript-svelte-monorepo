use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use postgres::{Connection, TlsMode};
// use actix_web_actors::ws;

mod handlers;
mod queries;

fn get_connection() -> Connection {
    Connection::connect("postgres://postgres@localhost:5432", TlsMode::None)
        .expect("wut happen postgres")
}

fn get_people_list() -> impl Responder {
    let conn = get_connection();
    let people = handlers::fetch_people_list(conn);

    HttpResponse::Ok().json(people)
}

fn create_person(person_json: web::Json<handlers::Person>) -> impl Responder {
    let conn = get_connection();
    let id = person_json.id.to_string();
    let name = &person_json.name;

    handlers::create_person(conn, &id, &name);
    HttpResponse::NoContent()
}

fn get_person_by_id(req: HttpRequest) -> impl Responder {
    let conn = get_connection();
    let id = req.match_info().get("id").expect("wut happen get id");
    let person = handlers::fetch_person_by_id(conn, id);

    HttpResponse::Ok().json(person)
}

fn update_person_by_id(person_json: web::Json<handlers::Person>) -> impl Responder {
    let conn = get_connection();
    let id = person_json.id.to_string();
    let updated_name = &person_json.name;

    handlers::update_person_by_id(conn, &id, &updated_name);
    HttpResponse::NoContent()
}

fn delete_person_by_id(req: HttpRequest) -> impl Responder {
    let conn = get_connection();
    let id = req.match_info().get("id").expect("wut happen delete id");

    handlers::delete_person_by_id(conn, id);
    HttpResponse::NoContent()
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:10001")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .route("/people", web::get().to(get_people_list))
            .route("/people", web::post().to(create_person))
            .route("/people", web::put().to(update_person_by_id))
            .route("/people/{id}", web::get().to(get_person_by_id))
            .route("/people/{id}", web::delete().to(delete_person_by_id))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
