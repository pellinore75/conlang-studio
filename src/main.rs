use axum::{
    extract::{Form, State},
    response::{Html, Redirect, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use rusqlite::Connection;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use askama::Template;
use tower_sessions::{Session, Expiry, SessionManagerLayer};
use tower_sessions_rusqlite_store::RusqliteStore;
use time::Duration;

mod db;
mod models;
mod auth;

use models::{CreateProjectForm, AuthPayload};

#[derive(Clone)]
struct AppState {
    conn: Arc<Mutex<Connection>>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate { projects: Vec<models::Project>, username: String }

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate { error: Option<String> }

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate { error: Option<String> }

#[tokio::main]
async fn main() {
    let conn_app = Connection::open("studio.db").expect("Failed to open DB for App");
    let conn_sessions = Connection::open("studio.db").expect("Failed to open DB for Sessions");

    conn_app.execute("PRAGMA foreign_keys = ON;", []).expect("FKs failed");
    conn_app.execute_batch(include_str!("../schema.sql")).expect("Schema failed");

    // OLDER STORE: 0.10 doesn't need .into() or async conversion
    let session_store = RusqliteStore::new(conn_sessions);
    session_store.migrate().await.unwrap();

    let session_layer = SessionManagerLayer::new(session_store)
    .with_secure(false)
    .with_expiry(Expiry::OnInactivity(Duration::hours(24)));

    let state = AppState {
        conn: Arc::new(Mutex::new(conn_app)),
    };

    let app = Router::new()
    .route("/", get(home))
    .route("/projects/new", post(create_project))
    .route("/logout", get(logout))
    .route("/login", get(show_login).post(login))
    .route("/register", get(show_register).post(register))
    .layer(session_layer)
    .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Conlang Studio active on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home(session: Session, State(state): State<AppState>) -> impl IntoResponse {
    let user_id: Option<i64> = session.get("user_id").await.unwrap();
    let username: Option<String> = session.get("username").await.unwrap();

    if let (Some(uid), Some(name)) = (user_id, username) {
        let conn = state.conn.lock().unwrap();
        let projects = db::get_projects_for_user(&conn, uid).unwrap_or_default();
        let template = IndexTemplate { projects, username: name };
        Html(template.render().unwrap()).into_response()
    } else {
        Redirect::to("/login").into_response()
    }
}

async fn show_login() -> impl IntoResponse {
    Html(LoginTemplate { error: None }.render().unwrap())
}

async fn login(session: Session, State(state): State<AppState>, Form(payload): Form<AuthPayload>) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    if let Ok(user) = db::get_user_by_username(&conn, &payload.username) {
        let hash: String = conn.query_row("SELECT password_hash FROM users WHERE id = ?1", [user.id], |row| row.get(0)).unwrap_or_default();
        if auth::verify_password(&payload.password, &hash) {
            session.insert("user_id", user.id).await.unwrap();
            session.insert("username", user.username).await.unwrap();
            return Redirect::to("/").into_response();
        }
    }
    Redirect::to("/login?error=invalid").into_response()
}

async fn show_register() -> impl IntoResponse {
    Html(RegisterTemplate { error: None }.render().unwrap())
}

async fn register(session: Session, State(state): State<AppState>, Form(payload): Form<AuthPayload>) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let hash = auth::hash_password(&payload.password).unwrap();
    if let Ok(uid) = db::create_user(&conn, &payload.username, &hash) {
        session.insert("user_id", uid).await.unwrap();
        session.insert("username", payload.username).await.unwrap();
        Redirect::to("/").into_response()
    } else {
        Redirect::to("/register?error=exists").into_response()
    }
}

async fn logout(session: Session) -> impl IntoResponse {
    session.flush().await.unwrap();
    Redirect::to("/login")
}

async fn create_project(session: Session, State(state): State<AppState>, Form(payload): Form<CreateProjectForm>) -> impl IntoResponse {
    let user_id: Option<i64> = session.get("user_id").await.unwrap();
    if let Some(uid) = user_id {
        let conn = state.conn.lock().unwrap();
        let desc = if payload.description.trim().is_empty() { None } else { Some(payload.description.as_str()) };
        let _ = db::create_project(&conn, uid, &payload.name, desc);
    }
    Redirect::to("/")
}
