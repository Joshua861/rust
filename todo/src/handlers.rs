use sea_orm::EntityTrait;
use crate::entities::{Entity, Model};
use crate::db::establish_connection;

pub async fn add_task(task: &str) {
    let db = establish_connection().await.unwrap();
    let todo = Entity::new(task.to_string(), false);
    todo.insert(&db).await.unwrap();
}

pub async fn list_tasks() {
    let db = establish_connection().await.unwrap();
    let todos = Entity::find().all(&db).await.unwrap();
    for todo in todos {
        println!("{}", todo.task);
    }
}

pub async fn complete_task(task: &str) {
    let db = establish_connection().await.unwrap();
    let mut todo = Entity::find_by_id(task.to_string()).one(&db).await.unwrap();
    todo.completed = true;
    todo.update(&db).await.unwrap();
}
