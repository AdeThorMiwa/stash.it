#[derive(Debug, Clone)]
pub enum DomainError {
    EntityNotFound,
    EntityAlreadyExist,
    EntityInvalid,
}

#[derive(Debug, Clone)]
pub enum Error {
    DomainError(DomainError),
}
