# This module contains a number of training examples and bench mark comparisons

## Training Examples
* follow [here](./examples/readme.md)

## Bench Marks

* These benchmarks are a comparison of two other popular formats, which are [serde_json](https://crates.io/crates/serde_json) and [rmp-serde](https://crates.io/crates/rmp-serde), with [byteserde](https://crates.io/crates/byteserde)
* To make the results of the comparison fare the following [reference](./benches/sample.rs) structure was used to produce below measurement.

### Results
  
### `byteserde` - takes about ~`15ns` to read or write to a byte stream

![alt text](readme/byteserde_bench.png)

### `bincode` - takes about ~`100ns` to write and ~`15ns` to read from a byte stream

![alt text](readme/byteserde_bench.png)

### `rmp-serde` - takes about ~`215ns` to read or write to a byte stream, which is about `seven` times slower, likely due to the `serde` interface overhead

![alt text](readme/serde_rmp_bench.png)

### `serde_json` - takes about ~`600ns` to read or write to a byte stream, which is about `twenty` times slower, undestandably due to its reliance on a string result.

![alt text](readme/serde_json_bench.png)

### To try your self run.
* `byteserde`
```shell
cargo bench --bench byteserde_bench
```
* `rmp-serde`
```shell
cargo bench --bench serde_rmp_bench
```
* `serde_json`
```shell
cargo bench --bench serde_json_bench
```