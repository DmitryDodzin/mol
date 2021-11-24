> This project is still in development and api may change

# Mol

Mol - מו"ל  (Motzi Laor) is a versioning and publishig toolset with much inspiration from [atlasian/changesets](https://github.com/atlassian/changesets) but with no specific coupling to any package mananger or versioning scheme.

## cargo-mol

Cargo + Semantic Versioning implemintation of Mol bundled with current version of Mol

```bash
cargo install mol
cargo mol --help
```

Most of the api is ithere cli wizzard or very similar to [changesets](https://github.com/atlassian/changesets)

### 101 Commands

Create a new changeset to describe the change you are plannig to add
```bash
cargo mol add -v patch -p packge1 -p package2 -m "I did some changes"

# what changed
# + .changeset/lorem_ipsum.md >
#   + ---
#   + packge1: patch
#   + package2: patch
#   + --- 
#   +
#   + I did some changes
#   +
```

Now when you want to update all the versions you requested. Packages will be updated by the most severe version that is requested, ie 0.1.4 + (patch + patch + minor) == 0.2.0
```bash
cargo mol version

# what changed
# - .changeset/lorem_ipsum.md
# - .changeset/lorem_minor.md
#   ~ CHANGELOG.md >
#   ~ # package1
#   ~
#   + ## 0.8.0
#   +
#   + ### Minor Changes
#   +
#   + - I did some minor changes i did
#   +
#   + ### Patch Changes
#   +
#   + - I did some changes
#   +
#   ~
#   ~ ## 0.7.1
```


#### mol --dry-run

You can always preview the changes you are about to add by runnig with --dry-run before the coomand

```bash
cargo mol --dry-run version

# will print out all the changes that would have happend
```
