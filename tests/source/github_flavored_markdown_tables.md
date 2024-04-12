<!-- Some examples of regular and nested  tables -->
<!-- Examples from https://github.com/rust-lang/rustfmt/blob/728939191e4218e2c1296c7ba3eb36590cbcb9bd/tests/target/issue-4210.rs -->

| table | heading is longer than content |
| ----- | ------------------------------ |
| val   | x                              |

* | table | heading is longer than content (in list) |
  | ----- | ---------------------------------------- |
  | val   | x                                        |

| table    | heading is shorter than content                                                                                             |
| -------- | --------------------------------------------------------------------------------------------------------------------------- |
| long val | Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. |


* | table    | heading is shorter than content                                                                                             |
  | -------- | --------------------------------------------------------------------------------------------------------------------------- |
  | long val | Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. |


> * | table | heading is longer than content (in list) |
>   | ----- | ---------------------------------------- |
>   | val   | x                                        |


> * | table    | heading is shorter than content                                                                                             |
>   | -------- | --------------------------------------------------------------------------------------------------------------------------- |
>   | long val | Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. |


<!-- Some examples with unicode chars with different widths -->

| column 1 | column 2 | column 3|
| :---: | :--- | ---: |
| values for column 1 | values for column 2 | values for column 3 |
| 😁😁 | 🎉🎉🎉 | 😁 :^) :^)|


<!-- Example from https://github.com/rust-lang/rust/blob/b14d8b2ef20c64c1002e2c6c724025c3d0846b91/compiler/rustc_codegen_cranelift/Readme.md?plain=1 -->

|OS \ architecture|x86\_64|AArch64|Riscv64|s390x (System-Z)|
|---|---|---|---|---|
|Linux|✅|✅|✅[^no-rustup]|✅[^no-rustup]|
|FreeBSD|✅[^no-rustup]|❓|❓|❓|
|AIX|❌[^xcoff]|N/A|N/A|❌[^xcoff]|
|Other unixes|❓|❓|❓|❓|
|macOS|✅|❌[^apple-silicon]|N/A|N/A|
|Windows|✅[^no-rustup]|❌|N/A|N/A|


<!-- More examples from https://www.markdownguide.org/extended-syntax -->

| Syntax    | Description |
| --------- | ----------- |
| Header    | Title       |
| Paragraph | Text        |

| Syntax    | Description | Test Text   |
| :-------- | :---------: | ----------: |
| Header    | Title       | Here's this |
| Paragraph | Text        | And more    |

<!-- Example from https://github.com/rust-lang/rust/blob/b14d8b2ef20c64c1002e2c6c724025c3d0846b91/src/doc/rustdoc/src/how-to-write-documentation.md?plain=1#L208 -->

| ASCII sequence | Unicode |
|----------------|---------|
| `--`           | –       |
| `---`          | —       |
| `...`          | …       |
| `"`            | “ or ”, depending on context |
| `'`            | ‘ or ’, depending on context |


<!-- Example from https://github.com/rust-lang/rust/blob/b14d8b2ef20c64c1002e2c6c724025c3d0846b91/src/doc/rustc/src/platform-support/netbsd.md?plain=1#L15 -->

|          Target name           | NetBSD Platform |
|--------------------------------|-----------------|
| `x86_64-unknown-netbsd`        | [amd64 / x86_64 systems](https://wiki.netbsd.org/ports/amd64/) |
| `armv7-unknown-netbsd-eabihf`  | [32-bit ARMv7 systems with hard-float](https://wiki.netbsd.org/ports/evbarm/) |
| `armv6-unknown-netbsd-eabihf`  | [32-bit ARMv6 systems with hard-float](https://wiki.netbsd.org/ports/evbarm/) |
| `aarch64-unknown-netbsd`       | [64-bit ARM systems, little-endian](https://wiki.netbsd.org/ports/evbarm/) |
| `aarch64_be-unknown-netbsd`    | [64-bit ARM systems, big-endian](https://wiki.netbsd.org/ports/evbarm/) |
| `i586-unknown-netbsd`          | [32-bit i386, restricted to Pentium](https://wiki.netbsd.org/ports/i386/) |
| `i686-unknown-netbsd`          | [32-bit i386 with SSE](https://wiki.netbsd.org/ports/i386/) |
| `mipsel-unknown-netbsd`        | [32-bit mips, requires mips32 cpu support](https://wiki.netbsd.org/ports/evbmips/) |
| `powerpc-unknown-netbsd`       | [Various 32-bit PowerPC systems, e.g. MacPPC](https://wiki.netbsd.org/ports/macppc/) |
| `riscv64gc-unknown-netbsd`     | [64-bit RISC-V](https://wiki.netbsd.org/ports/riscv/) |
| `sparc64-unknown-netbsd`       | [Sun UltraSPARC systems](https://wiki.netbsd.org/ports/sparc64/) |


<!-- Example from https://github.com/rust-lang/rust/blob/b14d8b2ef20c64c1002e2c6c724025c3d0846b91/src/doc/rustc/src/platform-support/nto-qnx.md?plain=1#L24 -->

| QNX Neutrino Version | Target Architecture | Full support | `no_std` support |
|----------------------|---------------------|:------------:|:----------------:|
| 7.1 | AArch64 | ✓ | ✓ |
| 7.1 | x86_64  | ✓ | ✓ |
| 7.0 | x86     |   | ✓ |

<!-- Example from https://github.com/rust-lang/rust/blob/b14d8b2ef20c64c1002e2c6c724025c3d0846b91/src/doc/rustc/src/platform-support.md?plain=1#L34 -->

target | notes
-------|-------
`aarch64-unknown-linux-gnu` | ARM64 Linux (kernel 4.1, glibc 2.17+)
`i686-pc-windows-gnu` | 32-bit MinGW (Windows 10+) [^x86_32-floats-return-ABI]
`i686-pc-windows-msvc` | 32-bit MSVC (Windows 10+) [^x86_32-floats-return-ABI]
`i686-unknown-linux-gnu` | 32-bit Linux (kernel 3.2+, glibc 2.17+) [^x86_32-floats-return-ABI]
`x86_64-apple-darwin` | 64-bit macOS (10.12+, Sierra+)
`x86_64-pc-windows-gnu` | 64-bit MinGW (Windows 10+)
`x86_64-pc-windows-msvc` | 64-bit MSVC (Windows 10+)
`x86_64-unknown-linux-gnu` | 64-bit Linux (kernel 3.2+, glibc 2.17+)

<!-- Example from https://github.com/rust-lang/rust/blob/b14d8b2ef20c64c1002e2c6c724025c3d0846b91/compiler/rustc_data_structures/src/sync.rs#L17 -->

| Type                    | Serial version      | Parallel version                |
| ----------------------- | ------------------- | ------------------------------- |
| `Lrc<T>`                | `rc::Rc<T>`         | `sync::Arc<T>`                  |
|` Weak<T>`               | `rc::Weak<T>`       | `sync::Weak<T>`                 |
| `LRef<'a, T>` [^2]      | `&'a mut T`         | `&'a T`                         |
|                         |                     |                                 |
| `AtomicBool`            | `Cell<bool>`        | `atomic::AtomicBool`            |
| `AtomicU32`             | `Cell<u32>`         | `atomic::AtomicU32`             |
| `AtomicU64`             | `Cell<u64>`         | `atomic::AtomicU64`             |
| `AtomicUsize`           | `Cell<usize>`       | `atomic::AtomicUsize`           |
|                         |                     |                                 |
| `Lock<T>`               | `RefCell<T>`        | `RefCell<T>` or                 |
|                         |                     | `parking_lot::Mutex<T>`         |
| `RwLock<T>`             | `RefCell<T>`        | `parking_lot::RwLock<T>`        |
| `MTLock<T>`        [^1] | `T`                 | `Lock<T>`                       |
| `MTLockRef<'a, T>` [^2] | `&'a mut MTLock<T>` | `&'a MTLock<T>`                 |
|                         |                     |                                 |
| `ParallelIterator`      | `Iterator`          | `rayon::iter::ParallelIterator` |


<!-- Example from https://github.com/tokio-rs/axum/blob/50c035c20b7bf7987b9b9b126574852318e92e2c/axum/src/lib.rs#L332 -->

Name | Description | Default?
---|---|---
`http1` | Enables hyper's `http1` feature | Yes
`http2` | Enables hyper's `http2` feature | No
`json` | Enables the [`Json`] type and some similar convenience functionality | Yes
`macros` | Enables optional utility macros | No
`matched-path` | Enables capturing of every request's router path and the [`MatchedPath`] extractor | Yes
`multipart` | Enables parsing `multipart/form-data` requests with [`Multipart`] | No
`original-uri` | Enables capturing of every request's original URI and the [`OriginalUri`] extractor | Yes
`tokio` | Enables `tokio` as a dependency and `axum::serve`, `SSE` and `extract::connect_info` types. | Yes
`tower-log` | Enables `tower`'s `log` feature | Yes
`tracing` | Log rejections from built-in extractors | Yes
`ws` | Enables WebSockets support via [`extract::ws`] | No
`form` | Enables the `Form` extractor | Yes
`query` | Enables the `Query` extractor | Yes
