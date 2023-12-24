# bench
```shell
cargo bench
```

# local build
```shell
cargo nextest run --all-features ; \
cargo nextest run --examples --all-features ; \
cargo test --doc --all-features ; \
cargo doc --all-features ; \
cargo clippy --all-features -- --deny warnings
```


# byteserde_types - will potentially fail until byteserde & byteserde_derive are published

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