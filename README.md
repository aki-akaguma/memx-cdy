# memx-cdy
c interface of memx: the fast memory functions like a libc memcmp(), memchr(), memmem(), memcpy(), memset()

## Features

* Faster os libc and musl libc
* Replace dynamic os libc using LD_PRELOAD

## Quick install

1. you can install this into cargo lib path:

```text
cargo build --release
mkdir -p $HOME/.cargo/lib
cp -a target/release/libmemx_cdy.so target/release/libmemx_cdy.a $HOME/.cargo/lib
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.

# Usage
Easy to use, LD_PRELOAD.

1. usecase: cargo install
```text
$ LD_PRELOAD=~/.cargo/lib/libmemx_cdy.so command
```

or

```text
$ export LD_PRELOAD=~/.cargo/lib/libmemx_cdy.so
```

2. usecase: debian package

```text
$ LD_PRELOAD=/usr/lib/libmemx_cdy.so.0.1 command
```

or

```text
$ export LD_PRELOAD=/usr/lib/libmemx_cdy.so.0.1
```

# References

- [`memx`](https://crates.io/crates/memx) - rust crate for the fast mem lib

# The memx benchmark results

- [`memchr()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memchr.md) - map to c memchr()
- [`memcmp()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memcmp.md) - map to c memcmp()
- [`memcpy()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memcpy.md) - map to c memcpy()
- [`memeq()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memeq.md) - map to c bcmpr()
- [`memmem()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memmem.md) - map to c memmem()
- [`memset()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memset.md) - map to c memset()
