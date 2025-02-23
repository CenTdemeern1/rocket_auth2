# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- changelog
- can list all existing users

### Fixed

- 'bson' made mandatory, since it is used in all backends

### Changed

- redis warnings fixed and crate updated

### Removed

- 'fehler' dependency

## [0.6.1] - 2025-02-22

### Added

- User roles as set of strings. AdminUser will check for role 'admin'.

### Fixed

- Sled backend won't allow duplicate users.

## [0.5.1] - 2025-02-17

### Added

- Sled backend

### Fixed

- Dependencies updated


