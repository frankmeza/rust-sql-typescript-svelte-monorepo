pub fn get_name_id_people() -> String {
    let query = format!("SELECT id, name FROM person");
    query
}

pub fn get_name_id_person(id: &str) -> String {
    let query = format!("SELECT id, name FROM person WHERE id = {}", id);
    query
}

pub fn create_person(id: &str, name: &str) -> String {
    // TODO implement incremental id, or use guid()
    let query = format!("INSERT INTO person (id, name) VALUES ('{}', '{}')", id, name);
    query
}

pub fn update_person_by_id(id: &str, name: &str) -> String {
    let query = format!("UPDATE person SET name = '{}' WHERE id = {}", name, id);
    query
}

pub fn delete_person_by_id(id: &str) -> String {
    let query = format!("DELETE FROM person WHERE id = {}", id);
    query
}
