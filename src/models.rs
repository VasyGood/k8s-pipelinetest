use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    
    pub title: String,
    
    pub description: Option<String>,
    
    pub completed: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTodoRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    
    #[validate(length(max = 500))]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTodoRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,

    #[validate(length(max = 500))]
    pub description: Option<String>,
    
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct TodoResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool
}

impl From<Todo> for TodoResponse {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id.map(|id| id.to_hex()).unwrap_or_default(),
            title: todo.title,
            description: todo.description,
            completed: todo.completed
        }
    }
}