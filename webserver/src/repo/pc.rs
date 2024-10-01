use sqlx::{query_as, Error, Pool, Postgres, FromRow};
use super::base::EntityId;
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Pc {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub hostname: String,
    pub key: String,
    pub value: String
}


pub async fn insert_pc(conn: &Pool<Postgres>, hostname: String, key: String, value: String) -> Result<EntityId, Error> {
    query_as::<_, EntityId>("insert into pc (hostname, key, value) values ($1, $2, $3) returning id")
        .bind(hostname)
        .bind(key)
        .bind(value)
        .fetch_one(conn)
        .await
}

pub async fn query_all(conn: &Pool<Postgres>) -> Result<Vec<Pc>, Error> {
    query_as::<_, Pc>("select * from pc order by updated_at desc")
        .fetch_all(conn)
        .await
}