#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&database_url).await.unwrap();

    let todo = Todo {
        id: 1,
        title: "Buy milk".to_string(),
        description: Some("Go to the store and buy milk".to_string()),
        doney: Some(false),
    };

    let created_todo = create_todo(&pool, todo).await.unwrap();
    println!("{:?}", created_todo);
}

#[derive(Debug)]
struct Todo {
    id: i32,
    title: String,
    description: Option<String>,
    doney: Option<bool>,
}

async fn create_todo(pool: &sqlx::PgPool, todo: Todo) -> Result<Todo, sqlx::Error> {
    sqlx::query_as!(Todo,
        r#"
        INSERT INTO todo (id, title, description, done)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, description, done AS doney
        "#,
        todo.id,
        todo.title,
        todo.description,
        todo.doney
    )
    .fetch_one(pool)
    .await
}
