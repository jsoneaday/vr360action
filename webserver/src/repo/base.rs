use sqlx::{Pool, Postgres, FromRow};
use async_trait::async_trait;
use tokio::time::{sleep, Duration};

#[derive(FromRow, Clone)]
pub struct EntityId {
    pub id: i64
}

#[async_trait]
pub trait Repository{
    async fn init() -> Self;
}

#[derive(Debug, Clone)]
pub struct DbRepo {
    conn: Pool<Postgres>
}

#[async_trait]
impl Repository for DbRepo {
    async fn init() -> Self {
        DbRepo {
            conn: get_conn_pool().await
        }
    }
}

#[async_trait]
pub trait ConnGetter: Repository {
    type Output;

    fn get_conn(&self) -> &Self::Output;    

    async fn disconnect(&mut self);
}

#[async_trait]
impl ConnGetter for DbRepo {
    type Output = Pool<Postgres>;

    fn get_conn(&self) -> &Self::Output {
        &self.conn
    }

    async fn disconnect(&mut self) {
        self.conn.close().await;
    }
}

async fn get_conn_pool() -> Pool<Postgres> {
    let postgres_host = "127.0.0.1";
    let postgres_port: u16 = 5432;
    let postgres_password = "vr360action";
    let postgres_user = "vr360action";
    let postgres_db = "vr360action";

    let postgres_url = format!(
        "postgres://{postgres_user}:{postgres_password}@{postgres_host}:{postgres_port}/{postgres_db}"
    );
    
    let mut retry_limit = 0;
    let mut conn: Option<Pool<Postgres>> = None;
    loop {
        if retry_limit > 3 {
            panic!("Attempts to connect to db have failed, exiting ...");
        }
        let conn_result = sqlx::postgres::PgPool::connect(&postgres_url).await;
        match conn_result {
            Ok(some_conn) => {
                println!("db connection successful");
                conn = Some(some_conn);
                break;
            },
            Err(ref e) => {
                println!("Failed to connect to db: {}", e);
                retry_limit += 1;
                sleep(Duration::from_secs(5)).await;
                continue;
            }
        }                  
    }

    conn.unwrap()
}