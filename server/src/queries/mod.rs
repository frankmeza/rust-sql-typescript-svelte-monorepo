pub fn get_name_id_people() -> String {
    format!("SELECT id, name FROM person")
}

pub fn get_name_id_person(id: &str) -> String {
    format!("SELECT id, name FROM person WHERE id = {}", id)
}

// TODO implement incremental id, or use guid()
pub fn create_person(id: &str, name: &str) -> String {
    format!(
        "INSERT INTO person (id, name) VALUES ('{}', '{}')",
        id, name
    )
}

pub fn update_person_by_id(id: &str, name: &str) -> String {
    format!("UPDATE person SET name = '{}' WHERE id = {}", name, id)
}

pub fn delete_person_by_id(id: &str) -> String {
    format!("DELETE FROM person WHERE id = {}", id)
}
