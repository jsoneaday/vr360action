use thiserror::Error;

// this may not be used at all later
#[derive(Debug, Error)]
pub enum MyError {
    #[error("General Error")]
    GeneralError,
}