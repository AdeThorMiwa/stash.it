use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum DomainError {
    #[error("Entity not found")]
    EntityNotFound,
    #[error("Entity already exist")]
    EntityAlreadyExist,
    #[error("Invalid entity")]
    EntityInvalid,
}

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Domain error: {0}")]
    DomainError(DomainError),
    #[error("Service error")]
    ServiceError,
    #[error("Assert error: {0}")]
    AssertError(String),
    #[error("Builder error: {0}")]
    BuilderError(String),
    #[error("Parse error: ")]
    ParseError,
}
