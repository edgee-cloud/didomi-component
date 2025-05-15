<div align="center">
<p align="center">
  <a href="https://www.edgee.cloud">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://cdn.edgee.cloud/img/component-dark.svg">
      <img src="https://cdn.edgee.cloud/img/component.svg" height="100" alt="Edgee">
    </picture>
  </a>
</p>
</div>

<h1 align="center">Didomi component for Edgee</h1>

[![Coverage Status](https://coveralls.io/repos/github/edgee-cloud/didomi-component/badge.svg)](https://coveralls.io/github/edgee-cloud/didomi-component)
[![GitHub issues](https://img.shields.io/github/issues/edgee-cloud/didomi-component.svg)](https://github.com/edgee-cloud/didomi-component/issues)
[![Edgee Component Registry](https://img.shields.io/badge/Edgee_Component_Registry-Public-green.svg)](https://www.edgee.cloud/edgee/didomi-consent-mapping)

This component enables seamless integration between [Edgee](https://www.edgee.cloud) and [Didomi](https://www.didomi.io/consent-management-platform), allowing you to use Didomi as the Consent Management Platform on Edgee.

## Quick Start

1. Download the latest component version from our [releases page](../../releases)
2. Place the `didomi.wasm` file in your server (e.g., `/var/edgee/components`)
3. Add the following configuration to your `edgee.toml`:

```toml
[[destinations.consent_management]]
id = "didomi"
file = "/var/edgee/components/didomi.wasm"
```

## Development

### Building from Source
Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)
- [Edgee CLI](https://github.com/edgee-cloud/edgee)

Build command:
```bash
edgee components build
```

Test commands:
```bash
edgee components test
cargo test
```

Test coverage command:
```bash
cargo llvm-cov --all-features
```

### Security
Report security vulnerabilities to [security@edgee.cloud](mailto:security@edgee.cloud)
