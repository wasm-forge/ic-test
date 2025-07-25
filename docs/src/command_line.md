# Command line options



## Tool usage

```bash
ic-test <COMMAND> [OPTIONS]
```

Without arguments it starts in interactive mode to create a new test project. If an `ic-test.json` config file exists already, the `update` mode will regenerate the existing test project bindings.

### Create a new test project

```bash
ic-test new tests
```

- Creates a new test project in the `tests` folder.
- Looks for canisters and contracts, generates API bindings and a sample test.
- Generates an `ic-test.json` configuration file.
- Fails if the `tests` folder already exists, the user would need to choose a different name.


### Update/regenerate an existing test project

```bash
ic-test update
```

Regenerates bindings using the configuration in `ic-test.json`.

