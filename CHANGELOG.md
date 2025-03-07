# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## v0.3.0

### Changed

- Bumped rustix to 1 and windows-sys to 0.59.

  Now `WaitHandle::wait()` on apple/darwin is alloc-free.

## v0.2.0
