> This project is still in development and api may change

# Mol

Mol - מו"ל  (Motzi Laor) is a versioning and publishig toolset with much inspiration from [atlassian/changesets](https://github.com/atlassian/changesets) but with no specific coupling to any package mananger or versioning scheme. <br />
Simply manage the act of adding code to the codebase and the version change of the library without requiring commit message formatting nor multiple publishes for changes that can be summed up together

## cargo-mol

Cargo + Semantic Versioning implemintation of Mol bundled with current version of Mol

```bash
cargo install mol
cargo mol --help
```

Most of the api is either cli wizard or very similar to [changesets](https://github.com/atlassian/changesets)

![](./images/add-sample.gif)

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

# Will print out all the changes that would have happend
```

## Roadmap
- Github action recipe
- Plugins
- mol-nodejs (npm/yarn)
- mol-pip?
- mol-maven?
