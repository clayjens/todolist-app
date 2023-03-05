use std::fmt::Display;

pub use color_eyre::eyre::Result;
use diesel::pg::PgConnection;
pub use diesel::prelude::*;

pub mod models;
pub mod schema;

/// Establishes a connection to the database through the `DATABASE_URL` environment variable.
pub fn establish_connection() -> Result<PgConnection> {
    let db_url = std::env::var("DATABASE_URL")?;
    let db_connection = PgConnection::establish(&db_url)?;

    Ok(db_connection)
}

/// Creates a new task in the `tasks` table.
pub fn create_task(db_conn: &mut PgConnection) -> Result<()> {
    use schema::tasks;

    let title = inquire::Text::new("Title").prompt()?;
    let description = inquire::Text::new("Description").prompt()?;

    let new_task = models::NewTask {
        title: &title,
        description: &description,
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(db_conn)?;

    Ok(())
}

pub fn get_tasks(db_conn: &mut PgConnection) -> Result<Vec<models::Task>> {
    use schema::tasks::dsl::*;

    let results = tasks.order(id.desc()).load::<models::Task>(db_conn)?;

    Ok(results)
}

pub fn complete_task(db_conn: &mut PgConnection, task_id: i32) -> Result<()> {
    use schema::tasks::dsl::*;

    diesel::update(tasks.find(task_id))
        .set((
            complete.eq(true),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(db_conn)?;

    Ok(())
}

pub fn delete_task(db_conn: &mut PgConnection, task_id: i32) -> Result<()> {
    use schema::tasks::dsl::*;

    diesel::delete(tasks.find(task_id)).execute(db_conn)?;

    Ok(())
}

pub fn task_id_from_selection(task_selection: &str) -> Result<i32> {
    let task_id = task_selection
        .split(": ")
        .next()
        .expect("Failed to get task id")
        .parse::<i32>()?;

    Ok(task_id)
}

#[derive(Debug)]
pub enum MenuOption {
    CreateTask,
    DeleteTasks,
    CompleteTasks,
    ListTasks,
}

#[derive(Debug)]
pub enum ListTypeOption {
    All,
    Completed,
    Incomplete,
}

impl Display for MenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuOption::CreateTask => write!(f, "Create A Task"),
            MenuOption::DeleteTasks => write!(f, "Delete Tasks"),
            MenuOption::CompleteTasks => write!(f, "Complete Tasks"),
            MenuOption::ListTasks => write!(f, "List Tasks"),
        }
    }
}

impl Display for ListTypeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListTypeOption::All => write!(f, "All Tasks"),
            ListTypeOption::Completed => write!(f, "Completed Tasks"),
            ListTypeOption::Incomplete => write!(f, "Incomplete Tasks"),
        }
    }
}
