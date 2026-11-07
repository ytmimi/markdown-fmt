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

Once a failure has been found you can choose to minimize it with

```
cargo fuzz tmin idempotent /path/to/crash/file
```

Whether or not you've chosen to minimize the idempotent input, you can run the following command to
see what markdown events are being emitted and the formatting that we're producing

```
MARKDOWN_FMT_LOG=markdown_fmt::formatter=debug \
cargo run -F build-binary --bin markdown-fmt -- --echo --idempotent --stdout  path/to/crash/file
```
