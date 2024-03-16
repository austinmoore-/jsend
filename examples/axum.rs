use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use jsend::JSendResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
struct Post {
    id: Uuid,
    title: String,
    body: String,
}

#[derive(Debug, Deserialize, Clone)]
struct CreatePost {
    title: String,
    body: String,
}

type Db = Arc<RwLock<HashMap<Uuid, Post>>>;

#[tokio::main]
async fn main() {
    let db = Db::default();
    let post = Post {
        id: Uuid::new_v4(),
        title: "Blog Post Title".to_string(),
        body: "Blog post body".to_string(),
    };
    db.write().unwrap().insert(post.id, post);

    let app = Router::new()
        .route("/posts", get(list_posts).post(create_post))
        .route("/posts/:id", get(get_post_by_id).delete(delete_post_by_id))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn list_posts(State(db): State<Db>) -> impl IntoResponse {
    let mut posts: Vec<Post> = vec![];

    for (_id, post) in db.read().unwrap().clone() {
        posts.push(post);
    }

    Json(JSendResponse::success(Some(json!({"posts": posts}))))
}

async fn create_post(State(db): State<Db>, Json(input): Json<CreatePost>) -> impl IntoResponse {
    let post = Post {
        id: Uuid::new_v4(),
        title: input.title,
        body: input.body,
    };

    let id = post.id;

    db.write().unwrap().insert(post.id, post);

    Json(JSendResponse::success(Some(json!({"id": id}))))
}

async fn get_post_by_id(Path(id): Path<Uuid>, State(db): State<Db>) -> impl IntoResponse {
    let db = db.read().unwrap();
    let post = db.get(&id);
    match post {
        Some(post) => Json(JSendResponse::success(Some(json!({"post": post})))).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(JSendResponse::fail(json!({"id": "not found"}))),
        )
            .into_response(),
    }
}

async fn delete_post_by_id(Path(id): Path<Uuid>, State(db): State<Db>) -> impl IntoResponse {
    let mut db = db.write().unwrap();
    db.remove(&id).unwrap();
    Json(JSendResponse::success(None::<()>))
}
