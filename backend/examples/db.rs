use chrono::NaiveTime;
use dotenvy;
use sqlx::postgres::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    let url = env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&url).await?;

    // 接続確認
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.0, 150);

    // データ登録確認
    let new_todo = NewTodo {
        id: 1,
        title: "test_title".to_string(),
        content: "test_content".to_string(),
        done: false,
    };
    let id = add_todo(&pool, new_todo).await;
    assert_eq!(id, 1);

    // データ取得確認
    let inserted_todo = get_todo(&pool, 1).await;
    println!("{:?}", inserted_todo);

    match inserted_todo {
        Some(mut todo) => {
            // データ更新確認
            todo.title = "updated title".to_string();
            todo.content = "updated content".to_string();
            assert_eq!(update_todo(&pool, &todo).await, true);

            let updated_todo = get_todo(&pool, 1).await.unwrap();
            assert_eq!("updated title", updated_todo.title);
            assert_eq!("updated content", updated_todo.content);
            println!("{:?}", updated_todo);

            // データ削除確認
            assert_eq!(delete_todo(&pool, 1).await, true);
        }
        _ => eprintln!("何かがおかしい🤔🤔🤔🤔"),
    }

    Ok(())
}

#[derive(Debug)]
struct Todo {
    id: i64,
    title: String,
    content: String,
    done: bool,
    created_at: NaiveTime,
    updated_at: NaiveTime,
}
struct NewTodo {
    id: i64,
    title: String,
    content: String,
    done: bool,
}

// Todoリスト追加
async fn add_todo(pool: &PgPool, todo: NewTodo) -> u64 {
    let rec = sqlx::query!(
        r#"
        INSERT INTO todos VALUES($1, $2, $3, $4)
        RETURNING id
        "#,
        todo.id,
        todo.title,
        todo.content,
        todo.done,
    )
    .fetch_one(pool)
    .await
    .unwrap();

    rec.id.try_into().unwrap()
}

// Todo取得
async fn get_todo(pool: &PgPool, id: u64) -> Option<Todo> {
    let record =
        sqlx::query!(r#"SELECT * FROM todos WHERE id=$1"#, id as i64)
            .fetch_one(pool)
            .await;

    match record {
        Ok(row) => Some(Todo {
            id: row.id,
            title: row.title,
            content: row.content.unwrap(),
            done: row.done,
            created_at: row.created_at.time,
            updated_at: row.updated_at.time,
        }),
        Err(_) => None,
    }
}

// Todoリスト更新
async fn update_todo(pool: &PgPool, todo: &Todo) -> bool {
    let rec = sqlx::query!(
        r#"
        UPDATE todos SET title=$1, content=$2, done=$3, updated_at=CURRENT_TIMESTAMP WHERE id=$4
        "#,
        todo.title,
        todo.content,
        todo.done,
        todo.id
    )
    .execute(pool)
    .await
    .unwrap();

    rec.rows_affected() > 0
}

// Todoリスト削除
async fn delete_todo(pool: &PgPool, id: u64) -> bool {
    let rec = sqlx::query!(
        r#"
        DELETE FROM todos WHERE id=$1
        "#,
        id as i64
    )
    .execute(pool)
    .await
    .unwrap();

    rec.rows_affected() > 0
}
