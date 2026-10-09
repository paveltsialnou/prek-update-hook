# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0]

### Added

- `--exclude-repo-tag <REPO>=<PATTERN>` option to ignore tags matching a glob for a given
  repository. May be specified multiple times. Translates to prek's `--repo-exclude-tag`.

### Changed

- A missing `prek` binary on `PATH` now produces a clear error with an install link
  instead of a generic spawn failure.

### Fixed

- The MSRV (Rust 1.70) CI job can now read `Cargo.lock` (lockfile version 3).

## [0.4.1]

### Fixed

- Nonzero `prek` exit codes other than 1 (e.g. a missing or malformed config) are now
  reported as errors (exit 2) instead of being mistaken for available updates (exit 1).

## [0.4.0]

### Added

- `--version` / `-V` flag to print the tool version.

### Changed

- Signal-killed child processes now return an error instead of `UpdateNeeded`.

### Fixed

- `--exclude-repo=` with empty value now returns an error instead of silently accepting it.

## [0.3.0]

### Added

- `--repo-tag <REPO>=<PATTERN>` option to filter which tags are considered for a given
  repository. May be specified multiple times. Translates to prek's `--repo-include-tag`.

## [0.2.0]

### Added

- `--exclude-repo=<REPO>` option to skip specific repositories during `--fix`. May be
  specified multiple times.

## [0.1.0]

### Added

- `prek-update` hook that checks whether pinned hook repositories in the prek
  config are up to date via `prek update --check`.
- `--fix` flag to apply available updates via `prek update`.
