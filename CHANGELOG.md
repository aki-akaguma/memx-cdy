# Changelog: memx-cdy

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] *
### Removed
* `COPYING`

### Fixed
* `LICENSE-APACHE`, `LICENSE-MIT`


## [0.1.11] (2023-01-31)
### Added
* `.github/workflows/test-ubuntu.yml`
* `.github/workflows/test-macos.yml`
* `.github/workflows/test-windows.yml`
* test status badges into `README.tpl`
* `rust-version = "1.56.0"` into `Cargo.toml`

## [0.1.10] (2023-01-10)
### Added
* badges into `README.tpl`
* http link to `memx` on a document

### Changed
* reformat `CHANGELOG.md`

## [0.1.9] (2023-01-05)
### Changed
* update crates: memx(0.1.23)

### Fixed
* clippy: this let-binding has unit value
* clippy: `0 as *mut _` detected

## [0.1.8] (2022-06-13)
### Changed
* change to edition 2021

## [0.1.7] (2021-11-14)
### Changed
* update crates: memx(0.1.18)

## [0.1.6] (2021-09-11)
### Added
* feature = "no_memcpy" cause by buggy memcpy() on new rustc

### Changed
* default_feature has "no_memcpy"

## [0.1.5] (2021-09-10)
### Changed
* update crates: mem(0.1.16), libc(0.2.101), semver(1.0.4)

## [0.1.4] (2021-07-06)
### Changed
* rewrite doc
* update licenses
* update depends: memx(0.1.14)

## [0.1.3] (2021-06-24)
### Added
* add memx_init(): for a easy linking

### Fixed
* bug: "ld: error: duplicate symbol: memcpy" in the case that target is "xxx-musl".
 - for the time being, i add `#[cfg(not(target_env = "musl"))]` to the `memcpy()`.

## [0.1.2] (2021-06-23)
### Changed
* update crates: memx(0.1.13) - brush up

## [0.1.1] (2021-06-20)
### Added
* add "staticlib" and "rlib" to crate-type

### Changed
* update crates: memx(0.1.12) - the important bugs fixed

## [0.1.0] (2021-06-08)
* first commit

[Unreleased]: https://github.com/aki-akaguma/memx-cdy/compare/v0.1.11..HEAD
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
