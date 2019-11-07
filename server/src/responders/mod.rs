use crate::get_connection;
use crate::handlers;
use crate::models;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub fn get_people_list() -> impl Responder {
    let conn = get_connection();
    let people = handlers::fetch_people_list(conn);

    HttpResponse::Ok().json(people)
}

pub fn create_person(person_json: web::Json<models::Person>) -> impl Responder {
    let conn = get_connection();
    let id = person_json.id.to_string();
    let name = &person_json.name;

    handlers::create_person(conn, &id, &name);
    HttpResponse::NoContent()
}

pub fn get_person_by_id(req: HttpRequest) -> impl Responder {
    let conn = get_connection();
    let id = req.match_info().get("id").expect("wut happen get id");
    let person = handlers::fetch_person_by_id(conn, id);

    HttpResponse::Ok().json(person)
}

pub fn update_person_by_id(person_json: web::Json<models::Person>) -> impl Responder {
    let conn = get_connection();
    let id = person_json.id.to_string();
    let updated_name = &person_json.name;

    handlers::update_person_by_id(conn, &id, &updated_name);
    HttpResponse::NoContent()
}

pub fn delete_person_by_id(req: HttpRequest) -> impl Responder {
    let conn = get_connection();
    let id = req.match_info().get("id").expect("wut happen delete id");

    handlers::delete_person_by_id(conn, id);
    HttpResponse::NoContent()
}
