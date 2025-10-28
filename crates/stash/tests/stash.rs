use shared::{configure_insta, infrastructure::types::Result};

use crate::utils::bootstrap::bootstrap;

mod utils;

#[tokio::test]
async fn can_create_stash() -> Result<()> {
    // Arrange
    configure_insta!();
    let provider = bootstrap();
    Ok(())
}
