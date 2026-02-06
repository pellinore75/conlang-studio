use rusqlite::{params, Connection, Result};
use crate::models::{User, Project};

// --- USER OPERATIONS ---

pub fn create_user(conn: &Connection, username: &str, password_hash: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO users (username, password_hash) VALUES (?1, ?2)",
                 params![username, password_hash],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_user_by_username(conn: &Connection, username: &str) -> Result<User> {
    conn.query_row(
        "SELECT id, username FROM users WHERE username = ?1",
        params![username],
        |row| {
            Ok(User {
                id: row.get(0)?,
               username: row.get(1)?,
            })
        },
    )
}

// Update get_projects to filter by USER_ID
pub fn get_projects_for_user(conn: &Connection, user_id: i64) -> Result<Vec<Project>> {
    // We need to SELECT the user_id (even though we know it, the struct needs it)
    let mut stmt = conn.prepare("SELECT id, user_id, name, description, created_at FROM projects WHERE user_id = ?1 ORDER BY created_at DESC")?;

    let rows = stmt.query_map(params![user_id], |row| {
        Ok(Project {
            id: row.get(0)?,
           user_id: row.get(1)?, // <--- This was missing!
           name: row.get(2)?,
           description: row.get(3)?,
           created_at: row.get(4)?,
        })
    })?;

    let mut projects = Vec::new();
    for project in rows {
        projects.push(project?);
    }
    Ok(projects)
}

// Update create_project to require USER_ID
pub fn create_project(conn: &Connection, user_id: i64, name: &str, desc: Option<&str>) -> Result<i64> {
    conn.execute(
        "INSERT INTO projects (user_id, name, description) VALUES (?1, ?2, ?3)",
                 params![user_id, name, desc],
    )?;
    Ok(conn.last_insert_rowid())
}
