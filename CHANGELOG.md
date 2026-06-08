# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.10](https://github.com/near/near-validator-cli-rs/compare/v0.1.9...v0.1.10) - 2026-06-08

### Added

- Added styling options for "newline", "option selection" and "answer" for "inquire" crate ([#35](https://github.com/near/near-validator-cli-rs/pull/35))

### Other

- bump near-* deps to nearcore 2.12 / protocol 84 + fix protocol-config deserialization ([#37](https://github.com/near/near-validator-cli-rs/pull/37))
- upgrade to Rust edition 2024 ([#34](https://github.com/near/near-validator-cli-rs/pull/34))
- migrate to org-wide NEARPROTOCOL_CI_PR_ACCESS token ([#33](https://github.com/near/near-validator-cli-rs/pull/33))

## [0.1.9](https://github.com/near-cli-rs/near-validator-cli-rs/compare/v0.1.8...v0.1.9) - 2026-01-09

### Added

- Enabled aarch64 Windows binary builds (upgraded cargo-dist to 0.29.0) ([#30](https://github.com/near-cli-rs/near-validator-cli-rs/pull/30))

### Other

- Updated near-cli-rs crate to 0.23.5 release ([#32](https://github.com/near-cli-rs/near-validator-cli-rs/pull/32))

## [0.1.8](https://github.com/near-cli-rs/near-validator-cli-rs/compare/v0.1.7...v0.1.8) - 2025-08-31

### Added

- Added self update for near-validator ([#27](https://github.com/near-cli-rs/near-validator-cli-rs/pull/27))

### Other

- update near-* dependencies to 0.31 release ([#28](https://github.com/near-cli-rs/near-validator-cli-rs/pull/28))

## [0.1.7](https://github.com/near-cli-rs/near-validator-cli-rs/compare/v0.1.6...v0.1.7) - 2025-07-08

### Fixed

- Use RpcConfigError error type instead of () for ProtocolConfig requests ([#26](https://github.com/near-cli-rs/near-validator-cli-rs/pull/26))
- Fixed error RpcError: [missing field disable_9393_fix]  ([#22](https://github.com/near-cli-rs/near-validator-cli-rs/pull/22))

### Other

- Replaced ubuntu-20.04, being browned out ([#25](https://github.com/near-cli-rs/near-validator-cli-rs/pull/25))

## [0.1.6](https://github.com/near-cli-rs/near-validator-cli-rs/compare/v0.1.5...v0.1.6) - 2025-03-29

### Added

- Upgraded cargo-dist to 0.28.0 ([#21](https://github.com/near-cli-rs/near-validator-cli-rs/pull/21))

### Other

- Replaced integer casts with explicit From trait calls

## [0.1.5](https://github.com/near-cli-rs/near-validator-cli-rs/compare/v0.1.4...v0.1.5) - 2025-03-27

### Other

- Updated near-cli-rs crate to 0.19 release ([#18](https://github.com/near-cli-rs/near-validator-cli-rs/pull/18))

## [0.1.4](https://github.com/near-cli-rs/near-validator-cli-rs/compare/v0.1.3...v0.1.4) - 2024-08-21

### Added
- Use whole NEAR amounts when displaying stake in the tables since the <0.1 NEAR is irrelevant for the users ([#16](https://github.com/near-cli-rs/near-validator-cli-rs/pull/16))

## [0.1.3](https://github.com/near-cli-rs/near-validator-cli-rs/compare/v0.1.2...v0.1.3) - 2024-08-21

### Added
- Added "Endorsements" columns to the output and updated the formula for online % calculation ([#15](https://github.com/near-cli-rs/near-validator-cli-rs/pull/15))

### Fixed
- Fixed a syntax error in CI (publish-to-npm.yml)

### Other
- Updated near-cli-rs crate to 0.14 release ([#14](https://github.com/near-cli-rs/near-validator-cli-rs/pull/14))
- Added installation instructions to the README

## [0.1.2](https://github.com/near-cli-rs/near-validator-cli-rs/compare/v0.1.1...v0.1.2) - 2024-02-03

### Other
- Updated binary releases pipeline to use cargo-dist v0.9.0 (previously v0.7.2) ([#8](https://github.com/near-cli-rs/near-validator-cli-rs/pull/8))

## [0.1.1](https://github.com/near-cli-rs/near-validator-cli-rs/compare/v0.1.0...v0.1.1) - 2024-01-27

### Other
- removed explicit rust-toolchain, so let's use the latest stable by default
- Upgraded NEAR crates to 0.20.0 release ([#5](https://github.com/near-cli-rs/near-validator-cli-rs/pull/5))
- fixed code_style.yml to use ubuntu-latest instead of ubuntu-20.04
- removed old CI integrations that are now replaced by cargo-dist (release.yml workflow)
