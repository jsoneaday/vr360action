use crate::repo::base::Repository;

pub struct AppState<T: Repository> {
    pub repo: T,
}