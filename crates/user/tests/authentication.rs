use crate::common::{bootstrap::bootstrap, prepare::prepare_authenticated_user, string_utils::extract_otp};
use insta::{assert_debug_snapshot, with_settings};
use shared::{
    configure_insta,
    infrastructure::{mailing::Mailer, types::Result},
    testing::insta_filters::redactions::cleanup_model_generics,
};
use std::{str::FromStr, sync::Arc};
use user::{
    application::{auth::AuthenticationService, user::UserManagementService},
    domain::value_objects::email::EmailAddress,
};

mod common;

#[tokio::test]
async fn can_create_auth_session() -> Result<()> {
    // Arrange
    configure_insta!();
    let provider = bootstrap();
    let mailer: Arc<dyn Mailer> = provider.get_required();
    let authentication_service: Arc<AuthenticationService> = provider.get_required();
    let email = EmailAddress::from_str("tom@stash.it").unwrap();

    // Act
    let result = authentication_service.create_new_session(&email).await;
    let deliveries = mailer.deliveries().await;
    let code = extract_otp(&deliveries.messages.first().unwrap()).unwrap();

    // Assert
    assert!(result.is_ok(), "Request authentication code should succeed");
    assert_eq!(deliveries.count, 1, "Mailer should have delivered 1 email");
    assert_eq!(&code, "112358");
    Ok(())
}

#[tokio::test]
async fn can_activate_auth_session() -> Result<()> {
    // Arrange
    configure_insta!();
    let provider = bootstrap();

    let (pid, ..) = prepare_authenticated_user(&provider).await?;

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

#[tokio::test]
async fn can_terminate_auth_session() -> Result<()> {
    // Arrange
    configure_insta!();
    let provider = bootstrap();
    let authentication_service: Arc<AuthenticationService> = provider.get_required();

    let (_, session_id) = prepare_authenticated_user(&provider).await?;

    let is_valid_session = authentication_service.is_valid_session(&session_id).await?;

    assert_eq!(is_valid_session, true);

    authentication_service.terminate_session(&session_id).await?;
    let is_valid_session = authentication_service.is_valid_session(&session_id).await?;

    assert_eq!(is_valid_session, false);

    Ok(())
}
