use std::collections::HashMap;
use std::sync::RwLock;
use rocket::serde::{Serialize};

use lazy_static::lazy_static;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct Entity {
    data: String,
}

lazy_static! {
    static ref STORAGE: RwLock<HashMap<Uuid, Entity>> = RwLock::new(HashMap::new());
}

pub fn list_entities() -> Vec<Entity> {
    let mut result: Vec<Entity> = Vec::new();
    let storage = STORAGE.read().unwrap();
    for (_, e) in storage.iter() {
        result.push(e.clone())
    }
    result
}
