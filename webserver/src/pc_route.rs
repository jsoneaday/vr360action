use actix_web::web::Data;
use sqlx::{Pool, Postgres};
use crate::{app_state::AppState, repo::{base::{get_conn_pool, ConnGetter, DbRepo, Repository}, pc::{query_all, Pc}}, user_error::UserError};


pub async fn get_all_pc_info<T: Repository + ConnGetter>(
    app_data: Data<AppState<T>>
) -> Result<Vec<Pc>, UserError> {
    let conn = get_conn_pool().await;
    let result = query_all(&conn).await;

    match result {
        Ok(result) => {
        
            Ok(result)
        },
        Err(e) => Err(e.into())
    }
}