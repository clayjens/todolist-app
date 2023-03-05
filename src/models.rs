use std::fmt::Display;

use crate::schema::tasks;
use diesel::prelude::*;

/// A task in the `tasks` table represented as a struct.
#[derive(Queryable, Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub complete: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Displays the Task struct as a string.
impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}\nDescription: {}\nComplete: {}\nCreated At: {}\nUpdated At: {}",
            self.title,
            self.description,
            self.complete,
            self.created_at.format("%Y-%m-%d %H:%M:%S"),
            self.updated_at.format("%Y-%m-%d %H:%M:%S")
        )
    }
}

/// Allows for inserting a new task into the `tasks` table.
#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub description: &'a str,
}
