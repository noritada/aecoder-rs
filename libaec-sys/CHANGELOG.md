# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3] - 2025-12-23

- Fix build failures when CMake outputs the `lib64` directory instead of the `lib` directory
  (PR #3 (thanks @P-E-Meunier))

## [0.1.2] - 2025-08-06

- Update libaec to v1.1.4

## [0.1.1] - 2025-08-04

- Link statically on Windows platforms as well
- Correct `repository` and `homepage` fields in the metadata table in Cargo.toml

## [0.1.0] - 2025-08-03

- Initial release
- Based on libaec 1.1.3

[unreleased]: https://github.com/noritada/aecoder-rs/compare/libaec-sys%2F0.1.3...HEAD
[0.1.3]: https://github.com/noritada/aecoder-rs/compare/libaec-sys%2F0.1.2...libaec-sys%2F0.1.3
[0.1.2]: https://github.com/noritada/aecoder-rs/compare/libaec-sys%2F0.1.1...libaec-sys%2F0.1.2
[0.1.1]: https://github.com/noritada/aecoder-rs/compare/libaec-sys%2F0.1.0...libaec-sys%2F0.1.1
[0.1.0]: https://github.com/noritada/aecoder-rs/releases/tag/libaec-sys%2F0.1.0
