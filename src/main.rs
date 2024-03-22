use crate::domain::models::user::{User, Users};
use axum::{
  extract::{Path, State},
  routing::{get, patch},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

mod domain;

// I'm referring the web page to try creating CRUD.
// https://qiita.com/otohusan/items/491d110aba406f5b3fa0

// To try creating DDD.
// https://qiita.com/tsuchinoko0402/items/dda60c43dbe4e83e729d

type UsersState = State<Arc<Mutex<Users>>>;

async fn get_users(State(users_state): UsersState) -> Json<Users> {
  let users_lock = users_state.lock().await;

  // usersを返す
  Json(users_lock.clone())
}

#[derive(Clone, Serialize, Deserialize)]
struct CreateUser {
  name: String,
}

async fn post_user(
  State(users_state): UsersState,
  create_user: Json<CreateUser>,
) -> Json<Users> {
  let mut users_lock = users_state.lock().await;

  let new_user = User {
    id: (users_lock.users.len() + 1) as u32,
    name: create_user.name.to_string(),
  };

  users_lock.users.push(new_user);

  Json(users_lock.clone())
}

async fn patch_user(
  State(users_state): UsersState,
  Path(user_id): Path<u32>,
  Json(update_user): Json<CreateUser>,
) -> Result<Json<Users>, String> {
  let mut users_lock = users_state.lock().await;

  let find_user = users_lock.users.iter_mut().find(|user| user.id == user_id);

  match find_user {
    Some(user) => {
      user.name = update_user.name.clone();
      Ok(Json(users_lock.clone()))
    }
    None => Err("User not found".to_string()),
  }
}

async fn delete_user(
  State(users_state): UsersState,
  Path(user_id): Path<u32>,
) -> Result<Json<Users>, String> {
  let mut users_lock = users_state.lock().await;
  let has_user_by_id = users_lock.users.iter().any(|user| user.id == user_id);

  if !has_user_by_id {
    return Err("User not found".to_string());
  }

  users_lock.users.retain(|user| user.id != user_id);

  Ok(Json(users_lock.clone()))
}

#[tokio::main]
async fn main() {
  // Hello Worldと返すハンドラーを定義
  async fn root_handler() -> String {
    "Hello World".to_string()
  }

  let users = Arc::new(Mutex::new(Users::new()));

  // ルートを定義
  // "/"を踏むと、上で定義したroot_handlerを実行する
  let app = Router::new()
    .route("/", get(root_handler))
    .route("/users", get(get_users).post(post_user))
    .route("/users/:user_id", patch(patch_user).delete(delete_user))
    .with_state(users);

  // 指定したポートにサーバを開く
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
