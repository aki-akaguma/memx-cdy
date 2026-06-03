# Changelog: memx-cdy
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-09-24
### Changed
- Dependency `memx` to 0.2.0.
- Minimum Supported Rust Version (MSRV) to 1.65.0.

## [0.1.13] - 2023-10-09
### Changed
- Bypass `memx` in `memcmp` for lengths less than 4.

### Fixed
- `stack overflow` on initial `memcmp` call with length 3.

## [0.1.12] - 2023-02-13
### Changed
- `Makefile` structure and organization.

### Removed
- `COPYING` file.

### Fixed
- Licensing information in `LICENSE-APACHE` and `LICENSE-MIT`.

## [0.1.11] - 2023-01-31
### Added
- GitHub Actions workflows for Ubuntu, macOS, and Windows.
- Test status badges to `README.tpl`.
- Minimum Supported Rust Version (MSRV) of 1.56.0 to `Cargo.toml`.

## [0.1.10] - 2023-01-10
### Added
- Status badges to `README.tpl`.
- Documentation link to `memx`.

### Changed
- `CHANGELOG.md` formatting for better readability.

## [0.1.9] - 2023-01-05
### Changed
- Dependency `memx` to 0.1.23.

### Fixed
- Clippy warning regarding unit value let-bindings.
- Clippy warning regarding `0 as *mut _`.

## [0.1.8] - 2022-06-13
### Changed
- Project to Rust 2021 edition.

## [0.1.7] - 2021-11-14
### Changed
- Dependency `memx` to 0.1.18.

## [0.1.6] - 2021-09-11
### Added
- `no_memcpy` feature to address buggy `memcpy()` in newer Rust versions.

### Changed
- Default features to include `no_memcpy`.

## [0.1.5] - 2021-09-10
### Changed
- Dependencies `mem` to 0.1.16, `libc` to 0.2.101, and `semver` to 1.0.4.

## [0.1.4] - 2021-07-06
### Changed
- Documentation for clarity and completeness.
- License files.
- Dependency `memx` to 0.1.14.

## [0.1.3] - 2021-06-24
### Added
- `memx_init()` function for simplified linking.

### Fixed
- Duplicate symbol `memcpy` error when targeting `musl` by adding `#[cfg(not(target_env = "musl"))]`.

## [0.1.2] - 2021-06-23
### Changed
- Dependency `memx` to 0.1.13.

## [0.1.1] - 2021-06-20
### Added
- `staticlib` and `rlib` to crate types.

### Changed
- Dependency `memx` to 0.1.12, including critical bug fixes.

## [0.1.0] - 2021-06-08
### Added
- Initial release.

[Unreleased]: https://github.com/aki-akaguma/memx-cdy/compare/v0.2.0..HEAD
[0.2.0]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.13..v0.2.0
[0.1.13]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.12..v0.1.13
[0.1.12]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.11..v0.1.12
[0.1.11]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.10..v0.1.11
[0.1.10]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.9..v0.1.10
[0.1.9]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.8..v0.1.9
[0.1.8]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.7..v0.1.8
[0.1.7]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.6..v0.1.7
[0.1.6]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/memx-cdy/releases/tag/v0.1.0
