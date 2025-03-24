use std::sync::Arc;

use sqlx::{MySql, MySqlPool, Pool};
use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let sql_connection_url = "mysql://root:strong_password@localhost:3307";
    println!("trying to connect");

    let pool = MySqlPool::connect(sql_connection_url).await?;
    let pool = Arc::new(pool); // Оборачиваем в Arc

    let mut app = tide::with_state(pool.clone());

    // Example Data
    app.at("/example").get(order_shoes);
    app.at("/db/list_all").get(list_all_db);

    // Run server
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn list_all_db(req: Request<Arc<Pool<MySql>>>) -> tide::Result {
    let pool = req.state();

    let dbs = sqlx::query("SHOW DATABASES").fetch_all(&**pool).await?;

    for db in &dbs {
        println!("{:#?}", db);
    }

    Ok(tide::Response::builder(200)
        .body(format!("{:#?}", dbs))
        .content_type(tide::http::mime::PLAIN)
        .build())
}

async fn order_shoes(req: tide::Request<Arc<Pool<MySql>>>) -> tide::Result {
    println!("trying to send request");

    let pool = req.state();

    sqlx::query("CREATE DATABASE IF NOT EXISTS piska_db")
        .execute(&**pool)
        .await?;

    println!("db created");

    Ok(tide::Response::builder(200)
        .body(format!("db created"))
        .content_type(tide::http::mime::PLAIN)
        .build())
}
