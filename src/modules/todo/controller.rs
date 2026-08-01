use super::entities::todos::{
    ActiveModel as TodosActiveModel, Column as TodosColumn, Entity as TodosEntity,
};
use crate::app::AppState;
use crate::modules::auth::jwt::Claims;
use crate::modules::todo::dto::{IdParam, TodoCreateDto, TodoItemResponse, TodoUpdateDto};
use axum::{Extension, Json, extract::Path, http::StatusCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use sea_orm::{ColumnTrait, Condition, QueryFilter};

pub async fn list(
    claims: Claims,
    Extension(state): Extension<AppState>,
) -> Result<Json<Vec<TodoItemResponse>>, StatusCode> {
    let condition = Condition::all().add(TodosColumn::UserId.eq(claims.sub));
    let todos = TodosEntity::find()
        .filter(condition)
        .all(&state.db)
        .await
        .unwrap();

    let response = todos
        .into_iter()
        .map(|item| TodoItemResponse {
            id: item.id,
            title: item.title,
            completed: item.completed,
            created_at: item.created_at.to_string(),
        })
        .collect();
    Ok(Json(response))
}

pub async fn show(
    claims: Claims,
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
) -> Result<Json<TodoItemResponse>, StatusCode> {
    let condition = Condition::all().add(TodosColumn::UserId.eq(claims.sub));
    let todo = TodosEntity::find_by_id(id.0)
        .filter(condition)
        .one(&state.db)
        .await
        .unwrap();

    if todo.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let todo = todo.unwrap();

    Ok(Json(TodoItemResponse {
        id: todo.id,
        title: todo.title,
        completed: todo.completed,
        created_at: todo.created_at.to_string(),
    }))
}

pub async fn add(
    claims: Claims,
    Extension(state): Extension<AppState>,
    Json(payload): Json<TodoCreateDto>,
) -> Result<Json<TodoItemResponse>, StatusCode> {
    let new_todo = TodosActiveModel {
        title: Set(payload.title),
        completed: match payload.completed {
            Some(completed) => Set(completed),
            None => Set(false),
        },
        user_id: Set(claims.sub),
        ..Default::default()
    };
    let inserted_todo = new_todo.insert(&state.db).await.unwrap();

    Ok(Json(TodoItemResponse {
        id: inserted_todo.id,
        title: inserted_todo.title,
        completed: inserted_todo.completed,
        created_at: inserted_todo.created_at.to_string(),
    }))
}

pub async fn update(
    claims: Claims,
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
    Json(payload): Json<TodoUpdateDto>,
) -> Result<Json<TodoItemResponse>, StatusCode> {
    let condition = Condition::all().add(TodosColumn::UserId.eq(claims.sub));
    let existing_tood = TodosEntity::find_by_id(id.0)
        .filter(condition)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? // Best practice instead of .unwrap
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut active_todo: TodosActiveModel = existing_tood.into();

    if let Some(title) = payload.title {
        active_todo.title = Set(title);
    }

    if let Some(completed) = payload.completed {
        active_todo.completed = Set(completed);
    }

    let update_todo = active_todo
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TodoItemResponse {
        id: update_todo.id,
        title: update_todo.title,
        completed: update_todo.completed,
        created_at: update_todo.created_at.to_string(),
    }))
}

pub async fn remove(
    claims: Claims,
    Extension(state): Extension<AppState>,
    Path(id): Path<IdParam>,
) -> Result<(), StatusCode> {
    let condition = Condition::all().add(TodosColumn::UserId.eq(claims.sub));
    let delete_result = TodosEntity::delete_by_id(id.0)
        .filter(condition)
        .exec(&state.db)
        .await
        .unwrap();

    if delete_result.rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(())
}
