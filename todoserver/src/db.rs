use deadpool_postgres::Client;
use std::error::Error;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::models::{TodoItem, TodoList};

// pub async fn get_todo_lists(client: &Client) -> Result<Vec<TodoList>, Error> {
//     let query = client.prepare("select * from todo_list").await.unwrap();

//     let res = client
//         .query(&query, &[])
//         .await
//         .expect("Error getting lists.")
//         .iter()
//         .map(|row| TodoList::from_row_ref(row).unwrap())
//         .collect::<Vec<TodoList>>();

//     Ok(res)
// }
pub async fn get_todo_lists(client: &Client) -> Result<Vec<TodoList>, Error> {
    let q = client.prepare("select * from todo_list").await.unwrap();

    let res = client
        .query(&q, &[])
        .await
        .expect("Error getting lists.")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(res)
}
