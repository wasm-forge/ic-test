use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use candid::Principal;
use pocket_ic::{nonblocking::PocketIc, PocketIcBuilder};
use test_principals::TEST_PRINCIPALS;

pub mod caller;
pub mod deployer;
pub mod provider;

pub(crate) mod http_outcalls;
pub(crate) mod test_principals;

pub struct Ic {
    pub pic: Arc<PocketIc>,
}

impl Ic {
    pub async fn new() -> Self {
        std::env::set_var("RUST_LOG", "error");

        let pic = PocketIcBuilder::new()
            .with_nns_subnet()
            .with_ii_subnet()
            .build_async()
            .await;

        pic.set_time(
            SystemTime::UNIX_EPOCH
                .checked_add(Duration::from_secs(1740000000))
                .unwrap(),
        )
        .await;

        Self { pic: Arc::new(pic) }
    }

    pub fn test_user_count(&self) -> usize {
        TEST_PRINCIPALS.len()
    }

    pub fn test_user(&self, index: usize) -> IcUser {
        if index >= self.test_user_count() {
            panic!(
                "Reached maximum number of test users: {}",
                self.test_user_count()
            );
        }
        self.user_from(Principal::from_text(TEST_PRINCIPALS[index]).unwrap())
    }

    pub fn default_user(&self) -> IcUser {
        self.test_user(0)
    }

    pub fn user_from(&self, principal: Principal) -> IcUser {
        IcUser {
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

#[derive(Clone)]
pub struct IcUser {
    pub principal: Principal,
    pic: Arc<PocketIc>,
}
