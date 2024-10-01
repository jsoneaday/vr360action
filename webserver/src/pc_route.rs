use actix_web::web::Data;
use sqlx::{Pool, Postgres};
use crate::{app_state::AppState, repo::{base::{ConnGetter, DbRepo, Repository}, pc::{query_all, Pc}}, user_error::UserError};


pub async fn get_all_pc_info<T: Repository + ConnGetter>(
    app_data: Data<AppState<T>>
) -> Result<Vec<Pc>, UserError> {
    let conn: &Pool<Postgres> = app_data.repo.get_conn();
    let result = query_all(conn).await;

    match result {
        Ok(result) => {
        
            Ok(result)
        },
        Err(e) => Err(e.into())
    }
}