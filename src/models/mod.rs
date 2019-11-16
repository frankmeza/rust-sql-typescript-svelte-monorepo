use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonReq {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub id: String,
    pub name: String,
}
