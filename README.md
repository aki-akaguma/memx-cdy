# memx-cdy
c interface of memory functions like a libc memcmp(), memchr(), memmem(), memcpy(), memset()

## Features

* Rewriting with rust lang.
* Faster os libc.

## Quick install

1. you can install this into cargo bin path:

```text
cargo install memx-cdy
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
$ LD_PRELOAD=~/.cargo/bin/libmemx_cdy command
```

or

```text
$ export LD_PRELOAD=~/.cargo/bin/libmemx_cdy
```

2. usecase: debian package

```text
$ LD_PRELOAD=/usr/lib/libmemx_cdy.0.1 command
```

or

```text
$ export LD_PRELOAD=/usr/lib/libmemx_cdy.0.1
```

# References

- [`memx`](https://crates.io/crates/memx)
