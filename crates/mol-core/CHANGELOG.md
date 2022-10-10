# mol-core

## 0.4.0

### Minor Changes

- feat: add validate_package function to PackageManager trait

- feat: move internal mol-cargo functions to a mol-core struct

- chore: update function names and use thiserror

- feat: add metadata type to PackageManager trait to allow passing of context for commands

### Patch Changes

- chore: remove code to focus on core functionality

- chore: update dependencies to latest releases

## 0.3.0

### Minor Changes

- feat: api changes and renames for Version -> VersionMod and VersionValue -> Version

- feat: add waiting for package update

- feat: create ToBox trait and auto derive

- feat: plugin api

### Patch Changes

- chore: add more error contexts

- fix: add root_path to pacakge path before reading

## 0.2.0

### Minor Changes

- feat: add VersionValue type and updated package accordingly

- feat: add initial publish command

- feat: add package definition + dependecy graphing api

- feat: support dependency bumping and running build post version

- feat: initialize changesets directory with README.md file

### Patch Changes

- feat: cleanup changelog code by splitting to traits and self explaining functions

## 0.1.0

### Minor Changes

- Inital
