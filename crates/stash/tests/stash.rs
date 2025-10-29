use insta::{assert_debug_snapshot, with_settings};
use shared::{
    configure_insta,
    domain::value_objects::{asset::Asset, mula::Mula, pid::Pid},
    infrastructure::types::Result,
    testing::insta_filters::redactions::cleanup_model_generics,
};
use stash::{
    application::stash::{
        StashService,
        command::{CreateStashCommand, UpdateStashBalanceCommand, UpdateStashStatusCommand},
    },
    domain::stash::{name::StashName, status::StashStatus, tag::Tag},
};
use std::str::FromStr;

use crate::utils::{bootstrap::bootstrap, prepare::prepare_stash};

mod utils;

#[tokio::test]
async fn can_create_stash() -> Result<()> {
    // Arrange
    configure_insta!();
    let provider = bootstrap();
    let stash_service = provider.get_required::<StashService>();
    let command = CreateStashCommand {
        name: StashName::from_str("General").unwrap(),
        user_id: Pid::new(),
        tags: vec![Tag::from_str("personal").unwrap()],
    };

    // Act
    let stash = stash_service.create_stash(command).await?;

    // Assert
    with_settings!({
        filters => cleanup_model_generics(), snapshot_suffix => "create_stash"
    }, {
        assert_debug_snapshot!(stash)
    });

    Ok(())
}

#[tokio::test]
async fn can_update_stash_status() -> Result<()> {
    // Arrange
    let provider = bootstrap();
    let stash_service = provider.get_required::<StashService>();
    let stash = prepare_stash(&provider).await?;
    let command = UpdateStashStatusCommand {
        stash_id: stash.get_pid().to_owned(),
        new_status: StashStatus::CLOSED,
    };

    // Act
    let stash = stash_service.update_stash_status(command).await?;

    // Assert
    assert_eq!(stash.get_status(), &StashStatus::CLOSED, "Stash status must be `CLOSED`");

    Ok(())
}

#[tokio::test]
#[cfg(feature = "testing")]
async fn can_update_stash_balance() -> Result<()> {
    // Arrange
    let provider = bootstrap();
    let stash_service = provider.get_required::<StashService>();
    let stash = prepare_stash(&provider).await?;
    let new_balance = Mula::new(10, &Asset::usdt());
    let command = UpdateStashBalanceCommand {
        stash_id: stash.get_pid().to_owned(),
        new_balance: new_balance.clone(),
    };

    // Act
    let stash = stash_service.update_stash_balance(command).await?;
    let balance = stash.get_balances().iter().find(|b| b.get_asset() == new_balance.get_asset()).unwrap();

    // Assert
    assert_eq!(balance, &new_balance, "Balance must equal to new balance");

    Ok(())
}
