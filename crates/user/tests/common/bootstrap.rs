use di::{Injectable, ServiceCollection, ServiceProvider, singleton_as_self};
use shared::infrastructure::{config::get_config, mailing::stub_mailer::StubMailer, messaging::memory::InMemoryEventBus};
use std::sync::Arc;

use user::{
    application::{auth::AuthenticationService, mailing::MailingService, session::SessionManagementService, user::UserManagementService},
    infrastructure::{
        auth::jwt_service::JWTService,
        config::Config,
        persistence::{
            profile_repository::PostgresProfileRepository, session_repository::PostgresSessionRepository, user_repository::PostgresUserRepository,
        },
    },
};

pub fn bootstrap() -> ServiceProvider {
    let config = Arc::new(get_config::<Config>().unwrap());

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
