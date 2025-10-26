use di::{Injectable, ServiceCollection, ServiceProvider, singleton_as_self};
use regex::Regex;
use shared::{
    domain::value_objects::pid::Pid,
    infrastructure::{
        mailing::{Mailer, stub_mailer::StubMailer},
        messaging::memory::InMemoryEventBus,
    },
};
use std::{str::FromStr, sync::Arc};
use user::{
    application::{
        authentication::AuthenticationService, mailing::MailingService, session_management::SessionManagementService,
        user_management::UserManagementService,
    },
    domain::value_objects::{email::EmailAddress, user_status::UserStatus},
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

pub fn extract_otp(input: &str) -> Option<String> {
    let re = Regex::new(r"(?i)otp(?:\s*code)?\D*(\d{6})").unwrap();

    re.captures(input).and_then(|caps| caps.get(1)).map(|m| m.as_str().to_string())
}

#[tokio::test]
async fn can_login() {
    println!("hahahah");
    let provider = bootstrap();
    let authentication_service: Arc<AuthenticationService> = provider.get_required();
    let email = EmailAddress::from_str("tom@stash.it").unwrap();
    let session_id = authentication_service.request_authentication_code(&email).await.unwrap();

    let mailer: Arc<dyn Mailer> = provider.get_required();
    let deliveries = mailer.deliveries().await;
    assert_eq!(deliveries.count, 1);

    let code = extract_otp(&deliveries.messages.first().unwrap()).unwrap();

    let token = authentication_service.authenticate(&session_id, &code).await.unwrap();

    println!("token: {}", token);
    let jwt_service: Arc<JWTService> = provider.get_required();
    let claims = jwt_service.decode_token(&token).unwrap();
    let pid = Pid::from_str(claims.sub.as_str()).unwrap();

    let user_service: Arc<UserManagementService> = provider.get_required();
    let user = user_service.get_user_by_pid(&pid).await.unwrap().unwrap();

    assert_eq!(user.get_email(), &email);
    assert_eq!(user.get_status(), &UserStatus::PendingProfile);
}
