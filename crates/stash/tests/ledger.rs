use crate::utils::bootstrap::bootstrap;
use insta::{assert_debug_snapshot, with_settings};
use shared::{
    configure_insta,
    domain::value_objects::{asset::Asset, mula::Mula, pid::Pid},
    infrastructure::types::Result,
    testing::insta_filters::redactions::cleanup_model_generics,
};
use stash::{
    application::ledger::{LedgerService, command::WriteLedgerEntryCommand},
    domain::ledger_entry::entry_type::LedgerEntryType,
};

mod utils;

#[tokio::test]
async fn can_write_ledger_entry() -> Result<()> {
    // Arrange
    configure_insta!();
    let provider = bootstrap();
    let ledger_service = provider.get_required::<LedgerService>();
    let stash_id = Pid::new();
    let amount = Mula::new(10, &Asset::usdt());
    let upstream_ref_id = Pid::new();
    let command = WriteLedgerEntryCommand {
        stash_id: stash_id.clone(),
        amount: amount.clone(),
        entry_type: LedgerEntryType::CREDIT,
        upstream_ref_id: upstream_ref_id.clone(),
    };

    // Act
    let entry = ledger_service.write_ledger_entry(command).await?;

    // Assert
    assert_eq!(entry.get_stash_id(), &stash_id, "stash id must match");
    assert_eq!(entry.get_type(), &LedgerEntryType::CREDIT, "entry type must be `CREDIT`");
    assert_eq!(entry.get_amount(), &amount, "amount must match");
    assert_eq!(entry.get_upstream_ref_id(), &upstream_ref_id, "upstream_ref_id must match");

    with_settings!({
        filters => cleanup_model_generics(), snapshot_suffix => "write_entry"
    }, {
        assert_debug_snapshot!(entry)
    });

    Ok(())
}
