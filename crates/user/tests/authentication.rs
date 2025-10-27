use crate::common::{bootstrap::bootstrap, insta_filters::redactions::cleanup_model_generics, string_utils::extract_otp};
use insta::{assert_debug_snapshot, with_settings};
use shared::{
    domain::value_objects::pid::Pid,
    infrastructure::{mailing::Mailer, types::Result},
};
use std::{str::FromStr, sync::Arc};
use user::{
    application::{authentication::AuthenticationService, user_management::UserManagementService},
    domain::value_objects::email::EmailAddress,
    infrastructure::auth::jwt_service::JWTService,
};

mod common;

#[tokio::test]
async fn can_request_authentication_code() -> Result<()> {
    // Arrange
    configure_insta!();
    let provider = bootstrap();
    let mailer: Arc<dyn Mailer> = provider.get_required();
    let authentication_service: Arc<AuthenticationService> = provider.get_required();
    let email = EmailAddress::from_str("tom@stash.it").unwrap();

    // Act
    let result = authentication_service.request_authentication_code(&email).await;
    let deliveries = mailer.deliveries().await;
    let code = extract_otp(&deliveries.messages.first().unwrap()).unwrap();

    // Assert
    assert!(result.is_ok(), "Request authentication code should succeed");
    assert_eq!(deliveries.count, 1, "Mailer should have delivered 1 email");
    assert_eq!(&code, "112358");
    Ok(())
}

#[tokio::test]
async fn can_authenticate() -> Result<()> {
    // Arrange
    configure_insta!();
    let provider = bootstrap();
    let mailer: Arc<dyn Mailer> = provider.get_required();
    let authentication_service: Arc<AuthenticationService> = provider.get_required();
    let email = EmailAddress::from_str("test@stash.it").unwrap();

    // Act
    let session_id = authentication_service.request_authentication_code(&email).await?;
    let deliveries = mailer.deliveries().await;
    let code = extract_otp(&deliveries.messages.first().unwrap()).unwrap();
    let token = authentication_service.authenticate(&session_id, &code).await?;

    let jwt_service: Arc<JWTService> = provider.get_required();
    let claims = jwt_service.decode_token(&token)?;
    let pid = Pid::from_str(claims.sub.as_str()).unwrap();

    let user_service: Arc<UserManagementService> = provider.get_required();
    let user = user_service.get_user_by_pid(&pid).await?.unwrap();

    with_settings!({
        filters => cleanup_model_generics(),
        snapshot_suffix => "authenticated_user"
    }, {
        assert_debug_snapshot!(user);
    });

    Ok(())
}
