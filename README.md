# Playwright Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/playwright)](https://pkg.fluentci.io/playwright)
[![ci](https://github.com/fluentci-io/playwright-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/playwright-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with [Playwright](https://playwright.dev/).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm playwright install
fluentci run --wasm playwright test
```

## Functions

| Name    | Description                                                     |
| ------- | --------------------------------------------------------------- |
| install | ensure browsers necessary for this version of Playwright are installed |
| install_deps | install dependencies necessary to run browsers (will ask for sudo permissions) |
| screenshot | capture a page screenshot                                    |
| pdf        | save page as pdf                                             |
| test       | run tests with Playwright Test                               |
| help       | display help for command                                             |
| version    | output the version number                             |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/playwright@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: playwright
    args: |
      install
- name: Run Playwright tests
  run: |
    fluentci run --wasm playwright test
```
