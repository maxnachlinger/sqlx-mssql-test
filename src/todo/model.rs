use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{Done, FromRow, MssqlPool};

// this struct will use to receive user input
#[derive(Serialize, Deserialize)]
pub struct TodoRequest {
    pub description: String,
    pub done: bool,
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
        let results = sqlx::query!(
            r#"
                SELECT
                  t.id,
                  t.description,
                  t.done
                FROM todos t
                ORDER BY t.id
            "#
        )
        .fetch_all(pool)
        .await?;

        for result in results {
            todos.push(Todo {
                id: result.id,
                description: result.description,
                done: result.done,
            });
        }

        Ok(todos)
    }

    pub async fn find_by_id(id: i32, pool: &MssqlPool) -> Result<Todo> {
        let result = sqlx::query!(
            r#"
                SELECT
                  t.id,
                  t.description,
                  t.done
                FROM todos t
                WHERE id = @p1
                "#,
            id
        )
        .fetch_one(&*pool)
        .await?;

        Ok(Todo {
            id: result.id,
            description: result.description,
            done: result.done,
        })
    }

    pub async fn create(todo: TodoRequest, pool: &MssqlPool) -> Result<Todo> {
        let result = sqlx::query!(
            r#"
        INSERT INTO todos (description, done)
        OUTPUT Inserted.ID AS id
        VALUES (@p1, @p2)
        "#,
            &todo.description,
            &todo.done
        )
        .fetch_one(&*pool)
        .await?;

        Ok(Todo {
            id: result.id,
            description: todo.description,
            done: todo.done,
        })
    }

    pub async fn update(id: i32, todo: TodoRequest, pool: &MssqlPool) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE todos
            SET description = @p1,
            done = @p2
            WHERE id = @p3
        "#,
            &todo.description,
            &todo.done,
            &id
        )
        .execute(&*pool)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn delete(id: i32, pool: &MssqlPool) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM todos WHERE id = @p1
        "#,
            &id
        )
        .execute(&*pool)
        .await?;

        Ok(result.rows_affected())
    }
}
