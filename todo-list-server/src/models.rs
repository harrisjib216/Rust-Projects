use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "todo_list")]
pub struct TodoList {
    pub id: u32,
    pub title: String,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "todo_item")]
pub struct TodoItem {
    pub id: i32,
    pub list_id: i32,
    pub title: String,
    pub checked: bool,
}
