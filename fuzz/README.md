# Fuzzing

Fuzz testing is enabled by [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).


# Idempotent

The formatter aims to produce output that is idempotent. In most common cases it does, but there are
inputs where it fails. The `idempotent` fuzz target has been invaluable in discovering and fixing
these issues.

You can run the `idempotent` fuzz target as follows:

```
cargo fuzz run idempotent -- -only_ascii=1 -max_total_time=30 -max_len=30
```
