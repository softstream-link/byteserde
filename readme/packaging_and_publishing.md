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


# release

## Publish --dry-run
```shell
cargo release version minor #--execute
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