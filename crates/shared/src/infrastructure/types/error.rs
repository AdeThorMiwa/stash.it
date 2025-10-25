#[derive(Debug, Clone)]
pub enum DomainError {
    EntityNotFound,
    EntityAlreadyExist,
}

#[derive(Debug, Clone)]
pub enum Error {
    DomainError(DomainError),
}
