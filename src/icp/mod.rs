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

pub struct Icp {
    pub pic: Arc<PocketIc>,
}

impl Icp {
    pub async fn new() -> Self {
        let pic = PocketIcBuilder::new()
            .with_nns_subnet()
            .with_ii_subnet()
            .with_log_level(slog::Level::Error)
            .build_async()
            .await;

        let time = Time::from_nanos_since_unix_epoch(1_740_000_000_000_000_000);

        pic.set_time(time).await;

        Self { pic: Arc::new(pic) }
    }

    pub fn test_user_count(&self) -> usize {
        TEST_PRINCIPALS.len()
    }

    pub fn test_user(&self, index: usize) -> IcpUser {
        if index >= self.test_user_count() {
            panic!(
                "Reached maximum number of test users: {}",
                self.test_user_count()
            );
        }
        self.user_from(Principal::from_text(TEST_PRINCIPALS[index]).unwrap())
    }

    pub fn default_user(&self) -> IcpUser {
        self.test_user(0)
    }

    pub fn user_from(&self, principal: Principal) -> IcpUser {
        IcpUser {
            principal,
            pic: Arc::clone(&self.pic),
        }
    }

    pub async fn tick(&self) {
        self.pic.advance_time(Duration::from_secs(1)).await;
        self.pic.tick().await;
    }

    pub fn pocket_ic(&self) -> &PocketIc {
        &self.pic
    }
}
