# mol

## 0.4.0

### Minor Changes

- chore: update function names and use thiserror

- feat: move internal mol-cargo functions to a mol-core struct

### Patch Changes

- chore: update dependencies to latest releases

- chore: import README.md to mol docs

- chore: remove code to focus on core functionality

- chore: user friendly message when no packages were found

## 0.3.0

### Minor Changes

- feat: api changes and renames for Version -> VersionMod and VersionValue -> Version

- feat: add waiting for package update

### Patch Changes

- fix: add root_path to pacakge path before reading

- chore: add more error contexts

## 0.2.0

### Minor Changes

- feat: initialize changesets directory with README.md file

- feat: add initial publish command

- feat: add package definition + dependecy graphing api

- feat: support dependency bumping and running build post version

- feat: add VersionValue type and updated package accordingly

### Patch Changes

- chore: join changesets into context

- bug: fix dependency version bumping due to rewrite error

- fix: move changesets deletion to end of version command

- bug: partial support for dry_run on init

- fix: remove warning on init

## 0.1.0

### Minor Changes

- Inital
