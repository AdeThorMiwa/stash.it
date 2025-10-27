use di::{Injectable, ServiceCollection, ServiceProvider, singleton_as_self};
use shared::infrastructure::{mailing::stub_mailer::StubMailer, messaging::memory::InMemoryEventBus};
use std::sync::Arc;

use user::{
    application::{
        authentication::AuthenticationService, mailing::MailingService, session_management::SessionManagementService,
        user_management::UserManagementService,
    },
    infrastructure::{
        auth::jwt_service::JWTService,
        config::{Config, get_config},
        persistence::{
            profile_repository::PostgresProfileRepository, session_repository::PostgresSessionRepository, user_repository::PostgresUserRepository,
        },
    },
};

pub fn bootstrap() -> ServiceProvider {
    let config = Arc::new(get_config().unwrap());

    ServiceCollection::new()
        .add(singleton_as_self::<Config>().from(move |_| config.clone()))
        .add(AuthenticationService::singleton())
        .add(MailingService::singleton())
        .add(SessionManagementService::singleton())
        .add(UserManagementService::singleton())
        .add(JWTService::singleton())
        .add(PostgresUserRepository::singleton())
        .add(PostgresSessionRepository::singleton())
        .add(PostgresProfileRepository::singleton())
        .add(InMemoryEventBus::singleton())
        .add(StubMailer::singleton())
        .build_provider()
        .unwrap()
}
