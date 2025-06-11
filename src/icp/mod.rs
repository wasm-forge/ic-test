use std::{sync::Arc, time::Duration};

use candid::Principal;
use pocket_ic::{nonblocking::PocketIc, PocketIcBuilder, Time};
use test_principals::TEST_PRINCIPALS;
use user::IcpUser;

pub mod caller;
pub mod deployer;
pub mod provider;
pub mod user;

#[cfg(feature = "evm")]
pub(crate) mod http_outcalls;

pub(crate) mod test_principals;

/// A local Internet Computer environment based on `PocketIc`.
pub struct Icp {
    /// Shared reference to the `PocketIc` instance.
    pub pic: Arc<PocketIc>,
}

impl Icp {
    /// Create a new `Icp` environment.
    pub async fn new() -> Self {
        let pic = PocketIcBuilder::new()
            .with_nns_subnet()
            .with_ii_subnet()
            .with_log_level(slog::Level::Error)
            .build_async()
            .await;

        // Set the starting time to a fixed value for determinism
        let time = Time::from_nanos_since_unix_epoch(1_740_000_000_000_000_000);

        pic.set_time(time).await;

        Self { pic: Arc::new(pic) }
    }

    /// The total number of predefined test users available.
    pub fn test_user_count(&self) -> usize {
        TEST_PRINCIPALS.len()
    }

    /// Get a test user by index.
    pub fn test_user(&self, index: usize) -> IcpUser {
        if index >= self.test_user_count() {
            panic!(
                "Reached maximum number of test users: {}",
                self.test_user_count()
            );
        }
        self.user_from(Principal::from_text(TEST_PRINCIPALS[index]).unwrap())
    }

    /// Return the default test user (index 0).
    pub fn default_user(&self) -> IcpUser {
        self.test_user(0)
    }

    /// Construct an `IcpUser` from a given principal.
    pub fn user_from(&self, principal: Principal) -> IcpUser {
        IcpUser {
            principal,
            pic: Arc::clone(&self.pic),
        }
    }

    /// Advance simulated time by 1 second and tick the IC.
    pub async fn tick(&self) {
        self.pic.advance_time(Duration::from_secs(1)).await;
        self.pic.tick().await;
    }

    /// Returns a reference to the underlying `PocketIc` instance.
    pub fn pocket_ic(&self) -> &PocketIc {
        &self.pic
    }
}
