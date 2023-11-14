# Local build
```shell
cargo nextest run --all-features
cargo nextest run --examples
cargo test --doc
cargo doc
cargo clippy --all-features -- --deny warnings
```

# Steps
## bench
```shell
cargo bench
```

## test
```shell
cargo test
cargo test --examples
```

## clippy
```shell
cargo clippy
```

## Package - on branch then master
### byteserde
```shell
pushd ./byteserde 
cargo package
popd
```
### byteserde_derive
```shell
pushd ./byteserde_derive 
cargo package
popd
```

### byteserde_types - will potentially fail until byteserde & byteserde_derive are published
```shell
pushd ./byteserde_types 
cargo package
popd
```

## Publish --dry-run
```shell
pushd ./byteserde 
cargo publish --dry-run
popd
```

```shell
pushd ./byteserde_derive 
cargo publish --dry-run
popd
```

```shell
pushd ./byteserde_types 
cargo publish --dry-run
popd
```