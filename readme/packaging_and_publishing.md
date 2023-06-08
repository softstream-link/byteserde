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

## on branch then master
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

```shell
cargo publish --dry-run
```