use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

use crate::dataobj::*;

#[derive(Database)]
#[database("social-media")]
pub struct Records(sqlx::PgPool);

#[get("/<id>")]
pub async fn read(mut db: Connection<Records>, id: i64) -> Option<String> {
    sqlx::query("SELECT * FROM logs WHERE id = ?").bind(id)
        .fetch_one(&mut **db).await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok()
}
