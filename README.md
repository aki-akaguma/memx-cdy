# memx-cdy

c interface of memx: the fast memory functions like libc memcmp(), memchr(), memmem(), memcpy(), memset()

## Features

* Faster os libc and musl libc
* Replace dynamic os libc using LD_PRELOAD

## Quick install

1. You can install this into cargo lib path:

```
cargo build --release
mkdir -p $HOME/.cargo/lib
cp -a target/release/libmemx_cdy.so target/release/libmemx_cdy.a $HOME/.cargo/lib
```

2. You can build debian package:

```
cargo deb
```

and install **.deb** into your local repository of debian package.

## Usage
Easy to use, LD_PRELOAD.

1. usecase: cargo install

```
$ LD_PRELOAD=~/.cargo/lib/libmemx_cdy.so command
```

or

```
$ export LD_PRELOAD=~/.cargo/lib/libmemx_cdy.so
```

2. usecase: debian package

```
$ LD_PRELOAD=/usr/lib/libmemx_cdy.so.0.1 command
```

or

```
$ export LD_PRELOAD=/usr/lib/libmemx_cdy.so.0.1
```

## 2-step of using on your rust source code

1. add to dependences of Cargo.toml:

```
[dependencies]
memx-cdy = "0.1"
```

2. call `memx_init()` in main function

```rust
fn main() {
    memx_cdy::memx_init();
    //
    // follow your code
}
```


# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/memx-cdy/blob/main/CHANGELOG.md)

# References

- [`memx`](https://crates.io/crates/memx) - rust crate for the fast mem lib

# The memx benchmark results

- [`memchr()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memchr.md) - map to c memchr()
- [`memcmp()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memcmp.md) - map to c memcmp()
- [`memcpy()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memcpy.md) - map to c memcpy()
- [`memeq()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memeq.md) - map to c bcmpr()
- [`memmem()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memmem.md) - map to c memmem()
- [`memset()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memset.md) - map to c memset()

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.
