use axum::{Json, extract::{Path, State}, http::StatusCode};
use futures::TryStreamExt;
use mongodb::{Collection, bson::{doc, oid::ObjectId}, options::{FindOneAndUpdateOptions, ReturnDocument}};
use validator::Validate;

use crate::{error::AppError, models::{CreateTodoRequest, Todo, TodoResponse, UpdateTodoRequest}};


pub type DbState = mongodb::Database;

pub async fn create_todo(
    State(db): State<DbState>,
    Json(payload): Json<CreateTodoRequest>,

) -> Result<Json<TodoResponse>, AppError> {
    payload.validate()?;

    let collection: Collection<Todo> = db.collection("todos");

    let todo = Todo {
        id: None,
        title: payload.title,
        description: payload.description,
        completed: false
    };

    let result = collection.insert_one(todo.clone()).await?;

    let mut created_todo = todo;
    created_todo.id = result.inserted_id.as_object_id();

    Ok(Json(TodoResponse::from(created_todo)))
}

pub async fn get_all_todos(
    State(db): State<DbState>,
) -> Result<Json<Vec<TodoResponse>>, AppError> {
    let collection: Collection<Todo> = db.collection("todos");

    let cursor  = collection.find(doc!{}).await?;
    let todos: Vec<Todo> = cursor.try_collect().await?;
    let todos_response: Vec<TodoResponse> = todos.into_iter()
        .map(|todo| TodoResponse::from(todo))
        .collect();

    Ok(Json(todos_response))
}

pub async fn get_todo(
    State(db): State<DbState>,
    Path(id): Path<String>,
) -> Result<Json<TodoResponse>, AppError> {
    let collection: Collection<Todo> = db.collection("todos");

    let object_id = ObjectId::parse_str(&id)?;
    let filter = doc! { "_id": object_id };

    match collection.find_one(filter).await? {
        Some(todo) => Ok(Json(TodoResponse::from(todo))),
        None => Err(AppError::NotFound(format!("Todo with id {} not found", id))),
    }
}

pub async fn update_todo(
    State(db): State<DbState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTodoRequest>
) -> Result<Json<TodoResponse>, AppError> {
    payload.validate()?;

    let collection: Collection<Todo> = db.collection("todos");

    let object_id = ObjectId::parse_str(&id)?;
    let filter = doc! { "_id": object_id };

    let mut update_doc = doc! {};

    if let Some(title) = payload.title {
        update_doc.insert("title", title);
    }

    if let Some(description) = payload.description {
        update_doc.insert("description", description);
    }

    if let Some(completed) = payload.completed {
        update_doc.insert("completed", completed);
    }

    let update = doc! { "$set": update_doc };

    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    match collection.find_one_and_update(filter, update).with_options(options).await? {
        Some(updated_todo) => Ok(Json(TodoResponse::from(updated_todo))),
        None => Err(AppError::NotFound(format!("Todo with id {} not found", id))),
    }
}

pub async fn delete_todo(
    State(db): State<DbState>,
    Path(id): Path<String>
) -> Result<StatusCode, AppError> {
    let collection: Collection<Todo> = db.collection("todos");

    let object_id = ObjectId::parse_str(&id)?;
    let filter = doc! { "_id": object_id };

    let result = collection.delete_one(filter).await?;

    if result.deleted_count == 1 {
        Ok(StatusCode::NO_CONTENT)
    }
    else {
        Err(AppError::NotFound(format!("Todo with id {} not found", id)))
    }
}