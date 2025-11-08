use crate::common::{bootstrap::bootstrap, prepare::prepare_authenticated_user};
use insta::{assert_debug_snapshot, with_settings};
use shared::{
    configure_insta, domain::value_objects::wallet_address::WalletAddress, infrastructure::types::Result,
    testing::insta_filters::redactions::cleanup_model_generics,
};
use std::{str::FromStr, sync::Arc};
use user::{
    application::user::{UserManagementService, command::CreateUserProfileCommand},
    domain::value_objects::display_name::DisplayName,
};

mod common;

#[tokio::test]
async fn can_create_user_profile() -> Result<()> {
    // Arrange
    configure_insta!();
    let provider = bootstrap();
    let user_service: Arc<UserManagementService> = provider.get_required();
    let (pid, ..) = prepare_authenticated_user(&provider).await?;
    let create_profile_command = CreateUserProfileCommand {
        user_id: pid.clone(),
        display_name: DisplayName::from_str("testuser").unwrap(),
        wallet_address: WalletAddress::from_str("0x52471a768b76B8cC647f2F28198cB0E44C38C2cF").unwrap(),
    };

    // Act
    let profile = user_service.create_user_profile(create_profile_command).await?;

    // Assert
    assert_eq!(profile.get_user_id(), &pid, "User id on profile must match user pid");
    with_settings!({
        filters => cleanup_model_generics(),
        snapshot_suffix => "user_profile"
    }, {
        assert_debug_snapshot!(profile);
    });

    Ok(())
}
