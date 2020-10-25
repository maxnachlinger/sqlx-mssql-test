use serde::{Serialize, Deserialize};
use sqlx::{MssqlPool, FromRow, Row, Done};
use sqlx::mssql::MssqlRow;
use anyhow::Result;

// this struct will use to receive user input
#[derive(Serialize, Deserialize)]
pub struct TodoRequest {
    pub description: String,
    pub done: bool
}

// this struct will be used to represent database record
#[derive(Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

// Implementation for Todo struct, functions for read/write/update and delete todo from database
impl Todo {
    pub async fn find_all(pool: &MssqlPool) -> Result<Vec<Todo>> {
        let mut todos = vec![];
        let recs = sqlx::query!(
            r#"
                SELECT id, description, done
                    FROM todos
                ORDER BY id
            "#
        )
            .fetch_all(pool)
            .await?;

        for rec in recs {
            todos.push(Todo {
                id: rec.id,
                description: rec.description,
                done: rec.done
            });
        }

        Ok(todos)
    }

    pub async fn find_by_id(id: i32, pool: &MssqlPool) -> Result<Todo> {
        let rec = sqlx::query!(
                r#"
                    SELECT * FROM todos WHERE id = $1
                "#,
                id
            )
            .fetch_one(&*pool)
            .await?;

        Ok(Todo {
            id: rec.id,
            description: rec.description,
            done: rec.done
        })
    }

    pub async fn create(todo: TodoRequest, pool: &MssqlPool) -> Result<i32> {
        let new_id = sqlx::query("INSERT INTO todos (description, done) VALUES ($1, $2) RETURNING id")
            .bind(&todo.description)
            .bind(todo.done)
            .map(|row: MssqlRow| row.get(0))
            .fetch_one(&*pool)
            .await?;
        Ok(new_id)
    }

    pub async fn update(id: i32, todo: TodoRequest, pool: &MssqlPool) -> Result<u64> {
        let updated = sqlx::query("UPDATE todos SET description = $1, done = $2 WHERE id = $3")
            .bind(&todo.description)
            .bind(todo.done)
            .bind(id)
            .execute(&*pool)
            .await?;
        Ok(updated.rows_affected())
    }

    pub async fn delete(id: i32, pool: &MssqlPool) -> Result<u64> {
        let deleted = sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(id)
            .execute(&*pool)
            .await?;
        Ok(deleted.rows_affected())
    }
}