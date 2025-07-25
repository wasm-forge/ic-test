
# "Counter" tutorial

Let's create a simple project and test it using the `ic-test`.

*Create a "Hello, World!" canister:*

```bash
dfx new hello-ic-test --type rust --no-frontend
```

*Compile the project:*

```bash
dfx start --clean --background

dfx canister create --all

dfx build
```

*Generate test bindings*

If there are uncommitted changes, either commit them before generating or use the `--force` flag:

```bash
ic-test new tests --force
```

This creates a tests package with:

* Canister API bindings in `tests/src/bindings`
* Test environment setup logic in `test_setup.rs`
* A test template in `tests.rs`


### Example test

*Edit `tests.rs`:*

```rust,ignore
use ic_test::IcpTest;

use crate::test_setup;

#[tokio::test]
async fn test_greet() {
    let test_setup::Env {
        icp_test,
        hello_ic_test_backend,
    } = test_setup::setup(IcpTest::new().await).await;

    let result = hello_ic_test_backend
        .greet("ic-test".to_string())
        .call()
        .await;

    assert_eq!(result, "Hello, ic-test!");
}
```

*Run tests:*

```bash
cargo test
```

### Adding a counter

*Update the canister backend:*

```rust
use std::cell::RefCell;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(Clone, Default)]
struct CounterState {
    value: u64,
    increment: u64,
}

thread_local! {
    static STATE: RefCell<CounterState> = RefCell::new(CounterState::default());
}

#[ic_cdk::init]
fn init(init_value: u64, increment: u64) {
    STATE.with(|state| {
        *state.borrow_mut() = CounterState {
            value: init_value,
            increment,
        };
    });
}

#[ic_cdk::update]
fn increment_counter() {
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.value += s.increment;
    });
}

#[ic_cdk::query]
fn get_counter() -> u64 {
    STATE.with(|state| state.borrow().value)
}
```

*Update Candid file `hello-ic-test-backend.did`:*

```candid
service : (nat64, nat64) -> {
  "greet": (text) -> (text) query;
  "get_counter": () -> (nat64) query;
  "increment_counter": () -> ();
}
```

*Set initialization arguments in `dfx.json`:*

```json
{
  "canisters": {
    "hello-ic-test-backend": {
      "candid": "src/hello-ic-test-backend/hello-ic-test-backend.did",
      "package": "hello-ic-test-backend",
      "type": "rust",
      "init_arg": "(50, 73)"
    }
  },
  "defaults": {
    "build": {
      "args": "",
      "packtool": ""
    }
  },
  "output_env_file": ".env",
  "version": 1
}
```

*Regenerate the bindings:*

```bash
dfx build

ic-test
```

The `ic-test` will enter interactive mode and prompt user to allow overwriting the `test_setup.rs` file. Upon confirmation the the `test_setup.rs` is regenerated with the initialization parameters:

```rust,ignore
//...
    let hello_ic_test_backend = hello_ic_test_backend::deploy(&icp_user, 50, 73)
        .call()
        .await;

// ...

```

### New test

*Add a new test in `tests.rs`:*

```rust
#[tokio::test]
async fn test_counter() {
    let test_setup::Env {
        icp_test,
        hello_ic_test_backend,
    } = test_setup::setup(IcpTest::new().await).await;

    let result = hello_ic_test_backend.get_counter().call().await;

    assert_eq!(result, 50u64);

    hello_ic_test_backend.increment_counter().call().await;

    let result = hello_ic_test_backend.get_counter().call().await;

    assert_eq!(result, 123u64); // 50 + 73
}
```

*Run tests:*
```bash
cargo test
```
