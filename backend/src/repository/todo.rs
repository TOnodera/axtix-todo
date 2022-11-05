use sqlx::PgPool;

use crate::config::types::Result;
use crate::config::types::Todo;
use crate::request::CreateTodoRequest;
use crate::request::UpdateTodoRequest;

pub struct TodoRepository<'a> {
    pub pool: &'a PgPool,
}

impl TodoRepository<'_> {
    pub async fn create(
        &self,
        todo: &CreateTodoRequest,
    ) -> Result<i64> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO todos(title, content, done) VALUES($1, $2, $3)
            RETURNING id"#,
            todo.title,
            todo.content,
            todo.done,
        )
        .fetch_one(self.pool)
        .await?;

        Ok(rec.id)
    }

    pub async fn read(&self, id: i64) -> Result<Todo> {
        let record =
            sqlx::query!(r#"SELECT * FROM todos WHERE id=$1"#, id)
                .fetch_one(self.pool)
                .await?;

        Ok(Todo {
            id: record.id,
            title: record.title,
            content: record.content,
            done: record.done,
            created_at: Some(record.created_at.time),
            updated_at: Some(record.updated_at.time),
        })
    }

    pub async fn update(
        &self,
        todo: UpdateTodoRequest,
    ) -> Result<bool> {
        let record = sqlx::query!(
            r#"
            UPDATE todos SET title=$1, content=$2, done=$3, updated_at=CURRENT_TIMESTAMP WHERE id=$4
            "#,
            todo.title,
            todo.content,
            todo.done,
            todo.id
        )
        .execute(self.pool)
        .await?;

        Ok(record.rows_affected() > 0)
    }

    pub async fn delete(&self, id: i64) -> Result<bool> {
        let record = sqlx::query!(
            r#"DELETE FROM todos WHERE id=$1"#,
            id as i64
        )
        .execute(self.pool)
        .await?;

        Ok(record.rows_affected() > 0)
    }
}
