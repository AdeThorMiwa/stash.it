use crate::common::string_utils::extract_otp;
use di::ServiceProvider;
use shared::{
    domain::value_objects::pid::Pid,
    infrastructure::{mailing::Mailer, types::Result},
};
use std::{str::FromStr, sync::Arc};
use user::{application::auth::AuthenticationService, domain::value_objects::email::EmailAddress, infrastructure::auth::jwt_service::JWTService};

pub async fn prepare_authenticated_user(provider: &ServiceProvider) -> Result<(Pid, Pid)> {
    let mailer: Arc<dyn Mailer> = provider.get_required();
    let authentication_service: Arc<AuthenticationService> = provider.get_required();
    let email = EmailAddress::from_str("test@stash.it").unwrap();

    // Act
    let session_id = authentication_service.create_new_session(&email).await?;
    let deliveries = mailer.deliveries().await;
    let code = extract_otp(&deliveries.messages.first().unwrap()).unwrap();
    let token = authentication_service.activate_session(&session_id, &code).await?;

    let jwt_service: Arc<JWTService> = provider.get_required();
    let claims = jwt_service.decode_token(&token)?;
    let user_id = Pid::from_str(claims.sub.as_str()).unwrap();
    Ok((user_id, session_id))
}
