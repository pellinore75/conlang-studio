use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub type_: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
}

// These are the ones the compiler says are missing:
#[derive(Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateProjectForm {
    pub name: String,
    pub description: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateProjectForm {
    pub name: String,
    pub description: String,
}
