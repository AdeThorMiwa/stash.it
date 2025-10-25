#[derive(Debug, Clone)]
pub enum DomainError {
    EntityNotFound,
}

#[derive(Debug, Clone)]
pub enum Error {
    DomainError(DomainError),
}
